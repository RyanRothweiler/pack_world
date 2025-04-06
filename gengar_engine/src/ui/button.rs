use crate::math::*;
use crate::{
    color::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::{UIContext, UIFrameState, *},
    vectors::*,
};

pub mod button_data;

pub use button_data::*;

pub enum ButtonStyle {
    OutlineColor,
    Shrink,
}

pub struct ButtonStyleData {
    pub style: ButtonStyle,
    pub image: Option<u32>,
}

impl ButtonStyleData {
    pub fn new_outline(image: Option<u32>) -> Self {
        Self {
            style: ButtonStyle::OutlineColor,
            image,
        }
    }

    pub fn new_shrink(image: Option<u32>) -> Self {
        Self {
            style: ButtonStyle::Shrink,
            image,
        }
    }
}

pub fn draw_text_button(
    display: &str,
    pos: VecTwo,
    style: &FontStyle,
    underline: bool,
    ui_state: &mut UIFrameState,
    line: u32,
    context: &mut UIContext,
) -> bool {
    let bounding = text_bounding_box(display, pos, style, ui_state);

    let button_on_down;

    // handle state
    let id = format!("{}{}", display, line);
    let button_state = context
        .button_state
        .entry(id.clone())
        .or_insert(ButtonData::new());
    button_state.update(bounding, &context.mouse, context.delta_time);

    button_on_down = button_state.on_down;

    let contains = bounding.contains(context.mouse.pos);
    if contains {
        ui_state.mouse_left = false;
    }

    // draw hover
    {
        let mut r = bounding;
        r.shrink(-5.0);
        r.top_left.y -= 10.0;

        let mut y_target = r.top_left.y;

        if button_state.state == ButtonState::Hovering || button_state.state == ButtonState::Down {
            y_target = r.bottom_right.y + 5.0;
        }

        button_state.y_current = lerp(button_state.y_current, y_target, context.delta_time * 25.0);
        r.bottom_right.y = button_state.y_current;

        let mut mat = Material::new();
        mat.shader = Some(context.color_shader);
        mat.set_color(Color::new(0.0, 0.51, 0.75, 0.5));
        context
            .render_commands
            .push(RenderCommand::new_rect(&r, -1.0, 0.0, &mat));
    }

    if underline {
        let mut mat = Material::new();
        mat.shader = Some(context.color_shader);
        mat.set_color(Color::new(0.0, 0.51, 0.75, 0.5));
        context.render_commands.push(RenderCommand::new_rect(
            &bounding.underline(2.0),
            -1.0,
            0.0,
            &mat,
        ));
    }

    draw_text(display, pos, COLOR_WHITE, style, ui_state, context);

    return contains && button_on_down;
}

pub fn draw_button(
    display: &str,
    style: ButtonStyleData,
    rect: &Rect,
    ui_state: &mut UIFrameState,
    line: u32,
    context: &mut UIContext,
) -> bool {
    draw_button_id(0, display, style, rect, ui_state, line, context)
}

pub fn draw_button_id(
    id: i32,
    display: &str,
    style: ButtonStyleData,
    rect: &Rect,
    ui_state: &mut UIFrameState,
    line: u32,
    context: &mut UIContext,
) -> bool {
    let origin = ui_state.get_origin();
    let mut rect = *rect;
    rect.translate(origin);

    // handle state
    let id = format!("{}{}{}", display, line, id);
    let button_state = context
        .button_state
        .entry(id.clone())
        .or_insert(ButtonData::new());
    button_state.update(rect, &context.mouse, context.delta_time);

    let contains = rect.contains(context.mouse.pos);
    if contains {
        ui_state.mouse_left = false;
    }

    let mut image_shrink_target: f64 = 2.0;

    match style.style {
        ButtonStyle::OutlineColor => {
            button_state.image_shrink = image_shrink_target;

            let mut color = COLOR_BLUE;
            if button_state.state == ButtonState::Hovering {
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
        }
        ButtonStyle::Shrink => {
            if button_state.state == ButtonState::Hovering {
                image_shrink_target = -12.0;
            }
            if button_state.state == ButtonState::Down {
                image_shrink_target = 5.0;
            }
        }
    }

    // draw icon
    if let Some(icon) = style.image {
        let mut icon_rect: Rect = rect.clone();

        button_state.image_shrink = lerp(
            button_state.image_shrink,
            image_shrink_target,
            context.delta_time * 30.0,
        );
        icon_rect.shrink(button_state.image_shrink);

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
        &context.font_body,
        rect.bottom_left() + VecTwo::new(7.0, -7.0),
        COLOR_WHITE,
        &mut context.render_commands,
    );

    return contains && button_state.on_down;
}
