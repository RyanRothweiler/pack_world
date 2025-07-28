use crate::vectors::*;

/// Used to accumulate up a bunch of draw commands.
/// Used for reducing draw calls. Add info to this then use this to create one big draw call.
pub struct AccumulateDraw {
    pub vertex: Vec<VecThreeFloat>,
    pub uv: Vec<VecTwo>,

    pub indices: Vec<u32>,
}

impl AccumulateDraw {
    pub fn new() -> Self {
        Self {
            vertex: vec![],
            uv: vec![],
            indices: vec![],
        }
    }

    pub fn add(&mut self, uv: VecTwo, vertex: VecThreeFloat) {
        self.indices.push(self.indices.len() as u32);
        self.uv.push(uv);
        self.vertex.push(vertex);
    }
}
