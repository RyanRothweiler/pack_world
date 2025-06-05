#![allow(unused_imports, clippy::all)]

use gengar_engine::{matricies::matrix_four_four::*, util::incrementing_map::*, vectors::*};

use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlShader, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject,
};

use js_sys;
use std::collections::HashMap;
use std::mem::size_of;

pub struct WebGlRenderMethods {
    context: WebGl2RenderingContext,
    state: crate::webgl::webgl_render_api::WebGLState,

    shaders: IncrementingMap<WebGlShader>,
    programs: IncrementingMap<WebGlProgram>,
}

impl gengar_render_opengl::OGLPlatformImpl for WebGlRenderMethods {
    fn create_shader(&mut self, id: i32) -> u32 {
        let prog: WebGlShader = self.context.create_shader(id as u32).unwrap();
        self.shaders.push(prog) as u32
    }

    fn shader_source(&self, id: u32, source: &str) {
        let shader: &WebGlShader = self.shaders.get(id as usize);
        self.context.shader_source(shader, &source);
    }

    fn compile_shader(&self, id: u32) {
        let shader: &WebGlShader = self.shaders.get(id as usize);
        self.context.compile_shader(shader);
    }

    fn get_shader_iv(&self, id: u32, info_type: i32, output: *mut i32) {
        let shader: &WebGlShader = self.shaders.get(id as usize);
        match self.context.get_shader_info_log(&shader) {
            Some(v) => {
                if v.len() > 0 {
                    // there is an error
                    // output = &mut 0;
                    return;
                }

                // no error so shader compilation was fine
                // output = &mut 1;
            }
            None => {
                // Error getting info;
                // output = &mut 0;
            }
        }
    }

    fn shader_info_log(
        &self,
        shader_id: u32,
        max_length: i32,
        output_length: *mut i32,
        output_buffer: &mut Vec<u8>,
    ) {
        // todo this needs to be changed. This should return a string and the windows platform impl should do the
        // libc conversion stuff
    }

    fn draw_buffers(&self, ty: i32, attachments: Vec<u32>) {
        // Convert to JSValue array
        let js_buffers = js_sys::Array::new();
        for &b in attachments.iter() {
            js_buffers.push(&wasm_bindgen::JsValue::from(b));
        }

        self.context.draw_buffers(&js_buffers);
    }

    fn create_program(&mut self) -> u32 {
        let prog: WebGlProgram = self.context.create_program().unwrap();
        self.programs.push(prog) as u32
    }

    fn attach_shader(&self, prog_id: u32, shader_id: u32) {}

    fn link_program(&self, prog_id: u32) {}

    fn gen_vertex_arrays(&self, count: i32, vao: *mut u32) {}

    fn delete_vertex_arrays(&self, count: i32, vao: u32) {}

    fn delete_buffers(&self, count: i32, buf_id: u32) {}

    fn bind_vertex_array(&self, vao_id: u32) {}

    fn gen_buffers(&self, count: i32, buffers: *mut u32) {}

    fn bind_buffer(&self, typ: i32, buf_id: u32) {}

    fn buffer_data_v3(&self, buf_id: i32, data: &Vec<VecThreeFloat>, usage: i32) {}

    fn buffer_data_v2(&self, buf_id: i32, data: &Vec<VecTwo>, usage: i32) {}

    fn buffer_data_u32(&self, buf_id: i32, data: &Vec<u32>, usage: i32) {}

    fn enable_vertex_attrib_array(&self, location: u32) {}

    fn vertex_attrib_pointer_v3(&self, location: u32) {}

    fn vertex_attrib_pointer_v2(&self, location: u32) {}

    fn gen_textures(&self, count: i32, id: *mut u32) {}

    fn gen_frame_buffers(&self, count: i32, id: *mut u32) {}

    fn bind_texture(&self, typ: i32, id: u32) {}

    fn bind_frame_buffer(&self, typ: u32, id: u32) {}

    fn frame_buffer_2d(&self, target: u32, attachment: u32, ty: u32, textarget: u32, level: i32) {}

    fn check_frame_buffer_status(&self, ty: u32) -> u32 {
        0
    }

    fn tex_parameter_i(&self, target: u32, pname: u32, param: i32) {}

    fn tex_image_2d(
        &self,
        target: u32,
        gl_storage_format: i32,
        image_format: u32,
        image_pixel_type: u32,
        width: i32,
        height: i32,
        image_data: Option<&Vec<u8>>,
    ) {
    }

    fn enable(&self, feature: u32) {}

    fn disable(&self, feature: u32) {}

    fn blend_func(&self, func: u32, setting: u32) {}

    fn depth_func(&self, func: u32) {}

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {}

    fn clear(&self) {}

    fn use_program(&self, prog_id: u32) {}

    fn get_uniform_location(&self, prog_id: u32, uniform_name: &str) -> i32 {
        0
    }

    fn uniform_matrix_4fv(&self, loc: i32, count: i32, transpose: bool, data: &M44) {}

    fn uniform_4fv(&self, loc: i32, count: i32, data: &VecFour) {}

    fn uniform_3fv(&self, loc: i32, count: i32, data: &VecThreeFloat) {}

    fn uniform_1f(&self, loc: i32, data: f32) {}

    fn uniform_1i(&self, loc: i32, data: i32) {}

    fn active_texture(&self, id: i32) {}

    fn draw_elements(&self, mode: i32, indecies: &Vec<u32>) {}

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {}

    fn gen_render_buffers(&self, count: i32, id: *mut u32) {}

    fn bind_render_buffer(&self, ty: u32, id: u32) {}

    fn render_buffer_storage(&self, ty: u32, stor_type: u32, width: i32, height: i32) {}

    fn frame_buffer_render_buffer(&self, target: u32, ty: u32, tar: u32, rbid: u32) {}
}
