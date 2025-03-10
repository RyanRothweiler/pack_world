use crate::{
    fixed_string::*,
    input::*,
    model::*,
    render::{camera::*, render_command::*, render_pack::*, shader::*, vao::*},
    transform::*,
    typeface::*,
    ui::*,
    vectors::*,
};
use std::{cell::RefCell, collections::HashMap};

pub struct State {
    pub window_resolution: VecTwo,
    pub frame: i64,

    pub font_sdf: Shader,
    pub shader_color: Shader,
    pub shader_color_ui: Shader,
    pub color_texture_shader: Shader,

    pub roboto_typeface: Typeface,

    pub ui_render_pack: RenderPack,
    pub game_render_pack: RenderPack,
}

impl State {
    pub fn new(window_resolution: VecTwo) -> Self {
        Self {
            window_resolution,
            frame: 0,

            shader_color: Shader::new_empty(),
            shader_color_ui: Shader::new_empty(),
            color_texture_shader: Shader::new_empty(),
            font_sdf: Shader::new_empty(),

            roboto_typeface: Typeface::new(),

            ui_render_pack: RenderPack::new(ProjectionType::Orthographic, window_resolution),
            game_render_pack: RenderPack::new(ProjectionType::Orthographic, window_resolution),
        }
    }
}
