#![allow(dead_code, unused_variables)]

use std::{fs::File, io::Write};

fn main() {
    let manifest_file =
        std::fs::read_to_string("resources/manifest.txt").expect("Require asset manifest file.");
    let mut output_file = File::create("src/state/assets/asset_library/GEN_game_assets.rs")
        .expect("Couldn't open output file");

    let imp = r#"
use gengar_engine::{
    model::*,
    render::{image::*, RenderApi, *},
};
use crate::{AssetLibrary, include_model, include_texture};
use std::collections::HashMap;
"#;
    writeln!(output_file, "{}", imp).unwrap();

    writeln!(
        output_file,
        "pub fn load_game_assets(al: &mut AssetLibrary, render_api: &impl RenderApi) {{"
    )
    .unwrap();

    let lines: Vec<String> = manifest_file.lines().map(|line| line.to_string()).collect();
    let mut i = 0;
    loop {
        if !lines[i].trim().is_empty() {
            let asset_type: &str = lines[i].trim();
            let id: &str = lines[i + 1].trim();
            let path: &str = lines[i + 2].trim();

            i += 3;

            match asset_type {
                "texture" => {
                    write!(output_file, "include_texture!(al, ").unwrap();
                    write!(output_file, "\"{}\", ", id).unwrap();
                    write!(output_file, "\"../../../../resources/{}\", ", path).unwrap();
                    writeln!(output_file, "render_api);").unwrap();
                }
                _ => panic!("Unknown asset type {}", asset_type),
            }
        }
        i += 1;

        if i >= lines.len() {
            break;
        }
    }
    writeln!(output_file, "}}").unwrap();
}
