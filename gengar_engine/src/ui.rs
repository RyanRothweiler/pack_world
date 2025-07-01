use crate::{
    color::*,
    font::*,
    input::{keyboard::*, mouse::*, Input},
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    vectors::*,
    KeyCode,
};
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

pub mod button;
pub mod input_field;
pub mod theme;

pub use button::*;
pub use input_field::*;
pub use theme::*;

pub struct UIContext {
    pub mouse: Mouse,
    pub keyboard: Keyboard,

    pub color_shader: Shader,
    pub color_shader_texture: Shader,

    pub render_commands: Vec<RenderCommand>,

    pub button_state: HashMap<String, ButtonData>,
    pub input_fields: HashMap<String, InputFieldData>,

    pub selected_input_field: Option<String>,

    pub font_body: FontStyle,
    pub font_header: FontStyle,
    pub font_nav: FontStyle,

    pub delta_time: f64,
}

pub struct UIFrameState {
    pub resolution: VecTwo,

    pub mouse_left: bool,
    pub panel_stack: Vec<Rect>,
}

impl UIFrameState {
    pub fn new(input: &Input, resolution: VecTwo) -> Self {
        Self {
            mouse_left: input.mouse.button_left.on_press,
            panel_stack: vec![],
            resolution,
        }
    }

    pub fn get_origin(&self) -> VecTwo {
        let mut origin = VecTwo::new(0.0, 0.0);

        for p in &self.panel_stack {
            origin = origin + p.top_left;
        }

        return origin;
    }
}

/// Custom for drawing framebuffers cuz they need to be flipped
pub fn draw_framebuffer(
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

    context.render_commands.push(RenderCommand::new_rect_uvs(
        &rect,
        -1.0,
        0.0,
        vec![
            VecTwo::new(0.0, 0.0),
            VecTwo::new(1.0, 0.0),
            VecTwo::new(0.0, -1.0),
            //
            VecTwo::new(0.0, -1.0),
            VecTwo::new(1.0, 0.0),
            VecTwo::new(1.0, -1.0),
        ],
        &mat,
    ));
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

pub fn text_bounding_box(
    display: &str,
    pos: VecTwo,
    style: &FontStyle,
    ui_state: &mut UIFrameState,
) -> Rect {
    let origin = ui_state.get_origin();

    let w = style.get_word_width(display);
    let h = style.get_word_height(display) * 0.9;

    let mut r = Rect::new_zero();
    r.top_left = pos;
    r.top_left.y -= h;
    r.bottom_right = pos + VecTwo::new(w, 0.0);

    // grow the box slightly
    r.shrink(-5.0);

    return r;
}

pub fn draw_text(
    display: &str,
    pos: VecTwo,
    color: Color,
    style: &FontStyle,
    ui_state: &mut UIFrameState,
    context: &mut UIContext,
) {
    let origin = ui_state.get_origin();
    render_word(
        display.into(),
        style,
        pos + origin,
        color,
        &mut context.render_commands,
    );
}

pub fn draw_paragraph(
    para: &str,
    mut rect: Rect,
    color: Color,
    style: &FontStyle,
    ui_state: &mut UIFrameState,
    context: &mut UIContext,
) {
    let origin = ui_state.get_origin();
    rect.translate(origin);

    render_paragraph(
        para.into(),
        rect,
        style,
        color,
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

pub fn begin_panel_relative(
    anchors: Anchors,
    color: Color,
    frame_state: &mut UIFrameState,
    context: &mut UIContext,
) {
    let parent_rect: Rect = *frame_state
        .panel_stack
        .last()
        .expect("This method does nothing if not nested");

    let w = parent_rect.width();
    let h = parent_rect.height();

    let mut rel_rect = Rect::new_zero();
    rel_rect.top_left.y = h * anchors.top;
    rel_rect.top_left.x = w * anchors.left;

    rel_rect.bottom_right.y = h - (h * anchors.bottom);
    rel_rect.bottom_right.x = w - (w * anchors.right);

    // draw rect
    draw_rect(rel_rect, color, frame_state, context);

    frame_state.panel_stack.push(rel_rect);
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
        .contains(context.mouse.pos)
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

pub struct GridLayoutInfo {
    pub bounds_width: f64,
    pub col_count: i32,
    pub cell_height: f64,
    pub gutter: f64,
    pub cells_count: i32,
}

/// Returns list of rects in a grid layout
/// This flows vertically for now with set columns count and width.
/// This is gives a layout at origin 0,0
pub fn get_grid_layout(layout_info: GridLayoutInfo) -> Vec<Rect> {
    let cell_width: f64 = layout_info.bounds_width / layout_info.col_count as f64;
    let mut ret: Vec<Rect> = vec![];

    let mut top_left = VecTwo::new(0.0, 0.0);
    for i in 1..(layout_info.cells_count + 1) {
        let r = Rect::new(
            top_left,
            top_left + VecTwo::new(cell_width, layout_info.cell_height),
        );
        ret.push(r);

        top_left.x += cell_width + layout_info.gutter;
        if i as f64 % layout_info.col_count as f64 == 0.0 {
            top_left.y += layout_info.cell_height + layout_info.gutter;
            top_left.x = 0.0;
        }
    }

    return ret;
}
