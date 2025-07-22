use crate::{
    constants::*,
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{rect::*, render::material::*, typeface::*, ui::*, vectors::*};

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
        assets: &mut Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        // begin panel
        let base_rect = Rect::new_top_size(VecTwo::new(0.0, 150.0), 400.0, ui_state.resolution.y);
        begin_panel(base_rect, *THEME_PANEL_BG, &mut ui_state, ui_context);
        {
            let mut item_hovering: Option<(Rect, ItemType)> = None;

            let col_count = 3;
            // generate a buncch more than we need
            let title_cells_count = 25;
            let grid_rects = get_grid_layout(GridLayoutInfo {
                bounds_width: 300.0,
                col_count: col_count,
                cell_height: 100.0,
                gutter: 10.0,
                cells_count: inventory.items.len() as i32 + title_cells_count,
            });

            let mut y_cursor: f64 = 50.0;
            let mut i: i32 = 0;

            begin_panel_relative(
                Anchors::new(0.0, 0.0, 0.0, 0.05),
                COLOR_INV,
                &mut ui_state,
                ui_context,
            );
            {
                draw_text(
                    "Tiles",
                    VecTwo::new(10.0, 40.0),
                    *THEME_TEXT,
                    &ui_context.font_body.clone(),
                    ui_state,
                    ui_context,
                );

                let all_tiles = inventory.get_all_tiles();

                // render tiles list
                for (item_type, count) in all_tiles {
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
                        grid_rects[(i + col_count) as usize],
                    ));

                    i += 1;
                }

                i += col_count - (i % col_count);
                draw_text(
                    "Items",
                    grid_rects[(i + col_count) as usize].top_left + VecTwo::new(10.0, 50.0),
                    *THEME_TEXT,
                    &ui_context.font_body.clone(),
                    ui_state,
                    ui_context,
                );
                i += col_count;

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
                        grid_rects[(i + col_count) as usize],
                    ));

                    i += 1;
                }
            }
            end_panel(&mut ui_state, ui_context);

            let grey = 0.2;
            let grey_color = Color::new(grey, grey, grey, 1.0);

            // draw hover
            if let Some((button_rect, item_type)) = item_hovering {
                // let y: f64 = 50.0 + (Y_OFFSET * index as f64);
                // let button_rect = Rect::new_top_size(VecTwo::new(10.0, y_pos), 50.0, 50.0);

                let mut details_rect = Rect::new_size(100.0, 50.0);
                details_rect.set_center(button_rect.get_center() + VecTwo::new(110.0, 0.0));

                draw_rect(details_rect, grey_color, ui_state, ui_context);

                draw_text(
                    &item_type.user_title(),
                    details_rect.bottom_left(),
                    COLOR_WHITE,
                    &ui_context.font_body.clone(),
                    ui_state,
                    ui_context,
                );
            }

            // details panel
            if let Some((index, item_type)) = self.item_selected {
                begin_panel_relative(
                    Anchors::new(0.68, 0.0, 0.0, 0.0),
                    grey_color,
                    &mut ui_state,
                    ui_context,
                );

                {
                    draw_text(
                        &item_type.user_title(),
                        VecTwo::new(10.0, 25.0),
                        *THEME_TEXT,
                        &ui_context.font_body.clone(),
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

                        let mut disp: &str = NO_ITEM_DESC;
                        let mut col = COLOR_WHITE;

                        match item_type.user_description() {
                            Some(desc) => {
                                disp = desc;
                                col = *THEME_TEXT_MUT;
                            }
                            _ => {}
                        }

                        draw_paragraph(
                            disp,
                            Rect::new(
                                VecTwo::new(0.0, 0.0),
                                VecTwo::new(last_r.width(), last_r.height()),
                            ),
                            col,
                            &ui_context.font_body.clone(),
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

                        /*
                        if draw_button("Sell", None, &sell_rect, ui_state, std::line!(), ui_context)
                        {
                            // ret.push(UpdateSignal::SetPlacingTile(Some(*tile_type)));
                        }
                        */
                    }
                }
                end_panel(&mut ui_state, ui_context);
            }
        }
        end_panel(&mut ui_state, ui_context);

        return ret;
    }

    fn render_item(
        &mut self,
        i: i32,
        item_type: &ItemType,
        count: &i64,
        mut item_hovering: &mut Option<(Rect, ItemType)>,
        mut y_cursor: &mut f64,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
        grid_rect: Rect,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let disp = format!("{count}");
        let icon = assets.get_item_icon(item_type);

        // let button_rect = Rect::new_top_size(VecTwo::new(10.0, *y_cursor), 50.0, 50.0);

        match item_type {
            ItemType::Tile(tile_type) => {
                if let Some(tile_thumbnail) = assets.get_tile_thumbnail(tile_type) {
                    if draw_button_id(
                        i,
                        &disp,
                        ButtonStyleData::new_shrink(None, Some(tile_thumbnail), 4.0),
                        &grid_rect,
                        ui_state,
                        std::line!(),
                        ui_context,
                    ) {
                        // ret.push(UpdateSignal::SetPlacingTile(Some(*tile_type)));
                        self.item_selected = Some((i, *item_type));
                    }
                }
            }

            ItemType::DirtClod
            | ItemType::Root
            | ItemType::Stick
            | ItemType::MudHeart
            | ItemType::Rock
            | ItemType::Berry
            | ItemType::FrogLeg
            | ItemType::Pearl
            | ItemType::OakLog
            | ItemType::OldBoot
            | ItemType::Seaweed
            | ItemType::Dew
            | ItemType::TrashBag
            | ItemType::OldHat
            | ItemType::EyeOfNewt
            | ItemType::DragonEgg
            | ItemType::Baby
            | ItemType::Acorn => {
                if draw_button_id(
                    i,
                    &disp,
                    ButtonStyleData::new_shrink(Some(icon), None, 2.0),
                    &grid_rect,
                    ui_state,
                    std::line!(),
                    ui_context,
                ) {
                    self.item_selected = Some((i, *item_type));
                }
            }
        };

        // render hover detials
        {
            let button_rect = grid_rect;

            let mut rect_offset = button_rect;
            rect_offset.translate(ui_state.get_origin());
            if rect_offset.contains(ui_context.mouse.pos) {
                *item_hovering = Some((button_rect, *item_type));
            }
        }

        *y_cursor += Y_OFFSET;
        return ret;
    }
}
