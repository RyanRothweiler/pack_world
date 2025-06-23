use crate::{account_call::*, analytics::*, error::Error};
use std::path::*;

pub struct PlatformApi {
    pub rand: fn() -> f64,
    pub send_event: fn(AnalyticsEvent),
    pub epoch_time_ms: fn() -> f64,
    pub open_url: fn(url: String),

    pub write_save_game_data: fn(data: Vec<u8>) -> Result<(), Error>,
    pub fetch_game_save: fn(),

    pub send_account_call: fn(call: AccountCall),
}
