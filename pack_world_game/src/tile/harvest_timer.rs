use crate::{drop_table::*, update_signal::*};
use gengar_engine::vectors::*;

pub struct HarvestTimer {
    // tile until we can harvest
    length: f64,
    time: f64,
    table_id: DropTableID,
}

impl HarvestTimer {
    pub fn new(length: f64, table_id: DropTableID) -> Self {
        Self {
            length,
            table_id,
            time: 0.0,
        }
    }

    pub fn percent_done(&self) -> f64 {
        (self.time / self.length).clamp(0.0, 1.0)
    }

    pub fn inc(&mut self, time: f64) {
        self.time += time;
        self.time = self.time.clamp(0.0, self.length);
    }

    pub fn can_harvest(&self) -> bool {
        self.time >= self.length
    }

    pub fn harvest(&mut self, tile_pos: VecTwo) -> Vec<UpdateSignal> {
        self.time = 0.0;

        vec![UpdateSignal::HarvestItemPullTable {
            table: self.table_id,
            origin: tile_pos,
        }]
    }
}
