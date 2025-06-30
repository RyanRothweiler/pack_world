use gengar_engine::{analytics::*, error::Error, platform_api::*};
use rand::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn rand() -> f64 {
    rand::random_range(0.0..1.0)
}

fn epoch_time_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

fn send_event(vent: AnalyticsEvent) {
    unimplemented!()
}

fn write_save_game_data(data: Vec<u8>) -> Result<(), Error> {
    unimplemented!()
}

fn get_save_game_data() {
    unimplemented!()
}

fn open_url(url: String) {
    unimplemented!()
}

fn local_persist_get(key: &str) -> Option<String> {
    unimplemented!()
}

fn local_persist_set(key: &str, data: &str) {
    // do nothing for now
}

fn local_persist_delete(key: &str) {
    // do nothing for now
}

//For testing use the windows platform api. Tests don't run on any other platform.s
pub fn windows_plaform_api() -> PlatformApi {
    PlatformApi {
        rand: rand,
        send_event: send_event,

        write_save_game_data: write_save_game_data,
        fetch_game_save: get_save_game_data,
        epoch_time_ms: epoch_time_ms,
        open_url: open_url,

        local_persist_get: local_persist_get,
        local_persist_set: local_persist_set,
        local_persist_delete: local_persist_delete,
    }
}
