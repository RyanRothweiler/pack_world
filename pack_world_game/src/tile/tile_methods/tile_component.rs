use crate::{
    assets::*,
    error::Error,
    grid::*,
    save_file::{load, *},
    tile::harvest_timer::*,
    update_signal::*,
};
use gengar_engine::render::{render_pack::*, shader::*, *};

pub mod tile_component_auto_death;
pub mod tile_component_wander;

pub use tile_component_auto_death::*;
pub use tile_component_wander::*;

#[derive(Debug)]
pub enum TileComponent {
    /// Harvesting behavior
    Harvestable { timer: HarvestTimer },

    /// Tile wanders around a grid (like frog and newt tile )
    Wander { state: WanderState },

    /// Destroy this tile after a set time
    AutoDeath { state: AutoDeathState },
}

impl TileComponent {
    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        match self {
            Self::Harvestable { timer } => {
                let key = format!("{}.ht", key_parent);
                timer.save_file_write(key, save_file)?;
            }
            Self::Wander { state } => {}
            Self::AutoDeath { state } => {
                // todo here
            }
        }
        Ok(())
    }

    pub fn save_file_load(
        &mut self,
        key_parent: String,
        save_file: &SaveFile,
    ) -> Result<(), Error> {
        match self {
            Self::Harvestable { timer } => {
                let key = format!("{}.ht", key_parent);
                *timer = HarvestTimer::save_file_load(key, save_file)?;
            }
            Self::Wander { state } => {}
            Self::AutoDeath { state } => {
                // todo here
            }
        }

        Ok(())
    }
}
