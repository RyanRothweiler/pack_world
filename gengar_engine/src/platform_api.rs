use crate::{analytics::*, error::Error};

pub struct PlatformApi {
    pub rand: fn() -> f64,
    pub send_event: fn(AnalyticsEvent),

    pub write_save_game_data: fn(data: Vec<u8>) -> Result<(), Error>,
    pub get_save_game_data: fn(loaded_callback: Box<dyn Fn(Vec<u8>)>),
}
