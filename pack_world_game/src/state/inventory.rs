use crate::{error::*, tiles::*};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ItemType {
    DirtClod,
    Tile(TileType),
}

impl ItemType {
    pub fn user_dislay(&self) -> String {
        match self {
            ItemType::DirtClod => "Dirt Clod".into(),
            ItemType::Tile(tile_type) => format!("{:?}", tile_type).into(),
        }
    }
}

pub struct Inventory {
    pub items: HashMap<ItemType, i32>,
}

impl Inventory {
    // returns new item count
    pub fn add_item(&mut self, item_type: ItemType, count: i32) -> Result<i32, Error> {
        *self.items.entry(item_type).or_insert(0) += count;

        let current_count = *self.items.get(&item_type).unwrap();
        if current_count < 0 {
            return Err(Error::NegativeItemCount);
        }

        return Ok(current_count);
    }
}
