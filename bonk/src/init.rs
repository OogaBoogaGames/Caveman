use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::manifest::{AssetBundleDescriptor, BuildOption, GameBundleManifest};

pub fn init_game(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir(path).unwrap();
    }

    let manifest_path = path.join("manifest.json");
    let mut manifest_file = File::create(&manifest_path).unwrap();
    let manifest_data = GameBundleManifest {
        title: "My Game".into(),
        description: "A game made with Bonk".into(),
        assets: vec![AssetBundleDescriptor::Id(
            "games.oogabooga.sprites".into(),
            Some("https://api.oogabooga.games/assets".into()),
        )],
        build: BuildOption::BunBuild { file: None },
    };
    manifest_file
        .write_all(
            serde_json::to_string_pretty(&manifest_data)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
}
