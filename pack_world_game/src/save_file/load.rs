use crate::error::Error;
use std::{
    fs::File,
    io::{Read, Seek},
};

pub fn read_u64<W: Read>(reader: &mut W) -> Result<u64, Error> {
    let mut buf: [u8; 8] = [0; 8];

    reader.read(&mut buf)?;

    let val: u64 = u64::from_le_bytes(buf);
    Ok(val)
}

pub fn read_i32<W: Read>(file: &mut W) -> Result<i32, Error> {
    let mut buf: [u8; 4] = [0; 4];

    file.read(&mut buf)?;

    let val: i32 = i32::from_le_bytes(buf);
    Ok(val)
}
