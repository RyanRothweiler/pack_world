use crate::{grid::*, tile::*, world::*};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum TileSnapshot {
    Dirt,
    Grass,
    Boulder,
    OakTree { has_nest: bool },
    BirdNest,
    Cave,
    Shrub,
    MudPit,
    TallGrass,
}

/// Snapshot of world state.
/// Allows world entities to interact with eachother without needing references to eachother.
pub struct WorldSnapshot {
    pub entity_map: HashMap<GridPos, WorldCell>,
    pub entities: HashMap<EntityID, TileSnapshot>,
}

impl WorldSnapshot {
    pub fn get_pos_snapshot(&self, grid_pos: GridPos) -> Vec<TileSnapshot> {
        let world_cell: WorldCell =
            (self.entity_map.get(&grid_pos).unwrap_or(&WorldCell::new())).clone();

        let mut ret: Vec<TileSnapshot> = vec![];
        for (layer, eid) in world_cell.layers {
            ret.push(*self.entities.get(&eid).unwrap());
        }

        ret
    }
}
