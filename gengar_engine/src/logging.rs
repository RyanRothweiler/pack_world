use crate::platform_api::*;
use std::sync::{LazyLock, Mutex};

pub struct Logger {
    platform_api: PlatformApi,
}

impl Logger {
    pub fn new(plat: &PlatformApi) -> Self {
        Self {
            platform_api: plat.clone(),
        }
    }

    pub fn println(&self, output: &str) {
        (self.platform_api.println)(output);
    }
}
