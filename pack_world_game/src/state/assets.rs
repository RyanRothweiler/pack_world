use gengar_engine::render::image::*;

use crate::{state::inventory::*, tiles::*};

pub struct Assets {
    pub image_dirt: Image,
    pub image_grass: Image,

    pub image_dirt_clod: Image,
}

impl Assets {
    pub fn get_tile_icon(&self, tile: &TileType) -> u32 {
        match tile {
            TileType::Dirt => return self.image_dirt.gl_id.unwrap(),
            TileType::Grass => return self.image_grass.gl_id.unwrap(),
        };
    }

    pub fn get_item_icon(&self, item: &ItemType) -> u32 {
        match item {
            ItemType::DirtClod => return self.image_dirt_clod.gl_id.unwrap(),
            ItemType::Tile(tile_type) => return self.get_tile_icon(tile_type),
        };
    }
}
