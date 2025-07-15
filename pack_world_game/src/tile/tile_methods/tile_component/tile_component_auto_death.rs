use gengar_engine::time::*;

#[derive(Debug)]
pub struct AutoDeathState {
    timer: Time,
}

impl AutoDeathState {
    pub fn new(timer: Time) -> Self {
        Self { timer }
    }
}
