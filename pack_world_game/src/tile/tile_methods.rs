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
}

impl TileMethods {
    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let type_key = format!("{}.t", key_parent);
        let state_key = format!("{}.s", key_parent);

        match self {
            TileMethods::Dirt => {
                let id: i32 = 1;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Grass => {
                let id: i32 = 2;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Boulder => {
                let id: i32 = 3;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::OakTree(state) => {
                let id: i32 = 4;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::BirdNest(state) => {
                let id: i32 = 5;

                save_file.save_i32(&type_key, id);
                state.save_file_write(state_key, save_file)?;
            }
            TileMethods::Cave => {
                let id: i32 = 6;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Shrub => {
                let id: i32 = 7;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::MudPit => {
                let id: i32 = 8;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::TallGrass => {
                let id: i32 = 9;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Frog => {
                let id: i32 = 10;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Water => {
                let id: i32 = 11;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Newt => {
                let id: i32 = 12;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Reed => {
                let id: i32 = 13;

                save_file.save_i32(&type_key, id);
            }
            TileMethods::Clam => {
                let id: i32 = 14;

                save_file.save_i32(&type_key, id);
            }
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

        todo!();

        /*
        let id = save_file.load_i32(&type_key).unwrap();
        match id {
            1 => Ok(tile_dirt::new_methods(GridPos::new(0, 0))),
            2 => Ok(tile_grass::save_file_load(state_key, save_file)?),
            3 => Ok(tile_boulder::save_file_load(state_key, save_file)?),
            4 => Ok(TileOakTree::save_file_load(state_key, save_file)?),
            5 => Ok(TileBirdNest::save_file_load(state_key, save_file)?),
            6 => Ok(tile_cave::save_file_load(state_key, save_file)?),
            7 => Ok(tile_shrub::save_file_load(state_key, save_file)?),
            8 => Ok(tile_mud_pit::save_file_load(state_key, save_file)?),
            9 => Ok(tile_tall_grass::save_file_load(state_key, save_file)?),
            10 => Ok(tile_frog::save_file_load(state_key, grid_pos, save_file)?),
            11 => Ok(tile_water::new_methods(GridPos::new(0, 0))),
            12 => Ok(tile_newt::save_file_load(state_key, grid_pos, save_file)?),
            13 => Ok(tile_reed::save_file_load(state_key, save_file)?),
            14 => Ok(tile_clam::save_file_load(state_key, save_file)?),
            _ => {
                return Err(Error::UnknownTileMethodID(id));
            }
        }
        */
    }
}

mod tests {
    use super::*;
    use crate::save_file::*;

    #[test]
    fn save_load() {
        let mut save_file = SaveFile::new();

        tile_dirt::new_methods(GridPos::new(0, 0))
            .save_file_write("dirt".into(), &mut save_file)
            .unwrap();
        tile_grass::new_methods(GridPos::new(0, 0))
            .save_file_write("grass".into(), &mut save_file)
            .unwrap();
        tile_boulder::new_methods(GridPos::new(0, 0))
            .save_file_write("boulder".into(), &mut save_file)
            .unwrap();
        TileOakTree::new_methods(GridPos::new(0, 0))
            .save_file_write("oak tree".into(), &mut save_file)
            .unwrap();
        TileBirdNest::new_methods(GridPos::new(0, 0))
            .save_file_write("bird nest".into(), &mut save_file)
            .unwrap();
        tile_cave::new_methods(GridPos::new(0, 0))
            .save_file_write("cave".into(), &mut save_file)
            .unwrap();
        tile_shrub::new_methods(GridPos::new(0, 0))
            .save_file_write("shrub".into(), &mut save_file)
            .unwrap();
        tile_mud_pit::new_methods(GridPos::new(0, 0))
            .save_file_write("mudpit".into(), &mut save_file)
            .unwrap();
        tile_tall_grass::new_methods(GridPos::new(0, 0))
            .save_file_write("tall_grass".into(), &mut save_file)
            .unwrap();
        tile_frog::new_methods(GridPos::new(5, 5))
            .save_file_write("frog".into(), &mut save_file)
            .unwrap();
        tile_water::new_methods(GridPos::new(0, 0))
            .save_file_write("water".into(), &mut save_file)
            .unwrap();
        tile_newt::new_methods(GridPos::new(5, 5))
            .save_file_write("newt".into(), &mut save_file)
            .unwrap();
        tile_reed::new_methods(GridPos::new(0, 0))
            .save_file_write("reed".into(), &mut save_file)
            .unwrap();
        tile_clam::new_methods(GridPos::new(0, 0))
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
