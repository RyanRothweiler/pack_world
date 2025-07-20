use crate::{grid::*, tile::*, world::*};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TileSnapshot {
    Dirt,
    Water,
    Grass,
    Boulder,
    OakTree { has_nest: bool },
    BirdNest,
    Cave,
    Shrub,
    MudPit,
    TallGrass,
    Frog,
    Newt,
    Reed,
    Clam,
    MudFish,
    Spring,
    Kelp,
    Crab,
    MudHenge,
}

/// Snapshot of world state.
/// Allows world entities to interact with eachother without needing references to eachother.
#[derive(Debug)]
pub struct WorldSnapshot {
    pub entity_map: HashMap<GridPos, WorldCell>,

    pub entities: HashMap<EntityID, TileSnapshot>,
    pub entity_harvest_perc: HashMap<GridPos, (EntityID, f64)>,

    pub valids: HashMap<GridPos, bool>,
    pub drop_count_mod: HashMap<GridPos, f64>,
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
