use crate::{assets::*, grid::*, tile::harvest_timer::*, update_signal::*};
use gengar_engine::render::{render_pack::*, shader::*, *};

pub mod tile_component_wander;

pub use tile_component_wander::*;

pub enum TileComponent {
    // Harvesting behavior
    Harvestable { timer: HarvestTimer },

    // Tile wanders around a grid (like frog and newt tile )
    Wander { state: WanderState },
}
