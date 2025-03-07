#![allow(
    unused_imports,
    dead_code,
    clippy::all,
    static_mut_refs,
    unused_variables
)]

use std::{include_str, io::Cursor};

pub mod ascii;
pub mod color;
pub mod debug;
pub mod error;
pub mod input;
pub mod math;
pub mod matricies;
pub mod memory_arena;
pub mod model;
pub mod rect;
pub mod render;
pub mod state;
pub mod transform;
pub mod typeface;
pub mod ui;
pub mod vectors;

// loaders
pub mod json;
pub mod obj;

use ascii::*;
use color::*;
use input::*;
use matricies::matrix_four_four::*;
// use memory_arena::*;
use model::*;
use render::{render_command::*, shader::*, vao::*};
use state::*;
use transform::*;
use typeface::*;
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

    es.shader_color_ui = Shader::compile(
        include_str!("../engine_resources/shaders/color_ui.vs"),
        include_str!("../engine_resources/shaders/color_ui.fs"),
        render_api,
    )
    .unwrap();

    es.model_sphere =
        Model::load_upload(include_str!("../engine_resources/sphere.obj"), render_api).unwrap();
    es.model_plane =
        Model::load_upload(include_str!("../engine_resources/plane.obj"), render_api).unwrap();

    // roboto
    {
        es.roboto_typeface.setup(es.font_sdf);

        es.roboto_typeface.load_weight(
            TypeWeight::Bold,
            include_str!("../engine_resources/fonts/roboto/roboto_bold_data.json").into(),
            Cursor::new(include_bytes!(
                "../engine_resources/fonts/roboto/roboto_bold_atlas.png"
            )),
            render_api,
        );
        es.roboto_typeface.load_weight(
            TypeWeight::Regular,
            include_str!("../engine_resources/fonts/roboto/roboto_regular_data.json").into(),
            Cursor::new(include_bytes!(
                "../engine_resources/fonts/roboto/roboto_regular_atlas.png"
            )),
            render_api,
        );
        es.roboto_typeface.load_weight(
            TypeWeight::Medium,
            include_str!("../engine_resources/fonts/roboto/roboto_medium_data.json").into(),
            Cursor::new(include_bytes!(
                "../engine_resources/fonts/roboto/roboto_medium_atlas.png"
            )),
            render_api,
        );
    }

    debug::init_context(
        es.shader_color,
        es.shader_color_ui,
        es.model_sphere.clone(),
        es.model_plane.clone(),
    );
}

pub fn engine_frame_start(es: &mut State, _input: &Input, _render_api: &impl render::RenderApi) {
    // reset render lists
    for (key, pack) in &mut es.render_packs {
        pack.commands.clear();
    }

    es.frame = es.frame + 1;

    debug::frame_start();
}

pub fn engine_frame_end(es: &mut State) {
    for (key, pack) in &mut es.render_packs {
        pack.camera.update_matricies();
    }

    Transform::update_all(&mut es.transforms);
}
