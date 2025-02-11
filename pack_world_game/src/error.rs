#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NegativeItemCount,
    InvalidTilePosition,
    HitBankLimit,
}
