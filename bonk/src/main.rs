mod assets;
mod game;
mod manifest;

use clap::{arg, Parser, Subcommand, ValueHint};
use game::bundle_game;

use crate::assets::bundle_assets;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A tool for bundling assets.")]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(author, version, about, long_about = "Bundle an asset pack.")]
    BundleAssets {
        #[arg(value_name = "bundle", value_hint = ValueHint::DirPath, help = "Bundle manifest to read")]
        manifest: std::path::PathBuf,
        #[arg(short = 'o', long = "out", value_name = "output", value_hint = ValueHint::DirPath, help = "The name of the output bundle")]
        out: std::path::PathBuf,
    },
    #[command(author, version, about, long_about = "Bundle a game.")]
    BundleGame {
        #[arg(value_name = "bundle", value_hint = ValueHint::DirPath, help = "Bundle manifest to read")]
        manifest: std::path::PathBuf,
        #[arg(
            short = 'S',
            long = "standalone",
            help = "Build standalone game bundle"
        )]
        standalone: bool,
        #[arg(short = 'o', long = "out", value_name = "output", value_hint = ValueHint::DirPath, help = "The name of the output bundle")]
        out: std::path::PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    match &args.command {
        Commands::BundleAssets { manifest, out } => bundle_assets(manifest, out).await,
        Commands::BundleGame {
            manifest,
            standalone,
            out,
        } => bundle_game(manifest, standalone, out).await,
    }
}
