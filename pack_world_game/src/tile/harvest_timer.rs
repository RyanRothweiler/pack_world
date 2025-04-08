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

    // modifies the length
    length_mod: f64,

    pub table: FixedTableID,
}

impl HarvestTimer {
    pub fn new(length: f64, table_id: FixedTableID) -> Self {
        Self {
            length,
            table: table_id,
            time: 0.0,
            length_mod: 1.0,
        }
    }

    fn modified_length(&self) -> f64 {
        self.length * self.length_mod
    }

    pub fn percent_done(&self) -> f64 {
        (self.time / self.modified_length()).clamp(0.0, 1.0)
    }

    pub fn inc(&mut self, time: f64) {
        self.time += time;
        self.time = self.time.clamp(0.0, self.modified_length());
    }

    pub fn can_harvest(&self) -> bool {
        self.time >= self.modified_length()
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
    }

    pub fn reset_length_mod(&mut self) {
        self.length_mod = 1.0;
    }

    pub fn get_length_mod(&self) -> f64 {
        self.length_mod
    }

    pub fn set_length_mod(&mut self, lm: f64) {
        if lm < 0.0 {
            eprintln!("Invalid length mod {lm}");
            return;
        }

        self.length_mod = lm;
    }

    pub fn harvest(&mut self, platform_api: &PlatformApi) -> Drop {
        self.reset();
        return get_drop(self.table, platform_api);
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

        let length = save_file
            .load_f64(&length_key)
            .expect(&format!("Misisng key {length_key}"));
        let time = save_file
            .load_f64(&time_key)
            .expect(&format!("Misisng key {length_key}"));

        let mut timer = Self::new(length, FixedTableID::Grass);
        timer.time = time;

        Ok(timer)
    }
}

mod test {
    use super::*;

    #[test]
    fn tests() {
        let mut ht = HarvestTimer::new(10.0, FixedTableID::Boulder);

        assert_eq!(ht.can_harvest(), false);

        ht.inc(10.0);
        assert_eq!(ht.can_harvest(), true);

        ht.reset();
        assert_eq!(ht.can_harvest(), false);

        ht.set_length_mod(0.5);
        ht.inc(5.0);
        assert_eq!(ht.can_harvest(), true);
    }
}
