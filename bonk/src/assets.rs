pub use std::path::PathBuf;
use std::{
    fs::{self, File},
    io::Read,
};

use caveman::proto::Caveman::{CavemanAsset, CavemanBundle};
use protobuf::Message;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use scorched::{logf, LogData, LogImportance};
use sha2::{Digest, Sha256};
use zstd::encode_all;

use crate::manifest::AssetBundleManifest;
pub async fn bundle_assets(manifest_path: &PathBuf, out_path: &PathBuf) {
    logf!(Info, "Reading {}", &manifest_path.display());

    let manifest: AssetBundleManifest =
        serde_json::from_reader(File::open(&manifest_path).unwrap()).unwrap();

    let mut bundle = CavemanBundle::new();
    bundle.title = manifest.title;
    bundle.description = manifest.description;

    let bundled_assets: Vec<CavemanAsset> = manifest
        .assets
        .par_iter()
        .map(|asset| {
            let mut bundled_asset = CavemanAsset::new();
            bundled_asset.token = asset.token.clone();
            bundled_asset.type_ = asset.mime_type.clone();
            bundled_asset.compressed = asset.compress;

            let mut asset_file = File::open(
                manifest_path
                    .parent()
                    .unwrap()
                    .to_path_buf()
                    .join(&asset.path),
            )
            .unwrap();
            let mut asset_bytes: Vec<u8> = Vec::new();
            asset_file.read_to_end(&mut asset_bytes).unwrap();

            let mut hasher = Sha256::new();

            hasher.update(&asset_bytes);
            bundled_asset.sum = hasher.finalize().to_vec();

            bundled_asset.data = match asset.compress {
                true => encode_all(asset_bytes.as_slice(), 0).unwrap(),
                false => asset_bytes,
            };

            println!(
                "Processed {} asset with token: {}",
                match asset.compress {
                    true => "compressed",
                    false => "uncompressed",
                },
                asset.token
            );

            bundled_asset
        })
        .collect();

    bundle.assets = bundled_assets;

    fs::write(&out_path, bundle.write_to_bytes().unwrap()).unwrap();

    logf!(
        Info,
        "Wrote {} assets to bundle at {}",
        bundle.assets.len(),
        out_path.display()
    );
}
