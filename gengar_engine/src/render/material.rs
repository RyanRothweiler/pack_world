use crate::render::shader::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Material {
    pub uniforms: HashMap<String, UniformData>,
    pub shader: Option<Shader>,
}

impl Material {
    pub fn new() -> Self {
        Material {
            uniforms: HashMap::new(),
            shader: None,
        }
    }
}
