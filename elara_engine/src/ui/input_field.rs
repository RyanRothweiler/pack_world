use crate::{rect::*, ui::*};

pub mod input_field_state;

pub use input_field_state::*;

pub struct InputField {}

impl InputField {
    pub fn draw(
        title: &str,
        placeholder: &str,
        content: &mut String,
        top_left: VecTwo,
        width: f64,
        header_style: &FontStyle,
        body_style: &FontStyle,
        ui_state: &mut UIFrameState,
        context: &mut UIContext,
        line: u32,
    ) {
        let origin = ui_state.get_origin();
        // top_left = top_left + origin;

        let mut hitbox = Rect::new_top_size(top_left, width, 38.0);
        hitbox.translate(origin);

        let id = format!("{}", line);
        let state = context
            .input_fields
            .entry(id.clone())
            .or_insert(InputFieldData::new());

        state.update(hitbox, &context.mouse);
        state.blink_accum += 1.0;

        let cursor_show = ((state.blink_accum as i32 / 30) % 2 == 0) && state.selected;
        let selected = state.selected;

        let contains = hitbox.contains(context.mouse.pos);
        if contains && ui_state.mouse_left {
            context.selected_input_field = Some(id);
            ui_state.mouse_left = false;
        }

        let mut bounding = hitbox.clone();
        bounding.shrink(2.0);

        let background_color: Color = {
            let col = COLOR_BLUE_BG;
            if state.selected {
                // col.a = 0.75;
            }
            col
        };

        // outline
        {
            let mut outline = bounding.clone();
            outline.shrink(-2.0);

            if contains {
                outline.shrink(-2.0);
            }

            let mut mat = Material::new();
            mat.shader = Some(context.color_shader);
            mat.set_color(COLOR_BLUE_FG);
            context
                .render_commands
                .push(RenderCommand::new_rect(&outline, -1.0, 0.0, &mat));
        }

        // draw background
        {
            let mut mat = Material::new();
            mat.shader = Some(context.color_shader);
            mat.set_color(background_color);
            context
                .render_commands
                .push(RenderCommand::new_rect(&bounding, -1.0, 0.0, &mat));
        }

        // selected underline
        if state.selected {
            let mut mat = Material::new();
            mat.shader = Some(context.color_shader);
            mat.set_color(COLOR_BLUE_FG);
            context.render_commands.push(RenderCommand::new_rect(
                &bounding.underline(4.0),
                -1.0,
                0.0,
                &mat,
            ));
        }

        let content_pos = top_left + VecTwo::new(10.0, 25.0);
        // text
        {
            // title
            draw_text(
                title,
                top_left + VecTwo::new(0.0, -8.0),
                COLOR_WHITE,
                header_style,
                ui_state,
                context,
            );

            // content
            if !content.is_empty() {
                draw_text(
                    content,
                    content_pos,
                    COLOR_WHITE,
                    body_style,
                    ui_state,
                    context,
                );
            } else {
                draw_text(
                    placeholder,
                    content_pos,
                    Color::new(1.0, 1.0, 1.0, 0.4),
                    body_style,
                    ui_state,
                    context,
                );
            }
        }

        // cursor
        {
            if cursor_show {
                let mut text_bounding =
                    text_bounding_box(content, content_pos, body_style, ui_state);
                text_bounding.translate(origin);
                text_bounding.top_left.x = text_bounding.bottom_right.x - 2.0;

                let shift = -2.0;
                text_bounding.top_left.x += shift;
                text_bounding.bottom_right.x += shift;

                let mut mat = Material::new();
                mat.shader = Some(context.color_shader);
                mat.set_color(Color::new(1.0, 1.0, 1.0, 0.7));
                context.render_commands.push(RenderCommand::new_rect(
                    &text_bounding,
                    -1.0,
                    0.0,
                    &mat,
                ));
            }
        }

        // state change
        if selected {
            if let Some(key) = context.keyboard.key_pressed() {
                if key == KeyCode::Backspace && !content.is_empty() {
                    content.remove(content.len() - 1);
                }
            }

            if let Some(ch) = context.keyboard.char_down {
                content.push(ch);
            }
        }

        // handle paste
        // note this must happen last because it overwrites the V key used for pasting.
        {
            if let Some(paste) = &context.paste {
                *content = paste.to_string();
                context.paste = None;
            }
        }
    }
}
