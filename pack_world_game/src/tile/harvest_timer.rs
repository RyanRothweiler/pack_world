use crate::{drop_table::*, error::*, grid::*, save_file::load, update_signal::*};
use gengar_engine::{platform_api::*, vectors::*};
use std::io::{Read, Write};

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

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write(&self.length.to_le_bytes())?;
        writer.write(&self.time.to_le_bytes())?;

        Ok(())
    }

    pub fn read<W: Read>(reader: &mut W) -> Result<Self, Error> {
        let length = load::read_f64(reader)?;
        let time = load::read_f64(reader)?;

        let mut timer = Self::new(length, FixedTableID::Grass);
        timer.time = time;

        Ok(timer)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tile::tiles::*;
    use std::io::Cursor;

    #[test]
    fn save_load_dirt() {
        let mut original = HarvestTimer::new(10.0, FixedTableID::Boulder);
        original.time = 100.5;

        let mut data: Vec<u8> = vec![];
        let mut cursor = Cursor::new(data);

        // write into buffer
        original.write(&mut cursor).unwrap();

        let save_file: Vec<u8> = cursor.get_ref().to_vec();

        // load from buffer
        let loaded: HarvestTimer = HarvestTimer::read(&mut Cursor::new(save_file)).unwrap();

        assert_eq!(loaded.length, original.length);
        assert_eq!(loaded.time, original.time);
    }
}
