use crate::{item::*, state::inventory::*, tile::*};
use rand::prelude::*;
use std::{collections::HashMap, sync::LazyLock};

mod instances;
pub use instances::*;

// external. this will get sent out to other systems.
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

#[derive(Debug, Eq, PartialEq)]
pub enum EntryOutputType {
    Gold,
    Item(ItemType),
    Table(DropTableID),
}

// internal to drop table
#[derive(Debug)]
pub struct EntryOutput {
    pub ty: EntryOutputType,
    pub amount: i64,
}

impl EntryOutput {
    pub fn new(ty: EntryOutputType, amount: i64) -> Self {
        Self { ty, amount }
    }

    pub fn new_item(item: ItemType, amount: i64) -> Self {
        Self::new(EntryOutputType::Item(item), amount)
    }

    pub fn new_table(table: DropTableID, amount: i64) -> Self {
        Self::new(EntryOutputType::Table(table), amount)
    }

    pub fn new_tile(tile: TileType, amount: i64) -> Self {
        Self::new(EntryOutputType::Item(ItemType::Tile(tile)), amount)
    }

    pub fn new_gold(amount: i64) -> Self {
        Self::new(EntryOutputType::Gold, amount)
    }
}

#[derive(Debug)]
struct Entry {
    pub chance_val: f64,
    pub output: EntryOutput,
}

#[derive(Debug)]
pub struct DropTable {
    entries_organized: Vec<Entry>,
    max: f64,
}

impl DropTable {
    pub fn new(entries: Vec<(EntryOutput, f64)>) -> Self {
        let mut org: Vec<Entry> = vec![];

        let mut accum: f64 = 0.0;
        for e in entries {
            accum += e.1;
            org.push(Entry {
                chance_val: accum,
                output: e.0,
            });
        }

        Self {
            entries_organized: org,
            max: accum,
        }
    }

    pub fn pull(&self, mut tables_visited: &mut Vec<DropTableID>) -> Drop {
        let num: f64 = rand::random_range(0.0..self.max);
        for e in &self.entries_organized {
            if e.chance_val > num {
                match e.output.ty {
                    EntryOutputType::Gold => return Drop::new_gold(e.output.amount),
                    EntryOutputType::Item(item_type) => {
                        return Drop::new_item(item_type, e.output.amount)
                    }
                    EntryOutputType::Table(table_id) => {
                        // todo this doesn't handle amount yet.

                        if tables_visited.contains(&table_id) {
                            panic!("Cycle detected. {:?} visited twice", table_id);
                        }
                        tables_visited.push(table_id);

                        return get_drop_cycle_check(table_id, tables_visited);
                    }
                };
            }
        }

        panic!("Error pulling item.");
    }

    // Will panic if a cycle exists
    pub fn check_cycle(&self) {
        let mut tables_visited: Vec<DropTableID> = vec![];

        for e in &self.entries_organized {
            match e.output.ty {
                // Only check the table drops
                EntryOutputType::Table(table_id) => {
                    let _ = get_drop_cycle_check(table_id, &mut tables_visited);
                }
                _ => {}
            }
        }
    }
}

mod test {
    use super::*;
    use crate::{state::inventory::*, tile::*};

    #[test]
    fn build() {
        let table = DropTable::new(vec![
            (EntryOutput::new_item(ItemType::DirtClod, 1), 10.0),
            (EntryOutput::new_tile(TileType::Grass, 1), 5.0),
        ]);

        assert_eq!(table.entries_organized.len(), 2);
        assert_eq!(table.max, 15.0);

        assert_eq!(table.entries_organized[0].chance_val, 10.0);
        assert_eq!(
            table.entries_organized[0].output.ty,
            EntryOutputType::Item(ItemType::DirtClod)
        );

        assert_eq!(table.entries_organized[1].chance_val, 15.0);
        assert_eq!(
            table.entries_organized[1].output.ty,
            EntryOutputType::Item(ItemType::Tile(TileType::Grass))
        );
    }

    #[test]
    fn table_drop() {
        let pull = get_drop(DropTableID::TestTable);

        assert_eq!(pull.drop_type, DropType::Gold);
        assert_eq!(pull.amount, 1);
    }

    #[test]
    #[should_panic]
    fn panic_on_cycle() {
        let pull = get_drop(DropTableID::TestCycleA);
    }
}
