use crate::{error::Error, grid::*, tile::tile_instance::TileInstance, world::*};
use gengar_engine::platform_api::*;
use std::io::{Cursor, Read, Seek, Write};

pub mod kvp_file;
pub mod load;

pub use kvp_file::SaveFile;

pub const TILE_INSTANCE_ID_CHAR: char = 'E';
pub const VALID_ADJ_ID_CHAR: char = 'V';

pub fn build_save_file(world: &World) -> Result<SaveFile, Error> {
    let mut save_file = SaveFile::new();

    // write tile instances
    for t in &world.entities {
        let key = format!("{}.{}", TILE_INSTANCE_ID_CHAR, t.0.id);
        save_file.save_u64(&key, t.0.id);

        t.1.save_file_write(format!("{}", t.0.id), &mut save_file)?;
    }

    // write valid adjacent positions
    for (i, (key, value)) in world.valids.iter().enumerate() {
        let id_key = format!("{}.{}", VALID_ADJ_ID_CHAR, i);
        save_file.save_i32(&id_key, i as i32);

        save_file.save_i32(&format!("valid_x.{}", i as i32), key.x);
        save_file.save_i32(&format!("valid_y.{}", i as i32), key.y);
    }

    save_file.save_u64("next_entity_id", world.next_entity_id);

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

pub fn load_game(world: &mut World, data: &Vec<u8>) {
    world.clear();

    let mut cursor = Cursor::new(data);
    let save_file = SaveFile::read_file(&mut cursor).unwrap();

    world.next_entity_id = save_file.load_u64("next_entity_id").unwrap();

    for (key, value) in &save_file.entries {
        // check if is tile
        let parts: Vec<&str> = key.split('.').collect();

        // tile instance
        if parts[0].starts_with(TILE_INSTANCE_ID_CHAR) {
            let eid = EntityID {
                id: save_file.load_u64(key).unwrap(),
            };

            let tile_instance =
                TileInstance::save_file_load(format!("{}", eid.id), &save_file).unwrap();

            world.raw_insert_entity(eid, tile_instance);
        } else if parts[0].starts_with(VALID_ADJ_ID_CHAR) {
            let i = save_file.load_i32(key).unwrap();

            let key_x = &format!("valid_x.{}", i as i32);
            let key_y = &format!("valid_y.{}", i as i32);

            let grid_pos = GridPos::new(
                save_file.load_i32(&key_x).unwrap(),
                save_file.load_i32(&key_y).unwrap(),
            );

            world.valids.insert(grid_pos, true);
        }
    }
}
