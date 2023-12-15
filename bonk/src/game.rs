use core::panic;
pub use std::path::PathBuf;
use std::{
    fs::{self, File},
    io::Read,
    process::Command,
};

use caveman::proto::Caveman::{
    bundle_identifier::Identifier, BundleIdentifier, CavemanAsset, CavemanBundle, CavemanGameBundle,
};
use futures::future::join_all;
use protobuf::Message;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use scorched::{logf, LogData, LogImportance};
use sha2::{Digest, Sha256};
use zstd::encode_all;

use crate::manifest::{
    AssetBundleDescriptor, AssetBundleManifest, BuildOption, GameBundleManifest,
};
pub async fn bundle_game(manifest_path: &PathBuf, standalone: &bool, out_path: &PathBuf) {
    logf!(Info, "Reading {}", &manifest_path.display());

    let manifest: GameBundleManifest =
        serde_json::from_reader(File::open(&manifest_path).unwrap()).unwrap();

    let mut bundle = CavemanGameBundle::new();
    bundle.title = manifest.title;
    bundle.description = manifest.description;

    match standalone {
        true => {
            let bundles: Vec<BundleIdentifier> =
                join_all(manifest.assets.iter().map(|descriptor| async move {
                    match descriptor {
                        AssetBundleDescriptor::File(path) => {
                            let data = fs::read(path).unwrap();

                            let mut identifier = BundleIdentifier::new();
                            identifier.set_bundle(CavemanBundle::parse_from_bytes(&data).unwrap());
                            identifier
                        }
                        AssetBundleDescriptor::Id(id, mirror) => {
                            let mirror = mirror
                                .clone()
                                .unwrap_or("https://api.oogabooga.games/assets".into());
                            let url = format!("{}/bundle/{}", mirror, id);
                            let data = reqwest::get(url).await.unwrap().bytes().await.unwrap();

                            let mut identifier = BundleIdentifier::new();
                            identifier.set_bundle(CavemanBundle::parse_from_bytes(&data).unwrap());
                            identifier
                        }
                    }
                }))
                .await;
            bundle.bundles = bundles;
        }
        false => {
            let bundles: Vec<BundleIdentifier> =
                join_all(manifest.assets.iter().filter(|descriptor| {
                    match descriptor {
                        AssetBundleDescriptor::File(path) => {
                            logf!(Error, "Cannot bundle file assets in non-standalone mode. The file at \"{}\" will not be included in the bundle.", path);
                            false
                        }
                        AssetBundleDescriptor::Id(_, _) => {
                            true
                        }
                    }
                }).map(|descriptor| async move {
                    let id = match descriptor {
                        AssetBundleDescriptor::Id(id, _) => id,
                        _ =>unreachable!(),
                    };

                    let mut identifier = BundleIdentifier::new();
                    identifier.set_bundle_id(id.to_string());
                    identifier
                }))
                .await;
            bundle.bundles = bundles;
        }
    }

    let bytes = match manifest.build {
        BuildOption::NoBuild { file } => {
            let mut file = File::open(file).unwrap();
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).unwrap();
            bytes
        }
        BuildOption::BunBuild { file } => {
            Command::new("bun")
                .current_dir(manifest_path.parent().unwrap())
                .arg("run")
                .arg("build")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            let path = file.unwrap_or(
                manifest_path
                    .parent()
                    .unwrap()
                    .join("dist/index.js")
                    .to_str()
                    .unwrap()
                    .into(),
            );
            let mut file = File::open(path).unwrap();
            let mut bytes = Vec::new();

            file.read_to_end(&mut bytes).unwrap();
            bytes
        }
        BuildOption::CustomBuild { command, file } => {
            Command::new("sh")
                .current_dir(manifest_path.parent().unwrap())
                .arg("-c")
                .arg(command)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            let mut file = File::open(file).unwrap();
            let mut bytes = Vec::new();

            file.read_to_end(&mut bytes).unwrap();
            bytes
        }
    };

    bundle.runtime = bytes;

    fs::write(&out_path, bundle.write_to_bytes().unwrap()).unwrap();

    logf!(
        Info,
        "Wrote {} bundles to bundle at {}",
        bundle.bundles.len(),
        out_path.display()
    );
}
