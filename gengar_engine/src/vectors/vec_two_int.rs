use crate::vectors::*;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Hash)]
pub struct VecTwoInt {
    pub x: i32,
    pub y: i32,
}

impl VecTwoInt {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for VecTwoInt {
    type Output = Self;

    fn add(self, input: Self) -> Self {
        Self {
            x: self.x + input.x,
            y: self.y + input.y,
        }
    }
}

impl From<&VecTwo> for VecTwoInt {
    fn from(input: &VecTwo) -> Self {
        Self {
            x: input.x as i32,
            y: input.y as i32,
        }
    }
}

impl From<VecTwo> for VecTwoInt {
    fn from(input: VecTwo) -> Self {
        Self {
            x: input.x as i32,
            y: input.y as i32,
        }
    }
}
