use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct TileLibraryPanel {
    pub item_selected: Option<(i32, ItemType)>,
}

impl TileLibraryPanel {
    pub fn new() -> Self {
        Self {
            item_selected: None,
        }
    }
}

impl TileLibraryPanel {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        // begin panel
        let base_rect = Rect::new_top_size(VecTwo::new(0.0, 150.0), 400.0, ui_state.resolution.y);
        begin_panel(base_rect, BG_COLOR, &mut ui_state, ui_context);

        let mut item_hovering: Option<(i32, ItemType)> = None;

        let y_offset: f64 = 80.0;
        let mut i: i32 = 0;
        for (item_type, count) in &inventory.items {
            if *count == 0 {
                continue;
            }

            let disp = format!("{count}");
            let icon = assets.get_item_icon(item_type);

            let y: f64 = 50.0 + (y_offset * i as f64);
            let button_rect = Rect::new_top_size(VecTwo::new(10.0, y), 50.0, 50.0);

            match item_type {
                ItemType::Tile(tile_type) => {
                    if draw_button_id(
                        i,
                        &disp,
                        Some(icon),
                        &button_rect,
                        ui_state,
                        std::line!(),
                        ui_context,
                    ) {
                        ret.push(UpdateSignal::SetPlacingTile(Some(*tile_type)));
                        self.item_selected = Some((i, *item_type));
                    }
                }
                ItemType::DirtClod | ItemType::Stick | ItemType::Rock | ItemType::OakLog => {
                    draw_image(button_rect, icon, COLOR_WHITE, ui_state, ui_context);
                    draw_text(&disp, VecTwo::new(10.0, y), ui_state, ui_context);
                }
            };

            let y: f64 = 50.0 + (y_offset * i as f64);
            let button_rect = Rect::new_top_size(VecTwo::new(10.0, y), 50.0, 50.0);

            // render hover detials
            let mut rect_offset = button_rect;
            rect_offset.translate(ui_state.get_origin());
            if rect_offset.contains(ui_context.mouse_pos) {
                item_hovering = Some((i, *item_type));
            }

            i += 1;
        }

        let grey = 0.2;
        let grey_color = Color::new(grey, grey, grey, 1.0);

        // draw hover
        if let Some((index, item_type)) = item_hovering {
            let y: f64 = 50.0 + (y_offset * index as f64);
            let button_rect = Rect::new_top_size(VecTwo::new(10.0, y), 50.0, 50.0);

            // render hover detials
            let mut rect_offset = button_rect;

            let mut details_rect = Rect::new_size(100.0, 50.0);
            details_rect.set_center(rect_offset.get_center() + VecTwo::new(110.0, 0.0));

            draw_rect(details_rect, grey_color, ui_state, ui_context);

            draw_text(
                &item_type.user_title(),
                details_rect.bottom_left(),
                ui_state,
                ui_context,
            );
        }

        // details panel
        if let Some((index, item_type)) = self.item_selected {
            begin_panel_relative(
                Anchors::new(0.5, 0.0, 0.0, 0.0),
                grey_color,
                &mut ui_state,
                ui_context,
            );

            {
                draw_text(
                    &item_type.user_title(),
                    VecTwo::new(10.0, 25.0),
                    ui_state,
                    ui_context,
                );

                begin_panel_relative(
                    Anchors::new(0.1, 0.05, 0.0, 0.05),
                    COLOR_INV,
                    &mut ui_state,
                    ui_context,
                );
                {
                    let last_r = *ui_state.panel_stack.last().unwrap();

                    draw_paragraph(
                        &item_type.user_description(),
                        Rect::new(
                            VecTwo::new(0.0, 0.0),
                            VecTwo::new(last_r.width(), last_r.height()),
                        ),
                        ui_state,
                        ui_context,
                    );
                }
                end_panel(&mut ui_state, ui_context);

                // buttons
                let last_r = *ui_state.panel_stack.last().unwrap();

                // sell button
                {
                    let sell_button_y = 300.0;
                    let sell_rect = Rect::new(
                        VecTwo::new(0.0, sell_button_y),
                        VecTwo::new(last_r.width(), sell_button_y + 30.0),
                    );

                    if draw_button("Sell", None, &sell_rect, ui_state, std::line!(), ui_context) {
                        // ret.push(UpdateSignal::SetPlacingTile(Some(*tile_type)));
                    }
                }
            }
            end_panel(&mut ui_state, ui_context);
        }

        end_panel(&mut ui_state, ui_context);

        return ret;
    }
}
