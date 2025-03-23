use crate::{
    drop_table::*,
    error::*,
    grid::*,
    save_file::{load, *},
    update_signal::*,
};
use gengar_engine::{platform_api::*, vectors::*};

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

    pub fn harvest(&mut self, platform_api: &PlatformApi) -> Drop {
        self.reset();
        return self.table.get_drop(platform_api);
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let length_key = format!("{}.l", key_parent);
        let time_key = format!("{}.t", key_parent);

        save_file.save_f64(&length_key, self.length);
        save_file.save_f64(&time_key, self.time);

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let length_key = format!("{}.l", key_parent);
        let time_key = format!("{}.t", key_parent);

        let length = save_file.load_f64(&length_key).unwrap();
        let time = save_file.load_f64(&time_key).unwrap();

        let mut timer = Self::new(length, FixedTableID::Grass);
        timer.time = time;

        Ok(timer)
    }
}
