mod bundle_data;
use std::{fs::File, path::PathBuf, fs::write, io::Read};
use caveman::proto::Caveman::{CavemanBundle, CavemanAsset};

use hex::ToHex;
use protobuf::Message;

use clap::{arg, Parser, ValueHint};
use sha2::{Sha256, Digest};
use zstd::{Encoder, encode_all};

use crate::bundle_data::BundleManifest;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Hii")]
struct Arguments {
    #[arg(value_name = "bundle", value_hint = ValueHint::DirPath, help = "Bundle manifest to read")]
    manifest: std::path::PathBuf,
    #[arg(short = 'o', long = "out", value_name = "output", value_hint = ValueHint::DirPath, help = "The name of the output bundle")]
    out: std::path::PathBuf,
}

fn main() {
    let args = Arguments::parse();

    println!("Reading {}", args.manifest.display());

    let manifest_file = File::open(args.manifest.clone()).unwrap();

    let manifest: BundleManifest = serde_json::from_reader(manifest_file).unwrap();

    let mut bundle = CavemanBundle::new();

    bundle.title = manifest.title;
    bundle.description = manifest.description;
    bundle.provides = manifest.provides;

    let basepath: PathBuf = args.manifest.parent().unwrap().to_path_buf();

    let mut bundled_assets: Vec<CavemanAsset> = Vec::new();

    for asset in manifest.assets.iter() {
        let mut bundled_asset = CavemanAsset::new();

        bundled_asset.token = asset.token.clone();
        bundled_asset.type_ = asset.mime_type.clone();
        bundled_asset.compressed = asset.compress;

        let mut asset_file = File::open(basepath.join(asset.path.clone())).unwrap();

        let mut hasher = Sha256::new();

        let mut asset_bytes: Vec<u8> = Vec::new();
        asset_file.read_to_end(&mut asset_bytes).unwrap();

        hasher.update(asset_bytes.clone());

        bundled_asset.sum = hasher.finalize().to_vec();

        if bundled_asset.compressed {
            bundled_asset.data = encode_all(asset_bytes.as_slice(), 0).unwrap()
        } else {
            bundled_asset.data = asset_bytes;
        }
        bundled_assets.push(bundled_asset);
        println!("Processed {} asset with token: {}", if asset.compress { "compressed" } else { "uncompressed" }, asset.token);
    }

    bundle.assets = bundled_assets;

    let out_bytes: Vec<u8> = bundle.write_to_bytes().unwrap();

    write(args.out, out_bytes).unwrap();
}
