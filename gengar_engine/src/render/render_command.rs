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
    Vao {
        id: u32,
    },
    DynamicMesh {
        mesh: Vec<VecThreeFloat>,
        uvs: Vec<VecTwo>,
    },
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
        let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
        let uvs: Vec<VecTwo> = vec![
            VecTwo::new(0.0, 0.0),
            VecTwo::new(1.0, 0.0),
            VecTwo::new(0.0, 1.0),
            //
            VecTwo::new(0.0, 1.0),
            VecTwo::new(1.0, 0.0),
            VecTwo::new(1.0, 1.0),
        ];

        let mut uniforms: HashMap<String, UniformData> = material.uniforms.clone();
        uniforms.insert("model".to_string(), UniformData::M44(M44::new_identity()));

        RenderCommand {
            kind: VertexDataKind::DynamicMesh {
                mesh: rect.get_mesh(z),
                uvs: uvs,
            },

            prog_id: material.shader.unwrap().prog_id,
            indices: indices,
            uniforms: uniforms,
        }
    }
}
