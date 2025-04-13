use crate::{tile::harvest_timer::*, update_signal::*};

pub mod tile_component_wander;

pub use tile_component_wander::*;

pub enum TileComponent {
    // Harvesting behavior
    Harvestable { timer: HarvestTimer },

    // Tile wanders around a grid (like frog and newt tile )
    Wander { state: WanderState },
}
