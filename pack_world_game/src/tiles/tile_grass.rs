// const HARVEST_SECONDS: f64 = 5.0;

use crate::tiles::*;

pub struct TileGrass {
    pub time: f64,
}

impl TileMethods for TileGrass {
    fn update(&mut self, time_step: f64) {
        // println!("grass updating!");
    }
}

impl TileGrass {
    pub fn new() -> TileInstance {
        TileInstance {
            tile_type: TileType::Grass,
            methods: Box::new(TileGrass { time: 0.0 }),
        }
    }
}
