use crate::tile::*;

mod item_data;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ItemType {
    DirtClod,
    Stick,
    Rock,
    OakLog,
    Tile(TileType),
}

impl ItemType {
    pub fn user_title(&self) -> &str {
        match self {
            ItemType::DirtClod => item_data::dirt_clod::TITLE,
            ItemType::Stick => item_data::stick::TITLE,
            ItemType::Rock => item_data::rock::TITLE,
            ItemType::OakLog => item_data::oak_wood::TITLE,
            ItemType::Tile(tile_type) => tile_type.get_user_title(),
        }
    }

    pub fn user_description(&self) -> Option<String> {
        match self {
            _ => None,
        }
    }
}
