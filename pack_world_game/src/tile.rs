use crate::{
    grid::*,
    state::{assets::*, *},
    update_signal::*,
    world::*,
};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    vectors::*,
};
use std::collections::HashMap;

pub mod harvest_timer;
pub mod tiles;

use tiles::tile_dirt::*;
use tiles::tile_grass::*;
use tiles::tile_rock::*;

pub trait TileMethods {
    fn update(&mut self, time_step: f64) -> Vec<UpdateSignal>;
    fn can_harvest(&self) -> bool;
    fn harvest(&mut self, tile_pos: VecTwo) -> Vec<UpdateSignal>;
    fn render_hover_info(&self, shader_color: Shader, render_pack: &mut RenderPack);
    fn render(
        &self,
        rot_time: f64,
        pos: &VecTwoInt,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum TileType {
    Dirt,
    Grass,
    Rock,
}

// TODO make these private?
pub struct TileInstance {
    pub tile_type: TileType,
    pub methods: Box<dyn TileMethods>,
}

impl TileType {
    pub fn can_place_here(&self, pos: VecTwoInt, world: &World) -> bool {
        // check adjacency
        if !world.tiles.contains_key(&pos) {
            if !world.valids.contains_key(&pos) {
                return false;
            }
        }

        // check types
        return match self {
            TileType::Dirt => true,
            TileType::Grass | TileType::Rock => {
                if !world.tiles.contains_key(&pos) {
                    return false;
                }

                return world.tiles.get(&pos).unwrap().tile_type == TileType::Dirt;
            }
        };
    }

    pub fn create_instance(&self) -> TileInstance {
        match self {
            TileType::Dirt => TileDirt::new(),
            TileType::Grass => TileGrass::new(),
            TileType::Rock => TileRock::new(),
        }
    }
}

pub fn draw_tile(
    tile_type: TileType,
    rotation: f64,
    pos: &VecTwoInt,
    shader_color: Shader,
    render_pack: &mut RenderPack,
    assets: &Assets,
) {
    let mut r = Rect::new_square(GRID_SIZE);

    r.set_center(grid_to_world(pos));

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
}
