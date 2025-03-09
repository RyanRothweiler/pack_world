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

// slowly start migrating things from state into StateArena
pub struct NewState {
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
}

impl NewState {
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
            
            // ui
        }
    }
}

// TODO rename engine state
pub struct State {
    pub render_packs: HashMap<RenderPackID, RenderPack>,

    // Pseudo ecs stuff.
    // This doesn't handle 'deallocation'
    pub transforms: Vec<Transform>,
}

impl State {
    pub fn new(window_resolution: VecTwo) -> Self {
        let mut state = State {
            render_packs: HashMap::new(),

            transforms: vec![],
        };

        state.render_packs.insert(
            RenderPackID::World,
            RenderPack::new(ProjectionType::Orthographic, window_resolution),
        );

        state.render_packs.insert(
            RenderPackID::UI,
            RenderPack::new(ProjectionType::Orthographic, window_resolution),
        );

        return state;
    }

    pub fn new_transform(&mut self) -> usize {
        self.transforms.push(Transform::new());
        return self.transforms.len() - 1;
    }
}
