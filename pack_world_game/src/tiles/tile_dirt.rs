use crate::tiles::*;

pub struct TileDirt {}

impl TileMethods for TileDirt {
    fn update(&mut self, time_step: f64) {}
}

impl TileDirt {
    pub fn new() -> TileInstance {
        TileInstance {
            tile_type: TileType::Dirt,
            methods: Box::new(TileDirt {}),
        }
    }
}
