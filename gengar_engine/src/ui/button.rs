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
    Shrink { amount: f64 },
}

pub struct ButtonStyleData {
    pub style: ButtonStyle,
    pub image: Option<u32>,

    // This doesn't really need to be separate from the image.
    // But I'm being lazy. Its only here because we need a
    // different call on the draw_image with different uvs to flip the framebuffer image.
    pub framebuffer: Option<u32>,
}

impl ButtonStyleData {
    pub fn new_outline(image: Option<u32>) -> Self {
        Self {
            style: ButtonStyle::OutlineColor,
            image,
            framebuffer: None,
        }
    }

    pub fn new_shrink(image: Option<u32>, framebuffer: Option<u32>, amount: f64) -> Self {
        Self {
            style: ButtonStyle::Shrink { amount: amount },
            image,
            framebuffer,
        }
    }
}

pub fn draw_text_button(
    display: &str,
    pos: VecTwo,
    style: &FontStyle,
    underline: bool,
    background: Option<Color>,
    ui_state: &mut UIFrameState,
    line: u32,
    context: &mut UIContext,
) -> bool {
    draw_text_button_id(
        0, display, pos, style, underline, background, ui_state, line, context,
    )
}

pub fn draw_text_button_id(
    id: i32,
    display: &str,
    pos: VecTwo,
    style: &FontStyle,
    underline: bool,
    background: Option<Color>,
    ui_state: &mut UIFrameState,
    line: u32,
    context: &mut UIContext,
) -> bool {
    let origin = ui_state.get_origin();
    let text_bounding_outline = 5.0;

    let mut bounding = text_bounding_box(display, pos, style, ui_state);
    bounding.translate(origin);

    let mut hitbox = bounding.clone();
    hitbox.shrink(-text_bounding_outline);

    let button_on_down;

    // handle state
    let id = format!("{}{}{}", display, line, id);
    let button_state = context
        .button_state
        .entry(id.clone())
        .or_insert(ButtonData::new());
    button_state.update(hitbox, &context.mouse, context.delta_time);

    button_on_down = button_state.on_down;

    let contains = hitbox.contains(context.mouse.pos);
    if contains {
        ui_state.mouse_left = false;
    }

    if let Some(bg_color) = background {
        let mut r = bounding;
        r.shrink(-5.0);

        let mut mat = Material::new();
        mat.shader = Some(context.color_shader);
        mat.set_color(bg_color);
        context
            .render_commands
            .push(RenderCommand::new_rect(&r, -1.0, 0.0, &mat));
    }

    // draw hover
    {
        let mut r = bounding;
        r.shrink(-text_bounding_outline);
        r.top_left.y -= 0.0;

        let mut y_target = r.top_left.y;
        let mut shrink_target = 0.0;

        if button_state.state == ButtonState::Hovering || button_state.state == ButtonState::Down {
            y_target = r.bottom_right.y;
        }

        if button_state.state == ButtonState::Down {
            shrink_target = 5.0;
        }

        // hacky way to set the initial state.
        if button_state.y_current == 0.0 {
            button_state.y_current = y_target;
        }

        button_state.y_current = lerp(button_state.y_current, y_target, context.delta_time * 35.0);
        r.bottom_right.y = button_state.y_current;

        button_state.shrink_current = lerp(
            button_state.shrink_current,
            shrink_target,
            context.delta_time * 35.0,
        );

        if (button_state.y_current - r.top_left.y).abs() > 0.01 {
            r.shrink(button_state.shrink_current);

            let mut mat = Material::new();
            mat.shader = Some(context.color_shader);
            mat.set_color(Color::new(0.0, 0.51, 0.75, 0.5));
            context
                .render_commands
                .push(RenderCommand::new_rect(&r, -1.0, 0.0, &mat));
        }
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

    draw_text(display, pos, *THEME_TEXT, style, ui_state, context);

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
        ButtonStyle::Shrink { amount } => {
            if button_state.state == ButtonState::Hovering {
                image_shrink_target = -12.0 * amount;
            }
            if button_state.state == ButtonState::Down {
                image_shrink_target = 5.0 * amount;
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

    // draw framebuffer icon
    if let Some(icon) = style.framebuffer {
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

        context.render_commands.push(RenderCommand::new_rect_uvs(
            &icon_rect,
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

    render_word(
        display.into(),
        &context.font_body,
        rect.bottom_left() + VecTwo::new(7.0, -7.0),
        COLOR_WHITE,
        &mut context.render_commands,
    );

    return contains && button_state.on_down;
}
