use crate::{
    font::*,
    input::*,
    model::*,
    render::{camera::*, render_command::*, render_pack::*, shader::*, vao::*},
    transform::*,
    ui::*,
    vectors::*,
};
use std::{cell::RefCell, collections::HashMap};

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

    pub render_packs: HashMap<RenderPackID, RenderPack>,

    // Pseudo ecs stuff.
    // This doesn't handle 'deallocation'
    pub transforms: Vec<Transform>,

    pub roboto_font: Typeface,
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

            render_packs: HashMap::new(),

            window_resolution,
            transforms: vec![],

            model_sphere: Model::new(),
            model_plane: Model::new(),

            roboto_font: Default::default(),

            frame: 0,
        };

        state.render_packs.insert(
            RenderPackID::UI,
            RenderPack::new(ProjectionType::Orthographic, window_resolution),
        );

        state.render_packs.insert(
            RenderPackID::World,
            RenderPack::new(ProjectionType::Orthographic, window_resolution),
        );

        return state;
    }

    pub fn new_transform(&mut self) -> usize {
        self.transforms.push(Transform::new());
        return self.transforms.len() - 1;
    }
}
