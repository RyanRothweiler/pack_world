use crate::render::{shader::*, RenderApi};
use std::collections::HashMap;

pub mod font;

pub use font::*;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TypeWeight {
    Regular,
    Bold,
    Light,
}

pub struct Typeface {
    fonts: HashMap<TypeWeight, Font>,
    shader: Shader,
}

impl Typeface {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            shader: Shader::new_empty(),
        }
    }

    pub fn setup(&mut self, shader: Shader) {
        self.shader = shader;
    }

    pub fn load_weight(
        &mut self,
        weight: TypeWeight,
        metrics: String,
        image_bytes: impl std::io::Read,
        render_api: &impl RenderApi,
    ) {
        let font = font::load(image_bytes, &metrics, self.shader, render_api).unwrap();
        self.fonts.insert(weight, font);
    }

    pub fn get_weight(&self, weight: TypeWeight) -> Font {
        self.fonts
            .get(&weight)
            .expect(&format!("Missing type weight {:?}", weight))
            .clone()
    }
}
