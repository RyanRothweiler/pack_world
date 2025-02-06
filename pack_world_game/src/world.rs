use crate::{error::*, tile::*};
use gengar_engine::vectors::*;
use std::collections::HashMap;

pub struct World {
    pub entity_map: HashMap<VecTwoInt, Vec<usize>>,

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<VecTwoInt, bool>,

    pub entities: Vec<TileInstance>,
}

impl World {
    // Won't place tile if not valid.
    pub fn try_place_tile(&mut self, grid_pos: VecTwoInt, tile: TileType) -> Result<(), Error> {
        if !tile.can_place_here(grid_pos, self) {
            return Err(Error::InvalidTilePosition);
        }

        self.force_insert_tile(grid_pos, tile);

        return Ok(());
    }

    pub fn force_insert_tile(&mut self, grid_pos: VecTwoInt, tile: TileType) {
        let inst_id = self.entities.len();
        let inst = tile.create_instance(grid_pos);
        self.entities.push(inst);

        for p in tile.get_tile_footprint() {
            let pos = grid_pos + p;

            // update adjacents
            self.valids.insert(pos, true);
            self.valids.insert(VecTwoInt::new(pos.x + 1, pos.y), true);
            self.valids.insert(VecTwoInt::new(pos.x - 1, pos.y), true);
            self.valids.insert(VecTwoInt::new(pos.x, pos.y + 1), true);
            self.valids.insert(VecTwoInt::new(pos.x, pos.y - 1), true);

            self.entity_map.entry(pos).or_insert(vec![]).push(inst_id);
        }
    }

    pub fn get_entities(&self, grid_pos: VecTwoInt) -> Option<Vec<usize>> {
        if !self.entity_map.contains_key(&grid_pos) {
            return None;
        }

        return Some(self.entity_map.get(&grid_pos).unwrap().to_owned());
    }
}
