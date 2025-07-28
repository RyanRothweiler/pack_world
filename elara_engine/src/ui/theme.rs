use crate::color::*;
use std::sync::LazyLock;

pub const COLOR_BLUE_BG: Color = Color {
    r: 21.0 / 255.0,
    g: 58.0 / 255.0,
    b: 87.0 / 255.0,
    a: 1.0,
};

pub const COLOR_BLUE_FG: Color = Color {
    r: 34.0 / 255.0,
    g: 93.0 / 255.0,
    b: 139.0 / 255.0,
    a: 1.0,
};

pub const THEME_PANEL_BG: LazyLock<Color> = LazyLock::new(|| HSL::new(0.0, 0.0, 0.1).to_rgb());
pub const THEME_TEXT: LazyLock<Color> = LazyLock::new(|| HSL::new(0.0, 0.0, 0.95).to_rgb());
pub const THEME_TEXT_MUT: LazyLock<Color> = LazyLock::new(|| HSL::new(0.0, 0.0, 0.7).to_rgb());
