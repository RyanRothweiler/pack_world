use crate::{drop_table::*, error::*, grid::*, item::*, tile::*};
use gengar_engine::vectors::*;
use std::collections::HashMap;

pub mod world_snapshot;
pub use world_snapshot::*;

pub struct World {
    pub entity_map: HashMap<GridPos, Vec<usize>>,

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<GridPos, bool>,

    pub entities: Vec<TileInstance>,
}

impl World {
    // Won't place tile if not valid.
    pub fn try_place_tile(&mut self, grid_pos: GridPos, tile: TileType) -> Result<(), Error> {
        if !tile.can_place_here(grid_pos, self) {
            return Err(Error::InvalidTilePosition);
        }

        self.force_insert_tile(grid_pos, tile);

        return Ok(());
    }

    pub fn force_insert_tile(&mut self, grid_pos: GridPos, tile: TileType) {
        let inst_id = self.entities.len();
        let inst = tile.create_instance(grid_pos);

        // tell below tiles that something was placed above. They might care.
        let pos_entities_index: Vec<usize> = self.get_entities(grid_pos).unwrap_or(vec![]);
        for p in pos_entities_index {
            match self.entities.get_mut(p) {
                Some(tile_inst) => tile_inst.methods.tile_placed_ontop(tile, inst_id),

                // Nobody there to noify
                None => {}
            };
        }

        self.entities.push(inst);

        for p in tile.get_tile_footprint() {
            let pos = grid_pos + p;

            // update adjacents
            self.valids.insert(pos, true);
            self.valids.insert(GridPos::new(pos.x + 1, pos.y), true);
            self.valids.insert(GridPos::new(pos.x - 1, pos.y), true);
            self.valids.insert(GridPos::new(pos.x, pos.y + 1), true);
            self.valids.insert(GridPos::new(pos.x, pos.y - 1), true);

            self.entity_map.entry(pos).or_insert(vec![]).push(inst_id);
        }
    }

    pub fn get_world_snapshot(&self) -> WorldSnapshot {
        let mut ret = WorldSnapshot {
            entity_map: HashMap::new(),
            entities: vec![TileSnapshot::Grass; self.entities.len()],
        };

        ret.entity_map = self.entity_map.clone();

        for (key, value) in &self.entity_map {
            for eid in value {
                let inst: &TileInstance = self.entities.get(*eid).unwrap();
                ret.entities[*eid] = inst.methods.into_snapshot();
            }
        }

        ret
    }

    pub fn get_entities(&self, grid_pos: GridPos) -> Option<Vec<usize>> {
        if !self.entity_map.contains_key(&grid_pos) {
            return None;
        }

        return Some(self.entity_map.get(&grid_pos).unwrap().to_owned());
    }
}
