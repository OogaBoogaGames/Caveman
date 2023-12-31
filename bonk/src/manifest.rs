use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameBundleManifest {
    pub title: String,
    pub description: String,
    pub assets: Vec<AssetBundleDescriptor>,
    pub build: BuildOption,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildOption {
    NoBuild { file: String },
    BunBuild { file: Option<String> },
    CustomBuild { command: String, file: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AssetBundleDescriptor {
    File(String),               // path
    Id(String, Option<String>), // id and optional mirror
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundleManifest {
    pub title: String,
    pub description: String,
    pub assets: Vec<AssetDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetDescriptor {
    pub token: String,
    pub compress: bool,
    pub mime_type: String,
    pub path: String,
}
