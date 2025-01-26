use std::collections::HashMap;

use gengar_engine::vectors::*;

use crate::world::*;

pub mod tile_grass;

use tile_grass::*;

pub trait TileMethods {
    fn update(&mut self, time_step: f64);
    fn get_icon(&self) -> u32;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Dirt,
    Grass,
}

pub enum TileType {
    Dirt,
    Grass,
}

pub struct TileInstance {
    pub tile_type: TileType,
    pub methods: Box<dyn TileMethods>,
}

impl Tile {
    pub fn can_place_here(&self, pos: VecTwoInt, world: &World) -> bool {
        // check adjacency
        if !world.tiles.contains_key(&pos) {
            if !world.valids.contains_key(&pos) {
                return false;
            }
        }

        // check types
        return match self {
            Tile::Dirt => true,
            Tile::Grass => {
                if !world.tiles.contains_key(&pos) {
                    return false;
                }

                return *world.tiles.get(&pos).unwrap() == Tile::Dirt;
            }
        };
    }

    /*
    pub fn update(&mut self, delta_time: f64) {
        match self {
            Tile::Dirt => {}
            Tile::Grass => s.update(delta_time),
        }
    }
    */
}
