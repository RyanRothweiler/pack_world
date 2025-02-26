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
