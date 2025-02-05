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
    input::*,
    matricies::matrix_four_four::*,
    model::*,
    obj,
    rect::*,
    render::{
        image::Image, load_image, load_image_cursor, material::*, render_command::RenderCommand,
        render_pack::*, shader::*, vao::*, RenderApi,
    },
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
pub mod tile;
pub mod ui_panels;
pub mod update_signal;
pub mod world;

use grid::*;
use harvest_drop::*;
use item::*;
use state::inventory::*;
use tile::*;
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
    gs.assets.image_boulder =
        load_image_cursor(include_bytes!("../resources/boulder.png"), render_api).unwrap();
    gs.assets.image_rock =
        load_image_cursor(include_bytes!("../resources/rock.png"), render_api).unwrap();
    gs.assets.image_oak_tree =
        load_image_cursor(include_bytes!("../resources/oak_tree.png"), render_api).unwrap();
    gs.assets.image_oak_wood =
        load_image_cursor(include_bytes!("../resources/oak_wood.png"), render_api).unwrap();
    gs.assets.image_bird_nest =
        load_image_cursor(include_bytes!("../resources/bird_nest.png"), render_api).unwrap();

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

    // make debug panel. Needs to happen here so that the memory is in dll space.
    {
        gs.debug_state.debug_panel = Some(UIPanel {
            panel_id: PanelID::DebugPanel,
            lifecycle: Box::new(ui_panels::debug_panel::DebugPanel {}),
        });
    }
}

// Prev delta time is in seconds. So for 60 fps 0.016666.
#[no_mangle]
pub fn game_loop(prev_delta_time: f64, gs: &mut State, es: &mut EngineState, input: &mut Input) {
    gengar_engine::debug::init_context(
        es.shader_color.clone(),
        es.shader_color_ui.clone(),
        es.model_sphere.clone(),
        es.model_plane.clone(),
    );
    gengar_engine::debug::frame_start();
    gengar_engine::ui::frame_start(&input, es.shader_color_ui, es.color_texture_shader);

    let mut ui_frame_state = UIFrameState::new(&input, es.window_resolution);

    // update UI
    {
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

    // debug panel
    #[cfg(feature = "dev")]
    {
        if input.get_key(KeyCode::Tab).on_press {
            gs.debug_state.showing_debug_panel = !gs.debug_state.showing_debug_panel;
        }

        if gs.debug_state.showing_debug_panel {
            if let Some(panel) = &mut gs.debug_state.debug_panel {
                let sigs = panel.lifecycle.update(
                    gs.ui_panel_common.as_ref().unwrap(),
                    &mut ui_frame_state,
                    &gs.inventory,
                    &gs.assets,
                );

                handle_signals(sigs, gs);
            }
        }
    }

    // update tiles
    {
        let mut frame_delta: f64 = prev_delta_time;

        #[cfg(feature = "dev")]
        if input.get_key(KeyCode::One).on_press {
            frame_delta = 100.0;
        }

        let mut update_signals: Vec<UpdateSignal> = vec![];
        /*
        for (key, value) in &mut gs.world.entity_map {
            update_signals.append(&mut value.methods.update(frame_delta));
        }
        */
        for entity in &mut gs.world.entities {
            update_signals.append(&mut entity.methods.update(frame_delta));
        }
        handle_signals(update_signals, gs);
    }

    // camera controls
    {
        let cam_pack = es.render_packs.get_mut(&RenderPackID::World).unwrap();

        let cam_speed = 1000.0;
        if input.get_key(KeyCode::W).pressing {
            cam_pack.camera.transform.local_position.y -= cam_speed * prev_delta_time;
        }
        if input.get_key(KeyCode::S).pressing {
            cam_pack.camera.transform.local_position.y += cam_speed * prev_delta_time;
        }
        if input.get_key(KeyCode::A).pressing {
            cam_pack.camera.transform.local_position.x -= cam_speed * prev_delta_time;
        }
        if input.get_key(KeyCode::D).pressing {
            cam_pack.camera.transform.local_position.x += cam_speed * prev_delta_time;
        }
    }

    // render tiles
    {
        // TODO chagne this to use delta_time
        gs.rotate_time += 0.08;

        for entity in &gs.world.entities {
            entity.methods.render(
                gs.rotate_time,
                &entity.grid_pos,
                es.color_texture_shader,
                es.render_packs.get_mut(&RenderPackID::World).unwrap(),
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
                es.render_packs.get_mut(&RenderPackID::World).unwrap(),
                &gs.assets,
            );

            if h.is_finished() {
                gs.inventory.add_item(h.item_type, 1).unwrap();
            }
        }

        // remove fnished
        gs.harvest_drops.retain(|h| !h.is_finished());
    }

    let mouse_grid: VecTwoInt = {
        let cam_pack = es.render_packs.get_mut(&RenderPackID::World).unwrap();
        let mouse_world = cam_pack.camera.screen_to_world(input.mouse_pos);
        let mouse_grid: VecTwoInt = world_to_grid(&mouse_world);

        mouse_grid
    };

    // placing tiles
    if let Some(tile) = gs.tile_placing {
        // escape key reseting
        if input.get_key(KeyCode::Escape).on_press {
            gs.tile_placing = None;
        }

        let can_place = tile.can_place_here(mouse_grid, &gs.world);

        // render tile placing
        let footprint = tile.get_tile_footprint();

        for p in footprint {
            let pos = mouse_grid + p;

            let mut r = Rect::new_square(GRID_SIZE * 0.5);
            r.set_center(grid_to_world(&pos));

            let mut color = COLOR_WHITE;
            if !can_place {
                color = COLOR_RED;
            }

            let mut mat = Material::new();
            mat.shader = Some(es.color_texture_shader);
            mat.set_image(gs.assets.get_tile_icon(&tile));
            mat.set_color(color);

            es.render_packs
                .get_mut(&RenderPackID::World)
                .unwrap()
                .commands
                .push(RenderCommand::new_rect(&r, -1.0, 0.0, &mat));
        }

        // place tile
        if input.mouse_left.on_press && can_place {
            if gs.world.try_place_tile(mouse_grid, tile).is_ok() {
                println!("placing {:?}", tile);
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
            let mouse_snapped_screen = es
                .render_packs
                .get_mut(&RenderPackID::World)
                .unwrap()
                .camera
                .world_to_screen(mouse_snapped);

            let entities: Vec<usize> = gs.world.get_entities(mouse_grid).unwrap_or(vec![]);

            // if let Some(tile) = gs.world.get_entity_mut(mouse_grid) {
            for idx in entities {
                let tile = &mut gs.world.entities[idx];

                // Harvesting
                if input.mouse_left.pressing && tile.methods.can_harvest() {
                    update_signals.append(&mut tile.methods.harvest(mouse_snapped));
                }

                // render hover rect
                {
                    let r = Rect::new(
                        mouse_snapped_screen - VecTwo::new(GRID_SIZE * 0.5, GRID_SIZE * 0.5),
                        mouse_snapped_screen + VecTwo::new(GRID_SIZE * 0.5, GRID_SIZE * 0.5),
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
