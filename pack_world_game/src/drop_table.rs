use crate::{state::inventory::*, tiles::*};
use rand::prelude::*;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum DropTableID {
    Grass,
}

#[derive(Debug)]
pub struct Entry {
    pub val: f64,
    pub item_type: ItemType,
}

#[derive(Debug)]
pub struct DropTable {
    entries_organized: Vec<Entry>,
    max: f64,
}

impl DropTable {
    pub fn new(entries: Vec<(ItemType, f64)>) -> Self {
        let mut org: Vec<Entry> = vec![];

        let mut accum: f64 = 0.0;
        for e in entries {
            accum += e.1;
            org.push(Entry {
                val: accum,
                item_type: e.0,
            });
        }

        Self {
            entries_organized: org,
            max: accum,
        }
    }

    pub fn pull(&self) -> ItemType {
        let num: f64 = rand::random_range(0.0..self.max);
        for e in &self.entries_organized {
            if e.val > num {
                return e.item_type;
            }
        }

        panic!("Error pulling item.");
    }
}

static GRASS_TABLE: LazyLock<DropTable> =
    LazyLock::new(|| DropTable::new(vec![(ItemType::DirtClod, 10.0), (ItemType::Stick, 10.0)]));

pub fn get_drop(table: DropTableID) -> ItemType {
    match table {
        DropTableID::Grass => GRASS_TABLE.pull(),
    }
}

mod test {
    use super::*;
    use crate::{state::inventory::*, tiles::*};

    #[test]
    fn build() {
        let table = DropTable::new(vec![
            (ItemType::DirtClod, 10.0),
            (ItemType::Tile(TileType::Grass), 5.0),
        ]);

        assert_eq!(table.entries_organized.len(), 2);
        assert_eq!(table.max, 15.0);

        assert_eq!(table.entries_organized[0].val, 10.0);
        assert_eq!(table.entries_organized[0].item_type, ItemType::DirtClod);

        assert_eq!(table.entries_organized[1].val, 15.0);
        assert_eq!(
            table.entries_organized[1].item_type,
            ItemType::Tile(TileType::Grass)
        );
    }
}
