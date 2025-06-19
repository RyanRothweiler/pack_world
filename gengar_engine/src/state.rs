use crate::{
    input::*,
    model::*,
    render::{camera::*, render_command::*, render_pack::*, shader::*, vao::*},
    transform::*,
    typeface::*,
    ui::*,
    vectors::*,
};
use std::{cell::RefCell, collections::HashMap};

pub mod components;
pub mod render_system;

pub use components::*;
pub use render_system::*;

// TODO rename engine state
pub struct State {
    pub window_resolution: VecTwo,

    pub pbr_shader: Shader,
    pub shader_color: Shader,
    pub shader_color_ui: Shader,
    pub color_texture_shader: Shader,
    pub font_sdf: Shader,

    pub model_sphere: Model,
    pub model_plane: Model,

    pub frame: i64,

    // the game runs its its own dll. so the debug render commands is in the dll memory space
    // after the game frame ends, the game passes its debug render commands here
    pub game_debug_render_commands: Vec<RenderCommand>,
    pub game_ui_debug_render_commands: Vec<RenderCommand>,

    pub roboto_typeface: Typeface,

    pub game_to_load: Vec<u8>,

    pub render_commands_len: i32,

    pub components: Components,
    pub render_system: RenderSystem,

    pub title_bar_height: i32,
}

impl State {
    pub fn new(window_resolution: VecTwo) -> Self {
        let mut state = State {
            pbr_shader: Shader::new_empty(),
            color_texture_shader: Shader::new_empty(),
            shader_color: Shader::new_empty(),
            shader_color_ui: Shader::new_empty(),
            font_sdf: Shader::new_empty(),

            game_debug_render_commands: vec![],
            game_ui_debug_render_commands: vec![],

            window_resolution,

            model_sphere: Model::new(),
            model_plane: Model::new(),

            roboto_typeface: Typeface::new(),

            // if this is not empty then load the game data from this
            game_to_load: vec![],

            frame: 0,

            render_commands_len: 0,

            components: Components::new(),
            render_system: RenderSystem::new(),

            title_bar_height: 0,
        };

        state.render_system.render_packs.insert(
            RenderPackID::World,
            RenderPack::new(ProjectionType::Orthographic, window_resolution),
        );

        state.render_system.render_packs.insert(
            RenderPackID::UI,
            RenderPack::new(ProjectionType::Orthographic, window_resolution),
        );

        state.render_system.render_packs.insert(
            RenderPackID::NewWorld,
            RenderPack::new(
                ProjectionType::Perspective { focal_length: 0.95 },
                window_resolution,
            ),
        );

        state.render_system.render_packs.insert(
            RenderPackID::Shop,
            RenderPack::new(
                ProjectionType::Perspective { focal_length: 0.95 },
                window_resolution,
            ),
        );

        return state;
    }
}
