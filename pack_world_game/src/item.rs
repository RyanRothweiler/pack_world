use crate::tile::*;

mod item_data;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ItemType {
    DirtClod,
    Stick,
    Rock,
    OakLog,
    Acorn,
    DragonEgg,
    Baby,
    Berry,

    Tile(TileType),
}

impl ItemType {
    pub fn user_title(&self) -> &str {
        match self {
            ItemType::Acorn => item_data::acorn::TITLE,
            ItemType::DirtClod => item_data::dirt_clod::TITLE,
            ItemType::Stick => item_data::stick::TITLE,
            ItemType::Rock => item_data::rock::TITLE,
            ItemType::OakLog => item_data::oak_wood::TITLE,
            ItemType::DragonEgg => item_data::dragon_egg::TITLE,
            ItemType::Baby => item_data::baby::TITLE,
            ItemType::Berry => item_data::berry::TITLE,
            ItemType::Tile(tile_type) => tile_type.user_title(),
        }
    }

    pub fn user_description(&self) -> Option<&str> {
        let mut ret = match self {
            ItemType::Tile(tile_type) => tile_type.user_description(),
            _ => None,
        };

        ret
    }

    pub fn is_tile(&self) -> bool {
        match self {
            ItemType::Tile(_) => true,
            _ => false,
        }
    }
}
