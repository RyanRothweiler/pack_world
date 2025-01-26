const HARVEST_SECONDS: f64 = 10.0;

use crate::{state::*, tiles::*};

pub struct TileGrass {
    pub time: f64,
}

impl TileMethods for TileGrass {
    fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        self.time += time_step;

        if self.time > HARVEST_SECONDS {
            self.time = 0.0;
            return vec![UpdateSignal::GiveItem {
                item_type: ItemType::DirtClod,
                count: 5,
            }];
        }

        vec![]
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
