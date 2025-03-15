use gengar_engine::{analytics::*, platform_api::*};
use rand::prelude::*;

fn rand() -> f64 {
    rand::random_range(0.0..1.0)
}

fn send_event(vent: AnalyticsEvent) {}

//For testing use the windows platform api. Tests don't run on any other platform.s
pub fn windows_plaform_api() -> PlatformApi {
    PlatformApi {
        rand: rand,
        send_event: send_event,
    }
}
