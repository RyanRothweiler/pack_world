pub mod ease;

pub use ease::*;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

mod test {
    use super::*;

    #[test]
    fn lerp_test() {
        let n: f64 = lerp(0.0, 1.0, 0.5);
        assert!(n > 0.45 && n < 0.55);

        let n: f64 = lerp(0.0, 1.0, 1.5);
        assert!(n > 0.99 && n < 1.01);
    }
}
