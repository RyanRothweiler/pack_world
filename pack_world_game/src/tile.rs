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
pub mod tile_type;
pub mod tiles;

pub use {tile_definition::*, tile_instance::*, tile_methods::*, tile_type::*};

pub fn draw_tile(
    tile_type: TileType,
    rotation: f64,
    pos: &GridPos,
    shader_color: Shader,
    render_pack: &mut RenderPack,
    assets: &Assets,
) {
    let world_pos = grid_to_world(pos);
    draw_tile_world_pos(
        tile_type,
        rotation,
        &world_pos,
        shader_color,
        render_pack,
        assets,
    );
}

pub fn draw_tile_world_pos(
    tile_type: TileType,
    rotation: f64,
    pos: &VecThreeFloat,
    shader_color: Shader,
    render_pack: &mut RenderPack,
    assets: &Assets,
) {
    /*
    let mut r = Rect::new_square(GRID_SIZE);

    r.set_center(*pos);

    let mut mat = Material::new();
    mat.shader = Some(shader_color);

    mat.uniforms.insert(
        "tex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: assets.get_tile_icon(&tile_type),
            texture_slot: 0,
        }),
    );

    mat.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(COLOR_WHITE.into()),
    );

    render_pack
        .commands
        .push(RenderCommand::new_rect(&r, -1.0, rotation, &mat));
    */

    let mut trans = Transform::new();
    trans.local_position = *pos;
    trans.update_global_matrix(&M44::new_identity());

    /*
    es.render_packs
        .get_mut(&RenderPackID::NewWorld)
        .unwrap()
        .commands
        .push(RenderCommand::new_model(
            &trans,
            &gs.assets.model_tile_grass,
            &gs.assets.tile_grass_material,
        ));
        */
}
