#![allow(
    unused_imports,
    unused_variables,
    clippy::all,
    unused_mut,
    unreachable_code
)]

use crate::state::*;
use gengar_engine::{
    analytics::*,
    ascii::*,
    color::*,
    debug::*,
    input::*,
    matricies::matrix_four_four::*,
    model::*,
    obj,
    platform_api::*,
    rect::*,
    render::{
        image::Image, load_image, load_image_cursor, material::*, render_command::RenderCommand,
        render_pack::*, shader::*, vao::*, RenderApi,
    },
    state::State as EngineState,
    transform::*,
    typeface::*,
    ui::*,
    vectors::*,
};
use gengar_render_opengl::*;
use std::{
    collections::HashMap,
    fs::File,
    io::Cursor,
    path::Path,
    sync::{LazyLock, Mutex},
};

pub mod constants;
pub mod drop_table;
pub mod error;
pub mod grid;
pub mod harvest_drop;
pub mod item;
pub mod pack;
pub mod save_file;
pub mod state;
pub mod tile;
pub mod ui_panels;
pub mod update_signal;
pub mod world;

#[cfg(test)]
pub mod testing_infra;

pub use constants::*;
use grid::*;
use harvest_drop::*;
use item::*;
use save_file::*;
use state::inventory::*;
use tile::*;
use ui_panels::{debug_panel::*, nav_tabs_panel::*, tile_library_panel::*, *};
use update_signal::*;
use world::*;

// Used for windows platform loading dlls
pub const PACKAGE_NAME: &str = "pack_world_game";

/// maximum ms to forwad sim when loading ame
const MAX_SIM_MS: f64 = 500.0;

// The render_api is hard-coded here instead of using a trait so that we can support hot reloading
#[no_mangle]
pub fn game_init_ogl(
    gs: &mut State,
    es: &mut EngineState,
    render_api: &OglRenderApi,
    platform_api: &PlatformApi,
) {
    game_init(gs, es, render_api, platform_api)
}

pub fn game_init(
    gs: &mut State,
    es: &mut EngineState,
    render_api: &impl RenderApi,
    platform_api: &PlatformApi,
) {
    (platform_api.send_event)(AnalyticsEvent::AppStart);

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
    gs.assets.image_gold =
        load_image_cursor(include_bytes!("../resources/gold.png"), render_api).unwrap();
    gs.assets.image_acorn =
        load_image_cursor(include_bytes!("../resources/acorn.png"), render_api).unwrap();
    gs.assets.image_cave =
        load_image_cursor(include_bytes!("../resources/cave.png"), render_api).unwrap();
    gs.assets.image_dragon_egg =
        load_image_cursor(include_bytes!("../resources/dragon_egg.png"), render_api).unwrap();
    gs.assets.image_baby =
        load_image_cursor(include_bytes!("../resources/baby.png"), render_api).unwrap();
    gs.assets.image_shrub =
        load_image_cursor(include_bytes!("../resources/shrub.png"), render_api).unwrap();
    gs.assets.image_berry =
        load_image_cursor(include_bytes!("../resources/berry.png"), render_api).unwrap();
    gs.assets.image_pack_stick =
        load_image_cursor(include_bytes!("../resources/pack_stick.png"), render_api).unwrap();
    gs.assets.image_question_mark =
        load_image_cursor(include_bytes!("../resources/question_mark.png"), render_api).unwrap();
    gs.assets.image_mud_pit =
        load_image_cursor(include_bytes!("../resources/mud_pit.png"), render_api).unwrap();
    gs.assets.image_tall_grass =
        load_image_cursor(include_bytes!("../resources/tall_grass.png"), render_api).unwrap();
    gs.assets.image_mud_baby =
        load_image_cursor(include_bytes!("../resources/mud_baby.png"), render_api).unwrap();

    gs.light_trans = Some(es.new_transform());

    // setup font styles
    {
        gs.font_style_body = FontStyle {
            size: 2.0,
            typeface: es.roboto_typeface.get_weight(TypeWeight::Regular),
        };

        gs.font_style_header = FontStyle {
            size: 4.0,
            typeface: es.roboto_typeface.get_weight(TypeWeight::Bold),
        };
    }

    // setup initial UI
    gs.active_page = Some(CreatePanelData::Home.create_panel());

    // setup first map
    {
        let init_dirt: Vec<GridPos> = vec![
            GridPos::new(20, 10),
            // GridPos::new(21, 10),
            // GridPos::new(20, 11),
            // GridPos::new(21, 11),
        ];
        for p in init_dirt {
            let _ = gs.world.force_insert_tile(p, TileType::Dirt);
        }
    }

    // setup initial inventory
    {
        gs.inventory
            .give_item(ItemType::Tile(TileType::Dirt), 4)
            .unwrap();
        gs.inventory
            .give_item(ItemType::Tile(TileType::Grass), 4)
            .unwrap();
    }

    // make debug panel. Needs to happen here so that the memory is in dll space.
    {
        gs.debug_state.debug_panel = Some(UIPanel::DebugPanel(DebugPanel {}));
    }
}

fn sim_world(gs: &mut State, ms: f64, platform_api: &PlatformApi) {
    let mut update_signals: Vec<UpdateSignal> = vec![];
    for (eid, entity) in &mut gs.world.entities {
        update_signals.append(&mut entity.methods.update(ms));
    }
    handle_signals(update_signals, gs, platform_api);
}

// Prev delta time is in seconds. So for 60 fps 0.016666.
#[no_mangle]
pub fn game_loop(
    prev_delta_time: f64,
    gs: &mut State,
    es: &mut EngineState,
    input: &mut Input,
    platform_api: &PlatformApi,
) {
    gengar_engine::debug::init_context(
        es.shader_color.clone(),
        es.shader_color_ui.clone(),
        es.model_sphere.clone(),
        es.model_plane.clone(),
    );
    gengar_engine::debug::frame_start();

    // update ui_context
    {
        let ui_context = gs.ui_context.get_or_insert(UIContext {
            mouse: input.mouse.clone(),

            color_shader: es.shader_color_ui,
            color_shader_texture: es.color_texture_shader,

            font_body: gs.font_style_body.clone(),
            font_header: gs.font_style_header.clone(),

            render_commands: vec![],
            button_state: HashMap::new(),

            delta_time: prev_delta_time,
        });

        ui_context.mouse = input.mouse.clone();
        ui_context.delta_time = prev_delta_time;
    }

    let mut ui_frame_state = UIFrameState::new(&input, es.window_resolution);

    #[cfg(feature = "dev")]
    {
        let fps = 1.0 / prev_delta_time;
        draw_text(
            &format!(
                "{:?}fps {:?}ms",
                fps as i32,
                (prev_delta_time * 1000.0) as i32
            ),
            VecTwo::new(es.window_resolution.x - 200.0, 60.0),
            COLOR_WHITE,
            &gs.font_style_body,
            &mut ui_frame_state,
            &mut gs.ui_context.as_mut().unwrap(),
        );
    }

    // save game
    {
        // manual save for testing
        #[cfg(feature = "dev")]
        {
            if input.get_key(KeyCode::S).on_press {
                save_game(&gs.world, &gs.inventory, platform_api).expect("Error saving game.");
            }

            if input.get_key(KeyCode::L).on_press {
                (platform_api.fetch_game_save)();
            }
        }

        // check for data to load
        {
            if !es.game_to_load.is_empty() {
                let mut ms_to_sim = load_game(
                    &mut gs.world,
                    &mut gs.inventory,
                    &es.game_to_load,
                    platform_api,
                );
                es.game_to_load.clear();

                while ms_to_sim > 0.0 {
                    println!("Forward Simulating {}ms remaining", ms_to_sim);
                    let ms_step = ms_to_sim.clamp(0.0, MAX_SIM_MS);
                    sim_world(gs, ms_step / 1000.0, platform_api);
                    ms_to_sim -= ms_step;
                }
            }
        }
    }

    // update UI
    {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        // Render and update active UI
        for panel in &mut gs.active_ui_panels {
            update_signals.append(&mut panel.update(
                &mut ui_frame_state,
                &gs.inventory,
                &gs.assets,
                &mut gs.ui_context.as_mut().unwrap(),
                platform_api,
            ));
        }

        // update active page
        match &mut gs.active_page {
            Some(page) => update_signals.append(&mut page.update(
                &mut ui_frame_state,
                &gs.inventory,
                &gs.assets,
                &mut gs.ui_context.as_mut().unwrap(),
                platform_api,
            )),
            None => {}
        }

        // Handle signals
        handle_signals(update_signals, gs, platform_api);

        // Update input
        input.mouse.button_left.on_press = ui_frame_state.mouse_left;
    }

    // debug panel
    #[cfg(feature = "dev")]
    {
        if input.get_key(KeyCode::Tab).on_press {
            gs.debug_state.showing_debug_panel = !gs.debug_state.showing_debug_panel;
        }

        if gs.debug_state.showing_debug_panel {
            if let Some(panel) = &mut gs.debug_state.debug_panel {
                let sigs = panel.update(
                    &mut ui_frame_state,
                    &gs.inventory,
                    &gs.assets,
                    &mut gs.ui_context.as_mut().unwrap(),
                    platform_api,
                );

                handle_signals(sigs, gs, platform_api);
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

        sim_world(gs, frame_delta, platform_api);
    }

    // camera controls
    {
        let keyboard_speed = 1000.0;
        let drag_speed = 0.75;

        let cam_pack = es.render_packs.get_mut(&RenderPackID::World).unwrap();

        if input.get_key(KeyCode::W).pressing {
            cam_pack.camera.transform.local_position.y -= keyboard_speed * prev_delta_time;
        }
        if input.get_key(KeyCode::S).pressing {
            cam_pack.camera.transform.local_position.y += keyboard_speed * prev_delta_time;
        }
        if input.get_key(KeyCode::A).pressing {
            cam_pack.camera.transform.local_position.x -= keyboard_speed * prev_delta_time;
        }
        if input.get_key(KeyCode::D).pressing {
            cam_pack.camera.transform.local_position.x += keyboard_speed * prev_delta_time;
        }

        // camera click dragging
        /*
        {
            if input.mouse.button_left.pressing {
                if input.mouse.pos_delta.dist_from(VecTwo::new(0.0, 0.0)) > 1.0 {
                    cam_pack.camera.transform.local_position.x +=
                        input.mouse.pos_delta.x * drag_speed;
                    cam_pack.camera.transform.local_position.y +=
                        input.mouse.pos_delta.y * drag_speed;
                }
            }
        }
        */
    }

    // run tile updates
    {
        let mut update_sigs: Vec<UpdateSignal> = vec![];
        for (eid, tile_inst) in &mut gs.world.entities {
            update_sigs.append(&mut tile_inst.update(prev_delta_time));
        }
        handle_signals(update_sigs, gs, platform_api);
    }

    // render tiles. Render each layer separately.
    // Kinda fucked but whatver. Maybe could setup a new data structure to handle this.
    {
        // TODO chagne this to use delta_time
        gs.rotate_time += 0.08;

        for (grid_pos, world_cell) in &gs.world.entity_map {
            for (layer, eid) in &world_cell.layers {
                let entity = &gs.world.get_entity(&eid);
                match layer {
                    WorldLayer::Ground => {
                        entity.methods.render(
                            gs.rotate_time,
                            &entity.grid_pos,
                            es.color_texture_shader,
                            es.render_packs.get_mut(&RenderPackID::World).unwrap(),
                            &gs.assets,
                        );
                    }
                    _ => {}
                }
            }
        }

        for (grid_pos, world_cell) in &gs.world.entity_map {
            for (layer, eid) in &world_cell.layers {
                let entity = &gs.world.get_entity(&eid);
                match layer {
                    WorldLayer::Floor => {
                        entity.methods.render(
                            gs.rotate_time,
                            &entity.grid_pos,
                            es.color_texture_shader,
                            es.render_packs.get_mut(&RenderPackID::World).unwrap(),
                            &gs.assets,
                        );
                    }
                    _ => {}
                }
            }
        }

        for (grid_pos, world_cell) in &gs.world.entity_map {
            for (layer, eid) in &world_cell.layers {
                let entity = &gs.world.get_entity(&eid);
                match layer {
                    WorldLayer::TreeAttachment => {
                        entity.methods.render(
                            gs.rotate_time,
                            &entity.grid_pos,
                            es.color_texture_shader,
                            es.render_packs.get_mut(&RenderPackID::World).unwrap(),
                            &gs.assets,
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    // update harvest drops
    {
        let mut sigs: Vec<UpdateSignal> = vec![];

        for h in &mut gs.harvest_drops {
            h.update_and_draw(
                0.001,
                es.color_texture_shader,
                es.render_packs.get_mut(&RenderPackID::World).unwrap(),
                &gs.assets,
            );

            if h.is_finished() {
                sigs.push(UpdateSignal::GiveDrop(h.drop));
            }
        }

        // remove fnished
        gs.harvest_drops.retain(|h| !h.is_finished());

        handle_signals(sigs, gs, platform_api);
    }

    let mouse_grid: GridPos = {
        let cam_pack = es.render_packs.get_mut(&RenderPackID::World).unwrap();
        let mouse_world = cam_pack.camera.screen_to_world(input.mouse.pos);
        let mouse_grid: GridPos = world_to_grid(&mouse_world);

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
        if input.mouse.button_left.on_press && can_place {
            if let Ok(update_sigs) = gs.world.try_place_tile(mouse_grid, tile) {
                let count = gs.inventory.give_item(ItemType::Tile(tile), -1).unwrap();
                if count == 0 {
                    gs.tile_placing = None;
                }

                handle_signals(update_sigs, gs, platform_api);
            }
        }
    }

    // tile hovering
    {
        let world_snapshot = gs.world.get_world_snapshot();

        if gs.tile_placing.is_none() {
            let mouse_snapped: VecTwo = grid_to_world(&mouse_grid);
            let mouse_snapped_screen = es
                .render_packs
                .get_mut(&RenderPackID::World)
                .unwrap()
                .camera
                .world_to_screen(mouse_snapped);

            let world_cell: WorldCell = gs.world.get_entities(mouse_grid);

            for (i, (layer, eid)) in world_cell.layers.iter().enumerate() {
                let tile = gs.world.get_entity_mut(eid);

                // Harvesting
                if input.mouse.button_left.pressing && tile.methods.can_harvest() {
                    tile.harvest(&world_snapshot, platform_api);
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

                    let y = layer.to_index() as f64 * 40.0;

                    draw_text(
                        &format!("{:?}", tile.tile_type),
                        VecTwo::new(450.0, 100.0 + y),
                        COLOR_WHITE,
                        &gs.font_style_body,
                        &mut ui_frame_state,
                        &mut gs.ui_context.as_mut().unwrap(),
                    );

                    tile.methods.render_hover_info(
                        y,
                        es.shader_color.clone(),
                        es.render_packs.get_mut(&RenderPackID::UI).unwrap(),
                    );
                }
            }
        }
    }

    es.render_packs
        .get_mut(&RenderPackID::UI)
        .unwrap()
        .commands
        .append(&mut gs.ui_context.as_mut().unwrap().render_commands);

    es.game_ui_debug_render_commands = gengar_engine::debug::get_ui_render_list().clone();
    es.game_debug_render_commands = gengar_engine::debug::get_render_list().clone();
}
