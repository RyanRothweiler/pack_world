use crate::{error::Error, tile::tile_instance::TileInstance, world::*};
use gengar_engine::platform_api::*;
use std::io::{Cursor, Read, Seek, Write};

pub mod kvp_file;
pub mod load;

pub use kvp_file::SaveFile;

pub const TILE_INSTANCE_ID_CHAR: char = 'E';

/*
fn save_game_cursor<W: Write + Seek>(world: &World, writer: &mut W) -> Result<(), Error> {
    // tile instances count
    let el: u64 = world.entities.len() as u64;
    writer.write(&el.to_le_bytes())?;

    // tile instances
    for t in &world.entities {
        // entity id
        t.0.write(writer)?;

        // tile instance
        t.1.write(writer)?;
    }

    Ok(())
}
*/

pub fn build_save_file(world: &World) -> Result<SaveFile, Error> {
    let mut save_file = SaveFile::new();

    // save_file.save_i32("e_count", world.entities.len() as i32);

    for t in &world.entities {
        let key = format!("{}.{}", TILE_INSTANCE_ID_CHAR, t.0.id);
        save_file.save_u64(&key, t.0.id);

        t.1.save_file_write(format!("{}", t.0.id), &mut save_file)?;

        // entity id
        // t.0.save_file_write(&mut save_file)?;

        // tile instance
        // t.1.write(writer)?;
    }

    Ok(save_file)
}

pub fn save_game(world: &World, platform_api: &PlatformApi) -> Result<(), Error> {
    let save_file = build_save_file(world)?;

    let mut write_data: Vec<u8> = vec![];
    let mut write_cursor = Cursor::new(write_data);
    save_file.write_file(&mut write_cursor).unwrap();

    (platform_api.write_save_game_data)(write_cursor.get_ref().to_vec())
        .map_err(|e| Error::EngineError(format!("{:?}", e)))?;
    Ok(())
}

/*
fn load_game_cursor<W: Read>(world: &mut World, reader: &mut W) -> Result<(), Error> {
    let tiles_count = load::read_u64(reader)?;

    for i in 0..tiles_count {
        let eid = EntityID::read(reader)?;
        let tile_inst = TileInstance::read(reader)?;

        world.raw_insert_entity(eid, tile_inst);
    }

    Ok(())
}
*/

pub fn load_game(world: &mut World, data: &Vec<u8>) {
    world.clear();

    let mut cursor = Cursor::new(data);
    let save_file = SaveFile::read_file(&mut cursor).unwrap();

    for (key, value) in &save_file.entries {
        // check if is tile
        let parts: Vec<&str> = key.split('.').collect();

        if parts[0].starts_with(TILE_INSTANCE_ID_CHAR) {
            let eid = EntityID {
                id: save_file.load_u64(key).unwrap(),
            };

            let tile_instance =
                TileInstance::save_file_load(format!("{}", eid.id), &save_file).unwrap();

            world.raw_insert_entity(eid, tile_instance);
            // println!("loaded tile {} {:?}", eid.id, tile_instance.tile_type);
        }
    }

    // let mut cursor = Cursor::new(data);
    // load_game_cursor(world, &mut cursor).unwrap();
}
