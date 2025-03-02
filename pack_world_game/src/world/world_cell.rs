use crate::world::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorldCell {
    pub layers: HashMap<WorldLayer, EntityID>,
}

impl WorldCell {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
        }
    }
}
