#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),

    NegativeItemCount,
    InvalidTilePosition,
    HitBankLimit,
    InvalidTileTypeIndex(i32),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}
