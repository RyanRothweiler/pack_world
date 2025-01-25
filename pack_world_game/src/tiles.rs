use std::collections::HashMap;

use gengar_engine::vectors::*;

use crate::world::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Dirt,
    Grass,
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
}
