use crate::{constants::*, drop_table::*, error::*, item::*, tile::*};
use std::collections::HashMap;

pub struct Inventory {
    pub items: HashMap<ItemType, i64>,
    pub gold: i64,
    pub limit: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            limit: BANK_LIMIT_START,
            gold: 0,
        }
    }

    pub fn give_drop(&mut self, drop: Drop) -> Result<i64, Error> {
        match drop.drop_type {
            DropType::Gold => self.give_gold(drop.amount),
            DropType::Item { item_type } => self.give_item(item_type, drop.amount),
        }
    }

    // returns new item count
    pub fn give_item(&mut self, item_type: ItemType, count: i64) -> Result<i64, Error> {
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

    pub fn give_gold(&mut self, count: i64) -> Result<i64, Error> {
        self.gold += count;
        if self.gold < 0 {
            self.gold = 0;
            println!("Gold now at negative. That is an issue.");
            return Err(Error::NegativeItemCount);
        }
        Ok(self.gold)
    }

    pub fn has_atleast(&self, item_type: ItemType, count: i64) -> bool {
        if !self.items.contains_key(&item_type) {
            return false;
        }

        let c = *self.items.get(&item_type).unwrap();
        return c >= count;
    }

    pub fn at_limit(&self) -> bool {
        return self.items.len() >= self.limit;
    }

    pub fn next_slot_cost(&self) -> i64 {
        let next: f64 = (self.limit - BANK_LIMIT_START) as f64;
        BANK_LIMIT_COST_BASE + next.powf(BANK_LIMIT_EXPO_PRICE) as i64
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
