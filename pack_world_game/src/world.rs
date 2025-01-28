use crate::{error::*, tiles::*};
use gengar_engine::vectors::*;
use std::collections::HashMap;

pub struct World {
    pub tiles: HashMap<VecTwoInt, TileInstance>,

    // valid positions, and all adjacent valid positions
    pub valids: HashMap<VecTwoInt, bool>,
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
