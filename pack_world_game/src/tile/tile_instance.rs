use crate::{
    drop_table::*,
    error::Error,
    grid::*,
    save_file::{load, *},
    tile::{TileMethods, TileType},
    update_signal::*,
    world::*,
};
use gengar_engine::platform_api::*;

// TODO make these private?
pub struct TileInstance {
    pub tile_type: TileType,
    pub grid_pos: GridPos,
    pub methods: TileMethods,

    // for giving offset drops
    pub drop_timer: f64,
    pub drops_queue: Vec<Drop>,
    pub destroy_after_drops: bool,
}

impl TileInstance {
    pub fn new(tile_type: TileType, grid_pos: GridPos, methods: TileMethods) -> Self {
        Self {
            tile_type,
            grid_pos,
            methods,

            drop_timer: 0.0,
            drops_queue: vec![],
            destroy_after_drops: false,
        }
    }

    pub fn harvest(
        &mut self,
        world_snapshot: &WorldSnapshot,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        let mut harvest_data = self
            .methods
            .harvest(self.grid_pos, world_snapshot, platform_api);

        match harvest_data.0 {
            Some(drop) => {
                self.drops_queue.append(&mut drop.to_individual());

                match self.tile_type {
                    TileType::Reed => {
                        self.destroy_after_drops = true;
                    }
                    _ => {}
                }
            }
            None => {
                println!("Attempted to harvest something which isn't harvestable.");
                println!(
                    "This is fine. Nothing will break. But this indicates an issue somewhere."
                );
            }
        }

        harvest_data.1
    }

    pub fn update(&mut self, delta_time: f64) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        if self.drops_queue.len() > 0 {
            self.drop_timer += delta_time;

            if self.drop_timer > 0.06 {
                self.drop_timer = 0.0;

                return vec![UpdateSignal::AddHarvestDrop {
                    drop: self.drops_queue.pop().unwrap(),
                    origin: grid_to_world(&self.grid_pos),
                }];
            }
        } else {
            if self.destroy_after_drops {
                ret.push(UpdateSignal::DestroyTile {
                    pos: self.grid_pos,
                    layer: self.tile_type.get_layer(),
                });
            }
        }

        ret
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let type_key = format!("{}.type", key_parent);
        let grid_x_key = format!("{}.x", key_parent);
        let grid_y_key = format!("{}.y", key_parent);

        save_file.save_i32(&type_key, self.tile_type.to_index());
        save_file.save_i32(&grid_x_key, self.grid_pos.x);
        save_file.save_i32(&grid_y_key, self.grid_pos.y);

        let methods_key = format!("{}.m", key_parent);
        self.methods.save_file_write(methods_key, save_file)?;

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let type_key = format!("{}.type", key_parent);
        let grid_x_key = format!("{}.x", key_parent);
        let grid_y_key = format!("{}.y", key_parent);

        let type_index = save_file.load_i32(&type_key).unwrap();

        let tile_type: TileType = TileType::from_index(type_index)?;

        let mut grid_pos = GridPos::new(0, 0);
        grid_pos.x = save_file.load_i32(&grid_x_key).unwrap();
        grid_pos.y = save_file.load_i32(&grid_y_key).unwrap();

        let methods =
            TileMethods::save_file_load(format!("{}.m", key_parent), grid_pos, save_file)?;

        Ok(TileInstance::new(tile_type, grid_pos, methods))
    }
}
