pub struct Anchors {
    pub top: f64,
    pub bottom: f64,
    pub right: f64,
    pub left: f64,
}

impl Anchors {
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Anchors {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}
