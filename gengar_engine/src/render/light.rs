use crate::color::*;

pub struct Light {
    pub transform: usize,
}

impl Light {
    pub fn new(transform: usize) -> Self {
        Self { transform }
    }
}
