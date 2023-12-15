use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AssetBundleManifest {
    pub title: String,
    pub description: String,
    pub provides: String,
    pub assets: Vec<AssetDescriptor>,
}

#[derive(Debug, Deserialize)]
pub struct AssetDescriptor {
    pub token: String,
    pub compress: bool,
    pub mime_type: String,
    pub path: String,
}
