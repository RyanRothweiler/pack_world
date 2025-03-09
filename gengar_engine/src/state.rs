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

    pub model_sphere: Model,
    pub model_plane: Model,

    pub roboto_typeface: Typeface,

    // the game runs its its own dll. so the debug render commands is in the dll memory space
    // after the game frame ends, the game passes its debug render commands here
    pub game_debug_render_commands: Vec<RenderCommand>,
    pub game_ui_debug_render_commands: Vec<RenderCommand>,

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

            model_sphere: Model::new(),
            model_plane: Model::new(),

            game_debug_render_commands: vec![],
            game_ui_debug_render_commands: vec![],

            roboto_typeface: Typeface::new(),

            ui_render_pack: RenderPack::new(ProjectionType::Orthographic, window_resolution),
            game_render_pack: RenderPack::new(ProjectionType::Orthographic, window_resolution),
        }
    }
}
