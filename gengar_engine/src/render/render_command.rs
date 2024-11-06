use crate::{
    matricies::matrix_four_four::*,
    model::*,
    rect::*,
    render::{camera::*, material::Material, shader::*, vao::Vao},
    transform::*,
    vectors::*,
};
use std::collections::HashMap;

#[derive(Clone)]
pub enum VertexDataKind {
    Vao { id: u32 },
    DynamicMesh { mesh: Vec<VecThreeFloat> },
}

#[derive(Clone)]
pub struct RenderCommand {
    pub kind: VertexDataKind,

    pub prog_id: u32,
    pub indices: Vec<u32>,
    pub uniforms: HashMap<String, UniformData>,
}

impl RenderCommand {
    pub fn new_model(transform: &Transform, model: &Model, material: &Material) -> Self {
        let mut uniforms: HashMap<String, UniformData> = material.uniforms.clone();

        uniforms.insert(
            "model".to_string(),
            UniformData::M44(transform.global_matrix.clone()),
        );

        uniforms.insert(
            "lightPos".to_string(),
            UniformData::M44(transform.global_matrix.clone()),
        );

        RenderCommand {
            kind: VertexDataKind::Vao { id: model.vao.id },
            prog_id: material.shader.unwrap().prog_id,
            indices: model.indices.clone(),
            uniforms: uniforms,
        }
    }

    pub fn new_rect(rect: &Rect, z: f64, material: &Material) -> Self {
        let mut mesh: Vec<VecThreeFloat> = vec![];

        // left tri
        mesh.push(VecThreeFloat::new(rect.top_left.x, rect.top_left.y, z));
        mesh.push(VecThreeFloat::new(
            rect.top_right().x,
            rect.top_right().y,
            z,
        ));
        mesh.push(VecThreeFloat::new(
            rect.bottom_left().x,
            rect.bottom_left().y,
            z,
        ));

        // right tri
        mesh.push(VecThreeFloat::new(
            rect.bottom_left().x,
            rect.bottom_left().y,
            z,
        ));
        mesh.push(VecThreeFloat::new(
            rect.top_right().x,
            rect.top_right().y,
            z,
        ));
        mesh.push(VecThreeFloat::new(
            rect.bottom_right.x,
            rect.bottom_right.y,
            z,
        ));

        let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];

        let uniforms: HashMap<String, UniformData> = material.uniforms.clone();

        RenderCommand {
            kind: VertexDataKind::DynamicMesh { mesh: mesh },

            prog_id: material.shader.unwrap().prog_id,
            indices: indices,
            uniforms: uniforms,
        }
    }
}
