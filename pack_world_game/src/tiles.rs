use gengar_engine::vectors::*;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Dirt,
    Grass,
}

impl Tile {
    pub fn can_place_here(&self, pos: VecTwoInt, world: &HashMap<VecTwoInt, Tile>) -> bool {
        return match self {
            Tile::Dirt => true,
            Tile::Grass => {
                if !world.contains_key(&pos) {
                    return false;
                }

                return *world.get(&pos).unwrap() == Tile::Dirt;
            }
        };
    }
}
