use crate::{grid::*, tile::*};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum TileSnapshot {
    Dirt,
    Grass,
    Boulder,
    OakTree { has_nest: bool },
    BirdNest,
    Cave,
}

/// Snapshot of world state.
/// Allows world entities to interact with eachother without needing references to eachother.
pub struct WorldSnapshot {
    pub entity_map: HashMap<GridPos, Vec<usize>>,
    pub entities: Vec<TileSnapshot>,
}

impl WorldSnapshot {
    pub fn get_pos_snapshot(&self, grid_pos: GridPos) -> Vec<TileSnapshot> {
        let eids: Vec<usize> = (self.entity_map.get(&grid_pos).unwrap_or(&vec![])).clone();

        let mut ret: Vec<TileSnapshot> = vec![];
        for id in eids {
            ret.push(self.entities[id]);
        }

        ret
    }
}
