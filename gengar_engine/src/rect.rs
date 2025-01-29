use crate::vectors::*;
use std::ops::Mul;

#[derive(Default, Debug, Clone, Copy)]
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

    // top left doesn't move. width and height push out the bottom right
    pub fn new_top_size(top_left: VecTwo, width: f64, height: f64) -> Self {
        Self {
            top_left,
            bottom_right: top_left + VecTwo::new(width, height),
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

    // move entire rect
    pub fn translate(&mut self, trans: VecTwo) {
        self.top_left = self.top_left + trans;
        self.bottom_right = self.bottom_right + trans;
    }

    pub fn set_center(&mut self, center: VecTwo) {
        let half_width = self.width() * 0.5;
        let half_height = self.height() * 0.5;

        self.top_left = center - VecTwo::new(half_width, half_height);
        self.bottom_right = center + VecTwo::new(half_width, half_height);
    }

    pub fn get_center(&self) -> VecTwo {
        let half_width = self.width() * 0.5;
        let half_height = self.height() * 0.5;

        return self.top_left + VecTwo::new(half_width, half_height);
    }

    // resize by moving the right point. Top left remains unchanged
    pub fn resize_right(&mut self, width: f64) {
        self.bottom_right.x = self.top_left.x + width;
    }

    pub fn shrink(&mut self, size: f64) {
        let w = (self.width() * 0.5) - size;
        let h = (self.height() * 0.5) - size;
        let center = self.get_center();

        self.top_left = center - VecTwo::new(w, h);
        self.bottom_right = center + VecTwo::new(w, h);
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

    // returns a mesh with the center of the rect centered at 0,0,0
    pub fn get_mesh_centered(&self, z: f64) -> Vec<VecThreeFloat> {
        let half_width = self.width() * 0.5;
        let half_height = self.height() * 0.5;

        let top_left = VecTwo::new(-half_width, -half_height);
        let bottom_left = VecTwo::new(-half_width, half_height);
        let top_right = VecTwo::new(half_width, -half_height);
        let bottom_right = VecTwo::new(half_width, half_height);

        let mut mesh: Vec<VecThreeFloat> = vec![];

        // left tri
        mesh.push(VecThreeFloat::new(top_left.x, top_left.y, z));
        mesh.push(VecThreeFloat::new(top_right.x, top_right.y, z));
        mesh.push(VecThreeFloat::new(bottom_left.x, bottom_left.y, z));

        // right tri
        mesh.push(VecThreeFloat::new(bottom_left.x, bottom_left.y, z));
        mesh.push(VecThreeFloat::new(top_right.x, top_right.y, z));
        mesh.push(VecThreeFloat::new(bottom_right.x, bottom_right.y, z));

        return mesh;
    }

    pub fn contains(&self, pos: VecTwo) -> bool {
        if self.top_left.x <= pos.x
            && self.top_left.y <= pos.y
            && self.bottom_right.x >= pos.x
            && self.bottom_right.y >= pos.y
        {
            return true;
        }

        return false;
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

        assert_eq!(r.top_left, VecTwo::new(0.0, -5.0));
        assert_eq!(r.bottom_right, VecTwo::new(10.0, 15.0));
    }

    #[test]
    fn resize_right() {
        let mut r = Rect::new(VecTwo::new(10.0, 10.0), VecTwo::new(20.0, 30.0));
        r.resize_right(2.0);

        assert_eq!(r.top_left, VecTwo::new(10.0, 10.0));
        assert_eq!(r.bottom_right, VecTwo::new(12.0, 30.0));
    }
}
