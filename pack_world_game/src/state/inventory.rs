use crate::{constants::*, drop_table::*, error::*, item::*, save_file::*, tile::*};
use std::collections::HashMap;

pub struct Inventory {
    pub items_seen: HashMap<ItemType, bool>,
    pub items: HashMap<ItemType, i64>,
    pub gold: i64,
    pub limit: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            items_seen: HashMap::new(),
            limit: BANK_LIMIT_START,
            gold: 0,
        }
    }

    pub fn clear(&mut self) {
        self.gold = 0;
        self.limit = BANK_LIMIT_START;
        self.items_seen.clear();
        self.items.clear();
    }

    pub fn drop_seen(&self, drop: &Drop) -> bool {
        match drop.drop_type {
            DropType::Gold => true,
            DropType::Item { item_type } => *self.items_seen.get(&item_type).unwrap_or(&false),
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
        self.items_seen.insert(item_type, true);

        // Will we be above the limit?
        if !self.items.contains_key(&item_type) && self.items.len() >= self.limit {
            // Don't enforce bank limit for now.
            // return Err(Error::HitBankLimit);
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

    pub fn get_all_tiles(&self) -> Vec<(&ItemType, &i64)> {
        let all_tiles: Vec<(&ItemType, &i64)> = self
            .items
            .iter()
            .filter(|(item_type, count)| item_type.is_tile() && **count > 0)
            .collect();

        all_tiles
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let gold_key = format!("{}.g", key_parent);
        let limit_key = format!("{}.l", key_parent);

        save_file.save_i64(&gold_key, self.gold);
        save_file.save_i64(&limit_key, self.limit as i64);

        for (i, (key, value)) in self.items.iter().enumerate() {
            let index_id = format!("item_index.{}", i);
            let id = format!("item.{}.id", i);
            let count_key = format!("{}.item_count", i);

            save_file.save_i64(&index_id, i as i64);
            key.save_file_write(id, save_file).unwrap();
            save_file.save_i64(&count_key, *value);
        }

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let gold_key = format!("{}.g", key_parent);
        let limit_key = format!("{}.l", key_parent);

        let mut inv = Self::new();
        inv.gold = save_file.load_i64(&gold_key)?;
        inv.limit = save_file.load_i64(&limit_key)? as usize;

        for (key, value) in &save_file.entries {
            // check if is tile
            let parts: Vec<&str> = key.split('.').collect();
            if parts[0].starts_with("item_index") {
                let index = save_file.load_i64(key)?;

                let id = format!("item.{}.id", index);
                let count_key = format!("{}.item_count", index);

                let item = ItemType::save_file_load(id, save_file)?;
                let count = save_file.load_i64(&count_key)?;

                inv.give_item(item, count)?;
            }
        }

        Ok(inv)
    }
}

mod test {
    use super::*;
    use crate::{state::inventory::*, tile::*};

    #[test]
    pub fn add_item() {
        let mut inv = Inventory::new();
        inv.give_item(ItemType::DirtClod, 10).unwrap();

        assert_eq!(inv.has_atleast(ItemType::DirtClod, 10), true);

        let ret = inv.give_item(ItemType::DirtClod, -11);
        match ret {
            Err(e) => match e {
                Error::NegativeItemCount => {}
                _ => {
                    panic!("Incorrect error type")
                }
            },
            _ => {}
        }

        inv.give_item(ItemType::DirtClod, 20).unwrap();
        inv.give_item(ItemType::DirtClod, 20).unwrap();

        assert_eq!(inv.has_atleast(ItemType::DirtClod, 40), true);
    }

    #[test]
    pub fn has_atleast() {
        let mut inv = Inventory::new();
        inv.give_item(ItemType::DirtClod, 10).unwrap();

        assert_eq!(inv.has_atleast(ItemType::DirtClod, 10), true);
        assert_eq!(inv.has_atleast(ItemType::DirtClod, 1), true);
        assert_eq!(inv.has_atleast(ItemType::DirtClod, 11), false);
    }

    #[test]
    pub fn limit() {
        let mut inv = Inventory::new();
        inv.limit = 2;

        let _ = inv.give_item(ItemType::DirtClod, 10).is_ok();
        let _ = inv.give_item(ItemType::DirtClod, 10).is_ok();
        let _ = inv.give_item(ItemType::OakLog, 10).is_ok();
        let _ = inv.give_item(ItemType::OakLog, 10).is_ok();

        let _ = inv.give_item(ItemType::Rock, 10).is_err();

        let _ = inv.give_item(ItemType::OakLog, 10).is_ok();
    }

    #[test]
    fn save_load() {
        let mut save_file = SaveFile::new();

        let mut orig = Inventory::new();
        orig.gold = 100;
        orig.limit = 7;
        orig.give_item(ItemType::Acorn, 123).unwrap();
        orig.give_item(ItemType::Tile(TileType::Grass), 55).unwrap();

        orig.save_file_write("inv".into(), &mut save_file).unwrap();

        let new = Inventory::save_file_load("inv".into(), &save_file).unwrap();
        assert_eq!(new.gold, orig.gold);
        assert_eq!(new.limit, orig.limit);
        assert_eq!(*new.items.get(&ItemType::Acorn).unwrap(), 123);
        assert_eq!(
            *new.items.get(&ItemType::Tile(TileType::Grass)).unwrap(),
            55
        );
    }
}
