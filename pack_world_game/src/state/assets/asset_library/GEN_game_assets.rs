
use gengar_engine::{
    model::*,
    render::{image::*, RenderApi, *},
};
use crate::{AssetLibrary, include_model, include_texture};
use std::collections::HashMap;

pub fn load_game_assets(al: &mut AssetLibrary, render_api: &impl RenderApi) {
include_model!(al, "tile_water", "../../../../resources/tiles/tile_water/tile_water.obj", render_api);
include_texture!(al, "tile_water_base_color", "../../../../resources/tiles/tile_water/BaseColor.png", render_api);
include_texture!(al, "tile_water_metallic", "../../../../resources/tiles/tile_water/Metallic.png", render_api);
include_texture!(al, "tile_water_roughness", "../../../../resources/tiles/tile_water/Roughness.png", render_api);
include_texture!(al, "tile_water_ao", "../../../../resources/tiles/tile_water/AO.png", render_api);
include_texture!(al, "tile_water_normal", "../../../../resources/tiles/tile_water/Normal.png", render_api);
include_model!(al, "tile_grass", "../../../../resources/tiles/tile_grass/tile_grass.obj", render_api);
include_texture!(al, "tile_grass_base_color", "../../../../resources/tiles/tile_grass/BaseColor.png", render_api);
include_texture!(al, "tile_grass_metallic", "../../../../resources/tiles/tile_grass/Metallic.png", render_api);
include_texture!(al, "tile_grass_roughness", "../../../../resources/tiles/tile_grass/Roughness.png", render_api);
include_texture!(al, "tile_grass_ao", "../../../../resources/tiles/tile_grass/AO.png", render_api);
include_texture!(al, "tile_grass_normal", "../../../../resources/tiles/tile_grass/Normal.png", render_api);
include_model!(al, "tile_dirt", "../../../../resources/tiles/tile_dirt/tile_dirt.obj", render_api);
include_texture!(al, "tile_dirt_base_color", "../../../../resources/tiles/tile_dirt/BaseColor.png", render_api);
include_texture!(al, "tile_dirt_metallic", "../../../../resources/tiles/tile_dirt/Metallic.png", render_api);
include_texture!(al, "tile_dirt_roughness", "../../../../resources/tiles/tile_dirt/Roughness.png", render_api);
include_texture!(al, "tile_dirt_ao", "../../../../resources/tiles/tile_dirt/AO.png", render_api);
include_texture!(al, "tile_dirt_normal", "../../../../resources/tiles/tile_dirt/Normal.png", render_api);
include_model!(al, "tile_outline", "../../../../resources/models/tile_outline/tile_outline.obj", render_api);
}
