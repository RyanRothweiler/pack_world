use crate::{tile::harvest_timer::*, update_signal::*};

pub enum TileComponent {
    Harvestable { timer: HarvestTimer },
}
