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
            prog_id: material.shader.expect("Material missing shader").prog_id,
            indices: model.indices.clone(),
            uniforms: uniforms,
        }
    }

    pub fn new_rect(rect: &Rect, z: f64, rot_deg: f64, material: &Material) -> Self {
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

        let rect_center = rect.get_center();

        let mut model_mat = M44::new_identity();
        model_mat.translate(VecThreeFloat::new(rect_center.x, rect_center.y, 0.0));
        model_mat.rotate_z(rot_deg.to_radians());
        uniforms.insert("model".to_string(), UniformData::M44(model_mat));

        RenderCommand {
            kind: VertexDataKind::DynamicMesh {
                mesh: rect.get_mesh_centered(z),
                uvs: uvs,
            },

            prog_id: material.shader.unwrap().prog_id,
            indices: indices,
            uniforms: uniforms,
        }
    }

    pub fn new_rect_outline(rect: &Rect, z: f64, width: f64, material: &Material) -> Self {
        let left_edge = Rect::new(
            rect.top_left,
            VecTwo::new(rect.top_left.x + width, rect.bottom_right.y),
        );
        let right_edge = Rect::new(
            VecTwo::new(rect.bottom_right.x - width, rect.top_left.y),
            rect.bottom_right,
        );
        let bottom_edge = Rect::new(
            VecTwo::new(rect.top_left.x, rect.bottom_right.y - width),
            rect.bottom_right,
        );
        let top_edge = Rect::new(
            rect.top_left,
            VecTwo::new(rect.bottom_right.x, rect.top_left.y - width),
        );

        let mut mesh: Vec<VecThreeFloat> = vec![];
        mesh.append(&mut left_edge.get_mesh(z));
        mesh.append(&mut right_edge.get_mesh(z));
        mesh.append(&mut bottom_edge.get_mesh(z));
        mesh.append(&mut top_edge.get_mesh(z));

        let mut indices: Vec<u32> = vec![];
        for i in 0..mesh.len() {
            indices.push(i as u32);
        }

        let mut uniforms: HashMap<String, UniformData> = material.uniforms.clone();
        uniforms.insert("model".to_string(), UniformData::M44(M44::new_identity()));

        RenderCommand {
            kind: VertexDataKind::DynamicMesh {
                mesh: mesh,
                uvs: vec![],
            },

            prog_id: material.shader.unwrap().prog_id,
            indices: indices,
            uniforms: uniforms,
        }
    }
}
