use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VecThreeFloat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VecThreeFloat {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VecThreeFloat { x, y, z }
    }

    pub fn new_zero() -> Self {
        VecThreeFloat::new(0.0, 0.0, 0.0)
    }

    pub fn cross(a: Self, b: Self) -> Self {
        let mut ret = VecThreeFloat::new_zero();
        ret.x = (a.y * b.z) - (a.z * b.y);
        ret.y = (a.z * b.x) - (a.x * b.z);
        ret.z = (a.x * b.y) - (a.y * b.x);
        ret
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn normalize_self(&mut self) {
        let len = self.length();
        self.x = self.x / len;
        self.y = self.y / len;
        self.z = self.z / len;
    }

    pub fn normalize(self) -> Self {
        let mut ret = self;

        let len = ret.length();
        ret.x = ret.x / len;
        ret.y = ret.y / len;
        ret.z = ret.z / len;

        return ret;
    }

    pub fn close_enough(a: &Self, b: &Self) -> bool {
        let decs = 1000.0;
        ((a.x * decs) as i64 == (b.x * decs) as i64)
            && ((a.y * decs) as i64 == (b.y * decs) as i64)
            && ((a.z * decs) as i64 == (b.z * decs) as i64)
    }

    pub fn dot(a: &Self, b: &Self) -> f64 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
    }
}

impl Add for VecThreeFloat {
    type Output = Self;

    fn add(self, input: Self) -> Self {
        Self {
            x: self.x + input.x,
            y: self.y + input.y,
            z: self.z + input.z,
        }
    }
}

impl Sub for VecThreeFloat {
    type Output = Self;

    fn sub(self, input: Self) -> Self {
        Self {
            x: self.x - input.x,
            y: self.y - input.y,
            z: self.z - input.z,
        }
    }
}

impl Mul<f64> for VecThreeFloat {
    type Output = Self;

    fn mul(self, input: f64) -> Self {
        Self {
            x: self.x * input,
            y: self.y * input,
            z: self.z * input,
        }
    }
}

impl Div<f64> for VecThreeFloat {
    type Output = Self;

    fn div(self, input: f64) -> Self {
        Self {
            x: self.x / input,
            y: self.y / input,
            z: self.z / input,
        }
    }
}
