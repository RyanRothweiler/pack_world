use crate::color::*;

#[derive(Debug, Copy, Clone)]
pub struct HSL {
    h: f32,
    s: f32,
    l: f32,
}

impl HSL {
    pub fn new(in_h: f32, in_s: f32, in_l: f32) -> Self {
        Self {
            h: in_h.clamp(0.0, 360.0),
            s: in_s.clamp(0.0, 1.0),
            l: in_l.clamp(0.0, 1.0),
        }
    }

    pub fn to_rgb(&self) -> Color {
        fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }

            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 1.0 / 2.0 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        }

        if self.s == 0.0 {
            return Color {
                r: self.l,
                g: self.l,
                b: self.l,
                a: 1.0,
            };
        }

        let h_norm = self.h / 360.0;

        let q = if self.l < 0.5 {
            self.l * (1.0 + self.s)
        } else {
            self.l + self.s - self.l * self.s
        };
        let p = 2.0 * self.l - q;

        let r = hue_to_rgb(p, q, h_norm + (1.0 / 3.0));
        let g = hue_to_rgb(p, q, h_norm);
        let b = hue_to_rgb(p, q, h_norm - (1.0 / 3.0));

        return Color {
            r: r,
            g: g,
            b: b,
            a: 1.0,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conversion() {
        let hsl = HSL::new(30.0, 1.0, 0.5);
        let rgb = hsl.to_rgb();

        assert!((rgb.r - 1.0).abs() < 0.001);
        assert!((rgb.g - 0.5).abs() < 0.001);
        assert!((rgb.b - 0.0).abs() < 0.001);
    }

    #[test]
    fn green_conversion() {
        let hsl = HSL::new(120.0, 1.0, 0.25);
        let rgb = hsl.to_rgb();

        assert!((rgb.r - 0.0).abs() < 0.001);
        assert!((rgb.g - 0.5).abs() < 0.001);
        assert!((rgb.b - 0.0).abs() < 0.001);
    }
}
