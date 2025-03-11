use crate::{drop_table::*, pack::*};
use gengar_engine::platform_api::*;

#[cfg(feature = "dev")]
mod test_tables;
#[cfg(feature = "dev")]
pub use test_tables::*;

// item
mod drop_table_boulder;
mod drop_table_cave;
mod drop_table_grass;
mod drop_table_oak_tree;
mod drop_table_shrub;
mod drop_table_small_gold;

use drop_table_boulder::*;
use drop_table_cave::*;
use drop_table_grass::*;
use drop_table_oak_tree::*;
use drop_table_shrub::*;
use drop_table_small_gold::*;

// packs
mod drop_table_pack_starter;
mod drop_table_pack_stick;

use drop_table_pack_starter::*;
use drop_table_pack_stick::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum FixedTableID {
    // tiles
    Grass,
    Boulder,
    OakTree,
    SmallGold,
    Cave,
    Shrub,

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

pub fn get_drop(table: FixedTableID, platform_api: &PlatformApi) -> Drop {
    let mut tables_visited: Vec<FixedTableID> = vec![];
    get_drop_cycle_check(table, &mut tables_visited, platform_api)
}

pub fn get_drop_cycle_check(
    table_id: FixedTableID,
    tables_visited: &mut Vec<FixedTableID>,
    platform_api: &PlatformApi,
) -> Drop {
    let table = get_fixed_table(table_id);
    return table.pull(tables_visited, platform_api);
}

pub fn get_fixed_table<'a>(id: FixedTableID) -> &'a DropTable {
    match id {
        FixedTableID::Grass => &GRASS,
        FixedTableID::Boulder => &BOULDER,
        FixedTableID::OakTree => &OAK_TREE,
        FixedTableID::SmallGold => &SMALL_GOLD,
        FixedTableID::Cave => &CAVE,
        FixedTableID::Shrub => &SHRUB,

        FixedTableID::Pack(pack_id) => match pack_id {
            PackID::Starter => &PACK_STARTER,
            PackID::Stick => &PACK_STICK,
        },

        #[cfg(feature = "dev")]
        FixedTableID::TestTable => &TEST_TABLE,
        #[cfg(feature = "dev")]
        FixedTableID::TestGold => &TEST_GOLD,
        #[cfg(feature = "dev")]
        FixedTableID::TestCycleA => &TEST_CYCLE_A,
        #[cfg(feature = "dev")]
        FixedTableID::TestCycleB => &TEST_CYCLE_B,
    }
}
