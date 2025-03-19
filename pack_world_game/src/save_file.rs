use crate::{error::Error, tile::TileInstance, world::*};
use gengar_engine::platform_api::*;
use std::io::{Cursor, Read, Seek, Write};

pub mod load;

fn save_game_cursor<W: Write + Seek>(world: &World, writer: &mut W) -> Result<(), Error> {
    // tile instances count
    let el: u64 = world.entities.len() as u64;
    writer.write(&el.to_le_bytes())?;

    /*
    // tile instances
    for t in &world.entities {
        // entity id
        t.0.write(&mut file)?;

        // tile instance
        t.1.write(&mut file)?;
    }
    */

    Ok(())
}

fn load_game_cursor<W: Read>(reader: &mut W) -> Result<(), Error> {
    let tiles_count = load::read_u64(reader)?;

    println!("tiles count {}", tiles_count);

    /*
    for i in 0..tiles_count {
        let eid = EntityID::read(&mut file)?;

        TileInstance::read(&mut file)?;
    }
    */

    Ok(())
}

pub fn get_save_data(world: &World) -> Result<Vec<u8>, Error> {
    let mut buf: Vec<u8> = vec![];
    let mut cursor = Cursor::new(buf);

    save_game_cursor(world, &mut cursor)?;

    Ok(cursor.into_inner())
}

pub fn save_game(world: &World, platform_api: &PlatformApi) -> Result<(), Error> {
    let save_data = get_save_data(world)?;
    (platform_api.write_save_game_data)(save_data)
        .map_err(|e| Error::EngineError(format!("{:?}", e)))?;
    Ok(())
}

pub fn load_game(data: Vec<u8>) -> Result<(), Error> {
    let mut cursor = Cursor::new(data);

    load_game_cursor(&mut cursor).map_err(|e| Error::EngineError(format!("{:?}", e)))?;

    Ok(())
}
