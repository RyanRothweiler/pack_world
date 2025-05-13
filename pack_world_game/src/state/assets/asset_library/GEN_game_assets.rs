
use gengar_engine::{
    model::*,
    render::{image::*, RenderApi, *},
};
use crate::{AssetLibrary, include_model, include_texture};
use std::collections::HashMap;

pub fn load_game_assets(al: &mut AssetLibrary, render_api: &impl RenderApi) {
include_texture!(al, "tile_grass_base_color", "../../../../resources/models/tile_grass/BaseColor.png", render_api);
include_texture!(al, "tile_grass_ao", "../../../../resources/models/tile_grass/AO.png", render_api);
include_texture!(al, "tile_grass_normal", "../../../../resources/models/tile_grass/Normal.png", render_api);
include_texture!(al, "tile_grass_roughness", "../../../../resources/models/tile_grass/Roughness.png", render_api);
include_texture!(al, "tile_grass_metallic", "../../../../resources/models/tile_grass/Metallic.png", render_api);
include_model!(al, "tile_grass", "../../../../resources/models/tile_grass/tile_grass.obj", render_api);
include_model!(al, "tile_dirt", "../../../../resources/models/first_tile/first_tile.obj", render_api);
include_model!(al, "tile_water", "../../../../resources/models/tile_water/tile_water.obj", render_api);
}
