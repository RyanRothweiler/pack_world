pub use crate::{
    assets::*, constants::*, grid::*, inventory::*, item::*, pack::*, pack_shop_display::*,
    pack_shop_signals::*, save_file::*, state::*, theme::*, tile::*, update_signal::*, world::*,
};
pub use gengar_engine::{
    collisions::*,
    color::*,
    input::{Input, KeyCode},
    matricies::*,
    platform_api::*,
    rect::*,
    render::{camera::*, material::*, render_command::*, render_pack::*, *},
    state::State as EngineState,
    transform::*,
    ui::*,
    vectors::*,
};

#[derive(Debug)]
pub struct GameModeInventory {
    item_selected: Option<ItemType>,
}

impl GameModeInventory {
    pub fn new() -> Self {
        Self {
            item_selected: None,
        }
    }

    pub fn update(
        &mut self,
        prev_delta_time: f64,
        es: &mut EngineState,
        mut ui_frame_state: &mut UIFrameState,
        input: &mut Input,
        render_api: &mut impl RenderApi,
        platform_api: &PlatformApi,
        inventory: &mut Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        // margin panel holder
        begin_panel(
            Rect::new_center(
                ui_frame_state.resolution * 0.5,
                VecTwo::new(
                    ui_frame_state.resolution.x * 0.95,
                    ui_frame_state.resolution.y * 0.85,
                ),
            ),
            COLOR_INV,
            ui_frame_state,
            ui_context,
        );
        {
            // items box
            begin_panel_relative(
                Anchors::new(0.0, 0.36, 0.0, 0.0),
                *THEME_PANEL_BG,
                ui_frame_state,
                ui_context,
            );
            {
                draw_text(
                    "Tiles",
                    VecTwo::new(20.0, 50.0),
                    *THEME_TEXT,
                    &ui_context.font_header.clone(),
                    ui_frame_state,
                    ui_context,
                );

                let col_count = 10;
                // generate a buncch more than we need
                let title_cells_count = 100;
                let grid_rects = get_grid_layout(GridLayoutInfo {
                    bounds_width: col_count as f64 * 100.0,
                    col_count: col_count,
                    cell_height: 100.0,
                    gutter: 15.0,
                    cells_count: inventory.items.len() as i32 + title_cells_count,
                });

                let mut y_cursor: f64 = 50.0;
                let mut i: i32 = 0;

                // Draw tiles
                {
                    let all_tiles: Vec<(&ItemType, &i64)> = inventory
                        .items
                        .iter()
                        .filter(|(item_type, count)| item_type.is_tile() && **count > 0)
                        .collect();

                    // render tiles list
                    for (item_type, count) in all_tiles {
                        let mut grid_rect = grid_rects[(i + col_count) as usize];
                        grid_rect.translate(VecTwo::new(20.0, -40.0));

                        self.render_item(
                            i,
                            item_type,
                            count,
                            // &mut item_hovering,
                            &mut y_cursor,
                            ui_frame_state,
                            inventory,
                            assets,
                            ui_context,
                            grid_rect,
                        );

                        i += 1;
                    }
                }

                i += col_count - (i % col_count);
                draw_text(
                    "Items",
                    grid_rects[(i + col_count) as usize].top_left + VecTwo::new(20.0, 50.0),
                    *THEME_TEXT,
                    &ui_context.font_header.clone(),
                    ui_frame_state,
                    ui_context,
                );
                i += col_count;

                // render items list
                for (item_type, count) in inventory
                    .items
                    .iter()
                    .filter(|(item_type, count)| !item_type.is_tile() && **count > 0)
                {
                    let mut grid_rect = grid_rects[(i + col_count) as usize];
                    grid_rect.translate(VecTwo::new(20.0, -40.0));

                    self.render_item(
                        i,
                        item_type,
                        count,
                        // &mut item_hovering,
                        &mut y_cursor,
                        ui_frame_state,
                        inventory,
                        assets,
                        ui_context,
                        grid_rect,
                    );

                    i += 1;
                }
            }
            end_panel(&mut ui_frame_state, ui_context);

            // details box
            begin_panel_relative(
                Anchors::new(0.0, 0.0, 0.0, 0.65),
                *THEME_PANEL_BG,
                ui_frame_state,
                ui_context,
            );
            {
                if let Some(item_selected) = self.item_selected {
                    let icon_size = 150.0;
                    let icon_rect =
                        Rect::new_top_size(VecTwo::new(10.0, 10.0), icon_size, icon_size);

                    // draw item icon
                    match item_selected {
                        ItemType::Tile(tile_type) => {
                            if let Some(tile_thumbnail) = assets.get_tile_thumbnail(&tile_type) {
                                draw_framebuffer(
                                    icon_rect,
                                    tile_thumbnail,
                                    COLOR_WHITE,
                                    &mut ui_frame_state,
                                    ui_context,
                                );
                            }
                        }

                        _ => {
                            let icon = assets.get_item_icon(&item_selected);
                            draw_image(
                                icon_rect,
                                icon,
                                COLOR_WHITE,
                                &mut ui_frame_state,
                                ui_context,
                            );
                        }
                    };

                    draw_text(
                        item_selected.user_title(),
                        VecTwo::new(175.0, 100.0),
                        *THEME_TEXT,
                        &ui_context.font_header.clone(),
                        ui_frame_state,
                        ui_context,
                    );

                    let mut disp: &str = item_selected.user_description().unwrap_or(NO_ITEM_DESC);

                    draw_paragraph(
                        disp,
                        Rect::new(VecTwo::new(20.0, 160.0), VecTwo::new(600.0, 500.0)),
                        *THEME_TEXT_MUT,
                        &ui_context.font_body.clone(),
                        ui_frame_state,
                        ui_context,
                    );
                }
            }
            end_panel(&mut ui_frame_state, ui_context);
        }
        end_panel(&mut ui_frame_state, ui_context);

        ret
    }

    fn render_item(
        &mut self,
        i: i32,
        item_type: &ItemType,
        count: &i64,
        // mut item_hovering: &mut Option<(Rect, ItemType)>,
        mut y_cursor: &mut f64,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
        grid_rect: Rect,
    ) {
        let disp = format!("{count}");
        let icon = assets.get_item_icon(item_type);

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
                        self.item_selected = Some(*item_type);
                    }
                }
            }

            ItemType::DirtClod
            | ItemType::Stick
            | ItemType::Root
            | ItemType::FrogLeg
            | ItemType::MudHeart
            | ItemType::Rock
            | ItemType::Berry
            | ItemType::Pearl
            | ItemType::OakLog
            | ItemType::OldBoot
            | ItemType::Seaweed
            | ItemType::Dew
            | ItemType::TrashBag
            | ItemType::OldHat
            | ItemType::DragonEgg
            | ItemType::Baby
            | ItemType::EyeOfNewt
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
                    self.item_selected = Some(*item_type);
                }
            }
        };

        /*
        // render hover detials
        {
            let button_rect = grid_rect;

            let mut rect_offset = button_rect;
            rect_offset.translate(ui_state.get_origin());
            if rect_offset.contains(ui_context.mouse.pos) {
                // *item_hovering = Some((button_rect, *item_type));
            }
        }
        */

        *y_cursor += 80.0;
    }
}
