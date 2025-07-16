use gengar_engine::time::*;

#[derive(Debug)]
pub struct AutoDeath {
    timer: Time,
}

impl AutoDeath {
    pub fn new(timer: Time) -> Self {
        Self { timer }
    }

    pub fn inc(&mut self, len: Time) {
        self.timer = self.timer - len;
    }

    pub fn alive(&self) -> bool {
        self.timer.greater_than_zero()
    }
}
