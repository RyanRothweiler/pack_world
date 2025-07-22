use crate::grid::*;

// What to modify
pub enum GlobalModKind {
    /// Modify the drop count. This is multiplicative
    DropCount(f64),
}

pub struct GlobalMod {
    pub kind: GlobalModKind,

    /// Positions relative to origin to apply the mod
    pub positions: Vec<GridPos>,
}

impl GlobalMod {
    pub fn new(kind: GlobalModKind, positions: Vec<GridPos>) -> Self {
        Self { kind, positions }
    }
}
