use crate::{error::Error, tile::TileInstance, world::*};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    path::Path,
};

pub mod load;

const FILE_NAME: &str = "save_file.gsf";

pub fn save_game(world: &World) -> Result<(), Error> {
    let file_dir = Path::new(FILE_NAME);
    let mut file = File::create(file_dir)?;

    // tile instances count
    let el: u64 = world.entities.len() as u64;
    file.write(&el.to_le_bytes())?;

    // tile instances
    for t in &world.entities {
        // entity id
        t.0.write(&mut file)?;

        // tile instance
        t.1.write(&mut file)?;
    }

    Ok(())
}

pub fn load_game(world: &mut World) -> Result<(), Error> {
    let file_path = Path::new(FILE_NAME);
    let mut file = OpenOptions::new().read(true).open(file_path)?;

    let tiles_count = load::read_u64(&mut file)?;

    for i in 0..tiles_count {
        let eid = EntityID::read(&mut file)?;

        TileInstance::read(&mut file)?;
    }

    Ok(())
}
