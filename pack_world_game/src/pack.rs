#![allow(dead_code)]

use crate::{state::inventory::*, tiles::*};
use rand::prelude::*;
use std::{collections::HashMap, sync::OnceLock};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PackID {
    Starter,
}

#[derive(Debug)]
pub struct Entry {
    pub val: f64,
    pub item_type: ItemType,
}

#[derive(Debug)]
pub struct Pack {
    pub display_name: String,
    pub cost: Vec<(ItemType, i32)>,

    entries_organized: Vec<Entry>,
    max: f64,
}

impl Pack {
    pub fn new(
        display_name: String,
        cost: Vec<(ItemType, i32)>,
        entries: Vec<(ItemType, f64)>,
    ) -> Pack {
        let mut org: Vec<Entry> = vec![];

        let mut accum: f64 = 0.0;
        for e in entries {
            accum += e.1;
            org.push(Entry {
                val: accum,
                item_type: e.0,
            });
        }

        Pack {
            display_name,
            cost,
            entries_organized: org,
            max: accum,
        }
    }

    pub fn can_afford(&self, inventory: &Inventory) -> bool {
        for c in &self.cost {
            if !inventory.has_atleast(c.0, c.1) {
                return false;
            }
        }

        true
    }

    pub fn pull(&self, inventory: &Inventory) -> Option<ItemType> {
        if !self.can_afford(inventory) {
            return None;
        }

        let num: f64 = rand::random_range(0.0..self.max);
        for e in &self.entries_organized {
            if e.val > num {
                return Some(e.item_type);
            }
        }

        panic!("Error pulling item.");
    }
}

pub static PACK_LIB: OnceLock<HashMap<PackID, Pack>> = OnceLock::new();

pub fn init_static_packs() {
    let mut pack_lib: HashMap<PackID, Pack> = HashMap::new();

    // Starter
    pack_lib.insert(
        PackID::Starter,
        Pack::new(
            "Starter".into(),
            vec![(ItemType::DirtClod, 5)],
            vec![
                (ItemType::Tile(TileType::Dirt), 10.0),
                (ItemType::Tile(TileType::Grass), 10.0),
            ],
        ),
    );

    PACK_LIB.set(pack_lib).unwrap();
}

pub fn get_pack_info(pack_id: PackID) -> &'static Pack {
    &PACK_LIB.get().unwrap().get(&pack_id).unwrap()
}

mod test {
    use super::*;
    use crate::{state::inventory::*, tiles::*};

    #[test]
    fn build() {
        let pack = Pack::new(
            "testing".into(),
            vec![],
            vec![
                (ItemType::DirtClod, 10.0),
                (ItemType::Tile(TileType::Grass), 5.0),
            ],
        );

        assert_eq!(pack.entries_organized.len(), 2);
        assert_eq!(pack.max, 15.0);

        assert_eq!(pack.entries_organized[0].val, 10.0);
        assert_eq!(pack.entries_organized[0].item_type, ItemType::DirtClod);

        assert_eq!(pack.entries_organized[1].val, 15.0);
        assert_eq!(
            pack.entries_organized[1].item_type,
            ItemType::Tile(TileType::Grass)
        );
    }
}
