use crate::{drop_table::*, item::*, pack::*, state::inventory::*, tile::*};
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
    pub image_bird_nest: Image,
    pub image_gold: Image,
    pub image_acorn: Image,
    pub image_cave: Image,
    pub image_dragon_egg: Image,
    pub image_baby: Image,
    pub image_shrub: Image,
    pub image_berry: Image,
    pub image_question_mark: Image,
    pub image_mud_pit: Image,
    pub image_tall_grass: Image,
    pub image_mud_baby: Image,
    pub image_frog: Image,
    pub image_water: Image,
    pub image_newt: Image,
    pub image_reed: Image,

    pub image_pack_starter: Image,
    pub image_pack_stick: Image,
    pub image_pack_mud: Image,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            image_dirt: Image::new(),
            image_grass: Image::new(),
            image_stick: Image::new(),
            image_dirt_clod: Image::new(),
            image_boulder: Image::new(),
            image_rock: Image::new(),
            image_oak_tree: Image::new(),
            image_oak_wood: Image::new(),
            image_bird_nest: Image::new(),
            image_gold: Image::new(),
            image_acorn: Image::new(),
            image_cave: Image::new(),
            image_dragon_egg: Image::new(),
            image_baby: Image::new(),
            image_shrub: Image::new(),
            image_berry: Image::new(),
            image_question_mark: Image::new(),
            image_mud_pit: Image::new(),
            image_tall_grass: Image::new(),
            image_mud_baby: Image::new(),
            image_frog: Image::new(),
            image_water: Image::new(),
            image_newt: Image::new(),
            image_reed: Image::new(),

            image_pack_starter: Image::new(),
            image_pack_stick: Image::new(),
            image_pack_mud: Image::new(),
        }
    }

    pub fn get_tile_icon(&self, tile: &TileType) -> u32 {
        self.get_tile_image_opt(tile)
            .expect(&format!("Missing tile image for {:?}", tile))
    }

    pub fn get_item_icon(&self, item: &ItemType) -> u32 {
        self.get_item_image_opt(item)
            .expect(&format!("Missing item image for {:?}", item))
    }

    pub fn get_drop_icon(&self, drop: &DropType) -> u32 {
        match drop {
            DropType::Gold => return self.image_gold.gl_id.unwrap(),
            DropType::Item { item_type } => return self.get_item_icon(item_type),
        }
    }

    pub fn get_pack_icon(&self, pack_id: &PackID) -> u32 {
        self.get_pack_image_opt(pack_id)
            .expect(&format!("Missing pack image for {pack_id:?}"))
    }

    fn get_pack_image_opt(&self, pack: &PackID) -> Option<u32> {
        match pack {
            PackID::Starter => return self.image_pack_starter.gl_id,
            PackID::Stick => return self.image_pack_stick.gl_id,
            PackID::Mud => return self.image_pack_mud.gl_id,
        };
    }

    fn get_item_image_opt(&self, item: &ItemType) -> Option<u32> {
        match item {
            ItemType::DirtClod => return self.image_dirt_clod.gl_id,
            ItemType::Stick => return self.image_stick.gl_id,
            ItemType::Rock => return self.image_rock.gl_id,
            ItemType::OakLog => return self.image_oak_wood.gl_id,
            ItemType::Acorn => return self.image_acorn.gl_id,
            ItemType::DragonEgg => return self.image_dragon_egg.gl_id,
            ItemType::Baby => return self.image_baby.gl_id,
            ItemType::Berry => return self.image_berry.gl_id,
            ItemType::MudBaby => return self.image_mud_baby.gl_id,

            ItemType::Tile(tile_type) => return self.get_tile_image_opt(tile_type),
        };
    }

    fn get_tile_image_opt(&self, tile: &TileType) -> Option<u32> {
        match tile {
            TileType::Dirt => return self.image_dirt.gl_id,
            TileType::Grass => return self.image_grass.gl_id,
            TileType::Boulder => return self.image_boulder.gl_id,
            TileType::OakTree => return self.image_oak_tree.gl_id,
            TileType::BirdNest => return self.image_bird_nest.gl_id,
            TileType::Cave => return self.image_cave.gl_id,
            TileType::Shrub => return self.image_shrub.gl_id,
            TileType::MudPit => return self.image_mud_pit.gl_id,
            TileType::TallGrass => return self.image_tall_grass.gl_id,
            TileType::Frog => return self.image_frog.gl_id,
            TileType::Water => return self.image_water.gl_id,
            TileType::Newt => return self.image_newt.gl_id,
            TileType::Reed => return self.image_reed.gl_id,
        };
    }
}
