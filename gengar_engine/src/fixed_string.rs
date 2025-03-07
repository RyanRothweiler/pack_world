use crate::{error::*, memory_arena::*};

pub struct FixedString {
    pub mem: [u8; 256],
    pub len: usize,
}

impl FixedString {
    pub fn new() -> Self {
        Self {
            mem: [0; 256],
            len: 0,
        }
    }

    pub fn set(&mut self, input: &str) -> Result<(), Error> {
        let by = input.as_bytes();
        if by.len() > self.mem.len() {
            return Err(Error::FixedStringNotEnoughMemory);
        }

        self.len = input.len();
        for (i, b) in by.iter().enumerate() {
            self.mem[i] = *b;
        }
        Ok(())
    }

    pub fn get(&self) -> Result<&str, Error> {
        let slice = &self.mem[0..self.len];
        std::str::from_utf8(slice).map_err(|e| Error::FixedStringInvalidString)
    }
}

mod test {
    use super::*;

    #[test]
    fn general() {
        let mut fs = FixedString::new();
        fs.set("Heyo this is a string").unwrap();
        assert_eq!(fs.get().unwrap(), "Heyo this is a string");
    }
}
