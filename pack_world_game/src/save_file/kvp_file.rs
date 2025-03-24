use crate::error::*;
use std::{
    collections::HashMap,
    io::{Read, Write},
};

#[repr(C)]
pub struct WrittenEntry {
    key: [u8; 32],
    data: [u8; 8],
}

impl WrittenEntry {
    pub fn new() -> Self {
        Self {
            key: [b' '; 32],
            data: [0; 8],
        }
    }
}

#[derive(Debug)]
pub struct SaveFile {
    pub entries: HashMap<String, [u8; 8]>,
}

impl SaveFile {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Write out the entire file
    pub fn write_file<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let mut entries: Vec<WrittenEntry> = vec![];

        for (key, value) in &self.entries {
            let mut new_entry = WrittenEntry::new();

            for (i, b) in key.bytes().enumerate() {
                if i > new_entry.key.len() {
                    return Err(Error::KeyTooLong(key.into()));
                }

                new_entry.key[i] = b;
            }

            new_entry.data = *value;

            entries.push(new_entry);
        }

        for e in entries {
            writer.write(e.key.as_slice())?;
            writer.write(e.data.as_slice())?;
        }

        Ok(())
    }

    /// Build save file from reader
    pub fn read_file<W: Read>(reader: &mut W) -> Result<Self, Error> {
        let mut ret = Self::new();

        // loop reading entires until we run out of data
        loop {
            let mut new_entry = WrittenEntry::new();

            match reader.read(&mut new_entry.key) {
                Ok(v) => {
                    if v == 0 {
                        return Ok(ret);
                    }
                }

                // This should really return an error
                Err(e) => return Ok(ret),
            };

            match reader.read(&mut new_entry.data) {
                Ok(v) => {
                    if v == 0 {
                        return Ok(ret);
                    }
                }

                // This should really return an error
                Err(e) => return Ok(ret),
            };

            let key: String = String::from_utf8(new_entry.key.to_vec())?.trim().into();
            ret.entries.insert(key, new_entry.data);
        }
    }

    pub fn save_f64(&mut self, key: &str, data: f64) {
        if self.entries.contains_key(key) {
            panic!("Key already exists {}", key);
        }

        self.entries.insert(key.into(), data.to_le_bytes());
    }

    pub fn save_u64(&mut self, key: &str, data: u64) {
        if self.entries.contains_key(key) {
            panic!("Key already exists {}", key);
        }

        self.entries.insert(key.into(), data.to_le_bytes());
    }

    pub fn save_i64(&mut self, key: &str, data: i64) {
        if self.entries.contains_key(key) {
            panic!("Key already exists {}", key);
        }

        self.entries.insert(key.into(), data.to_le_bytes());
    }

    pub fn save_f32(&mut self, key: &str, data: f32) {
        if self.entries.contains_key(key) {
            panic!("Key already exists {}", key);
        }

        let b = data.to_le_bytes();

        let mut d = [0; 8];
        d[0] = b[0];
        d[1] = b[1];
        d[2] = b[2];
        d[3] = b[3];

        self.entries.insert(key.into(), d);
    }

    pub fn save_i32(&mut self, key: &str, data: i32) {
        if self.entries.contains_key(key) {
            panic!("Key already exists {}", key);
        }

        let b = data.to_le_bytes();

        let mut d = [0; 8];
        d[0] = b[0];
        d[1] = b[1];
        d[2] = b[2];
        d[3] = b[3];

        self.entries.insert(key.into(), d);
    }

    pub fn save_u32(&mut self, key: &str, data: u32) {
        if self.entries.contains_key(key) {
            panic!("Key already exists {}", key);
        }

        let b = data.to_le_bytes();

        let mut d = [0; 8];
        d[0] = b[0];
        d[1] = b[1];
        d[2] = b[2];
        d[3] = b[3];

        self.entries.insert(key.into(), d);
    }

    pub fn save_bool(&mut self, key: &str, data: bool) {
        if self.entries.contains_key(key) {
            panic!("Key already exists {}", key);
        }

        let mut d = [0; 8];
        if data {
            d[0] = 1;
        } else {
            d[0] = 0;
        }

        self.entries.insert(key.into(), d);
    }

    pub fn load_f64(&self, key: &str) -> Option<f64> {
        if !self.entries.contains_key(key) {
            return None;
        }

        let data = *self.entries.get(key).unwrap();
        let val = f64::from_le_bytes(data);
        Some(val)
    }

    pub fn load_i64(&self, key: &str) -> Option<i64> {
        if !self.entries.contains_key(key) {
            return None;
        }

        let data = *self.entries.get(key).unwrap();
        let val = i64::from_le_bytes(data);
        Some(val)
    }

    pub fn load_u64(&self, key: &str) -> Option<u64> {
        if !self.entries.contains_key(key) {
            return None;
        }

        let data = *self.entries.get(key).unwrap();
        let val = u64::from_le_bytes(data);
        Some(val)
    }

    pub fn load_f32(&self, key: &str) -> Option<f32> {
        if !self.entries.contains_key(key) {
            return None;
        }

        let data = *self.entries.get(key).unwrap();

        let mut d = [0; 4];
        d[0] = data[0];
        d[1] = data[1];
        d[2] = data[2];
        d[3] = data[3];

        let val = f32::from_le_bytes(d);
        Some(val)
    }

    pub fn load_i32(&self, key: &str) -> Option<i32> {
        if !self.entries.contains_key(key) {
            return None;
        }

        let data = *self.entries.get(key).unwrap();

        let mut d = [0; 4];
        d[0] = data[0];
        d[1] = data[1];
        d[2] = data[2];
        d[3] = data[3];

        let val = i32::from_le_bytes(d);
        Some(val)
    }

    pub fn load_u32(&self, key: &str) -> Option<u32> {
        if !self.entries.contains_key(key) {
            return None;
        }

        let data = *self.entries.get(key).unwrap();

        let mut d = [0; 4];
        d[0] = data[0];
        d[1] = data[1];
        d[2] = data[2];
        d[3] = data[3];

        let val = u32::from_le_bytes(d);
        Some(val)
    }

    pub fn load_bool(&self, key: &str) -> Option<bool> {
        if !self.entries.contains_key(key) {
            return None;
        }

        let data = *self.entries.get(key).unwrap();
        if data[0] == 1 {
            Some(true)
        } else {
            Some(false)
        }
    }
}

mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_write_f64() {
        let mut file = SaveFile::new();
        file.save_f64("key", 123.45);
        let val = file.load_f64("key").unwrap();

        assert_eq!(val, 123.45);
    }

    #[test]
    fn read_write_f32() {
        let mut file = SaveFile::new();
        file.save_f32("key", 123.45);
        let val = file.load_f32("key").unwrap();

        assert_eq!(val, 123.45);
    }

    #[test]
    fn read_write_bool() {
        let mut file = SaveFile::new();
        file.save_bool("true", true);
        file.save_bool("false", false);

        assert_eq!(file.load_bool("true").unwrap(), true);
        assert_eq!(file.load_bool("false").unwrap(), false);
    }

    #[test]
    fn read_write_safe() {
        let mut orig_file = SaveFile::new();
        orig_file.save_f32("key", 123.45);

        let mut write_data: Vec<u8> = vec![];
        let mut write_cursor = Cursor::new(write_data);
        orig_file.write_file(&mut write_cursor).unwrap();

        let read_data: Vec<u8> = write_cursor.get_ref().to_vec();
        let mut read_cursor = Cursor::new(read_data);

        let read_save_file = SaveFile::read_file(&mut read_cursor).unwrap();

        assert_eq!(read_save_file.load_f32("key").unwrap(), 123.45);
    }
}
