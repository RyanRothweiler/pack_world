use crate::vectors::*;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct VecTwo {
    pub x: f64,
    pub y: f64,
}

impl VecTwo {
    pub fn new(x: f64, y: f64) -> Self {
        VecTwo { x, y }
    }

    pub fn lerp(a: Self, b: Self, t: f64) -> Self {
        a + (b - a) * t.clamp(0.0, 1.0)
    }
}

impl Mul<f64> for VecTwo {
    type Output = Self;

    fn mul(self, input: f64) -> Self {
        Self {
            x: self.x * input,
            y: self.y * input,
        }
    }
}

impl Add for VecTwo {
    type Output = Self;

    fn add(self, input: Self) -> Self {
        Self {
            x: self.x + input.x,
            y: self.y + input.y,
        }
    }
}

impl Sub for VecTwo {
    type Output = Self;

    fn sub(self, input: Self) -> Self {
        Self {
            x: self.x - input.x,
            y: self.y - input.y,
        }
    }
}

impl From<VecTwoInt> for VecTwo {
    fn from(input: VecTwoInt) -> Self {
        Self {
            x: input.x as f64,
            y: input.y as f64,
        }
    }
}
