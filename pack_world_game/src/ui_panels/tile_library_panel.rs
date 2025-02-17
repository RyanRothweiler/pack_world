use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

const Y_OFFSET: f64 = 80.0;

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

        let mut item_hovering: Option<(f64, ItemType)> = None;

        draw_text("Tiles", VecTwo::new(10.0, 30.0), ui_state, ui_context);

        let mut y_cursor: f64 = 50.0;

        let mut i: i32 = 0;

        // render tiles list
        for (item_type, count) in inventory
            .items
            .iter()
            .filter(|(item_type, count)| item_type.is_tile() && **count > 0)
        {
            ret.append(&mut self.render_item(
                i,
                item_type,
                count,
                &mut item_hovering,
                &mut y_cursor,
                ui_state,
                inventory,
                assets,
                ui_context,
            ));

            i += 1;
        }

        y_cursor += 20.0;
        draw_text("Items", VecTwo::new(10.0, y_cursor), ui_state, ui_context);
        y_cursor += 50.0;

        // render items list
        for (item_type, count) in inventory
            .items
            .iter()
            .filter(|(item_type, count)| !item_type.is_tile() && **count > 0)
        {
            ret.append(&mut self.render_item(
                i,
                item_type,
                count,
                &mut item_hovering,
                &mut y_cursor,
                ui_state,
                inventory,
                assets,
                ui_context,
            ));

            i += 1;
        }

        let grey = 0.2;
        let grey_color = Color::new(grey, grey, grey, 1.0);

        // draw hover
        if let Some((y_pos, item_type)) = item_hovering {
            // let y: f64 = 50.0 + (Y_OFFSET * index as f64);
            let button_rect = Rect::new_top_size(VecTwo::new(10.0, y_pos), 50.0, 50.0);

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

    fn render_item(
        &mut self,
        i: i32,
        item_type: &ItemType,
        count: &i64,
        mut item_hovering: &mut Option<(f64, ItemType)>,
        mut y_cursor: &mut f64,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let disp = format!("{count}");
        let icon = assets.get_item_icon(item_type);

        let button_rect = Rect::new_top_size(VecTwo::new(10.0, *y_cursor), 50.0, 50.0);

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
            ItemType::DirtClod
            | ItemType::Stick
            | ItemType::Rock
            | ItemType::OakLog
            | ItemType::DragonEgg
            | ItemType::Baby
            | ItemType::Acorn => {
                if draw_button_id(
                    i,
                    &disp,
                    Some(icon),
                    &button_rect,
                    ui_state,
                    std::line!(),
                    ui_context,
                ) {
                    self.item_selected = Some((i, *item_type));
                }
            }
        };

        let button_rect = Rect::new_top_size(VecTwo::new(10.0, *y_cursor), 50.0, 50.0);

        // render hover detials
        let mut rect_offset = button_rect;
        rect_offset.translate(ui_state.get_origin());
        if rect_offset.contains(ui_context.mouse_pos) {
            *item_hovering = Some((*y_cursor, *item_type));
        }

        *y_cursor += Y_OFFSET;
        return ret;
    }
}
