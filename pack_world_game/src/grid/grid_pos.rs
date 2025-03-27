use gengar_engine::vectors::*;
use std::ops::{Add, Mul, Sub};

mod adjacents_iter;
mod rect_iter;

pub use adjacents_iter::*;
pub use rect_iter::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_adjacents_iter(&self) -> GridPosAdjacentsIter {
        GridPosAdjacentsIter { pos: *self, i: 0 }
    }

    /// Iterator through positions with self as top lefts in a rectangle of Width and Height
    pub fn to_rect_iter(&self, w: i32, h: i32) -> GridPosRectIter {
        GridPosRectIter::new(*self, w, h)
    }
}

impl Add for GridPos {
    type Output = Self;

    fn add(self, input: Self) -> Self {
        Self {
            x: self.x + input.x,
            y: self.y + input.y,
        }
    }
}
