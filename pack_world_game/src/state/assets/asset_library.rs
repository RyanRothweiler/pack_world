use gengar_engine::{
    model::*,
    render::{image::*, RenderApi, *},
};
use std::collections::HashMap;

#[allow(non_snake_case)]
pub mod GEN_game_assets;

pub struct AssetLibrary {
    models: HashMap<String, Model>,
    textures: HashMap<String, Image>,
}

impl AssetLibrary {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn get_model(&self, id: &str) -> &Model {
        self.models
            .get(id.into())
            .expect(&format!("Missing asset id {}", id))
    }

    pub fn get_texture(&self, id: &str) -> &Image {
        self.textures
            .get(id.into())
            .expect(&format!("Missing asset id {}", id))
    }
}

/*
macro_rules! asset_include {
    ($al:expr, $id:expr, $data_path:expr) => {
        check_unique!($al, $id);
        $al.library
            .insert($id.into(), include_bytes!($data_path).into());
    };
}
*/

#[macro_export]
macro_rules! include_model {
    ($al:expr, $id:expr, $data_path:expr, $render_api:expr) => {
        let model = Model::load_upload(include_str!($data_path), $render_api).unwrap();
        let val = $al.models.insert($id.into(), model);

        if val.is_some() {
            panic!("Asset id '{}' is already in use.", $id);
        }
    };
}

#[macro_export]
macro_rules! include_texture {
    ($al:expr, $id:expr, $data_path:expr, $render_api:expr) => {
        let image = load_image_cursor(include_bytes!($data_path), $render_api).unwrap();
        let val = $al.textures.insert($id.into(), image);

        if val.is_some() {
            panic!("Asset id '{}' is already in use.", $id);
        }
    };
}

pub fn load_game_assets(al: &mut AssetLibrary, render_api: &impl RenderApi) {
    GEN_game_assets::load_game_assets(al, render_api);

    include_model!(
        al,
        "tile_grass",
        "../../../resources/models/tile_grass/tile_grass.obj",
        render_api
    );

    include_model!(
        al,
        "tile_dirt",
        "../../../resources/models/first_tile/first_tile.obj",
        render_api
    );

    include_model!(
        al,
        "tile_water",
        "../../../resources/models/tile_water/tile_water.obj",
        render_api
    );
}
