/// https://easings.net/#easeOutQuint

pub fn ease_out_quint(i: f64) -> f64 {
    // originally was 5.0. modified to 2
    1.0 - f64::powf(1.0 - i, 2.0)
}

pub fn ease_in_quint(x: f64) -> f64 {
    x * x * x * x * x
}
