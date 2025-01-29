#![allow(dead_code)]

use crate::{state::inventory::*, tiles::*};
use rand::prelude::*;

pub struct Entry {
    pub val: f64,
    pub item_type: ItemType,
}

pub struct Pack {
    cost: Vec<(ItemType, i32)>,
    entries_organized: Vec<Entry>,
    max: f64,
}

impl Pack {
    pub fn new(cost: Vec<(ItemType, i32)>, entries: Vec<(ItemType, f64)>) -> Pack {
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
            cost,
            entries_organized: org,
            max: accum,
        }
    }

    pub fn pull(&self, inventory: &Inventory) -> ItemType {
        let num: f64 = rand::random_range(0.0..self.max);
        for e in &self.entries_organized {
            if e.val > num {
                return e.item_type;
            }
        }

        panic!("Error pulling item.");
    }
}

mod test {
    use super::*;
    use crate::{state::inventory::*, tiles::*};

    #[test]
    fn build() {
        let pack = Pack::new(
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
