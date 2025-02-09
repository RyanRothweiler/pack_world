use crate::{
    color::*,
    font::*,
    input::{ButtonState, Input},
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    vectors::*,
};
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

pub struct UIContext {
    pub mouse_pos: VecTwo,
    pub mouse_down: bool,

    pub color_shader: Shader,
    pub color_shader_texture: Shader,

    pub render_commands: Vec<RenderCommand>,
    pub button_state: HashMap<String, ButtonState>,

    pub button_font_style: FontStyle,
}

pub struct UIFrameState {
    pub resolution: VecTwo,

    pub mouse_left: bool,
    pub panel_stack: Vec<Rect>,
}

impl UIFrameState {
    pub fn new(input: &Input, resolution: VecTwo) -> Self {
        Self {
            mouse_left: input.mouse_left.on_press,
            panel_stack: vec![],
            resolution,
        }
    }

    pub fn get_origin(&self) -> VecTwo {
        match self.panel_stack.last() {
            Some(p) => p.top_left,
            None => VecTwo::new(0.0, 0.0),
        }
    }
}

pub fn draw_button(
    display: &str,
    maybe_icon: Option<u32>,
    rect: &Rect,
    ui_state: &mut UIFrameState,
    line: u32,
    context: &mut UIContext,
) -> bool {
    let origin = ui_state.get_origin();

    let mut rect = *rect;
    rect.translate(origin);

    let contains = rect.contains(context.mouse_pos);
    let mut color = COLOR_BLUE;
    if contains {
        color = COLOR_GREEN;
    }

    // draw button outline
    {
        let mut mat = Material::new();
        mat.shader = Some(context.color_shader);
        mat.set_color(color);
        context
            .render_commands
            .push(RenderCommand::new_rect_outline(&rect, -1.0, 1.0, &mat));
    }

    // draw icon
    if let Some(icon) = maybe_icon {
        let mut icon_rect: Rect = rect.clone();
        icon_rect.shrink(2.0);

        let mut mat = Material::new();
        mat.shader = Some(context.color_shader_texture);
        mat.set_image(icon);
        mat.set_color(COLOR_WHITE);

        context
            .render_commands
            .push(RenderCommand::new_rect(&icon_rect, -1.0, 0.0, &mat));
    }

    render_word(
        display.into(),
        &context.button_font_style,
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

pub fn draw_image(
    mut rect: Rect,
    image: u32,
    color: Color,
    ui_state: &mut UIFrameState,
    context: &mut UIContext,
) {
    let mut mat = Material::new();
    mat.shader = Some(context.color_shader_texture);
    mat.set_image(image);
    mat.set_color(color);

    let origin = ui_state.get_origin();
    rect.translate(origin);

    context
        .render_commands
        .push(RenderCommand::new_rect(&rect, -1.0, 0.0, &mat));
}

pub fn draw_text(display: &str, pos: VecTwo, ui_state: &mut UIFrameState, context: &mut UIContext) {
    let origin = ui_state.get_origin();

    render_word(
        display.into(),
        &context.button_font_style,
        pos + origin,
        &mut context.render_commands,
    );
}

pub fn begin_panel(
    rect: Rect,
    color: Color,
    frame_state: &mut UIFrameState,
    context: &mut UIContext,
) {
    draw_rect(rect, color, frame_state, context);
    frame_state.panel_stack.push(rect);
}

pub fn draw_rect(
    mut rect: Rect,
    color: Color,
    frame_state: &mut UIFrameState,
    context: &mut UIContext,
) {
    let origin = frame_state.get_origin();
    rect.translate(origin);

    let mut mat = Material::new();
    mat.shader = Some(context.color_shader);
    mat.set_color(color);
    context
        .render_commands
        .push(RenderCommand::new_rect(&rect, -1.0, 0.0, &mat));
}

pub fn end_panel(frame_state: &mut UIFrameState, context: &mut UIContext) {
    if frame_state
        .panel_stack
        .pop()
        .expect("End panel called without an associated beginning panel.")
        .contains(context.mouse_pos)
    {
        frame_state.mouse_left = false;
    }
}

pub fn draw_progress_bar(
    progress: f64,
    rect: &Rect,
    shader_color: Shader,
    render_pack: &mut RenderPack,
) {
    // draw fill
    {
        let mut fill_rect = rect.clone();
        let width = fill_rect.width();
        fill_rect.resize_right(width * progress.clamp(0.0, 1.0));

        let mut mat = Material::new();
        mat.shader = Some(shader_color);
        mat.uniforms.insert(
            "color".to_string(),
            UniformData::VecFour(Color::new(1.0, 1.0, 1.0, 0.5).into()),
        );

        render_pack
            .commands
            .push(RenderCommand::new_rect(&fill_rect, -1.0, 0.0, &mat));
    }

    // draw outline
    {
        let mut mat = Material::new();
        mat.shader = Some(shader_color);
        mat.uniforms.insert(
            "color".to_string(),
            UniformData::VecFour(Color::new(1.0, 1.0, 1.0, 0.5).into()),
        );

        render_pack
            .commands
            .push(RenderCommand::new_rect_outline(rect, -1.0, 1.0, &mat));
    }
}
