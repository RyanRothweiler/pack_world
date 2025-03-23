use crate::{
    error::*,
    save_file::{load, *},
};
use std::fs::File;

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
pub struct EntityID {
    // part of save file
    pub id: u64,
}

/*
impl EntityID {
    pub fn save_file_write(&self, save_file: &mut SaveFile) -> Result<(), Error> {
        // file.save
        // writer.write(&self.id.to_le_bytes())?;
        Ok(())
    }

    pub fn save_file_load(save_file: &SaveFile) -> Result<EntityID, Error> {
        Ok(EntityID {
            id: load::read_u64(reader)?,
        })
    }
}
*/
