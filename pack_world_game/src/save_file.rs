use crate::account_system::*;
use crate::{
    error::Error, grid::*, state::inventory::*, tile::tile_instance::TileInstance, world::*,
};
use elara_engine::platform_api::*;
use std::io::{Cursor, Read, Seek, Write};

pub mod kvp_file;
pub mod load;

pub use kvp_file::SaveFile;

pub const TILE_INSTANCE_ID_CHAR: char = 'E';
pub const VALID_ADJ_ID_CHAR: char = 'V';
pub const DROP_MOD_CHAR: char = 'G';

pub const SIM_LIMIT_H_FREE: i32 = 6;
pub const SIM_LIMIT_H_PREMIUM: i32 = 24;

pub fn build_save_file(
    world: &World,
    inventory: &Inventory,
    platform_api: &PlatformApi,
) -> Result<SaveFile, Error> {
    let mut save_file = SaveFile::new();

    // world stuff. This should probably move into world struct
    {
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

        // write global drop mods
        for (i, (key, value)) in world.drop_count_mod.iter().enumerate() {
            let id_key = format!("{}.{}", DROP_MOD_CHAR, i);
            save_file.save_i32(&id_key, i as i32);

            save_file.save_i32(&format!("drop_mod_x.{}", i as i32), key.x);
            save_file.save_i32(&format!("drop_mod_y.{}", i as i32), key.y);
            save_file.save_f64(&format!("drop_mod_v.{}", i as i32), *value);
        }

        save_file.save_u64("next_entity_id", world.next_entity_id);
    }

    inventory.save_file_write("".into(), &mut save_file)?;

    save_file.save_f64("unix_time_saved", (platform_api.epoch_time_ms)());

    Ok(save_file)
}

pub fn save_game(
    world: &World,
    inventory: &Inventory,
    platform_api: &PlatformApi,
) -> Result<(), Error> {
    let save_file = build_save_file(world, inventory, platform_api)?;

    let mut write_data: Vec<u8> = vec![];
    let mut write_cursor = Cursor::new(write_data);
    save_file.write_file(&mut write_cursor).unwrap();

    (platform_api.write_save_game_data)(write_cursor.get_ref().to_vec())
        .map_err(|e| Error::EngineError(format!("{:?}", e)))?;
    Ok(())
}

/// returns the ms that is needed to forward simulate
pub fn load_game(
    world: &mut World,
    inventory: &mut Inventory,
    data: &Vec<u8>,
    account_system: &AccountSystem,
    platform_api: &PlatformApi,
) -> Result<f64, Error> {
    world.clear();
    inventory.clear();

    let mut cursor = Cursor::new(data);
    let save_file = SaveFile::read_file(&mut cursor).unwrap();

    // world stuff
    {
        world.next_entity_id = save_file.load_u64("next_entity_id")?;

        for (key, value) in &save_file.entries {
            // check if is tile
            let parts: Vec<&str> = key.split('.').collect();

            // tile instance
            if parts[0].starts_with(TILE_INSTANCE_ID_CHAR) {
                let eid = EntityID {
                    id: save_file.load_u64(key)?,
                };

                let tile_instance =
                    TileInstance::save_file_load(format!("{}", eid.id), &save_file)?;

                world.raw_insert_entity(eid, tile_instance);
            } else if parts[0].starts_with(VALID_ADJ_ID_CHAR) {
                let i = save_file.load_i32(key)?;

                let key_x = &format!("valid_x.{}", i as i32);
                let key_y = &format!("valid_y.{}", i as i32);

                let grid_pos =
                    GridPos::new(save_file.load_i32(&key_x)?, save_file.load_i32(&key_y)?);

                world.valids.insert(grid_pos, true);
            } else if parts[0].starts_with(DROP_MOD_CHAR) {
                let i = save_file.load_i32(key)?;

                let key_x = &format!("drop_mod_x.{}", i as i32);
                let key_y = &format!("drop_mod_y.{}", i as i32);
                let key_v = &format!("drop_mod_v.{}", i as i32);

                let gp = GridPos::new(save_file.load_i32(&key_x)?, save_file.load_i32(&key_y)?);
                let val: f64 = save_file.load_f64(key_v)?;

                world.drop_count_mod.insert(gp, val);
            }
        }
    }

    let inv = Inventory::save_file_load("".into(), &save_file)?;
    inventory.items_seen = inv.items_seen;
    inventory.items = inv.items;
    inventory.gold = inv.gold;
    inventory.limit = inv.limit;

    let time_now = (platform_api.epoch_time_ms)();
    let time_saved = save_file.load_f64("unix_time_saved")?;

    let sim_limit_hour = if account_system.user_purchased_base() {
        SIM_LIMIT_H_PREMIUM
    } else {
        SIM_LIMIT_H_FREE
    };
    let sim_limit_ms = sim_limit_hour as f64 * 60.0 * 60.0 * 1000.0;

    return Ok((time_now - time_saved).clamp(0.0, sim_limit_ms));
}
