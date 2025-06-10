use png::*;
use std::{fs::File, io::Cursor, path::Path};

use crate::{
    error::*,
    model::*,
    rect::*,
    render::{components::*, frame_buffer_pack::*, material::Material, render_pack::*},
    state::*,
    transform::*,
    vectors::*,
};
use image::*;
use render_command::*;
use shader::*;

pub mod accumulate_draw;
pub mod camera;
pub mod frame_buffer_pack;
pub mod image;
pub mod light;
pub mod material;
pub mod render_command;
pub mod render_pack;
pub mod shader;
pub mod vao;

// Render backend independent calls. This abstracts over all render backend. That is the hope.
pub trait RenderApi {
    fn make_shader_program(&mut self, vert_shader: &str, frag_shader: &str) -> Result<u32, Error>;
    fn create_vao(&self) -> Result<u32, Error>;

    // if gamma_correct is true then we'll pass srgb color space so that the image is gamma corrected by the graphics card.
    fn upload_texture(&self, image: &Image, gamma_correct: bool) -> Result<u32, Error>;

    fn vao_upload_v3(
        &self,
        vao: &vao::Vao,
        data: &Vec<VecThreeFloat>,
        indices: &Vec<u32>,
        location: u32,
    ) -> Result<u32, Error>;

    fn vao_upload_v2(
        &self,
        vao: &vao::Vao,
        data: &Vec<VecTwo>,
        location: u32,
    ) -> Result<u32, Error>;

    fn build_frame_buffer(&self, width: i32, height: i32) -> Result<FrameBufferPack, Error>;

    fn draw_frame_buffer(
        &self,
        frame_buffer: u32,
        render_pack: &mut RenderPack,
        components: &Components,
    );
}

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub fn load_image_cursor(bytes: &[u8], render_api: &impl RenderApi) -> Result<Image, Error> {
    let cursor = Cursor::new(bytes);
    let mut img = load_image(cursor).unwrap();
    img.gl_id = Some(render_api.upload_texture(&img, false).unwrap());

    return Ok(img);
}

pub fn load_image_path(path: &Path) -> Result<Image, Error> {
    load_image(File::open(path)?)
}

pub fn load_image(read: impl std::io::Read) -> Result<Image, Error> {
    let mut image = Image::new();

    let image_dec = png::Decoder::new(read);
    let mut reader = image_dec.read_info().unwrap();
    image.data = vec![0; reader.output_buffer_size()];

    let info = reader.next_frame(&mut image.data).unwrap();
    image.width = info.width;
    image.height = info.height;

    // Check for image type that we support
    match info.color_type {
        ColorType::Rgba => image.format = ImageFormat::RGBA,
        ColorType::Rgb => image.format = ImageFormat::RGB,
        _ => {
            eprintln!("Invalid image format {:?}", info.color_type);
            return Err(Error::InvalidImageFormat);
        }
    }

    if info.bit_depth != BitDepth::Eight {
        return Err(Error::InvalidImageBitDepth);
    }

    Ok(image)
}
