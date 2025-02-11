use crate::{item::*, state::inventory::*, tile::*};
use rand::prelude::*;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum DropType {
    Gold,
    Item { item_type: ItemType },
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Drop {
    pub drop_type: DropType,
    pub amount: i64,
}

impl Drop {
    pub fn new(drop_type: DropType, amount: i64) -> Self {
        Self { drop_type, amount }
    }

    pub fn new_item(item: ItemType, amount: i64) -> Self {
        Drop::new(DropType::Item { item_type: item }, amount)
    }

    pub fn new_tile(tile: TileType, amount: i64) -> Self {
        Drop::new(
            DropType::Item {
                item_type: ItemType::Tile(tile),
            },
            amount,
        )
    }

    pub fn new_gold(amount: i64) -> Self {
        Drop::new(DropType::Gold, amount)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum DropTableID {
    Grass,
    Boulder,
    OakTree,
}

#[derive(Debug)]
pub struct Entry {
    pub chance_val: f64,
    pub drop: Drop,
}

#[derive(Debug)]
pub struct DropTable {
    entries_organized: Vec<Entry>,
    max: f64,
}

impl DropTable {
    pub fn new(entries: Vec<(Drop, f64)>) -> Self {
        let mut org: Vec<Entry> = vec![];

        let mut accum: f64 = 0.0;
        for e in entries {
            accum += e.1;
            org.push(Entry {
                chance_val: accum,
                drop: e.0,
            });
        }

        Self {
            entries_organized: org,
            max: accum,
        }
    }

    pub fn pull(&self) -> Drop {
        let num: f64 = rand::random_range(0.0..self.max);
        for e in &self.entries_organized {
            if e.chance_val > num {
                return e.drop;
            }
        }

        panic!("Error pulling item.");
    }
}

pub fn get_drop(table: DropTableID) -> Drop {
    match table {
        DropTableID::Grass => GRASS.pull(),
        DropTableID::Boulder => BOULDER.pull(),
        DropTableID::OakTree => OAK_TREE.pull(),
    }
}

static GRASS: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (Drop::new_item(ItemType::DirtClod, 1), 10.0),
        (Drop::new_item(ItemType::Stick, 1), 4.0),
    ])
});

static BOULDER: LazyLock<DropTable> =
    LazyLock::new(|| DropTable::new(vec![((Drop::new_item(ItemType::Rock, 1), 10.0))]));

static OAK_TREE: LazyLock<DropTable> = LazyLock::new(|| {
    DropTable::new(vec![
        (Drop::new_item(ItemType::Stick, 1), 6.0),
        (Drop::new_item(ItemType::OakLog, 1), 30.0),
    ])
});

mod test {
    use super::*;
    use crate::{state::inventory::*, tile::*};

    #[test]
    fn build() {
        let table = DropTable::new(vec![
            (Drop::new_item(ItemType::DirtClod, 1), 10.0),
            (Drop::new_tile(TileType::Grass, 1), 5.0),
        ]);

        assert_eq!(table.entries_organized.len(), 2);
        assert_eq!(table.max, 15.0);

        assert_eq!(table.entries_organized[0].chance_val, 10.0);
        assert_eq!(
            table.entries_organized[0].drop.drop_type,
            DropType::Item {
                item_type: ItemType::DirtClod
            }
        );

        assert_eq!(table.entries_organized[1].chance_val, 15.0);
        assert_eq!(
            table.entries_organized[1].drop.drop_type,
            DropType::Item {
                item_type: ItemType::Tile(TileType::Grass)
            }
        );
    }
}
