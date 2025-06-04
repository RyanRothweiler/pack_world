
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
include_model!(al, "tile_boulder", "../../../../resources/tiles/tile_boulder/tile_boulder.obj", render_api);
include_texture!(al, "tile_boulder_base_color", "../../../../resources/tiles/tile_boulder/BaseColor.png", render_api);
include_texture!(al, "tile_boulder_metallic", "../../../../resources/tiles/tile_boulder/Metallic.png", render_api);
include_texture!(al, "tile_boulder_roughness", "../../../../resources/tiles/tile_boulder/Roughness.png", render_api);
include_texture!(al, "tile_boulder_ao", "../../../../resources/tiles/tile_boulder/AO.png", render_api);
include_texture!(al, "tile_boulder_normal", "../../../../resources/tiles/tile_boulder/Normal.png", render_api);
include_model!(al, "tile_cave", "../../../../resources/tiles/tile_cave/tile_cave.obj", render_api);
include_texture!(al, "tile_cave_base_color", "../../../../resources/tiles/tile_cave/BaseColor.png", render_api);
include_texture!(al, "tile_cave_metallic", "../../../../resources/tiles/tile_cave/Metallic.png", render_api);
include_texture!(al, "tile_cave_roughness", "../../../../resources/tiles/tile_cave/Roughness.png", render_api);
include_texture!(al, "tile_cave_ao", "../../../../resources/tiles/tile_cave/AO.png", render_api);
include_texture!(al, "tile_cave_normal", "../../../../resources/tiles/tile_cave/Normal.png", render_api);
include_model!(al, "tile_tallgrass", "../../../../resources/tiles/tile_tallgrass/tile_tallgrass.obj", render_api);
include_texture!(al, "tile_tallgrass_base_color", "../../../../resources/tiles/tile_tallgrass/BaseColor.png", render_api);
include_texture!(al, "tile_tallgrass_metallic", "../../../../resources/tiles/tile_tallgrass/Metallic.png", render_api);
include_texture!(al, "tile_tallgrass_roughness", "../../../../resources/tiles/tile_tallgrass/Roughness.png", render_api);
include_texture!(al, "tile_tallgrass_ao", "../../../../resources/tiles/tile_tallgrass/AO.png", render_api);
include_texture!(al, "tile_tallgrass_normal", "../../../../resources/tiles/tile_tallgrass/Normal.png", render_api);
include_model!(al, "tile_shrub", "../../../../resources/tiles/tile_shrub/tile_shrub.obj", render_api);
include_texture!(al, "tile_shrub_base_color", "../../../../resources/tiles/tile_shrub/BaseColor.png", render_api);
include_texture!(al, "tile_shrub_metallic", "../../../../resources/tiles/tile_shrub/Metallic.png", render_api);
include_texture!(al, "tile_shrub_roughness", "../../../../resources/tiles/tile_shrub/Roughness.png", render_api);
include_texture!(al, "tile_shrub_ao", "../../../../resources/tiles/tile_shrub/AO.png", render_api);
include_texture!(al, "tile_shrub_normal", "../../../../resources/tiles/tile_shrub/Normal.png", render_api);
include_model!(al, "tile_oaktree", "../../../../resources/tiles/tile_oaktree/tile_oaktree.obj", render_api);
include_texture!(al, "tile_oaktree_base_color", "../../../../resources/tiles/tile_oaktree/BaseColor.png", render_api);
include_texture!(al, "tile_oaktree_metallic", "../../../../resources/tiles/tile_oaktree/Metallic.png", render_api);
include_texture!(al, "tile_oaktree_roughness", "../../../../resources/tiles/tile_oaktree/Roughness.png", render_api);
include_texture!(al, "tile_oaktree_ao", "../../../../resources/tiles/tile_oaktree/AO.png", render_api);
include_texture!(al, "tile_oaktree_normal", "../../../../resources/tiles/tile_oaktree/Normal.png", render_api);
include_model!(al, "tile_mudpit", "../../../../resources/tiles/tile_mudpit/tile_mudpit.obj", render_api);
include_texture!(al, "tile_mudpit_base_color", "../../../../resources/tiles/tile_mudpit/BaseColor.png", render_api);
include_texture!(al, "tile_mudpit_metallic", "../../../../resources/tiles/tile_mudpit/Metallic.png", render_api);
include_texture!(al, "tile_mudpit_roughness", "../../../../resources/tiles/tile_mudpit/Roughness.png", render_api);
include_texture!(al, "tile_mudpit_ao", "../../../../resources/tiles/tile_mudpit/AO.png", render_api);
include_texture!(al, "tile_mudpit_normal", "../../../../resources/tiles/tile_mudpit/Normal.png", render_api);
include_model!(al, "tile_birdnest", "../../../../resources/tiles/tile_birdnest/tile_birdnest.obj", render_api);
include_texture!(al, "tile_birdnest_base_color", "../../../../resources/tiles/tile_birdnest/BaseColor.png", render_api);
include_texture!(al, "tile_birdnest_metallic", "../../../../resources/tiles/tile_birdnest/Metallic.png", render_api);
include_texture!(al, "tile_birdnest_roughness", "../../../../resources/tiles/tile_birdnest/Roughness.png", render_api);
include_texture!(al, "tile_birdnest_ao", "../../../../resources/tiles/tile_birdnest/AO.png", render_api);
include_texture!(al, "tile_birdnest_normal", "../../../../resources/tiles/tile_birdnest/Normal.png", render_api);
include_model!(al, "tile_frog", "../../../../resources/tiles/tile_frog/tile_frog.obj", render_api);
include_texture!(al, "tile_frog_base_color", "../../../../resources/tiles/tile_frog/BaseColor.png", render_api);
include_texture!(al, "tile_frog_metallic", "../../../../resources/tiles/tile_frog/Metallic.png", render_api);
include_texture!(al, "tile_frog_roughness", "../../../../resources/tiles/tile_frog/Roughness.png", render_api);
include_texture!(al, "tile_frog_ao", "../../../../resources/tiles/tile_frog/AO.png", render_api);
include_texture!(al, "tile_frog_normal", "../../../../resources/tiles/tile_frog/Normal.png", render_api);
include_model!(al, "tile_outline", "../../../../resources/models/tile_outline/tile_outline.obj", render_api);
include_model!(al, "model_missing", "../../../../resources/models/model_missing/model_missing.obj", render_api);
}
