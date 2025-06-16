#![allow(unused_imports, clippy::all)]

/// This should work, but I realized I didn't really need it.
/// Once all the methods here have been implemented then webgl_render can be swapped with this
/// Then we don't need two gl rendering backends.
use gengar_engine::{matricies::matrix_four_four::*, util::incrementing_map::*, vectors::*};
use gengar_render_opengl::*;

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
    vertex_arrays: IncrementingMap<WebGlVertexArrayObject>,
    buffers: IncrementingMap<WebGlBuffer>,
    framebuffers: IncrementingMap<WebGlFramebuffer>,
}

impl WebGlRenderMethods {
    fn buf_type_to_gl(buf_type: BufferType) -> u32 {
        match buf_type {
            BufferType::ArrayBuffer => WebGl2RenderingContext::ARRAY_BUFFER,
            BufferType::ElementArrayBuffer => WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        }
    }

    fn usage_type_to_gl(usage: BufferUsage) -> u32 {
        match usage {
            BufferUsage::StaticDraw => WebGl2RenderingContext::STATIC_DRAW,
        }
    }
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

    fn attach_shader(&self, prog_id: u32, shader_id: u32) {
        let shader: &WebGlShader = self.shaders.get(shader_id as usize);
        let prog: &WebGlProgram = self.programs.get(prog_id as usize);

        self.context.attach_shader(&prog, &shader);
    }

    fn link_program(&self, prog_id: u32) {
        let prog: &WebGlProgram = self.programs.get(prog_id as usize);
        self.context.link_program(&prog);
    }

    fn gen_vertex_arrays(&mut self, count: i32, vao: *mut u32) {
        let new_vao: WebGlVertexArrayObject = self
            .context
            .create_vertex_array()
            .expect("Error creating vertex array");
        unsafe {
            *vao = self.vertex_arrays.push(new_vao) as u32;
        }
    }

    fn delete_vertex_arrays(&self, count: i32, vao_id: u32) {
        let vao: &WebGlVertexArrayObject = self.vertex_arrays.get(vao_id as usize);
        self.context.delete_vertex_array(Some(&vao));
    }

    fn delete_buffers(&self, count: i32, buf_id: u32) {
        let buf: &WebGlBuffer = self.buffers.get(buf_id as usize);
        self.context.delete_buffer(Some(buf));
    }

    fn bind_vertex_array(&self, vao_id: u32) {
        let vao = self.vertex_arrays.get(vao_id as usize);
        self.context.bind_vertex_array(Some(vao));
    }

    fn gen_buffers(&mut self, count: i32, buffers: *mut u32) {
        assert!(count == 1, "Only count of 1 suported on webgl");

        let buf: WebGlBuffer = self
            .context
            .create_buffer()
            .expect("Error generating buffer");
        unsafe {
            *buffers = self.buffers.push(buf) as u32;
        }
    }

    fn bind_buffer(&self, typ: BufferType, buf_id: u32) {
        let buf = self.buffers.get(buf_id as usize);
        self.context
            .bind_buffer(Self::buf_type_to_gl(typ), Some(&buf));
    }

    fn buffer_data_v3(&self, target: BufferType, data: &Vec<VecThreeFloat>, usage: BufferUsage) {
        let bytes_total = size_of::<f32>() * 3 * data.len();

        let buf = js_sys::ArrayBuffer::new(bytes_total as u32);
        let buf_view = js_sys::DataView::new(&buf, 0, bytes_total);

        for i in 0..data.len() {
            let byte_offset = size_of::<f32>() * 3 * i;

            buf_view.set_float32_endian(byte_offset, data[i].x as f32, true);
            buf_view.set_float32_endian(byte_offset + size_of::<f32>(), data[i].y as f32, true);
            buf_view.set_float32_endian(
                byte_offset + (size_of::<f32>() * 2),
                data[i].z as f32,
                true,
            );
        }

        self.context.buffer_data_with_opt_array_buffer(
            Self::buf_type_to_gl(target),
            Some(&buf),
            Self::usage_type_to_gl(usage),
        );
    }

    fn buffer_data_v2(&self, buf_id: BufferType, data: &Vec<VecTwo>, usage: BufferUsage) {}

    fn buffer_data_u32(&self, buf_id: BufferType, data: &Vec<u32>, usage: BufferUsage) {}

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
