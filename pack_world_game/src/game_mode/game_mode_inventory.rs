pub use crate::{
    assets::*, grid::*, inventory::*, item::*, pack::*, pack_shop_display::*, pack_shop_signals::*,
    purchase_flow::*, save_file::*, state::*, tile::*, update_signal::*, world::*,
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

pub fn game_mode_inventory(
    prev_delta_time: f64,
    gs: &mut State,
    es: &mut EngineState,
    mut ui_frame_state: &mut UIFrameState,
    input: &mut Input,
    render_api: &mut impl RenderApi,
    platform_api: &PlatformApi,
) {
    let mut ret: Vec<UpdateSignal> = vec![];

    let ui_context = &mut gs.ui_context.as_mut().unwrap();

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
                cells_count: gs.inventory.items.len() as i32 + title_cells_count,
            });

            let mut y_cursor: f64 = 50.0;
            let mut i: i32 = 0;

            // Draw tiles
            {
                let all_tiles: Vec<(&ItemType, &i64)> = gs
                    .inventory
                    .items
                    .iter()
                    .filter(|(item_type, count)| item_type.is_tile() && **count > 0)
                    .collect();

                // render tiles list
                for (item_type, count) in all_tiles {
                    let mut grid_rect = grid_rects[(i + col_count) as usize];
                    grid_rect.translate(VecTwo::new(20.0, -40.0));

                    ret.append(&mut render_item(
                        i,
                        item_type,
                        count,
                        // &mut item_hovering,
                        &mut y_cursor,
                        ui_frame_state,
                        &gs.inventory,
                        &mut gs.assets,
                        ui_context,
                        grid_rect,
                    ));

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
            for (item_type, count) in gs
                .inventory
                .items
                .iter()
                .filter(|(item_type, count)| !item_type.is_tile() && **count > 0)
            {
                let mut grid_rect = grid_rects[(i + col_count) as usize];
                grid_rect.translate(VecTwo::new(20.0, -40.0));

                ret.append(&mut render_item(
                    i,
                    item_type,
                    count,
                    // &mut item_hovering,
                    &mut y_cursor,
                    ui_frame_state,
                    &gs.inventory,
                    &mut gs.assets,
                    ui_context,
                    grid_rect,
                ));

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
        {}
        end_panel(&mut ui_frame_state, ui_context);
    }
    end_panel(&mut ui_frame_state, ui_context);
}

fn render_item(
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
                    ret.push(UpdateSignal::SetPlacingTile(Some(*tile_type)));
                    // self.item_selected = Some((i, *item_type));
                }
            }
        }

        ItemType::DirtClod
        | ItemType::Stick
        | ItemType::MudBaby
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
                // self.item_selected = Some((i, *item_type));
            }
        }
    };

    // render hover detials
    {
        let button_rect = grid_rect;

        let mut rect_offset = button_rect;
        rect_offset.translate(ui_state.get_origin());
        if rect_offset.contains(ui_context.mouse.pos) {
            // *item_hovering = Some((button_rect, *item_type));
        }
    }

    *y_cursor += 80.0;
    return ret;
}
