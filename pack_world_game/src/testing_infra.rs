use gengar_engine::{analytics::*, error::Error, platform_api::*};
use rand::prelude::*;

fn rand() -> f64 {
    rand::random_range(0.0..1.0)
}

fn send_event(vent: AnalyticsEvent) {
    unimplemented!()
}

fn write_save_game_data(data: Vec<u8>) -> Result<(), Error> {
    unimplemented!()
}

fn get_save_game_data(callback: Box<dyn Fn(Vec<u8>)>) {
    unimplemented!()
}

//For testing use the windows platform api. Tests don't run on any other platform.s
pub fn windows_plaform_api() -> PlatformApi {
    PlatformApi {
        rand: rand,
        send_event: send_event,
        write_save_game_data: write_save_game_data,
        get_save_game_data: get_save_game_data,
    }
}
