use crate::transform::*;

pub struct Components {
    pub transforms: Vec<Transform>,
}

impl Components {
    pub fn new() -> Self {
        Self { transforms: vec![] }
    }

    pub fn new_transform(&mut self) -> usize {
        self.transforms.push(Transform::new());
        return self.transforms.len() - 1;
    }
}
