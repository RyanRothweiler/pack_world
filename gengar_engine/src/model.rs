use crate::{
    error::Error,
    obj,
    render::{vao::Vao, RenderApi},
    vectors::*,
};

#[derive(Debug, Clone)]
pub struct Model {
    pub vertices: Vec<VecThreeFloat>,
    pub uvs: Vec<VecTwo>,
    pub indices: Vec<u32>,
    pub vao: Vao,

    pub normals: Vec<VecThreeFloat>,
    pub normal_tans: Vec<VecThreeFloat>,
    pub normal_bi_tans: Vec<VecThreeFloat>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            vertices: vec![],
            uvs: vec![],
            indices: vec![],
            vao: Vao::new_empty(),

            normals: vec![],
            normal_tans: vec![],
            normal_bi_tans: vec![],
        }
    }

    // This assumes locations for shader layout data.k
    // If the layout locations in the shader changes this will break
    pub fn load_upload(&mut self, data: &str, render_api: &impl RenderApi) -> Result<(), Error> {
        obj::load(self, data)?;

        self.vao = Vao::new(render_api);

        // vertices
        self.vao
            .upload_v3(render_api, &self.vertices, &self.indices, 0)?;

        // uvs
        self.vao.upload_v2(render_api, &self.uvs, 1)?;

        // normals
        self.vao
            .upload_v3(render_api, &self.normals, &self.indices, 2)?;

        // tangents
        self.vao
            .upload_v3(render_api, &self.normal_tans, &self.indices, 3)?;

        // bi tans
        self.vao
            .upload_v3(render_api, &self.normal_bi_tans, &self.indices, 4)?;

        Ok(())
    }

    // Highly unsafe. Assumes vertices in triangles and data linear / un-spooled / not indexed
    pub fn calculate_tangents(&mut self) {
        // clear old data
        self.normal_tans.clear();
        self.normal_bi_tans.clear();

        self.normal_tans
            .resize(self.vertices.len(), VecThreeFloat::new_zero());
        self.normal_bi_tans
            .resize(self.vertices.len(), VecThreeFloat::new_zero());

        for i in (0..self.vertices.len()).step_by(3) {
            let point_one = self.vertices[i];
            let point_two = self.vertices[i + 1];
            let point_three = self.vertices[i + 2];

            let uv_one = self.uvs[i];
            let uv_two = self.uvs[i + 1];
            let uv_three = self.uvs[i + 2];

            let edge_one = point_two - point_one;
            let edge_two = point_three - point_one;

            let uv_delta_one = uv_two - uv_one;
            let uv_delta_two = uv_three - uv_one;

            let f = 1.0 / ((uv_delta_one.x * uv_delta_two.y) - (uv_delta_one.y * uv_delta_two.x));

            let mut tan = VecThreeFloat::new_zero();
            let mut bi_tan = VecThreeFloat::new_zero();

            tan.x = f * ((uv_delta_two.y * edge_one.x) - (uv_delta_one.y * edge_two.x));
            tan.y = f * ((uv_delta_two.y * edge_one.y) - (uv_delta_one.y * edge_two.y));
            tan.z = f * ((uv_delta_two.y * edge_one.z) - (uv_delta_one.y * edge_two.z));

            bi_tan.x = f * ((uv_delta_one.x * edge_two.x) - (uv_delta_two.x * edge_one.x));
            bi_tan.y = f * ((uv_delta_one.x * edge_two.y) - (uv_delta_two.x * edge_one.y));
            bi_tan.z = f * ((uv_delta_one.x * edge_two.z) - (uv_delta_two.x * edge_one.z));

            self.normal_tans[i] = tan;
            self.normal_tans[i + 1] = tan;
            self.normal_tans[i + 2] = tan;

            self.normal_bi_tans[i] = bi_tan;
            self.normal_bi_tans[i + 1] = bi_tan;
            self.normal_bi_tans[i + 2] = bi_tan;
        }
    }
}
