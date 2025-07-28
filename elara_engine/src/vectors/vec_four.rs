use crate::vectors::*;

#[derive(Clone, Copy, Debug)]
pub struct VecFour {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl VecFour {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Color> for VecFour {
    fn from(input: Color) -> Self {
        Self {
            x: input.r as f64,
            y: input.g as f64,
            z: input.b as f64,
            w: input.a as f64,
        }
    }
}
