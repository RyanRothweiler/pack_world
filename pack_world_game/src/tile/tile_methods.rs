use crate::{
    drop_table::*,
    error::*,
    grid::*,
    harvest_timer::*,
    save_file::{load, *},
    state::{assets::*, *},
    tile::{tile_instance::*, tile_type::*, tiles::*},
    update_signal::*,
    world::*,
};
use gengar_engine::{
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::*,
    vectors::*,
};

pub mod tile_component;

pub use tile_component::*;

/// This is just manual dynamic dispatch because Dyn breaks hot realoding.
#[derive(Debug, Clone)]
pub enum TileMethods {
    Dirt,
    Grass,
    Boulder,
    OakTree(TileOakTree),
    BirdNest(TileBirdNest),
    Cave,
    Shrub,
    MudPit,
    TallGrass,
    Frog,
    Water,
    Newt,
    Reed,
    Clam,
    MudFish,
}

impl TileMethods {
    pub fn to_index(&self) -> i32 {
        match self {
            TileMethods::Dirt => 1,
            TileMethods::Grass => 2,
            TileMethods::Boulder => 3,
            TileMethods::OakTree(state) => 4,
            TileMethods::BirdNest(state) => 5,
            TileMethods::Cave => 6,
            TileMethods::Shrub => 7,
            TileMethods::MudPit => 8,
            TileMethods::TallGrass => 9,
            TileMethods::Frog => 10,
            TileMethods::Water => 11,
            TileMethods::Newt => 12,
            TileMethods::Reed => 13,
            TileMethods::Clam => 14,
            TileMethods::MudFish => 15,
        }
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let type_key = format!("{}.t", key_parent);
        let state_key = format!("{}.s", key_parent);

        save_file.save_i32(&type_key, self.to_index());

        // Save tile specific state
        match self {
            TileMethods::OakTree(state) => {
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::BirdNest(state) => {
                state.save_file_write(state_key, save_file)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn save_file_load(
        key_parent: String,
        grid_pos: GridPos,
        save_file: &SaveFile,
    ) -> Result<Self, Error> {
        let type_key = format!("{}.t", key_parent);
        let state_key = format!("{}.s", key_parent);

        let id = save_file.load_i32(&type_key)?;
        match id {
            1 => Ok(Self::Dirt),
            2 => Ok(Self::Grass),
            3 => Ok(Self::Boulder),
            4 => Ok(TileOakTree::save_file_load(state_key, save_file)?),
            5 => Ok(TileBirdNest::save_file_load(state_key, save_file)?),
            6 => Ok(Self::Cave),
            7 => Ok(Self::Shrub),
            8 => Ok(Self::MudPit),
            9 => Ok(Self::TallGrass),
            10 => Ok(Self::Frog),
            11 => Ok(Self::Water),
            12 => Ok(Self::Newt),
            13 => Ok(Self::Reed),
            14 => Ok(Self::Clam),
            _ => {
                return Err(Error::UnknownTileMethodID(id));
            }
        }
    }
}

mod tests {
    use super::*;
    use crate::save_file::*;

    #[test]
    fn save_load() {
        let mut save_file = SaveFile::new();

        TileMethods::Dirt
            .save_file_write("dirt".into(), &mut save_file)
            .unwrap();
        TileMethods::Grass
            .save_file_write("grass".into(), &mut save_file)
            .unwrap();
        TileMethods::Boulder
            .save_file_write("boulder".into(), &mut save_file)
            .unwrap();
        TileMethods::OakTree(TileOakTree {
            has_nest: true,
            nest_id: Some(EntityID { id: 100 }),
        })
        .save_file_write("oak tree".into(), &mut save_file)
        .unwrap();
        TileMethods::BirdNest(TileBirdNest {
            tree_origin: GridPos::new(10, 20),
        })
        .save_file_write("bird nest".into(), &mut save_file)
        .unwrap();
        TileMethods::Cave
            .save_file_write("cave".into(), &mut save_file)
            .unwrap();
        TileMethods::Shrub
            .save_file_write("shrub".into(), &mut save_file)
            .unwrap();
        TileMethods::MudPit
            .save_file_write("mudpit".into(), &mut save_file)
            .unwrap();
        TileMethods::TallGrass
            .save_file_write("tall_grass".into(), &mut save_file)
            .unwrap();
        TileMethods::Frog
            .save_file_write("frog".into(), &mut save_file)
            .unwrap();
        TileMethods::Water
            .save_file_write("water".into(), &mut save_file)
            .unwrap();
        TileMethods::Newt
            .save_file_write("newt".into(), &mut save_file)
            .unwrap();
        TileMethods::Reed
            .save_file_write("reed".into(), &mut save_file)
            .unwrap();
        TileMethods::Clam
            .save_file_write("clam".into(), &mut save_file)
            .unwrap();

        match TileMethods::save_file_load("dirt".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Dirt => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("grass".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Grass => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("boulder".into(), GridPos::new(0, 0), &save_file).unwrap()
        {
            TileMethods::Boulder => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("oak tree".into(), GridPos::new(0, 0), &save_file)
            .unwrap()
        {
            TileMethods::OakTree(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("bird nest".into(), GridPos::new(0, 0), &save_file)
            .unwrap()
        {
            TileMethods::BirdNest(state) => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("cave".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Cave => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("shrub".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Shrub => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("mudpit".into(), GridPos::new(0, 0), &save_file).unwrap()
        {
            TileMethods::MudPit => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("tall_grass".into(), GridPos::new(0, 0), &save_file)
            .unwrap()
        {
            TileMethods::TallGrass => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("frog".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Frog => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("water".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Water => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("newt".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Newt => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("reed".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Reed => {}
            _ => panic!("Incorrect"),
        }
        match TileMethods::save_file_load("clam".into(), GridPos::new(0, 0), &save_file).unwrap() {
            TileMethods::Clam => {}
            _ => panic!("Incorrect"),
        }
    }
}
