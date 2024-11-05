use crate::vectors::*;

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
}
