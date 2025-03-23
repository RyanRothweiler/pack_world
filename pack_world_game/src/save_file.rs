use crate::{error::Error, tile::tile_instance::TileInstance, world::*};
use gengar_engine::platform_api::*;
use std::io::{Cursor, Read, Seek, Write};

pub mod load;

pub mod kvp_file;

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

fn load_game_cursor<W: Read>(world: &mut World, reader: &mut W) -> Result<(), Error> {
    let tiles_count = load::read_u64(reader)?;

    for i in 0..tiles_count {
        let eid = EntityID::read(reader)?;
        let tile_inst = TileInstance::read(reader)?;

        world.raw_insert_entity(eid, tile_inst);
    }

    Ok(())
}

pub fn load_game(world: &mut World, data: &Vec<u8>) {
    world.clear();

    let mut cursor = Cursor::new(data);
    load_game_cursor(world, &mut cursor).unwrap();
}

/*
#[cfg(test)]
mod test {
    use super::*;

    fn save_load() {
        let save_state = get_save_data();
    }
}
*/
