use crate::{
    color::*,
    rect::*,
    render::{material::*, render_command::*, shader::*},
    state::Input,
    vectors::*,
};

struct UIContext {
    pub mouse_pos: VecTwo,
    pub mouse_down: bool,

    pub render_commands: Vec<RenderCommand>,
}

static mut UI_CONTEXT: Option<UIContext> = None;

pub fn frame_start(input: &Input) {
    unsafe {
        UI_CONTEXT = Some(UIContext {
            mouse_pos: input.mouse_pos,
            mouse_down: input.mouse_left.pressing,
            render_commands: vec![],
        });
    }
}

pub fn get_render_commands() -> Vec<RenderCommand> {
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };
    return context.render_commands.clone();
}

pub fn draw_button(rect: &Rect, shader: Shader) -> bool {
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };

    let contains = rect.contains(context.mouse_pos);
    let mut color = Color::red();
    if contains {
        color = Color::green();
    }

    // draw button
    {
        let mut mat = Material::new();
        mat.shader = Some(shader);

        mat.uniforms
            .insert("color".to_string(), UniformData::VecFour(color.into()));

        let r = Rect::new(VecTwo::new(100.0, 100.0), VecTwo::new(500.0, 500.0));

        context
            .render_commands
            .push(RenderCommand::new_rect_outline(&r, -1.0, 1.0, &mat));
    }

    return contains && context.mouse_down;
}
