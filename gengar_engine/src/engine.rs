#![allow(
    unused_imports,
    dead_code,
    clippy::all,
    static_mut_refs,
    unused_variables
)]

use std::{include_str, io::Cursor};

pub mod ascii;
pub mod byte_conversion;
pub mod color;
pub mod debug;
pub mod error;
pub mod fixed_string;
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
use model::*;
use render::{render_command::*, shader::*, vao::*};
use state::*;
use transform::*;
use typeface::*;
use vectors::*;

pub fn load_resources(es: &mut State, nes: &mut NewState, render_api: &impl render::RenderApi) {
    nes.shader_color
        .compile(
            include_str!("../engine_resources/shaders/color.vs"),
            include_str!("../engine_resources/shaders/color.fs"),
            render_api,
        )
        .unwrap();

    nes.font_sdf
        .compile(
            include_str!("../engine_resources/shaders/font_sdf.vs"),
            include_str!("../engine_resources/shaders/font_sdf.fs"),
            render_api,
        )
        .unwrap();

    nes.color_texture_shader
        .compile(
            include_str!("../engine_resources/shaders/color_texture.vs"),
            include_str!("../engine_resources/shaders/color_texture.fs"),
            render_api,
        )
        .unwrap();

    nes.shader_color_ui
        .compile(
            include_str!("../engine_resources/shaders/color_ui.vs"),
            include_str!("../engine_resources/shaders/color_ui.fs"),
            render_api,
        )
        .unwrap();

    nes.model_sphere
        .load_upload(include_str!("../engine_resources/sphere.obj"), render_api)
        .unwrap();
    nes.model_plane
        .load_upload(include_str!("../engine_resources/plane.obj"), render_api)
        .unwrap();

    // roboto
    {
        nes.roboto_typeface.setup(nes.font_sdf);

        nes.roboto_typeface.load_weight(
            TypeWeight::Bold,
            include_str!("../engine_resources/fonts/roboto/roboto_bold_data.json").into(),
            Cursor::new(include_bytes!(
                "../engine_resources/fonts/roboto/roboto_bold_atlas.png"
            )),
            render_api,
        );
        nes.roboto_typeface.load_weight(
            TypeWeight::Regular,
            include_str!("../engine_resources/fonts/roboto/roboto_regular_data.json").into(),
            Cursor::new(include_bytes!(
                "../engine_resources/fonts/roboto/roboto_regular_atlas.png"
            )),
            render_api,
        );
        nes.roboto_typeface.load_weight(
            TypeWeight::Medium,
            include_str!("../engine_resources/fonts/roboto/roboto_medium_data.json").into(),
            Cursor::new(include_bytes!(
                "../engine_resources/fonts/roboto/roboto_medium_atlas.png"
            )),
            render_api,
        );
    }

    debug::init_context(
        nes.shader_color,
        nes.shader_color_ui,
        nes.model_sphere.clone(),
        nes.model_plane.clone(),
    );
}

pub fn engine_frame_start(
    es: &mut State,
    nes: &mut NewState,
    _input: &Input,
    _render_api: &impl render::RenderApi,
) {
    // reset render lists
    for (key, pack) in &mut es.render_packs {
        pack.commands.clear();
    }

    nes.frame = nes.frame + 1;

    debug::frame_start();
}

pub fn engine_frame_end(es: &mut State) {
    for (key, pack) in &mut es.render_packs {
        pack.camera.update_matricies();
    }

    Transform::update_all(&mut es.transforms);
}
