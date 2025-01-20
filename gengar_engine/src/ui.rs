use crate::{
    color::*,
    font::*,
    rect::*,
    render::{material::*, render_command::*, shader::*},
    state::{ButtonState, Input},
    vectors::*,
};
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

struct UIContext {
    pub mouse_pos: VecTwo,
    pub mouse_down: bool,

    pub color_shader: Shader,

    pub render_commands: Vec<RenderCommand>,
    pub button_state: HashMap<String, ButtonState>,
}

// TODO could use static mutex here to remove unsafe
static mut UI_CONTEXT: Option<UIContext> = None;

pub struct UIFrameState {
    pub resolution: VecTwo,

    pub mouse_left: bool,
    pub current_panel: Option<Rect>,
}

impl UIFrameState {
    pub fn new(input: &Input, resolution: VecTwo) -> Self {
        Self {
            mouse_left: input.mouse_left.on_press,
            current_panel: None,
            resolution,
        }
    }
}

pub fn frame_start(input: &Input, color_shader: Shader) {
    unsafe {
        match UI_CONTEXT.as_mut() {
            Some(c) => {
                c.mouse_pos = input.mouse_pos;
                c.mouse_down = input.mouse_left.pressing;

                c.render_commands.clear();
            }
            None => {
                UI_CONTEXT = Some(UIContext {
                    mouse_pos: input.mouse_pos,
                    mouse_down: input.mouse_left.pressing,

                    color_shader,

                    render_commands: vec![],
                    button_state: HashMap::new(),
                });
            }
        }
    }
}

pub fn get_render_commands() -> Vec<RenderCommand> {
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };
    return context.render_commands.clone();
}

pub fn draw_button(
    display: &str,
    line: u32,
    rect: &Rect,
    style: &FontStyle,
    ui_state: &mut UIFrameState,
) -> bool {
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };

    let contains = rect.contains(context.mouse_pos);
    let mut color = COLOR_BLUE;
    if contains {
        color = COLOR_GREEN;
    }

    // draw button
    {
        let mut mat = Material::new();
        mat.shader = Some(context.color_shader);

        mat.uniforms
            .insert("color".to_string(), UniformData::VecFour(color.into()));

        context
            .render_commands
            .push(RenderCommand::new_rect_outline(&rect, -1.0, 1.0, &mat));
    }

    render_word(
        display.into(),
        style,
        rect.bottom_left() + VecTwo::new(7.0, -7.0),
        &mut context.render_commands,
    );

    // handle state
    let id = format!("{}{}", display, line);
    let button_state = context.button_state.entry(id).or_insert(ButtonState::new());
    button_state.update(context.mouse_down);

    if contains {
        ui_state.mouse_left = false;
    }

    return contains && button_state.on_press;
}

pub fn begin_panel(rect: Rect, color: Color, frame_state: &mut UIFrameState) {
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };

    let mut mat = Material::new();
    mat.shader = Some(context.color_shader);

    mat.uniforms
        .insert("color".to_string(), UniformData::VecFour(color.into()));

    context
        .render_commands
        .push(RenderCommand::new_rect(&rect, -1.0, &mat));

    frame_state.current_panel = Some(rect);
}

pub fn end_panel(frame_state: &mut UIFrameState) {
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };

    if frame_state
        .current_panel
        .clone()
        .unwrap()
        .contains(context.mouse_pos)
    {
        frame_state.mouse_left = false;
    }

    frame_state.current_panel = None;
}
