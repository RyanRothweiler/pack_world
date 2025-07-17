// Where to apply the modification
pub enum GlobalModLocation {
    // Apply within a radius centered on self tile
    Radius(i32),
}

// What to modify
pub enum GlobalModKind {
    /// Modify the drop count. This is multiplicative
    DropCount(f64),
}

pub struct GlobalMod {
    pub kind: GlobalModKind,
    pub loc: GlobalModLocation,
}

impl GlobalMod {
    pub fn new(kind: GlobalModKind, loc: GlobalModLocation) -> Self {
        Self { kind, loc }
    }
}
