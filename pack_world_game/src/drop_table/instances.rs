use crate::{drop_table::*, pack::*};

#[cfg(feature = "dev")]
mod test_tables;
#[cfg(feature = "dev")]
pub use test_tables::*;

mod drop_table_boulder;
mod drop_table_grass;
mod drop_table_oak_tree;
mod drop_table_pack_starter;

use drop_table_boulder::*;
use drop_table_grass::*;
use drop_table_oak_tree::*;
use drop_table_pack_starter::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum DropTableID {
    // tiles
    Grass,
    Boulder,
    OakTree,

    Pack(PackID),

    // testing
    #[cfg(feature = "dev")]
    TestTable,
    #[cfg(feature = "dev")]
    TestGold,
    #[cfg(feature = "dev")]
    TestCycleA,
    #[cfg(feature = "dev")]
    TestCycleB,
}

pub fn get_drop(table: DropTableID) -> Drop {
    let mut tables_visited: Vec<DropTableID> = vec![];
    get_drop_cycle_check(table, &mut tables_visited)
}

pub fn get_drop_cycle_check(table_id: DropTableID, tables_visited: &mut Vec<DropTableID>) -> Drop {
    match table_id {
        DropTableID::Grass => GRASS.pull(tables_visited),
        DropTableID::Boulder => BOULDER.pull(tables_visited),
        DropTableID::OakTree => OAK_TREE.pull(tables_visited),

        DropTableID::Pack(pack_id) => match pack_id {
            PackID::Starter => PACK_STARTER.pull(tables_visited),
        },

        #[cfg(feature = "dev")]
        DropTableID::TestTable => TEST_TABLE.pull(tables_visited),
        #[cfg(feature = "dev")]
        DropTableID::TestGold => TEST_GOLD.pull(tables_visited),
        #[cfg(feature = "dev")]
        DropTableID::TestCycleA => TEST_CYCLE_A.pull(tables_visited),
        #[cfg(feature = "dev")]
        DropTableID::TestCycleB => TEST_CYCLE_B.pull(tables_visited),
    }
}
