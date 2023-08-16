mod bundle_data;
use std::fs::File;
use caveman::proto::Caveman::CavemanBundle;

use clap::{arg, Parser, ValueHint};

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

    let manifest_file = File::open(args.manifest).unwrap();

    let manifest: BundleManifest = serde_json::from_reader(manifest_file).unwrap();

    let mut bundle = CavemanBundle::new();

    bundle.title = manifest.title;

    println!("{}", bundle)
}
