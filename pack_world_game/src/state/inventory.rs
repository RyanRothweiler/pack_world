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
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    // returns new item count
    pub fn add_item(&mut self, item_type: ItemType, count: i32) -> Result<i32, Error> {
        *self.items.entry(item_type).or_insert(0) += count;

        let current_count = *self.items.get(&item_type).unwrap();
        if current_count < 0 {
            println!("Item at negative {:?}. Setting to 0.", item_type);
            *self.items.get_mut(&item_type).unwrap() = 0;
            return Err(Error::NegativeItemCount);
        }

        return Ok(current_count);
    }

    pub fn has_atleast(&self, item_type: ItemType, count: i32) -> bool {
        if !self.items.contains_key(&item_type) {
            return false;
        }

        let c = *self.items.get(&item_type).unwrap();
        return c >= count;
    }
}

mod test {
    use super::*;
    use crate::{state::inventory::*, tiles::*};

    #[test]
    pub fn add_item() {
        let mut inv = Inventory::new();
        inv.add_item(ItemType::DirtClod, 10).unwrap();

        assert_eq!(inv.has_atleast(ItemType::DirtClod, 10), true);

        let ret = inv.add_item(ItemType::DirtClod, -11);
        assert_eq!(ret, Err(Error::NegativeItemCount));

        inv.add_item(ItemType::DirtClod, 20).unwrap();
        inv.add_item(ItemType::DirtClod, 20).unwrap();

        assert_eq!(inv.has_atleast(ItemType::DirtClod, 40), true);
    }

    #[test]
    pub fn has_atleast() {
        let mut inv = Inventory::new();
        inv.add_item(ItemType::DirtClod, 10).unwrap();

        assert_eq!(inv.has_atleast(ItemType::DirtClod, 10), true);
        assert_eq!(inv.has_atleast(ItemType::DirtClod, 1), true);
        assert_eq!(inv.has_atleast(ItemType::DirtClod, 11), false);
    }
}
