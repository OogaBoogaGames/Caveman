mod manifest;

use caveman::proto::Caveman::{CavemanAsset, CavemanBundle};
use clap::{arg, Parser, ValueHint};
use protobuf::Message;
use rayon::prelude::*;
use scorched::*;
use sha2::{Digest, Sha256};
use std::{
    fs::{write, File},
    io::Read,
};
use zstd::encode_all;

use crate::manifest::BundleManifest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A tool for bundling assets.")]
struct Arguments {
    #[arg(value_name = "bundle", value_hint = ValueHint::DirPath, help = "Bundle manifest to read")]
    manifest: std::path::PathBuf,
    #[arg(short = 'o', long = "out", value_name = "output", value_hint = ValueHint::DirPath, help = "The name of the output bundle")]
    out: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    logf!(Info, "Reading {}", args.manifest.display());

    let manifest: BundleManifest =
        serde_json::from_reader(File::open(args.manifest.clone()).unwrap()).unwrap();

    let mut bundle = CavemanBundle::new();
    bundle.title = manifest.title;
    bundle.description = manifest.description;
    bundle.provides = manifest.provides;

    let bundled_assets: Vec<CavemanAsset> = manifest
        .assets
        .par_iter()
        .map(|asset| {
            let mut bundled_asset = CavemanAsset::new();
            bundled_asset.token = asset.token.clone();
            bundled_asset.type_ = asset.mime_type.clone();
            bundled_asset.compressed = asset.compress;

            let mut asset_file = File::open(
                args.manifest
                    .parent()
                    .unwrap()
                    .to_path_buf()
                    .join(asset.path.clone()),
            )
            .unwrap();
            let mut asset_bytes: Vec<u8> = Vec::new();
            asset_file.read_to_end(&mut asset_bytes).unwrap();

            let mut hasher = Sha256::new();

            hasher.update(asset_bytes.clone());
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

    write(args.out.clone(), bundle.write_to_bytes().unwrap()).unwrap();

    logf!(
        Info,
        "Wrote {} assets to bundle at {}",
        bundle.assets.len(),
        args.out.display()
    );
}
