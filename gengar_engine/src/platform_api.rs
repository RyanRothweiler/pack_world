use crate::analytics::*;

pub struct PlatformApi {
    pub rand: fn() -> f64,
    pub send_event: fn(AnalyticsEvent),
}
