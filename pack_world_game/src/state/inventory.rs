use crate::{error::*, item::*, tile::*};
use std::collections::HashMap;

pub struct Inventory {
    pub items: HashMap<ItemType, i32>,
    pub gold: i64,
    pub limit: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            limit: 15,
            gold: 0,
        }
    }

    // returns new item count
    pub fn add_item(&mut self, item_type: ItemType, count: i32) -> Result<i32, Error> {
        // Will we be above the limit?
        if !self.items.contains_key(&item_type) && self.items.len() >= self.limit {
            return Err(Error::HitBankLimit);
        }

        // Add item
        *self.items.entry(item_type).or_insert(0) += count;

        // Check if negative
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

    pub fn at_limit(&self) -> bool {
        return self.items.len() >= self.limit;
    }
}

mod test {
    use super::*;
    use crate::{state::inventory::*, tile::*};

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

    #[test]
    pub fn limit() {
        let mut inv = Inventory::new();
        inv.limit = 2;

        let _ = inv.add_item(ItemType::DirtClod, 10).is_ok();
        let _ = inv.add_item(ItemType::DirtClod, 10).is_ok();
        let _ = inv.add_item(ItemType::OakLog, 10).is_ok();
        let _ = inv.add_item(ItemType::OakLog, 10).is_ok();

        let _ = inv.add_item(ItemType::Rock, 10).is_err();

        let _ = inv.add_item(ItemType::OakLog, 10).is_ok();
    }
}
