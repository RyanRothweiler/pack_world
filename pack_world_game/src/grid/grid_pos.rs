use gengar_engine::vectors::*;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_adjacents_iter(&self) -> GridPosIter {
        GridPosIter { pos: *self, i: 0 }
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

pub struct GridPosIter {
    pub pos: GridPos,
    pub i: i32,
}

impl Iterator for GridPosIter {
    type Item = GridPos;

    fn next(&mut self) -> Option<GridPos> {
        let result = match self.i {
            0 => self.pos + GridPos::new(-1, 1),
            1 => self.pos + GridPos::new(-1, 0),
            2 => self.pos + GridPos::new(-1, -1),
            3 => self.pos + GridPos::new(0, 1),
            4 => self.pos + GridPos::new(0, -1),
            5 => self.pos + GridPos::new(1, 1),
            6 => self.pos + GridPos::new(1, 0),
            7 => self.pos + GridPos::new(1, -1),
            _ => return None,
        };

        self.i += 1;
        Some(result)
    }
}
