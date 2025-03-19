use crate::{error::*, save_file::load};
use std::{
    fs::File,
    io::{Read, Seek, Write},
};

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
pub struct EntityID {
    // part of save file
    pub id: u64,
}

impl EntityID {
    pub fn write(&self, file: &mut File) -> Result<(), Error> {
        file.write(&self.id.to_le_bytes())?;
        Ok(())
    }

    /*
    pub fn read<W: Read, Seek>(reader: &mut W) -> Result<EntityID, Error> {
        Ok(EntityID {
            id: load::read_u64(reader)?,
        })
    }
    */
}
