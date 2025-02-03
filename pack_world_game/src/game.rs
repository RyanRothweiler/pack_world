#![allow(
    unused_imports,
    unused_variables,
    clippy::all,
    unused_mut,
    unreachable_code
)]

use crate::state::*;
use gengar_engine::{
    ascii::*,
    color::*,
    debug::*,
    font::*,
    font::*,
    matricies::matrix_four_four::*,
    model::*,
    obj,
    rect::*,
    render::{
        image::Image, load_image, load_image_cursor, material::*, render_command::RenderCommand,
        render_pack::*, shader::*, vao::*, RenderApi,
    },
    state::Input,
    state::State as EngineState,
    transform::*,
    ui::*,
    vectors::*,
};
use gengar_render_opengl::*;
use std::{fs::File, io::Cursor, path::Path};

pub mod drop_table;
pub mod error;
pub mod grid;
pub mod harvest_drop;
pub mod item;
pub mod pack;
pub mod state;
pub mod tiles;
pub mod ui_panels;
pub mod update_signal;
pub mod world;

use grid::*;
use harvest_drop::*;
use item::*;
use state::inventory::*;
use tiles::*;
use ui_panels::{nav_tabs_panel::*, tile_library_panel::*, *};
use update_signal::*;
use world::*;

// Used for windows platform loading dlls
pub const PACKAGE_NAME: &str = "pack_world_game";
// The render_api is hard-coded here instead of using a trait so that we can support hot reloading
#[no_mangle]
pub fn game_init_ogl(gs: &mut State, es: &mut EngineState, render_api: &OglRenderApi) {
    game_init(gs, es, render_api)
}

pub fn game_init(gs: &mut State, es: &mut EngineState, render_api: &impl RenderApi) {
    gengar_engine::debug::init_context(
        es.shader_color.clone(),
        es.shader_color_ui,
        es.model_sphere.clone(),
        es.model_plane.clone(),
    );

    gs.assets.image_dirt =
        load_image_cursor(include_bytes!("../resources/dirt.png"), render_api).unwrap();
    gs.assets.image_grass =
        load_image_cursor(include_bytes!("../resources/grass.png"), render_api).unwrap();
    gs.assets.image_dirt_clod =
        load_image_cursor(include_bytes!("../resources/dirt_clod.png"), render_api).unwrap();
    gs.assets.image_stick =
        load_image_cursor(include_bytes!("../resources/stick.png"), render_api).unwrap();
    gs.assets.image_pack_starter =
        load_image_cursor(include_bytes!("../resources/pack_starter.png"), render_api).unwrap();

    gs.light_trans = Some(es.new_transform());

    // setup font styles
    {
        gs.font_style_button = FontStyle {
            size: 2.0,
            typeface: es.roboto_font.clone(),
        };

        gs.ui_panel_common = Some(UIPanelCommon {
            button_font_style: gs.font_style_button.clone(),
        });
    }

    // setup initial UI
    {
        // gs.active_ui_panels.push(PanelID::Home.create_panel());
        gs.active_page = Some(CreatePanelData::Home.create_panel());
    }

    // setup first map
    {
        let init_dirt: Vec<VecTwoInt> = vec![
            VecTwoInt::new(20, 10),
            VecTwoInt::new(21, 10),
            VecTwoInt::new(20, 11),
            VecTwoInt::new(21, 11),
        ];
        for p in init_dirt {
            gs.world.force_insert_tile(p, TileType::Dirt);
        }
    }

    // setup initial inventory
    {
        gs.inventory
            .add_item(ItemType::Tile(TileType::Dirt), 10)
            .unwrap();
        gs.inventory
            .add_item(ItemType::Tile(TileType::Grass), 10)
            .unwrap();
    }
}

#[no_mangle]
pub fn game_loop(gs: &mut State, es: &mut EngineState, input: &mut Input) {
    gengar_engine::debug::init_context(
        es.shader_color.clone(),
        es.shader_color_ui.clone(),
        es.model_sphere.clone(),
        es.model_plane.clone(),
    );
    gengar_engine::debug::frame_start();
    gengar_engine::ui::frame_start(&input, es.shader_color_ui, es.color_texture_shader);

    // update UI
    {
        let mut ui_frame_state = UIFrameState::new(&input, es.window_resolution);

        let mut update_signals: Vec<UpdateSignal> = vec![];

        // Render and update active UI
        for panel in &mut gs.active_ui_panels {
            update_signals.append(&mut panel.lifecycle.update(
                gs.ui_panel_common.as_ref().unwrap(),
                &mut ui_frame_state,
                &gs.inventory,
                &gs.assets,
            ));
        }

        // update active page
        match &mut gs.active_page {
            Some(page) => update_signals.append(&mut page.lifecycle.update(
                gs.ui_panel_common.as_ref().unwrap(),
                &mut ui_frame_state,
                &gs.inventory,
                &gs.assets,
            )),
            None => {}
        }

        // Handle signals
        handle_signals(update_signals, gs);

        // Update input
        input.mouse_left.on_press = ui_frame_state.mouse_left;
    }

    // update tiles
    {
        let frame_delta: f64 = 0.1;
        let mut update_signals: Vec<UpdateSignal> = vec![];
        for (key, value) in &mut gs.world.tiles {
            update_signals.append(&mut value.methods.update(frame_delta));
        }
        handle_signals(update_signals, gs);
    }

    // render tiles
    {
        gs.rotate_time += 0.08;

        for (pos, tile) in &gs.world.tiles {
            tile.methods.render(
                gs.rotate_time,
                pos,
                es.color_texture_shader,
                es.render_packs.get_mut(&RenderPackID::UI).unwrap(),
                &gs.assets,
            );
        }
    }

    // update harvest drops
    {
        for h in &mut gs.harvest_drops {
            h.update_and_draw(
                0.001,
                es.color_texture_shader,
                es.render_packs.get_mut(&RenderPackID::UI).unwrap(),
                &gs.assets,
            );

            if h.is_finished() {
                gs.inventory.add_item(h.item_type, 1).unwrap();
            }
        }

        // remove fnished
        gs.harvest_drops.retain(|h| !h.is_finished());
    }

    let mouse_grid: VecTwoInt = world_to_grid(&input.mouse_pos);

    // placing tiles
    if let Some(tile) = gs.tile_placing {
        // escape key reseting
        if input.keyboard[0x1B].on_press {
            gs.tile_placing = None;
        }

        let can_place = tile.can_place_here(mouse_grid, &gs.world);

        // render tile placing
        {
            let mut r = Rect::new_square(GRID_SIZE * 0.5);
            r.set_center(grid_to_world(&mouse_grid));

            let mut color = COLOR_WHITE;
            if !can_place {
                color = COLOR_RED;
            }

            let mut mat = Material::new();
            mat.shader = Some(es.color_texture_shader);
            mat.set_image(gs.assets.get_tile_icon(&tile));
            mat.set_color(color);

            es.render_packs
                .get_mut(&RenderPackID::UI)
                .unwrap()
                .commands
                .push(RenderCommand::new_rect(&r, -1.0, 0.0, &mat));
        }

        // place tile
        if input.mouse_left.on_press && can_place {
            if gs.world.try_place_tile(mouse_grid, tile).is_ok() {
                let count = gs.inventory.add_item(ItemType::Tile(tile), -1).unwrap();
                if count == 0 {
                    gs.tile_placing = None;
                }
            }
        }
    }

    // tile hovering
    {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        if gs.tile_placing.is_none() {
            let mouse_snapped: VecTwo = grid_to_world(&mouse_grid);

            if gs.world.tiles.contains_key(&mouse_grid) {
                let tile: &mut TileInstance = gs.world.tiles.get_mut(&mouse_grid).unwrap();

                // Harvesting
                if input.mouse_left.pressing && tile.methods.can_harvest() {
                    update_signals.append(&mut tile.methods.harvest(mouse_snapped));
                }

                // render hover rect
                {
                    let r = Rect::new(
                        mouse_snapped - VecTwo::new(GRID_SIZE * 0.5, GRID_SIZE * 0.5),
                        mouse_snapped + VecTwo::new(GRID_SIZE * 0.5, GRID_SIZE * 0.5),
                    );

                    let mut mat = Material::new();
                    mat.shader = Some(es.shader_color);
                    mat.set_color(Color::new(1.0, 1.0, 1.0, 0.5));

                    es.render_packs
                        .get_mut(&RenderPackID::UI)
                        .unwrap()
                        .commands
                        .push(RenderCommand::new_rect_outline(&r, -1.0, 1.0, &mat));
                }

                // render info
                {
                    let mut ui_frame_state = UIFrameState::new(&input, es.window_resolution);

                    draw_text(
                        &format!("{:?}", tile.tile_type),
                        &gs.font_style_button,
                        VecTwo::new(450.0, 100.0),
                        &mut ui_frame_state,
                    );

                    tile.methods.render_hover_info(
                        es.shader_color.clone(),
                        es.render_packs.get_mut(&RenderPackID::UI).unwrap(),
                    );
                }
            }
        }

        handle_signals(update_signals, gs);
    }

    es.render_packs
        .get_mut(&RenderPackID::UI)
        .unwrap()
        .commands
        .append(&mut gengar_engine::ui::get_render_commands());

    es.game_ui_debug_render_commands = gengar_engine::debug::get_ui_render_list().clone();
    es.game_debug_render_commands = gengar_engine::debug::get_render_list().clone();
}
