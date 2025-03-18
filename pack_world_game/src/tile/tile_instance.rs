use crate::{
    drop_table::*,
    error::Error,
    grid::*,
    save_file::load,
    tile::{TileMethods, TileType},
    update_signal::*,
    world::*,
};
use gengar_engine::platform_api::*;
use std::{fs::File, io::Write};

// TODO make these private?
pub struct TileInstance {
    pub tile_type: TileType,
    pub grid_pos: GridPos,
    pub methods: TileMethods,

    // for giving offset drops
    pub drop_timer: f64,
    pub drops_queue: Vec<Drop>,
}

impl TileInstance {
    pub fn new(tile_type: TileType, grid_pos: GridPos, methods: TileMethods) -> Self {
        Self {
            tile_type,
            grid_pos,
            methods,
            drop_timer: 0.0,
            drops_queue: vec![],
        }
    }

    pub fn harvest(&mut self, world_snapshot: &WorldSnapshot, platform_api: &PlatformApi) {
        let mut new_drop = self
            .methods
            .harvest(self.grid_pos, world_snapshot, platform_api);

        match new_drop {
            Some(drop) => {
                self.drops_queue.append(&mut drop.to_individual());
            }
            None => {
                println!("Attempted to harvest something which isn't harvestable.");
                println!(
                    "This is fine. Nothing will break. But this indicates an issue somewhere."
                );
            }
        }
    }

    pub fn update(&mut self, delta_time: f64) -> Vec<UpdateSignal> {
        if self.drops_queue.len() > 0 {
            self.drop_timer += delta_time;

            if self.drop_timer > 0.06 {
                self.drop_timer = 0.0;

                return vec![UpdateSignal::AddHarvestDrop {
                    drop: self.drops_queue.pop().unwrap(),
                    origin: grid_to_world(&self.grid_pos),
                }];
            }
        }

        vec![]
    }

    pub fn write(&self, file: &mut File) -> Result<(), Error> {
        file.write(&self.tile_type.to_index().to_le_bytes())?;

        file.write(&self.grid_pos.x.to_le_bytes())?;
        file.write(&self.grid_pos.y.to_le_bytes())?;

        Ok(())
    }

    pub fn read(file: &mut File) -> Result<(), Error> {
        let idx = load::read_i32(file)?;
        let tile_type: TileType = TileType::from_index(idx)?;

        let mut grid_pos = GridPos::new(0, 0);
        grid_pos.x = load::read_i32(file)?;
        grid_pos.y = load::read_i32(file)?;

        println!("tile type {:?} {:?}", tile_type, grid_pos);
        Ok(())
    }
}
