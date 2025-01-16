use crate::{
    color::*,
    matricies::matrix_four_four::*,
    model::*,
    rect::*,
    render::{camera::*, material::*, render_command::*, shader::*},
    state::*,
    transform::*,
    vectors::*,
};

use std::cell::RefCell;

pub struct DebugContext {
    pub render_commands: Vec<RenderCommand>,
    pub ui_render_commands: Vec<RenderCommand>,

    pub shader_color: Shader,
    pub shader_color_ui: Shader,

    pub model_sphere: Model,
    pub model_plane: Model,
}

static mut DEBUG_CONTEXT: Option<DebugContext> = None;

pub fn init_context(
    shader_color: Shader,
    shader_color_ui: Shader,
    model_sphere: Model,
    model_plane: Model,
) {
    unsafe {
        DEBUG_CONTEXT = Some(DebugContext {
            render_commands: vec![],
            ui_render_commands: vec![],
            shader_color,
            shader_color_ui,
            model_sphere,
            model_plane,
        });
    }
}

pub fn frame_start() {
    unsafe {
        DEBUG_CONTEXT.as_mut().as_mut().unwrap().render_commands = vec![];
        DEBUG_CONTEXT.as_mut().as_mut().unwrap().ui_render_commands = vec![];
    }
}

pub fn get_render_list<'a>() -> &'a mut Vec<RenderCommand> {
    unsafe {
        return &mut DEBUG_CONTEXT.as_mut().unwrap().render_commands;
    }
}

pub fn get_ui_render_list<'a>() -> &'a mut Vec<RenderCommand> {
    unsafe {
        return &mut DEBUG_CONTEXT.as_mut().unwrap().ui_render_commands;
    }
}

pub fn draw_sphere(center: VecThreeFloat, size: f64, color: Color) {
    let context: &mut DebugContext = unsafe { DEBUG_CONTEXT.as_mut().unwrap() };

    let mut trans = Transform::new();
    trans.local_position = center;
    trans.local_scale = VecThreeFloat::new(size, size, size);
    trans.update_global_matrix(&M44::new_identity());

    let mut material = Material::new();
    material.shader = Some(context.shader_color);
    material.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(VecFour::from(color)),
    );

    let model = context.model_sphere.clone();

    context
        .render_commands
        .push(RenderCommand::new_model(&trans, &model, &material));

    panic!("this won't render correctly because I've changed the world render pack to be orthographic.");
}

pub fn draw_plane(center: VecThreeFloat, size: f64, color: Color) {
    let context: &mut DebugContext = unsafe { DEBUG_CONTEXT.as_mut().unwrap() };

    let mut trans = Transform::new();
    trans.local_position = center;
    trans.local_scale = VecThreeFloat::new(size, size, size);
    trans.update_global_matrix(&M44::new_identity());

    let mut material = Material::new();
    material.shader = Some(context.shader_color);
    material.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(VecFour::from(color)),
    );

    let model = context.model_plane.clone();

    context
        .render_commands
        .push(RenderCommand::new_model(&trans, &model, &material));

    panic!("this won't render correctly because I've changed the world render pack to be orthographic.");
}

pub fn draw_rect(rect: &Rect, color: Color) {
    let context: &mut DebugContext = unsafe { DEBUG_CONTEXT.as_mut().unwrap() };

    let mut material = Material::new();
    material.shader = Some(context.shader_color_ui);
    material.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(VecFour::from(color)),
    );

    context
        .ui_render_commands
        .push(RenderCommand::new_rect(rect, -1.0, &material));
}
