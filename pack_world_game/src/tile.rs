use crate::{
    drop_table::*,
    error::*,
    grid::*,
    save_file::load,
    state::{assets::*, *},
    update_signal::*,
    world::*,
};
use gengar_engine::{
    color::*,
    matricies::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    transform::*,
    vectors::*,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

pub mod harvest_timer;
pub mod tile_definition;
pub mod tile_instance;
pub mod tile_methods;
// pub mod tile_thumbnail;
pub mod tile_type;
pub mod tiles;

pub use {tile_definition::*, tile_instance::*, tile_methods::*, tile_type::*};

pub fn draw_tile_grid_pos(
    tile_type: TileType,
    rotation: f64,
    pos: &GridPos,
    render_pack: &mut RenderPack,
    assets: &Assets,
) {
    let world_pos = grid_to_world(pos);
    draw_tile_world_pos(tile_type, rotation, &world_pos, render_pack, assets);
}

pub fn draw_tile_world_pos(
    tile_type: TileType,
    rotation: f64,
    pos: &VecThreeFloat,
    render_pack: &mut RenderPack,
    assets: &Assets,
) {
    let tile_asset_id = tile_type.to_string_id();

    let mut trans = Transform::new();
    trans.local_position = *pos;
    trans.update_global_matrix(&M44::new_identity());

    render_pack.commands.push(RenderCommand::new_model(
        &trans,
        assets.asset_library.get_model(&tile_asset_id),
        assets
            .tile_materials
            .get(&tile_type)
            .expect(&format!("Missing tile material {:?}", tile_type)),
    ));
}
