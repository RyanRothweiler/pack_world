use crate::tile::*;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ItemType {
    DirtClod,
    Stick,
    Rock,
    Tile(TileType),
}

impl ItemType {
    pub fn user_dislay(&self) -> String {
        match self {
            ItemType::DirtClod => "Dirt Clod".into(),
            ItemType::Stick => "Stick".into(),
            ItemType::Rock => "Rock".into(),
            ItemType::Tile(tile_type) => format!("{:?}", tile_type).into(),
        }
    }
}
