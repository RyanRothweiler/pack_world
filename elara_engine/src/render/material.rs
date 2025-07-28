use crate::{color::*, render::shader::*};
use std::collections::HashMap;

#[derive(Clone, Default)]
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

    pub fn set_image(&mut self, image: u32) {
        self.uniforms.insert(
            "tex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: image,
                texture_slot: 0,
            }),
        );
    }

    pub fn set_color(&mut self, color: Color) {
        self.uniforms
            .insert("color".to_string(), UniformData::VecFour(color.into()));
    }
}
