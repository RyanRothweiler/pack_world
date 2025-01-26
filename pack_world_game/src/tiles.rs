use std::collections::HashMap;

use gengar_engine::{
    render::{render_pack::*, shader::*},
    vectors::*,
};

use crate::{state::*, update_signal::*, world::*};

pub mod tile_dirt;
pub mod tile_grass;

use tile_dirt::*;
use tile_grass::*;

pub trait TileMethods {
    fn update(&mut self, time_step: f64) -> Vec<UpdateSignal>;
    fn can_harvest(&self) -> bool;
    fn harvest(&mut self) -> Vec<UpdateSignal>;
    fn render_hover_info(&self, shader_color: Shader, render_pack: &mut RenderPack);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TileType {
    Dirt,
    Grass,
}

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
            TileType::Grass => {
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
        }
    }
}
