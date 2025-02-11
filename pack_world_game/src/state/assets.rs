use crate::{item::*, state::inventory::*, tile::*};
use gengar_engine::render::image::*;

pub struct Assets {
    pub image_dirt: Image,
    pub image_grass: Image,
    pub image_dirt_clod: Image,
    pub image_stick: Image,
    pub image_boulder: Image,
    pub image_rock: Image,
    pub image_oak_tree: Image,
    pub image_oak_wood: Image,
    pub image_pack_starter: Image,
    pub image_bird_nest: Image,
    pub image_gold: Image,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            image_dirt: Image::new(),
            image_grass: Image::new(),
            image_stick: Image::new(),
            image_dirt_clod: Image::new(),
            image_pack_starter: Image::new(),
            image_boulder: Image::new(),
            image_rock: Image::new(),
            image_oak_tree: Image::new(),
            image_oak_wood: Image::new(),
            image_bird_nest: Image::new(),
            image_gold: Image::new(),
        }
    }

    pub fn get_tile_icon(&self, tile: &TileType) -> u32 {
        match tile {
            TileType::Dirt => return self.image_dirt.gl_id.unwrap(),
            TileType::Grass => return self.image_grass.gl_id.unwrap(),
            TileType::Boulder => return self.image_boulder.gl_id.unwrap(),
            TileType::OakTree => return self.image_oak_tree.gl_id.unwrap(),
            TileType::BirdNest => return self.image_bird_nest.gl_id.unwrap(),
        };
    }

    pub fn get_item_icon(&self, item: &ItemType) -> u32 {
        match item {
            ItemType::DirtClod => return self.image_dirt_clod.gl_id.unwrap(),
            ItemType::Stick => return self.image_stick.gl_id.unwrap(),
            ItemType::Rock => return self.image_rock.gl_id.unwrap(),
            ItemType::OakLog => return self.image_oak_wood.gl_id.unwrap(),
            ItemType::Tile(tile_type) => return self.get_tile_icon(tile_type),
        };
    }
}
