pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/protobuf/mod.rs"));
}

pub fn info() -> String {
    format!("OogaBoogaGames Caveman Library (libcaveman) version {}\n\
    Copyright (C) 2023 OogaBoogaGames\n\
    Licensed under GNU GPL-3.0-or-later\n\
    Compiled by rustc version {}\n\
    Source code is available at:\n\
    <https://github.com/OogaBoogaGames/Caveman>", env!("CARGO_PKG_VERSION"), rustc_version_runtime::version())
}
