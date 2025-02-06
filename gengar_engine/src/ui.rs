#![allow(unused_mut)]

use crate::{
    color::*,
    font::*,
    input::{ButtonState, Input},
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    vectors::*,
};
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

struct UIContext {
    pub mouse_pos: VecTwo,
    pub mouse_down: bool,

    pub color_shader: Shader,
    pub color_shader_texture: Shader,

    pub render_commands: Vec<RenderCommand>,
    pub button_state: HashMap<String, ButtonState>,
}

// TODO could use static mutex here to remove unsafe
// static mut UI_CONTEXT: Option<UIContext> = None;

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

pub fn frame_start(input: &Input, color_shader: Shader, color_shader_texture: Shader) {
    /*
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
                    color_shader_texture,

                    render_commands: vec![],
                    button_state: HashMap::new(),
                });
            }
        }
    }
    */
}

/*
pub fn get_render_commands() -> Vec<RenderCommand> {
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };
    return context.render_commands.clone();
}
    */

pub fn draw_button(
    display: &str,
    maybe_icon: Option<u32>,
    rect: &Rect,
    style: &FontStyle,
    ui_state: &mut UIFrameState,
    line: u32,
) -> bool {
    false

    /*
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };

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
    */
}

pub fn draw_image(mut rect: Rect, image: u32, color: Color, ui_state: &mut UIFrameState) {
    /*
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };

    let mut mat = Material::new();
    mat.shader = Some(context.color_shader_texture);
    mat.set_image(image);
    mat.set_color(color);

    let origin = ui_state.get_origin();
    rect.translate(origin);

    context
        .render_commands
        .push(RenderCommand::new_rect(&rect, -1.0, 0.0, &mat));
    */
}

pub fn draw_text(display: &str, style: &FontStyle, pos: VecTwo, ui_state: &mut UIFrameState) {
    /*
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().unwrap() };

    let origin = ui_state.get_origin();

    render_word(
        display.into(),
        style,
        pos + origin,
        &mut context.render_commands,
    );
    */
}

pub fn begin_panel(rect: Rect, color: Color, frame_state: &mut UIFrameState) {
    /*
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().expect("Missing ui context") };

    let mut mat = Material::new();
    mat.shader = Some(context.color_shader);

    mat.uniforms
        .insert("color".to_string(), UniformData::VecFour(color.into()));

    context
        .render_commands
        .push(RenderCommand::new_rect(&rect, -1.0, 0.0, &mat));

    frame_state.panel_stack.push(rect);
    */
}

pub fn end_panel(frame_state: &mut UIFrameState) {
    /*
    let context: &mut UIContext = unsafe { UI_CONTEXT.as_mut().expect("Missing ui context") };

    if frame_state
        .panel_stack
        .pop()
        .expect("End panel called without an associated beginning panel.")
        .contains(context.mouse_pos)
    {
        frame_state.mouse_left = false;
    }
    */
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
