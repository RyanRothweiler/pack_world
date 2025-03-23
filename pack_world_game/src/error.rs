#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    EngineError(String),

    // saveing / loading
    UnknownTileMethodID(i32),
    UnknownDropTableInstanceID(i32),
    UnknownPackID(i32),
    UnknownFixedTableID(i32),
    KeyTooLong(String),
    StringUTF8Error(std::string::FromUtf8Error),

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

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::StringUTF8Error(error)
    }
}
