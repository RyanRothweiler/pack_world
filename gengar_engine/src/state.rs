use crate::{
    font::*,
    model::*,
    render::{camera::*, render_command::*, shader::*, vao::*},
    transform::*,
    vectors::*,
};

pub struct State {
    pub window_resolution: VecTwo,

    pub pbr_shader: Shader,
    pub shader_color: Shader,
    pub color_texture_shader: Shader,
    pub font_sdf: Shader,

    pub model_sphere: Model,

    pub frame: i64,

    // the game runs its its own dll. so the debug render commands is in the dll memory space
    // after the game frame ends, the game passes its debug render cammers here
    pub game_debug_render_commands: Vec<RenderCommand>,

    pub render_commands: Vec<RenderCommand>,
    pub camera: Camera,

    pub ui_render_commands: Vec<RenderCommand>,
    pub ui_camera: Camera,

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
            font_sdf: Shader::new_empty(),

            render_commands: vec![],
            ui_render_commands: vec![],

            game_debug_render_commands: vec![],
            window_resolution,
            model_sphere: Model::new(),
            transforms: vec![],

            camera: Camera::new(
                ProjectionType::Perspective { focal_length: 0.95 },
                window_resolution,
            ),
            ui_camera: Camera::new(ProjectionType::Orthographic, window_resolution),

            roboto_font: Typeface::new(),

            frame: 0,
        };

        state.camera.transform.local_position.z = 4.0;
        state.ui_camera.transform.local_position.z = 0.0;

        return state;
    }

    pub fn new_transform(&mut self) -> usize {
        self.transforms.push(Transform::new());
        return self.transforms.len() - 1;
    }
}

#[derive(Copy, Clone)]
pub struct ButtonState {
    pub pressing: bool,
    pub on_press: bool,
    pub on_release: bool,
}

impl ButtonState {
    pub fn new() -> Self {
        ButtonState {
            pressing: false,
            on_press: false,
            on_release: false,
        }
    }

    pub fn update(&mut self, new_state: bool) {
        if new_state {
            self.on_release = false;

            if !self.pressing {
                self.on_press = true;
            } else {
                self.on_press = false;
            }
        } else {
            self.on_press = false;

            if self.pressing {
                self.on_release = true;
            } else {
                self.on_release = false;
            }
        }

        self.pressing = new_state;
    }
}

pub struct Input {
    pub mouse_pos: VecTwo,
    pub mouse_pos_delta: VecTwo,
    pub mouse_left: ButtonState,
    pub mouse_right: ButtonState,

    pub keyboard: [ButtonState; 128],
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse_left: ButtonState::new(),
            mouse_right: ButtonState::new(),
            mouse_pos: VecTwo::new(0.0, 0.0),
            mouse_pos_delta: VecTwo::new(0.0, 0.0),
            keyboard: [ButtonState::new(); 128],
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn press_update() {
        let mut button = ButtonState::new();

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, true);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, true);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, true);
        assert_eq!(button.on_release, false);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, true);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);
    }
}
