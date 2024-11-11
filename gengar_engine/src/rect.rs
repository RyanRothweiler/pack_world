use crate::vectors::*;
use std::ops::Mul;

#[derive(Default, Debug, Clone)]
pub struct Rect {
    pub top_left: VecTwo,
    pub bottom_right: VecTwo,
}

impl Rect {
    pub fn new(top_left: VecTwo, bottom_right: VecTwo) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn new_square(size: f64) -> Self {
        Rect::new_size(size, size)
    }

    pub fn new_size(width: f64, height: f64) -> Self {
        let half_width = width * 0.5;
        let half_height = height * 0.5;

        Self {
            top_left: VecTwo::new(-half_width, -half_height),
            bottom_right: VecTwo::new(half_width, half_height),
        }
    }

    pub fn top_right(&self) -> VecTwo {
        VecTwo {
            x: self.bottom_right.x,
            y: self.top_left.y,
        }
    }

    pub fn bottom_left(&self) -> VecTwo {
        VecTwo {
            x: self.top_left.x,
            y: self.bottom_right.y,
        }
    }

    pub fn width(&self) -> f64 {
        (self.top_left.x - self.bottom_right.x).abs()
    }

    pub fn height(&self) -> f64 {
        (self.top_left.y - self.bottom_right.y).abs()
    }

    pub fn left(&self) -> f64 {
        self.top_left.x
    }

    pub fn right(&self) -> f64 {
        self.bottom_right.x
    }

    pub fn top(&self) -> f64 {
        self.top_left.y
    }

    pub fn bottom(&self) -> f64 {
        self.bottom_right.y
    }

    pub fn set_center(&mut self, center: VecTwo) {
        let half_width = self.width();
        let half_height = self.height();

        self.top_left = center - VecTwo::new(half_width, half_height);
        self.bottom_right = center + VecTwo::new(half_width, half_height);
    }

    pub fn get_mesh(&self, z: f64) -> Vec<VecThreeFloat> {
        let mut mesh: Vec<VecThreeFloat> = vec![];

        // left tri
        mesh.push(VecThreeFloat::new(self.top_left.x, self.top_left.y, z));
        mesh.push(VecThreeFloat::new(
            self.top_right().x,
            self.top_right().y,
            z,
        ));
        mesh.push(VecThreeFloat::new(
            self.bottom_left().x,
            self.bottom_left().y,
            z,
        ));

        // right tri
        mesh.push(VecThreeFloat::new(
            self.bottom_left().x,
            self.bottom_left().y,
            z,
        ));
        mesh.push(VecThreeFloat::new(
            self.top_right().x,
            self.top_right().y,
            z,
        ));
        mesh.push(VecThreeFloat::new(
            self.bottom_right.x,
            self.bottom_right.y,
            z,
        ));

        return mesh;
    }
}

impl Mul<f64> for Rect {
    type Output = Self;

    fn mul(self, input: f64) -> Self {
        Self {
            top_left: self.top_left * input,
            bottom_right: self.bottom_right * input,
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn rect() {
        let r = Rect::new(VecTwo::new(10.0, 10.0), VecTwo::new(20.0, 20.0));
        assert_eq!(r.top_right(), VecTwo::new(20.0, 10.0));
        assert_eq!(r.bottom_left(), VecTwo::new(10.0, 20.0));
        assert_eq!(r.width(), 10.0);
        assert_eq!(r.height(), 10.0);
    }

    #[test]
    fn center() {
        let mut r = Rect::new(VecTwo::new(10.0, 10.0), VecTwo::new(20.0, 30.0));
        r.set_center(VecTwo::new(5.0, 5.0));

        assert_eq!(r.top_left, VecTwo::new(0.0, 10.0));
        assert_eq!(r.bottom_right, VecTwo::new(10.0, 10.0));
    }
}
