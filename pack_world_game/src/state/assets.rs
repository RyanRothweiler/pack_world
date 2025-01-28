use gengar_engine::render::image::*;

use crate::tiles::*;

pub struct Assets {
    pub image_dirt: Image,
    pub image_grass: Image,
}

impl Assets {
    pub fn get_tile_icon(&self, tile: &TileType) -> u32 {
        let image_id = match tile {
            TileType::Dirt => return self.image_dirt.gl_id.unwrap(),
            TileType::Grass => return self.image_grass.gl_id.unwrap(),
        };
    }
}
