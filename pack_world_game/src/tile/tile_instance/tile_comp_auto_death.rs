use crate::{error::*, save_file::*};
use gengar_engine::time::*;

#[derive(Debug)]
pub struct TileCompAutoDeath {
    timer: Time,
}

impl TileCompAutoDeath {
    pub fn new(timer: Time) -> Self {
        Self { timer }
    }

    pub fn inc(&mut self, len: Time) {
        self.timer = self.timer - len;
    }

    pub fn alive(&self) -> bool {
        self.timer.greater_than_zero()
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let time_key = format!("{}.t", key_parent);

        save_file.save_f64(&time_key, self.timer.as_milliseconds().value());

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let time_key = format!("{}.t", key_parent);

        let time = save_file.load_f64(&time_key)?;

        let comp = Self::new(Time::new(TimeUnit::MilliSeconds(time)));

        Ok(comp)
    }
}
