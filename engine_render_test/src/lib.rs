#![allow(
    unused_imports,
    unused_variables,
    clippy::all,
    unused_mut,
    unreachable_code
)]

use gengar_engine::{
    analytics::*,
    ascii::*,
    collisions::*,
    color::*,
    debug::*,
    input::*,
    matricies::matrix_four_four::*,
    model::*,
    obj,
    platform_api::*,
    rect::*,
    render::{
        camera::*, image::Image, light::*, load_image, load_image_cursor, material::*,
        render_command::RenderCommand, render_pack::*, shader::*, vao::*, RenderApi,
    },
    state::State as EngineState,
    transform::*,
    typeface::*,
    ui::*,
    vectors::*,
};
use gengar_render_opengl::*;
use std::{
    collections::HashMap,
    fs::File,
    io::Cursor,
    path::Path,
    sync::{LazyLock, Mutex},
};

pub mod state;

use state::*;

pub const PACKAGE_NAME: &str = "engine_test";

pub fn game_init(
    gs: &mut State,
    es: &mut EngineState,
    render_api: &mut impl RenderApi,
    platform_api: &PlatformApi,
) {
}

pub fn game_loop(
    prev_delta_time: f64,
    gs: &mut State,
    es: &mut EngineState,
    input: &mut Input,
    render_api: &mut impl RenderApi,
    platform_api: &PlatformApi,
) {
    // draw fill
    {
        let rect = Rect::new_center(VecTwo::new(500.0, 500.0), VecTwo::new(20.0, 20.0));

        let mut mat = Material::new();
        mat.shader = Some(es.shader_color);
        mat.uniforms
            .insert("color".to_string(), UniformData::VecFour(COLOR_RED.into()));

        es.render_system.add_command(
            RenderCommand::new_rect(&rect, -1.0, 0.0, &mat),
            RenderPackID::UI,
        );
    }
}
