// const HARVEST_SECONDS: f64 = 5.0;

use crate::{state::*, tiles::*};

pub struct TileGrass {
    pub time: f64,
}

impl TileMethods for TileGrass {
    fn get_icon(&self) -> u32 {
        0
    }

    fn update(&mut self, time_step: f64) {}
}

impl TileGrass {
    pub fn new(gs: &State) -> TileInstance {
        TileInstance {
            tile_type: TileType::Grass,
            methods: Box::new(TileGrass { time: 0.0 }),
        }
    }
}
