use crate::{drop_table::*, grid::*, update_signal::*};
use gengar_engine::vectors::*;

#[derive(Debug)]
pub struct HarvestTimer {
    // tile until we can harvest
    length: f64,
    time: f64,

    pub table: DropTableInstance,
}

impl HarvestTimer {
    pub fn new(length: f64, table_id: FixedTableID) -> Self {
        Self {
            length,
            table: DropTableInstance::new_fixed(table_id),
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

    pub fn reset(&mut self) {
        self.time = 0.0;
    }

    pub fn add_entry(&mut self, input: (EntryOutput, f64)) {
        self.table = self.table.add_entry(input);
    }

    pub fn harvest(&mut self, grid_pos: GridPos) -> Vec<UpdateSignal> {
        self.reset();

        vec![UpdateSignal::AddHarvestDrop {
            drop: self.table.get_drop(),
            origin: grid_to_world(&grid_pos),
        }]
    }
}
