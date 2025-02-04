use crate::color::*;
use std::ops::{Add, Mul, Sub};

mod vec_four;
mod vec_three_float;
mod vec_three_float_c;
mod vec_two;
mod vec_two_c;
mod vec_two_int;

pub use vec_four::*;
pub use vec_three_float::*;
pub use vec_three_float_c::*;
pub use vec_two::*;
pub use vec_two_c::*;
pub use vec_two_int::*;

mod test {
    use super::*;

    #[test]
    fn vec_three_mul() {
        let ret = VecThreeFloat::new(5.0, 0.0, 0.0) * 1.0;
        assert_eq!(ret.x, 5.0);
        assert_eq!(ret.y, 0.0);
        assert_eq!(ret.z, 0.0);
    }

    #[test]
    fn vec_three_normalize() {
        let mut ret = VecThreeFloat::new(5.0, 1.0, 2.5);
        ret.normalize();

        assert_eq!((ret.x * 100.0) as i32, 88);
        assert_eq!((ret.y * 100.0) as i32, 17);
        assert_eq!((ret.z * 100.0) as i32, 44);
    }

    #[test]
    fn vec_three_cross() {
        let a = VecThreeFloat::new(5.0, 1.0, 2.5);
        let b = VecThreeFloat::new(1.0, 0.0, -10.5);
        let c = VecThreeFloat::cross(a, b);

        assert_eq!(c.x, -10.5);
        assert_eq!(c.y, 55.0);
        assert_eq!(c.z, -1.0);
    }

    #[test]
    fn vec_three_equal() {
        let a = VecThreeFloat::new(5.0, 0.0, 0.0);
        let b = VecThreeFloat::new(5.0, 0.0, 0.0);
        let c = VecThreeFloat::new(0.0, 1.0, 0.0);

        assert_eq!(VecThreeFloat::close_enough(&a, &b), true);
        assert_eq!(VecThreeFloat::close_enough(&b, &a), true);
        assert_eq!(VecThreeFloat::close_enough(&a, &c), false);
    }
}
