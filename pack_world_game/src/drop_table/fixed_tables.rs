use crate::{drop_table::*, error::*, pack::*, save_file::*};
use gengar_engine::platform_api::*;

#[cfg(test)]
mod test_tables;
#[cfg(test)]
pub use test_tables::*;

// item
mod drop_table_boulder;
mod drop_table_cave;
mod drop_table_clam;
mod drop_table_dirt;
mod drop_table_frog;
mod drop_table_grass;
mod drop_table_mud_pit;
mod drop_table_oak_tree;
mod drop_table_shrub;
mod drop_table_small_gold;
mod drop_table_tall_grass;

use drop_table_boulder::*;
use drop_table_cave::*;
use drop_table_clam::*;
use drop_table_dirt::*;
use drop_table_frog::*;
use drop_table_grass::*;
use drop_table_mud_pit::*;
use drop_table_oak_tree::*;
use drop_table_shrub::*;
use drop_table_small_gold::*;
use drop_table_tall_grass::*;

// packs
mod drop_table_pack_mud;
mod drop_table_pack_starter;
mod drop_table_pack_stick;
mod drop_table_pack_water;

use drop_table_pack_mud::*;
use drop_table_pack_starter::*;
use drop_table_pack_stick::*;
use drop_table_pack_water::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum FixedTableID {
    // tiles
    Grass,
    Boulder,
    OakTree,
    SmallGold,
    Cave,
    Shrub,
    MudPit,
    TallGrass,
    Frog,
    Clam,
    Dirt,

    Pack(PackID),

    // testing
    #[cfg(test)]
    TestTable,
    #[cfg(test)]
    TestGold,
    #[cfg(test)]
    TestCycleA,
    #[cfg(test)]
    TestCycleB,
}

impl FixedTableID {
    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let type_key = format!("{}.t", key_parent);

        match self {
            FixedTableID::Grass => {
                save_file.save_i32(&type_key, 0);
            }
            FixedTableID::Boulder => {
                save_file.save_i32(&type_key, 1);
            }
            FixedTableID::OakTree => {
                save_file.save_i32(&type_key, 2);
            }
            FixedTableID::SmallGold => {
                save_file.save_i32(&type_key, 3);
            }
            FixedTableID::Cave => {
                save_file.save_i32(&type_key, 4);
            }
            FixedTableID::Shrub => {
                save_file.save_i32(&type_key, 5);
            }

            FixedTableID::Pack(pack_id) => {
                save_file.save_i32(&type_key, 6);

                let pack_type_key = format!("{}.t.p", key_parent);
                save_file.save_i32(&pack_type_key, pack_id.to_index());
            }

            FixedTableID::MudPit => {
                save_file.save_i32(&type_key, 7);
            }
            FixedTableID::TallGrass => {
                save_file.save_i32(&type_key, 8);
            }
            FixedTableID::Frog => {
                save_file.save_i32(&type_key, 9);
            }
            FixedTableID::Clam => {
                save_file.save_i32(&type_key, 10);
            }
            FixedTableID::Dirt => {
                save_file.save_i32(&type_key, 11);
            }

            #[cfg(test)]
            FixedTableID::TestTable
            | FixedTableID::TestGold
            | FixedTableID::TestCycleA
            | FixedTableID::TestCycleB => {
                panic!("Should never try to serialize a test table.");
            }
        };

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let type_key = format!("{}.t", key_parent);
        let ty = save_file.load_i32(&type_key)?;

        let fixed_id = match ty {
            0 => FixedTableID::Grass,
            1 => FixedTableID::Boulder,
            2 => FixedTableID::OakTree,
            3 => FixedTableID::SmallGold,
            4 => FixedTableID::Cave,
            5 => FixedTableID::Shrub,
            6 => {
                let pack_type_key = format!("{}.t.p", key_parent);
                let pack_type = save_file.load_i32(&pack_type_key)?;

                FixedTableID::Pack(PackID::from_index(pack_type))
            }
            7 => FixedTableID::MudPit,
            8 => FixedTableID::TallGrass,
            9 => FixedTableID::Frog,
            10 => FixedTableID::Clam,
            12 => FixedTableID::Dirt,

            _ => return Err(Error::UnknownFixedTableID(ty)),
        };

        Ok(fixed_id)
    }
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
        FixedTableID::MudPit => &MUD_PIT,
        FixedTableID::TallGrass => &TALL_GRASS,
        FixedTableID::Frog => &FROG,
        FixedTableID::Clam => &CLAM,
        FixedTableID::Dirt => &DIRT,

        FixedTableID::Pack(pack_id) => match pack_id {
            PackID::Starter => &PACK_STARTER,
            PackID::Stick => &PACK_STICK,
            PackID::Mud => &PACK_MUD,
            PackID::Water => &PACK_WATER,
        },

        #[cfg(test)]
        FixedTableID::TestTable => &TEST_TABLE,
        #[cfg(test)]
        FixedTableID::TestGold => &TEST_GOLD,
        #[cfg(test)]
        FixedTableID::TestCycleA => &TEST_CYCLE_A,
        #[cfg(test)]
        FixedTableID::TestCycleB => &TEST_CYCLE_B,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save_file::*;

    #[test]
    fn save_load() {
        let mut file = SaveFile::new();

        FixedTableID::Grass
            .save_file_write("0".into(), &mut file)
            .unwrap();
        FixedTableID::Boulder
            .save_file_write("1".into(), &mut file)
            .unwrap();
        FixedTableID::OakTree
            .save_file_write("2".into(), &mut file)
            .unwrap();
        FixedTableID::SmallGold
            .save_file_write("3".into(), &mut file)
            .unwrap();
        FixedTableID::Cave
            .save_file_write("4".into(), &mut file)
            .unwrap();
        FixedTableID::Shrub
            .save_file_write("5".into(), &mut file)
            .unwrap();
        FixedTableID::Pack(PackID::Starter)
            .save_file_write("6".into(), &mut file)
            .unwrap();
        FixedTableID::Pack(PackID::Stick)
            .save_file_write("7".into(), &mut file)
            .unwrap();

        FixedTableID::MudPit
            .save_file_write("mud_pit".into(), &mut file)
            .unwrap();

        FixedTableID::TallGrass
            .save_file_write("tall_grass".into(), &mut file)
            .unwrap();

        let loaded = FixedTableID::save_file_load("0".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::Grass);

        let loaded = FixedTableID::save_file_load("1".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::Boulder);

        let loaded = FixedTableID::save_file_load("2".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::OakTree);

        let loaded = FixedTableID::save_file_load("3".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::SmallGold);

        let loaded = FixedTableID::save_file_load("4".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::Cave);

        let loaded = FixedTableID::save_file_load("5".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::Shrub);

        let loaded = FixedTableID::save_file_load("6".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::Pack(PackID::Starter));

        let loaded = FixedTableID::save_file_load("7".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::Pack(PackID::Stick));

        let loaded = FixedTableID::save_file_load("mud_pit".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::MudPit);

        let loaded = FixedTableID::save_file_load("tall_grass".into(), &mut file).unwrap();
        assert_eq!(loaded, FixedTableID::TallGrass);
    }
}
