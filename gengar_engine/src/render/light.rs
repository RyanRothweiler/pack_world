use crate::{color::*, vectors::*};

pub struct Light {
    pub transform: usize,
    pub power: VecThreeFloat,
}

impl Light {
    pub fn new(transform: usize) -> Self {
        Self {
            transform,
            power: VecThreeFloat::new(150.0, 150.0, 150.0),
        }
    }
}
