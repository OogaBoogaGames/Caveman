use js_sys::{JsString, Uint8Array};
use protobuf::Message;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use caveman::info;
use caveman::proto::Caveman::{CavemanBundle, CavemanAsset};
use web_sys::{Blob, Request, RequestInit, RequestMode, Response, BlobPropertyBag};
use futures::executor;
use zstd::decode_all;

#[derive(Debug)]
#[wasm_bindgen]
pub struct FlintBundle {
    url: String,
    caveman_bundle: Option<CavemanBundle>
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn libcaveman_info() -> JsString {
    JsString::from(info())
}

#[wasm_bindgen]
impl FlintBundle {
    pub async fn get_asset(&self, token: String) -> Result<Blob, JsError> {
        return match self.caveman_bundle.clone() {
            Some(bundle) => match bundle.assets.iter().find(|element| element.token == token) {
                Some(asset) => { 
                    let mut options = BlobPropertyBag::new();
                    options.type_(&asset.type_);
                    match asset.compressed {
                        true => match decode_all(asset.data.as_slice()) {
                            Ok(data) => match Blob::new_with_blob_sequence_and_options(&js_sys::Array::of1(&Uint8Array::from(data.as_slice())), &options) {
                                Ok(blob) => Ok(blob),
                                Err(e) => Err(JsError::new(format!("Could not create blob: {:?}", e).as_str())),
                            },
                            Err(e) => Err(JsError::new(format!("Could not decompress asset: {}", e).as_str())),
                        }
                        false => match Blob::new_with_blob_sequence_and_options(&js_sys::Array::of1(&Uint8Array::from(asset.data.as_slice())), &options) {
                            Ok(blob) => Ok(blob),
                            Err(e) => Err(JsError::new(format!("Could not create blob: {:?}", e).as_str())),
                        }
                    }
                }
                None => Err(JsError::new("Bundle does not satisfy token."))
            },
            None => Err(JsError::new("Bundle has not been loaded yet."))
        };
    }
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> FlintBundle {
        FlintBundle { url, caveman_bundle: None }
    }
    pub async fn load(&mut self) {
        let bytes = get_bundle_bytes(self.url.clone()).await;

        self.caveman_bundle = Some(CavemanBundle::parse_from_bytes(&bytes).unwrap());
    }
}

async fn get_bundle_bytes(url: String) -> Vec<u8> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request: Request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window: web_sys::Window = web_sys::window().unwrap();
    let resp_value: JsValue = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let resp_fulfilled: JsValue = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();

    let array: Uint8Array = Uint8Array::new(&resp_fulfilled);
    
    array.to_vec()
}