use crate::{item::*, state::inventory::*, tile::*};
use rand::prelude::*;
use std::{collections::HashMap, sync::LazyLock};

mod fixed_tables;
mod table_instance;
pub use {fixed_tables::*, table_instance::*};

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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum EntryOutputType {
    Gold,
    Item(ItemType),
    Table(FixedTableID),
}

// internal to drop table
#[derive(Debug, Clone)]
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

    pub fn new_table(table: FixedTableID, amount: i64) -> Self {
        Self::new(EntryOutputType::Table(table), amount)
    }

    pub fn new_tile(tile: TileType, amount: i64) -> Self {
        Self::new(EntryOutputType::Item(ItemType::Tile(tile)), amount)
    }

    pub fn new_gold(amount: i64) -> Self {
        Self::new(EntryOutputType::Gold, amount)
    }
}

#[derive(Debug, Clone)]
struct Entry {
    // original input chance value
    pub orig_chance: f64,

    // accumulated chance value
    pub chance_val: f64,
    pub output: EntryOutput,
}

impl Entry {
    pub fn new(input: (EntryOutput, f64)) -> Self {
        Self {
            output: input.0,
            orig_chance: input.1,
            chance_val: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DropTable {
    entries: Vec<Entry>,
    max: f64,
}

impl DropTable {
    pub fn new(entries: Vec<(EntryOutput, f64)>) -> Self {
        let mut org: Vec<Entry> = vec![];

        for e in entries {
            org.push(Entry::new(e));
        }

        let mut ret = Self {
            entries: org,
            max: 0.0,
        };
        ret.calc_chance_values();
        ret
    }

    pub fn pull(&self, mut tables_visited: &mut Vec<FixedTableID>) -> Drop {
        let num: f64 = rand::random_range(0.0..self.max);
        for e in &self.entries {
            if e.chance_val > num {
                match e.output.ty {
                    EntryOutputType::Gold => return Drop::new_gold(e.output.amount),
                    EntryOutputType::Item(item_type) => {
                        return Drop::new_item(item_type, e.output.amount)
                    }
                    EntryOutputType::Table(table_id) => {
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
        let mut tables_visited: Vec<FixedTableID> = vec![];

        for e in &self.entries {
            match e.output.ty {
                // Only check the table drops
                EntryOutputType::Table(table_id) => {
                    let _ = get_drop_cycle_check(table_id, &mut tables_visited);
                }
                _ => {}
            }
        }
    }

    pub fn add_entry(&mut self, input: (EntryOutput, f64)) {
        self.entries.push(Entry::new(input));
        self.calc_chance_values();
    }

    fn calc_chance_values(&mut self) {
        let mut accum: f64 = 0.0;
        for e in &mut self.entries {
            accum += e.orig_chance;
            e.chance_val = accum;
        }

        self.max = accum;
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

        assert_eq!(table.entries.len(), 2);
        assert_eq!(table.max, 15.0);

        assert_eq!(table.entries[0].chance_val, 10.0);
        assert_eq!(
            table.entries[0].output.ty,
            EntryOutputType::Item(ItemType::DirtClod)
        );

        assert_eq!(table.entries[1].chance_val, 15.0);
        assert_eq!(
            table.entries[1].output.ty,
            EntryOutputType::Item(ItemType::Tile(TileType::Grass))
        );
    }

    #[test]
    fn table_drop() {
        let pull = get_drop(FixedTableID::TestTable);

        assert_eq!(pull.drop_type, DropType::Gold);
        assert_eq!(pull.amount, 1);
    }

    #[test]
    #[should_panic]
    fn panic_on_cycle() {
        let pull = get_drop(FixedTableID::TestCycleA);
    }

    // create teble by using the add_entry
    #[test]
    fn add_entry() {
        let mut table = DropTable::new(vec![]);
        table.add_entry((EntryOutput::new_item(ItemType::DirtClod, 1), 10.0));
        table.add_entry((EntryOutput::new_tile(TileType::Grass, 1), 5.0));

        assert_eq!(table.entries.len(), 2);
        assert_eq!(table.max, 15.0);

        assert_eq!(table.entries[0].chance_val, 10.0);
        assert_eq!(
            table.entries[0].output.ty,
            EntryOutputType::Item(ItemType::DirtClod)
        );

        assert_eq!(table.entries[1].chance_val, 15.0);
        assert_eq!(
            table.entries[1].output.ty,
            EntryOutputType::Item(ItemType::Tile(TileType::Grass))
        );
    }
}
