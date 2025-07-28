use crate::{
    drop_table::*,
    error::*,
    grid::*,
    save_file::load,
    state::{assets::*, *},
    update_signal::*,
    world::*,
};
use elara_engine::{
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

pub mod tile_definition;
pub mod tile_instance;
pub mod tile_methods;
pub mod tile_type;
pub mod tiles;

pub use {tile_definition::*, tile_instance::*, tile_methods::*, tile_type::*};

pub fn draw_tile_grid_pos(
    tile_type: TileType,
    rotation: f64,
    pos: &GridPos,
    can_place: bool,
    render_pack: &mut RenderPack,
    assets: &Assets,
) {
    let world_pos = grid_to_world(pos);
    draw_tile_world_pos(
        tile_type,
        rotation,
        &world_pos,
        can_place,
        render_pack,
        assets,
    );
}

pub fn draw_tile_world_pos(
    tile_type: TileType,
    rotation: f64,
    pos: &VecThreeFloat,
    can_place: bool,
    render_pack: &mut RenderPack,
    assets: &Assets,
) {
    let tile_asset_id = tile_type.to_string_id();

    let mut trans = Transform::new();
    trans.local_position = *pos;
    trans.local_rotation = VecThreeFloat::new(0.0, (rotation * 0.015).sin(), 0.0);
    trans.update_global_matrix(&M44::new_identity());

    let mut mat = assets.get_tile_material(tile_type).clone();
    let ambient = {
        let mut val = VecThreeFloat::new(0.05, 0.05, 0.05);
        if !can_place {
            val.x = 10.0;
        }
        val
    };

    mat.uniforms
        .insert("ambientColor".to_string(), UniformData::VecThree(ambient));

    render_pack.commands.push(RenderCommand::new_model(
        &trans,
        assets.asset_library.get_model(&tile_asset_id),
        &mat,
    ));
}
