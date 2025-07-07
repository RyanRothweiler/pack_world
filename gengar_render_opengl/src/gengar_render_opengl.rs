#![allow(unused_variables, unused_imports, dead_code, clippy::all)]

use gengar_engine::{
    color::*,
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{
        camera::*,
        frame_buffer_pack::*,
        image::{Image, ImageFormat},
        render_command::*,
        render_pack::*,
        shader::*,
        vao::Vao,
        RenderApi as EngineRenderApiTrait, ShaderType,
    },
    state::{components::*, State as EngineState},
    vectors::*,
};

pub mod gl_types;

use gl_types::*;

// resolved light info for rendering
struct LightRenderInfo {
    pub position: VecThreeFloat,
    pub power: VecThreeFloat,
}

pub enum BufferType {
    ArrayBuffer,
    ElementArrayBuffer,
}

pub enum BufferUsage {
    StaticDraw,
}

pub enum TextureTarget {
    Texture2D,
}

pub enum TextureParameter {
    MinFilter,
    MagFilter,
}

pub enum TextureFilterParameter {
    Linear,
}

pub enum Capability {
    DepthTest,
    Blend,
}

pub enum BlendFuncSourceFactor {
    SourceAlpha,
}

pub enum BlendFuncDestFactor {
    OneMinusSourceAlpha,
}

pub enum DepthComparison {
    LessThanOrEqualTo,
}

// Platformm specific opengl calls. This abstracts over the platforms.
pub trait OGLPlatformImpl {
    fn create_shader(&mut self, id: i32) -> u32;
    fn shader_source(&self, id: u32, source: &str);
    fn compile_shader(&self, id: u32);
    fn get_shader_iv(&self, id: u32, info_typ: i32, output: *mut i32);
    fn shader_info_log(
        &self,
        shader_id: u32,
        max_length: i32,
        output_length: *mut i32,
        output_buffer: &mut Vec<u8>,
    );
    fn create_program(&mut self) -> u32;
    fn attach_shader(&self, prog_id: u32, shader_id: u32);
    fn link_program(&self, prog_id: u32);
    fn gen_vertex_arrays(&mut self, count: i32, vao: *mut u32);
    fn bind_vertex_array(&self, vao_id: u32);
    fn gen_buffers(&mut self, count: i32, buffers: *mut u32);
    fn bind_buffer(&self, typ: BufferType, buf_id: Option<u32>);
    fn gen_textures(&mut self, count: i32, id: *mut u32);
    fn bind_texture(&self, target: TextureTarget, id: Option<u32>);
    fn tex_parameter_i(&self, target: u32, pname: TextureParameter, param: TextureFilterParameter);
    fn tex_image_2d(
        &self,
        target: u32,
        gl_storage_format: ImageFormat,
        image_format: u32,
        image_pixel_type: u32,
        width: i32,
        height: i32,
        image_data: Option<&Vec<u8>>,
    );
    fn enable(&self, cap: Capability);
    fn disable(&self, cap: Capability);
    fn depth_func(&self, comp: DepthComparison);
    fn blend_func(&self, source_func: BlendFuncSourceFactor, desc_func: BlendFuncDestFactor);
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32);
    fn clear(&self);
    fn use_program(&self, prog_id: u32);
    fn active_texture(&self, id: i32);
    fn draw_elements(&self, mode: i32, indecies: &Vec<u32>);
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);

    fn delete_vertex_arrays(&self, count: i32, vao: u32);
    fn delete_buffers(&self, count: i32, buf_id: u32);

    fn buffer_data_v3(&self, target: BufferType, data: &Vec<VecThreeFloat>, usage: BufferUsage);
    fn buffer_data_v2(&self, target: BufferType, data: &Vec<VecTwo>, usage: BufferUsage);
    fn buffer_data_u32(&self, target: BufferType, data: &Vec<u32>, usage: BufferUsage);

    fn enable_vertex_attrib_array(&self, location: u32);
    fn vertex_attrib_pointer_v3(&self, location: u32);
    fn vertex_attrib_pointer_v2(&self, location: u32);

    fn get_uniform_location(&mut self, prog_id: u32, uniform_name: &str) -> Option<i32>;
    fn uniform_matrix_4fv(&self, loc: i32, count: i32, transpose: bool, data: &M44);
    fn uniform_4fv(&self, loc: i32, count: i32, data: &VecFour);
    fn uniform_3fv(&self, loc: i32, count: i32, data: &VecThreeFloat);
    fn uniform_1f(&self, loc: i32, data: f32);
    fn uniform_1i(&self, loc: i32, data: i32);

    fn gen_frame_buffers(&mut self, count: i32, id: *mut u32);
    fn bind_frame_buffer(&self, ty: u32, id: Option<u32>);
    fn frame_buffer_2d(&self, target: u32, attachment: u32, ty: u32, textarget: u32, level: i32);
    fn check_frame_buffer_status(&self, ty: u32);
    fn draw_buffers(&self, ty: i32, attachments: Vec<u32>);

    fn gen_render_buffers(&mut self, count: i32, id: *mut u32);
    fn bind_render_buffer(&self, ty: u32, id: Option<u32>);
    fn render_buffer_storage(&self, ty: u32, stor_type: u32, width: i32, height: i32);
    fn frame_buffer_render_buffer(&self, target: u32, ty: u32, tar: u32, rbid: u32);

    // None opengl specific
    fn remove_internal_uniform_loc_position(&mut self, id: i32);
}

pub struct OglRenderApi {
    pub platform_api: Box<dyn OGLPlatformImpl>,
}

impl OglRenderApi {
    fn shader_info_log(&self, id: u32) -> Result<String, EngineError> {
        let mut string_buf: Vec<u8> = vec![0; 1024];

        let mut output_len: i32 = 0;
        self.platform_api
            .shader_info_log(id, 1024, &mut output_len, &mut string_buf);

        let error: String = std::ffi::CStr::from_bytes_until_nul(string_buf.as_ref())?
            .to_str()?
            .to_string();

        return Ok(error);
    }

    fn compile_shader(
        &mut self,
        shader_source: &str,
        shader_type: ShaderType,
    ) -> Result<u32, EngineError> {
        let gl_shader_type: i32 = match shader_type {
            ShaderType::Vertex => GL_VERTEX_SHADER,
            ShaderType::Fragment => GL_FRAGMENT_SHADER,
        };

        let source: String = "#version 300 es \n ".to_string() + shader_source;
        // let source: String = "#version 330 core \n ".to_string() + shader_source;

        let id: u32 = self.platform_api.create_shader(gl_shader_type);

        self.platform_api.shader_source(id, &source);
        self.platform_api.compile_shader(id);

        let mut status: i32 = -1;
        self.platform_api
            .get_shader_iv(id, GL_COMPILE_STATUS, &mut status);
        if status == GL_FALSE {
            let error_info: String = self.shader_info_log(id)?;
            return Err(EngineError::ShaderCompilation(error_info));
        }

        Ok(id)
    }
}

impl EngineRenderApiTrait for OglRenderApi {
    fn make_shader_program(
        &mut self,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<u32, EngineError> {
        let vert_id = self.compile_shader(vert_shader, ShaderType::Vertex)?;
        let frag_id = self.compile_shader(frag_shader, ShaderType::Fragment)?;

        let prog_id: u32 = self.platform_api.create_program();
        self.platform_api.attach_shader(prog_id, vert_id);
        self.platform_api.attach_shader(prog_id, frag_id);
        self.platform_api.link_program(prog_id);

        // we might need to change the error checking here to handle webgl.
        // get_shader_iv is different on webgl

        let mut status: i32 = -1;
        self.platform_api
            .get_shader_iv(prog_id, GL_LINK_STATUS, &mut status);
        if status == GL_FALSE {
            let error_info: String = self.shader_info_log(prog_id)?;
            return Err(EngineError::ShaderProgramLink(error_info));
        }

        // delete the shaders?

        Ok(prog_id)
    }

    fn create_vao(&mut self) -> Result<u32, EngineError> {
        let mut vao_id: u32 = 0;
        self.platform_api.gen_vertex_arrays(1, &mut vao_id);
        Ok(vao_id)
    }

    fn vao_upload_v3(
        &mut self,
        vao: &Vao,
        data: &Vec<VecThreeFloat>,
        indices: &Vec<u32>,
        location: u32,
    ) -> Result<Vec<u32>, EngineError> {
        self.platform_api.bind_vertex_array(vao.id);
        let mut ret: Vec<u32> = vec![];

        // setup the vertex buffer
        let mut vert_buf_id: u32 = 0;
        self.platform_api.gen_buffers(1, &mut vert_buf_id);

        self.platform_api
            .bind_buffer(BufferType::ArrayBuffer, Some(vert_buf_id));
        self.platform_api
            .buffer_data_v3(BufferType::ArrayBuffer, data, BufferUsage::StaticDraw);
        self.platform_api.vertex_attrib_pointer_v3(location);
        self.platform_api.enable_vertex_attrib_array(location);

        self.platform_api.bind_buffer(BufferType::ArrayBuffer, None);

        // setup the index buffer
        {
            let mut buf_id: u32 = 0;
            self.platform_api.gen_buffers(1, &mut buf_id);
            self.platform_api
                .bind_buffer(BufferType::ElementArrayBuffer, Some(buf_id));
            self.platform_api.buffer_data_u32(
                BufferType::ElementArrayBuffer,
                indices,
                BufferUsage::StaticDraw,
            );

            ret.push(buf_id);
        }

        self.platform_api.bind_buffer(BufferType::ArrayBuffer, None);

        self.platform_api.bind_vertex_array(0);
        self.platform_api
            .bind_buffer(BufferType::ElementArrayBuffer, None);

        ret.push(vert_buf_id);
        Ok(ret)
    }

    fn vao_upload_v2(
        &mut self,
        vao: &Vao,
        data: &Vec<VecTwo>,
        location: u32,
    ) -> Result<u32, EngineError> {
        self.platform_api.bind_vertex_array(vao.id);

        let mut buf_id: u32 = 0;
        self.platform_api.gen_buffers(1, &mut buf_id);

        self.platform_api
            .bind_buffer(BufferType::ArrayBuffer, Some(buf_id));
        self.platform_api
            .buffer_data_v2(BufferType::ArrayBuffer, data, BufferUsage::StaticDraw);
        self.platform_api.vertex_attrib_pointer_v2(location);
        self.platform_api.enable_vertex_attrib_array(location);

        self.platform_api.bind_buffer(BufferType::ArrayBuffer, None);

        self.platform_api.bind_vertex_array(0);

        Ok(buf_id)
    }

    fn upload_texture(&mut self, image: &Image, gamma_correct: bool) -> Result<u32, EngineError> {
        let mut tex_id: u32 = 0;
        self.platform_api.gen_textures(1, &mut tex_id);
        self.platform_api
            .bind_texture(TextureTarget::Texture2D, Some(tex_id));

        self.platform_api.tex_parameter_i(
            GL_TEXTURE_2D as u32,
            TextureParameter::MagFilter,
            TextureFilterParameter::Linear,
        );
        self.platform_api.tex_parameter_i(
            GL_TEXTURE_2D as u32,
            TextureParameter::MinFilter,
            TextureFilterParameter::Linear,
        );

        let image_format = match image.format {
            ImageFormat::RGBA => RGBA,
            ImageFormat::RGB => RGB,
        };

        /*
        if gamma_correct {
            image_format = GL_SRGB as i32;
        }
        */

        self.platform_api.tex_image_2d(
            GL_TEXTURE_2D as u32,
            image.format,
            image_format as u32,
            UNSIGNED_BYTE as u32,
            image.width as i32,
            image.height as i32,
            Some(&image.data),
        );

        Ok(tex_id)
    }

    fn build_frame_buffer(
        &mut self,
        width: i32,
        height: i32,
    ) -> Result<FrameBufferPack, EngineError> {
        let mut pack = FrameBufferPack {
            frame_buffer: 0,
            color_buffer: 0,
        };

        // framebuffer
        self.platform_api
            .gen_frame_buffers(1, &mut pack.frame_buffer);
        self.platform_api
            .bind_frame_buffer(GL_FRAMEBUFFER, Some(pack.frame_buffer));

        // color texture
        {
            self.platform_api.gen_textures(1, &mut pack.color_buffer);
            self.platform_api
                .bind_texture(TextureTarget::Texture2D, Some(pack.color_buffer));
            self.platform_api.tex_image_2d(
                GL_TEXTURE_2D as u32,
                ImageFormat::RGBA,
                RGBA as u32,
                UNSIGNED_BYTE as u32,
                width,
                height,
                None,
            );
            self.platform_api.tex_parameter_i(
                GL_TEXTURE_2D as u32,
                TextureParameter::MagFilter,
                TextureFilterParameter::Linear,
            );
            self.platform_api.tex_parameter_i(
                GL_TEXTURE_2D as u32,
                TextureParameter::MinFilter,
                TextureFilterParameter::Linear,
            );

            // attach to framebuffer object
            self.platform_api.frame_buffer_2d(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0 as u32,
                GL_TEXTURE_2D as u32,
                pack.color_buffer,
                0,
            );

            self.platform_api
                .bind_texture(TextureTarget::Texture2D, None);
        }

        // depth and stencil buffers
        {
            let mut rbo: u32 = 0;

            self.platform_api.gen_render_buffers(1, &mut rbo);
            self.platform_api
                .bind_render_buffer(GL_RENDERBUFFER, Some(rbo));
            self.platform_api.render_buffer_storage(
                GL_RENDERBUFFER,
                GL_DEPTH24_STENCIL8,
                width,
                height,
            );
            self.platform_api.bind_render_buffer(GL_RENDERBUFFER, None);

            self.platform_api.frame_buffer_render_buffer(
                GL_FRAMEBUFFER,
                GL_DEPTH_STENCIL_ATTACHMENT,
                GL_RENDERBUFFER,
                rbo,
            );
        }

        self.platform_api.draw_buffers(
            3,
            vec![
                GL_COLOR_ATTACHMENT0,
                GL_COLOR_ATTACHMENT1,
                GL_COLOR_ATTACHMENT2,
            ],
        );

        self.platform_api.check_frame_buffer_status(GL_FRAMEBUFFER);

        self.platform_api.bind_frame_buffer(GL_FRAMEBUFFER, None);

        Ok(pack)
    }

    fn draw_frame_buffer(
        &mut self,
        frame_buffer: u32,
        render_pack: &mut RenderPack,
        components: &Components,
    ) {
        self.platform_api
            .bind_frame_buffer(GL_FRAMEBUFFER, Some(frame_buffer));

        self.platform_api.viewport(
            0,
            0,
            render_pack.camera.resolution.x as i32,
            render_pack.camera.resolution.y as i32,
        );

        self.platform_api.enable(Capability::DepthTest);
        self.platform_api.enable(Capability::Blend);
        self.platform_api.blend_func(
            BlendFuncSourceFactor::SourceAlpha,
            BlendFuncDestFactor::OneMinusSourceAlpha,
        );

        self.platform_api
            .depth_func(DepthComparison::LessThanOrEqualTo);

        self.platform_api.clear_color(0.0, 0.0, 0.0, 0.0);
        self.platform_api.clear();

        /*
        vec![LightRenderInfo {
                position: VecThreeFloat::new(100.0, 100.0, 0.0),
            }],
            */

        render_render_pack(render_pack, components, self);

        self.platform_api.bind_frame_buffer(GL_FRAMEBUFFER, None);
    }
}

pub fn render(es: &mut EngineState, resolution: &VecTwo, render_api: &mut OglRenderApi) {
    render_api.platform_api.viewport(
        0,
        -es.title_bar_height,
        resolution.x as i32,
        resolution.y as i32,
    );

    render_api.platform_api.enable(Capability::DepthTest);
    render_api.platform_api.enable(Capability::Blend);

    render_api.platform_api.blend_func(
        BlendFuncSourceFactor::SourceAlpha,
        BlendFuncDestFactor::OneMinusSourceAlpha,
    );

    render_api
        .platform_api
        .depth_func(DepthComparison::LessThanOrEqualTo);

    render_api.platform_api.clear_color(0.0, 0.0, 0.0, 1.0);
    render_api.platform_api.clear();

    render_render_pack(
        es.render_system.get_pack(RenderPackID::NewWorld),
        &es.components,
        render_api,
    );
    render_render_pack(
        es.render_system.get_pack(RenderPackID::Shop),
        &es.components,
        render_api,
    );
    render_render_pack(
        es.render_system.get_pack(RenderPackID::World),
        &es.components,
        render_api,
    );
    render_render_pack(
        es.render_system.get_pack(RenderPackID::UI),
        &es.components,
        render_api,
    );
    /*
    if es.render_packs.len() > 4 {
        panic!("This assumes two render packs for now. If there is more then sometning needs to be done.");
    }
    */

    // Debug rendering
    /*
    {
        render_list(
            vec![],
            gengar_engine::debug::get_render_list(),
            &es.render_packs.get(&RenderPackID::World).unwrap().camera,
            &render_api,
        );
        render_list(
            vec![],
            gengar_engine::debug::get_ui_render_list(),
            &es.render_packs.get(&RenderPackID::World).unwrap().camera,
            &render_api,
        );
        render_list(
            vec![],
            &mut es.game_debug_render_commands,
            &es.render_packs.get(&RenderPackID::NewWorld).unwrap().camera,
            &render_api,
        );
        render_list(
            vec![],
            &mut es.game_ui_debug_render_commands,
            &es.render_packs.get(&RenderPackID::UI).unwrap().camera,
            &render_api,
        );
    }
    */
}

fn render_render_pack(
    pack: &mut RenderPack,
    components: &Components,
    render_api: &mut OglRenderApi,
) {
    let mut lights: Vec<LightRenderInfo> = vec![];
    for l in &pack.lights {
        lights.push(LightRenderInfo {
            position: components.transforms[l.transform]
                .global_matrix
                .get_position(),
            power: l.power,
        });
    }
    render_list(lights, &mut pack.commands, &pack.camera, render_api);
}

fn render_list(
    lights: Vec<LightRenderInfo>,
    render_commands: &mut Vec<RenderCommand>,
    camera: &Camera,
    render_api: &mut OglRenderApi,
) {
    for command in render_commands {
        render_api.platform_api.use_program(command.prog_id);

        // setup the camera transforms
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

        // lights
        {
            command.uniforms.insert(
                "lightsCount".to_string(),
                UniformData::Float(lights.len() as f64),
            );
            if let Some(li) = lights.get(0) {
                command
                    .uniforms
                    .insert("lightPos".to_string(), UniformData::VecThree(li.position));

                command
                    .uniforms
                    .insert("lightColor".to_string(), UniformData::VecThree(li.power));
            }
            if let Some(li) = lights.get(1) {
                command.uniforms.insert(
                    "lightPosTwo".to_string(),
                    UniformData::VecThree(li.position),
                );

                command
                    .uniforms
                    .insert("lightColorTwo".to_string(), UniformData::VecThree(li.power));
            }
            if let Some(li) = lights.get(2) {
                command.uniforms.insert(
                    "lightPosThree".to_string(),
                    UniformData::VecThree(li.position),
                );

                command.uniforms.insert(
                    "lightColorThree".to_string(),
                    UniformData::VecThree(li.power),
                );
            }
        }
        // upload uniform data
        for (key, value) in &command.uniforms {
            match value {
                UniformData::M44(data) => {
                    if let Some(loc) = render_api
                        .platform_api
                        .get_uniform_location(command.prog_id, key)
                    {
                        render_api
                            .platform_api
                            .uniform_matrix_4fv(loc, 1, false, data);

                        render_api
                            .platform_api
                            .remove_internal_uniform_loc_position(loc);
                    }
                }
                UniformData::VecFour(data) => {
                    if let Some(loc) = render_api
                        .platform_api
                        .get_uniform_location(command.prog_id, key)
                    {
                        render_api.platform_api.uniform_4fv(loc, 1, data);

                        render_api
                            .platform_api
                            .remove_internal_uniform_loc_position(loc);
                    }
                }
                UniformData::VecThree(data) => {
                    if let Some(loc) = render_api
                        .platform_api
                        .get_uniform_location(command.prog_id, key)
                    {
                        render_api.platform_api.uniform_3fv(loc, 1, data);

                        render_api
                            .platform_api
                            .remove_internal_uniform_loc_position(loc);
                    }
                }
                UniformData::Float(data) => {
                    if let Some(loc) = render_api
                        .platform_api
                        .get_uniform_location(command.prog_id, key)
                    {
                        render_api.platform_api.uniform_1f(loc, *data as f32);

                        render_api
                            .platform_api
                            .remove_internal_uniform_loc_position(loc);
                    }
                }
                UniformData::Texture(data) => {
                    if let Some(loc) = render_api
                        .platform_api
                        .get_uniform_location(command.prog_id, key)
                    {
                        render_api
                            .platform_api
                            .uniform_1i(loc, data.texture_slot as i32);
                        render_api
                            .platform_api
                            .active_texture(data.texture_slot as i32);

                        render_api
                            .platform_api
                            .bind_texture(TextureTarget::Texture2D, Some(data.image_id));

                        render_api
                            .platform_api
                            .remove_internal_uniform_loc_position(loc);
                    }
                }
            }
        }

        let mut dynamic_mesh_buffers: Vec<u32> = vec![];
        let vao_id: u32 = match &command.kind {
            VertexDataKind::Vao { id } => *id,
            VertexDataKind::DynamicMesh { mesh, uvs } => {
                let vao = Vao::new(render_api);

                // location is assumed 0. All shaders vertex positions are at location 0... for now.
                dynamic_mesh_buffers.append(
                    &mut vao
                        .upload_v3(render_api, mesh, &command.indices, 0)
                        .unwrap(),
                );

                // uvs
                dynamic_mesh_buffers.push(vao.upload_v2(render_api, uvs, 1).unwrap());

                vao.id
            }
        };

        render_api.platform_api.bind_vertex_array(vao_id);
        render_api
            .platform_api
            .draw_elements(GL_TRIANGLES, &command.indices);

        // Delete any dynamically created vao stuff
        {
            if let VertexDataKind::DynamicMesh { mesh, uvs } = &command.kind {
                render_api.platform_api.delete_vertex_arrays(1, vao_id);

                for b in dynamic_mesh_buffers {
                    render_api.platform_api.delete_buffers(1, b);
                }
            }
        }
    }
}
