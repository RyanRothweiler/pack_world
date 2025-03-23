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
    entries: HashMap<String, [u8; 8]>,
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

    pub fn save_f64(&mut self, data: f64, key: &str) {
        if self.entries.contains_key(key) {
            panic!("Key already exists {}", key);
        }

        self.entries.insert(key.into(), data.to_le_bytes());
    }

    pub fn save_f32(&mut self, data: f32, key: &str) {
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

    pub fn load_f64(&mut self, key: &str) -> Option<f64> {
        if !self.entries.contains_key(key) {
            return None;
        }

        let data = *self.entries.get(key).unwrap();
        let val = f64::from_le_bytes(data);
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
}

// #[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_write_f64() {
        let mut file = SaveFile::new();
        file.save_f64(123.45, "key");
        let val = file.load_f64("key").unwrap();

        assert_eq!(val, 123.45);
    }

    #[test]
    fn read_write_f32() {
        let mut file = SaveFile::new();
        file.save_f32(123.45, "key");
        let val = file.load_f32("key").unwrap();

        assert_eq!(val, 123.45);
    }

    #[test]
    fn read_write_safe() {
        let mut orig_file = SaveFile::new();
        orig_file.save_f32(123.45, "key");

        let mut write_data: Vec<u8> = vec![];
        let mut write_cursor = Cursor::new(write_data);
        orig_file.write_file(&mut write_cursor).unwrap();

        let read_data: Vec<u8> = write_cursor.get_ref().to_vec();
        let mut read_cursor = Cursor::new(read_data);

        let read_save_file = SaveFile::read_file(&mut read_cursor).unwrap();

        println!("{:?}", read_save_file.entries);

        assert_eq!(read_save_file.load_f32("key").unwrap(), 123.45);
    }
}
