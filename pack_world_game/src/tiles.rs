use std::collections::HashMap;

use gengar_engine::vectors::*;

use crate::{state::*, world::*};

pub mod tile_dirt;
pub mod tile_grass;

use tile_dirt::*;
use tile_grass::*;

pub trait TileMethods {
    fn update(&mut self, time_step: f64);
}

#[derive(Copy, Clone, Eq, PartialEq)]
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
