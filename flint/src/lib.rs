use caveman::{info, proto::Caveman::CavemanBundle};
use js_sys::{JsString, Uint8Array};
use protobuf::Message;
use scorched::*;
use url::Url;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, Request, RequestInit, RequestMode, Response};
use zstd::decode_all;

#[derive(Debug)]
#[wasm_bindgen]
pub struct FlintBundle {
    package_id: String,
    caveman_bundle: Option<CavemanBundle>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn libcaveman_info() -> JsString {
    JsString::from(info())
}

#[wasm_bindgen]
impl FlintBundle {
    pub async fn get_asset(&self, token: String) -> Result<Blob, JsError> {
        return match &self.caveman_bundle {
            Some(bundle) => match bundle.assets.iter().find(|element| element.token == token) {
                Some(asset) => {
                    let mut options = BlobPropertyBag::new();
                    options.type_(&asset.type_);
                    match asset.compressed {
                        true => match decode_all(asset.data.as_slice()) {
                            Ok(data) => match Blob::new_with_blob_sequence_and_options(
                                &js_sys::Array::of1(&Uint8Array::from(data.as_slice())),
                                &options,
                            ) {
                                Ok(blob) => Ok(blob),
                                Err(e) => {
                                    log_this(LogData {
                                        importance: LogImportance::Error,
                                        message: format!("Could not create blob: {:?}", e),
                                    })
                                    .await;

                                    Err(JsError::new(
                                        format!("Could not create blob: {:?}", e).as_str(),
                                    ))
                                }
                            },
                            Err(e) => {
                                log_this(LogData {
                                    importance: LogImportance::Error,
                                    message: format!("Could not decompress asset: {}", e),
                                })
                                .await;

                                Err(JsError::new(
                                    format!("Could not decompress asset: {}", e).as_str(),
                                ))
                            }
                        },
                        false => match Blob::new_with_blob_sequence_and_options(
                            &js_sys::Array::of1(&Uint8Array::from(asset.data.as_slice())),
                            &options,
                        ) {
                            Ok(blob) => Ok(blob),
                            Err(e) => {
                                log_this(LogData {
                                    importance: LogImportance::Error,
                                    message: format!("Could not create blob: {:?}", e),
                                })
                                .await;

                                Err(JsError::new(
                                    format!("Could not create blob: {:?}", e).as_str(),
                                ))
                            }
                        },
                    }
                }
                None => {
                    log_this(LogData {
                        importance: LogImportance::Error,
                        message: format!("Bundle does not satisfy token."),
                    })
                    .await;

                    Err(JsError::new("Bundle does not satisfy token."))
                }
            },
            None => {
                log_this(LogData {
                    importance: LogImportance::Error,
                    message: format!("Bundle has not been loaded yet."),
                })
                .await;

                Err(JsError::new("Bundle has not been loaded yet."))
            }
        };
    }
    #[wasm_bindgen(constructor)]
    pub fn new(package_id: String) -> FlintBundle {
        FlintBundle {
            package_id,
            caveman_bundle: None,
        }
    }
    pub async fn load(&mut self, mirror: String) {
        match Url::parse(&mirror) {
            Ok(url) => match url.join(&self.package_id) {
                Ok(url) => {
                    let bytes = fetch_bundle_bytes(url).await;
                    self.caveman_bundle = Some(CavemanBundle::parse_from_bytes(&bytes).unwrap());
                }
                Err(err) => {
                    log_this(LogData {
                        importance: LogImportance::Error,
                        message: format!("Failed to finalize Url: {}", err),
                    })
                    .await;

                    error(&format!("Failed to finalize Url: {}", err))
                }
            },
            Err(err) => {
                log_this(LogData {
                    importance: LogImportance::Error,
                    message: format!("Failed to parse Url: {}", err),
                })
                .await;

                error(&format!("Failed to parse Url: {}", err))
            }
        }
    }
}

async fn fetch_bundle_bytes(url: Url) -> Vec<u8> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request: Request = Request::new_with_str_and_init(url.as_str(), &opts).unwrap();

    let window: web_sys::Window = web_sys::window().unwrap();
    let resp_value: JsValue = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let resp_fulfilled: JsValue = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();

    let array: Uint8Array = Uint8Array::new(&resp_fulfilled);

    array.to_vec()
}
