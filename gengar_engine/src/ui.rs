use crate::{
    color::*,
    font::*,
    rect::*,
    render::{material::*, render_command::*, shader::*},
    state::{ButtonState, Input},
    vectors::*,
};
use std::collections::HashMap;

struct UIContext {
    pub mouse_pos: VecTwo,
    pub mouse_down: bool,

    pub button_shader: Shader,

    pub render_commands: Vec<RenderCommand>,
    pub button_state: HashMap<String, ButtonState>,
}

static mut UI_CONTEXT: Option<UIContext> = None;

pub fn frame_start(input: &Input, button_shader: Shader) {
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

                    button_shader,

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

pub fn draw_button(display: &str, line: u32, rect: &Rect, style: &FontStyle) -> bool {
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };

    let contains = rect.contains(context.mouse_pos);
    let mut color = Color::red();
    if contains {
        color = Color::green();
    }

    // draw button
    {
        let mut mat = Material::new();
        mat.shader = Some(context.button_shader);

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

    return contains && button_state.on_press;
}
