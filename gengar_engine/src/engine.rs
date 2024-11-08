#![allow(unused_imports, dead_code)]

use std::include_str;

pub mod ascii;
pub mod color;
pub mod debug;
pub mod error;
pub mod font;
pub mod matricies;
pub mod model;
pub mod rect;
pub mod render;
pub mod state;
pub mod transform;
pub mod vectors;

// loaders
pub mod json;
pub mod obj;

use ascii::*;
use color::*;
use matricies::matrix_four_four::*;
use model::*;
use render::{render_command::*, shader::*, vao::*};
use state::*;
use transform::*;
use vectors::*;

pub fn load_resources(es: &mut State, render_api: &impl render::RenderApi) {
    es.pbr_shader = Shader::compile(
        include_str!("../engine_resources/shaders/pbr.vs"),
        include_str!("../engine_resources/shaders/pbr.fs"),
        render_api,
    )
    .unwrap();

    es.shader_color = Shader::compile(
        include_str!("../engine_resources/shaders/color.vs"),
        include_str!("../engine_resources/shaders/color.fs"),
        render_api,
    )
    .unwrap();

    es.font_sdf = Shader::compile(
        include_str!("../engine_resources/shaders/font_sdf.vs"),
        include_str!("../engine_resources/shaders/font_sdf.fs"),
        render_api,
    )
    .unwrap();

    es.color_texture_shader = Shader::compile(
        include_str!("../engine_resources/shaders/color_texture.vs"),
        include_str!("../engine_resources/shaders/color_texture.fs"),
        render_api,
    )
    .unwrap();

    es.model_sphere =
        Model::load_upload(include_str!("../engine_resources/sphere.obj"), render_api).unwrap();

    debug::init_context(es.shader_color, es.model_sphere.clone());
}

pub fn engine_frame_start(state: &mut State, _input: &Input, _render_api: &impl render::RenderApi) {
    // reset render lists
    state.render_commands = vec![];
    state.ui_render_commands = vec![];

    state.frame = state.frame + 1;

    debug::frame_start();
}

pub fn engine_frame_end(es: &mut State) {
    es.camera.update_matricies();
    es.ui_camera.update_matricies();

    Transform::update_all(&mut es.transforms);
}
