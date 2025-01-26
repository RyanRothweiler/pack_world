use std::collections::HashMap;

use crate::tiles::*;
use gengar_engine::vectors::*;

pub struct World {
    pub tiles: HashMap<VecTwoInt, TileInstance>,

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<VecTwoInt, bool>,
}

impl World {
    // Won't place tile if not valid.
    pub fn try_place_tile(&mut self, grid_pos: VecTwoInt, tile: TileType) {
        if !tile.can_place_here(grid_pos, self) {
            return;
        }

        self.force_insert_tile(grid_pos, tile);
    }

    pub fn force_insert_tile(&mut self, grid_pos: VecTwoInt, tile: TileType) {
        // update adjacents
        self.valids.insert(grid_pos, true);
        self.valids
            .insert(VecTwoInt::new(grid_pos.x + 1, grid_pos.y), true);
        self.valids
            .insert(VecTwoInt::new(grid_pos.x - 1, grid_pos.y), true);
        self.valids
            .insert(VecTwoInt::new(grid_pos.x, grid_pos.y + 1), true);
        self.valids
            .insert(VecTwoInt::new(grid_pos.x, grid_pos.y - 1), true);

        self.tiles.insert(grid_pos, tile.create_instance());
    }
}
