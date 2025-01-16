#![allow(unreachable_code)]

use crate::webgl::webgl_render_api::*;

use gengar_engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{
        camera::*, render_command::*, shader::*, vao::Vao, RenderApi as EngineRenderApiTrait,
        ShaderType,
    },
    state::State as EngineState,
    vectors::*,
};
use web_sys::WebGl2RenderingContext;

pub fn render(
    es: &mut EngineState,
    render_api: &WebGLRenderApi,
    resolution: &VecTwo,
    context: &WebGl2RenderingContext,
    light_pos: VecThreeFloat,
) {
    context.viewport(0, 0, resolution.x as i32, resolution.y as i32);

    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);

    context.enable(WebGl2RenderingContext::BLEND);
    context.blend_func(
        WebGl2RenderingContext::SRC_ALPHA,
        WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context
        .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    todo!("render the render packs");

    /*
    render_list(
        light_pos,
        &mut es.render_commands,
        &es.camera,
        render_api,
        context,
    );
    render_list(
        light_pos,
        &mut es.ui_render_commands,
        &es.ui_camera,
        render_api,
        context,
    );

    // Debug render lists
    render_list(
        VecThreeFloat::new_zero(),
        gengar_engine::debug::get_render_list(),
        &es.camera,
        render_api,
        context,
    );
    render_list(
        VecThreeFloat::new_zero(),
        &mut es.game_debug_render_commands,
        &es.camera,
        render_api,
        context,
    );
    */
}

fn render_list(
    light_pos: VecThreeFloat,
    render_commands: &mut Vec<RenderCommand>,
    camera: &Camera,
    render_api: &WebGLRenderApi,
    context: &WebGl2RenderingContext,
) {
    for command in render_commands {
        (render_api.gl_use_program)(command.prog_id);

        command
            .uniforms
            .insert("view".to_string(), UniformData::M44(camera.view_mat));
        command.uniforms.insert(
            "projection".to_string(),
            UniformData::M44(camera.projection_mat),
        );
        command.uniforms.insert(
            "viewPos".to_string(),
            UniformData::VecThree(camera.transform.local_position),
        );
        command
            .uniforms
            .insert("lightPos".to_string(), UniformData::VecThree(light_pos));
        command.uniforms.insert(
            "lightColor".to_string(),
            UniformData::VecThree(VecThreeFloat::new(150.0, 150.0, 150.0)),
        );

        for (key, value) in &command.uniforms {
            match value {
                UniformData::M44(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => (render_api.gl_uniform_matrix_4fv)(&loc, false, data),

                        // That loc doesn't exist
                        None => {}
                    };
                }
                UniformData::Texture(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => {
                            context.uniform1i(Some(&loc), data.texture_slot as i32);
                            context.active_texture(
                                WebGl2RenderingContext::TEXTURE0 + data.texture_slot,
                            );

                            (render_api.gl_bind_texture)(data.image_id);
                        }

                        // That loc doesn't exist
                        None => {}
                    };
                }
                UniformData::VecThree(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => (render_api.gl_uniform_3fv)(&loc, data),

                        // That loc doesn't exist
                        None => {}
                    };
                }
                UniformData::VecFour(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => (render_api.gl_uniform_4fv)(&loc, data),

                        // That loc doesn't exist
                        None => {}
                    };
                }
                UniformData::Float(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => context.uniform1f(Some(&loc), *data as f32),

                        // That loc doesn't exist
                        None => {}
                    };
                }
            }
        }

        let vao_id: u32 = match &command.kind {
            VertexDataKind::Vao { id } => *id,
            VertexDataKind::DynamicMesh { mesh, uvs } => {
                let vao = Vao::new(render_api);

                // location is assumed 0. All shaders vertex positions are at location 0... for now.
                vao.upload_v3(render_api, mesh, &command.indices, 0)
                    .unwrap();

                // uvs
                vao.upload_v2(render_api, uvs, 1).unwrap();

                vao.id
            }
        };

        (render_api.gl_bind_vertex_array_engine)(vao_id).unwrap();
        (render_api.gl_draw_arrays)(WebGl2RenderingContext::TRIANGLES as i32, &command.indices);
    }
}
