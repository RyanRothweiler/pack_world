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

use tiles::{tile_boulder::*, tile_dirt::*, tile_grass::*, tile_oak_tree::*};

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
    Boulder,
    OakTree,
    BirdNest,
}

// TODO make these private?
pub struct TileInstance {
    pub tile_type: TileType,
    pub methods: Box<dyn TileMethods>,
    pub grid_pos: VecTwoInt,
}

impl TileType {
    pub fn can_place_here(&self, origin: VecTwoInt, world: &World) -> bool {
        let footprint = self.get_tile_footprint();

        for p in footprint {
            let pos = origin + p;

            // check adjacency
            if !world.entity_map.contains_key(&pos) {
                if !world.valids.contains_key(&pos) {
                    return false;
                }
            }

            // check types
            match self {
                TileType::Dirt => {}
                TileType::Grass | TileType::Boulder | TileType::OakTree | TileType::BirdNest => {
                    if !world.entity_map.contains_key(&pos) {
                        return false;
                    }

                    if let Some(tile) = world.get_entity(pos) {
                        if tile.tile_type != TileType::Dirt {
                            return false;
                        }
                    }
                }
            };
        }

        return true;
    }

    pub fn create_instance(&self, grid_pos: VecTwoInt) -> TileInstance {
        match self {
            TileType::Dirt => TileDirt::new(grid_pos),
            TileType::Grass => TileGrass::new(grid_pos),
            TileType::Boulder => TileBoulder::new(grid_pos),
            TileType::OakTree => TileOakTree::new(grid_pos),
            TileType::BirdNest => TileOakTree::new(grid_pos),
        }
    }

    pub fn get_tile_footprint(&self) -> Vec<VecTwoInt> {
        match self {
            TileType::Dirt | TileType::Grass | TileType::Boulder | TileType::BirdNest => {
                vec![VecTwoInt::new(0, 0)]
            }
            TileType::OakTree => vec![
                VecTwoInt::new(0, 0),
                VecTwoInt::new(1, 1),
                VecTwoInt::new(0, 1),
                VecTwoInt::new(1, 0),
            ],
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
