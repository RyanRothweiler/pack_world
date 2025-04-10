use crate::{tile::harvest_timer::*, update_signal::*};

pub enum TileComponent {
    /*
    Update {
        method: fn(time_step: f64) -> Vec<UpdateSignal>,
    },
    */
    Harvestable { can_harvest: fn() -> bool },
}

// pub struct TileComp<
