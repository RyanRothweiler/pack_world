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
    collisions::*,
    color::*,
    debug::*,
    input::*,
    matricies::matrix_four_four::*,
    model::*,
    obj,
    platform_api::*,
    rect::*,
    render::{
        camera::*, image::Image, load_image, load_image_cursor, material::*,
        render_command::RenderCommand, render_pack::*, shader::*, vao::*, RenderApi,
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

use assets::*;
pub use constants::*;
use grid::*;
use harvest_drop::*;
use harvest_timer::*;
use item::*;
use pack::*;
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

const BUTTON_BG: Color = Color {
    r: 0.0,
    g: 0.51,
    b: 0.75,
    a: 0.2,
};

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

    // gs.assets.binary_file_system = load_game_assets();
    load_game_assets(&mut gs.assets.asset_library, render_api);
    gs.assets.build_assets(es.pbr_shader, es.shader_color);

    gs.assets.image_dirt_clod =
        load_image_cursor(include_bytes!("../resources/dirt_clod.png"), render_api).unwrap();
    gs.assets.image_stick =
        load_image_cursor(include_bytes!("../resources/stick.png"), render_api).unwrap();
    gs.assets.image_pack_starter =
        load_image_cursor(include_bytes!("../resources/pack_starter.png"), render_api).unwrap();
    gs.assets.image_rock =
        load_image_cursor(include_bytes!("../resources/rock.png"), render_api).unwrap();
    gs.assets.image_oak_wood =
        load_image_cursor(include_bytes!("../resources/oak_wood.png"), render_api).unwrap();
    gs.assets.image_gold =
        load_image_cursor(include_bytes!("../resources/gold.png"), render_api).unwrap();
    gs.assets.image_acorn =
        load_image_cursor(include_bytes!("../resources/acorn.png"), render_api).unwrap();
    gs.assets.image_dragon_egg =
        load_image_cursor(include_bytes!("../resources/dragon_egg.png"), render_api).unwrap();
    gs.assets.image_baby =
        load_image_cursor(include_bytes!("../resources/baby.png"), render_api).unwrap();
    gs.assets.image_berry =
        load_image_cursor(include_bytes!("../resources/berry.png"), render_api).unwrap();
    gs.assets.image_pack_stick =
        load_image_cursor(include_bytes!("../resources/pack_stick.png"), render_api).unwrap();
    gs.assets.image_question_mark =
        load_image_cursor(include_bytes!("../resources/question_mark.png"), render_api).unwrap();
    gs.assets.image_mud_baby =
        load_image_cursor(include_bytes!("../resources/mud_baby.png"), render_api).unwrap();
    gs.assets.image_pack_mud =
        load_image_cursor(include_bytes!("../resources/pack_mud.png"), render_api).unwrap();
    gs.assets.image_pearl =
        load_image_cursor(include_bytes!("../resources/pearl.png"), render_api).unwrap();
    gs.assets.image_old_boot =
        load_image_cursor(include_bytes!("../resources/old_boot.png"), render_api).unwrap();
    gs.assets.image_seaweed =
        load_image_cursor(include_bytes!("../resources/seaweed.png"), render_api).unwrap();
    gs.assets.image_trash_bag =
        load_image_cursor(include_bytes!("../resources/trash_bag.png"), render_api).unwrap();
    gs.assets.image_old_hat =
        load_image_cursor(include_bytes!("../resources/old_hat.png"), render_api).unwrap();
    gs.assets.image_pack_water =
        load_image_cursor(include_bytes!("../resources/pack_water.png"), render_api).unwrap();
    gs.assets.image_dew =
        load_image_cursor(include_bytes!("../resources/dew.png"), render_api).unwrap();
    gs.assets.image_glow =
        load_image_cursor(include_bytes!("../resources/glow.png"), render_api).unwrap();
    gs.assets.image_twitter = load_image_cursor(
        include_bytes!("../resources/social_icons/twitter.png"),
        render_api,
    )
    .unwrap();
    gs.assets.image_bluesky = load_image_cursor(
        include_bytes!("../resources/social_icons/bluesky.png"),
        render_api,
    )
    .unwrap();

    // init world camera
    {
        let mut cam = &mut es
            .render_packs
            .get_mut(&RenderPackID::NewWorld)
            .unwrap()
            .camera;

        cam.transform.local_position = VecThreeFloat::new(1.0, 27.0, 20.0);
        cam.pitch = 55.0;
        cam.yaw = 90.0;
    }

    // init shop
    {
        let mut cam = &mut es.render_packs.get_mut(&RenderPackID::Shop).unwrap().camera;

        cam.transform.local_position = VecThreeFloat::new(1.0, 27.0, 20.0);
        cam.pitch = 70.0;
        cam.yaw = 90.0;
    }

    gs.light_trans = Some(es.new_transform());
    gs.center_trans = Some(es.new_transform());

    {
        let lt: &mut Transform = &mut es.transforms[gs.light_trans.unwrap()];
        lt.parent = gs.center_trans;

        let ct: &mut Transform = &mut es.transforms[gs.center_trans.unwrap()];
        ct.local_rotation.y = 90.0;
    }

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

        gs.font_style_nav = FontStyle {
            size: 3.0,
            typeface: es.roboto_typeface.get_weight(TypeWeight::Bold),
        };
    }

    // setup initial UI
    gs.active_page = Some(CreatePanelData::Home.create_panel());

    // setup first map
    {
        let init_dirt: Vec<GridPos> = vec![
            GridPos::new(0, 0),
            // GridPos::new(21, 10),
            // GridPos::new(20, 11),
            // GridPos::new(21, 11),
        ];
        for p in init_dirt {
            let _ = gs.world.insert_tile(p, TileType::Dirt);
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

fn sim_world(gs: &mut State, es: &EngineState, ms: f64, platform_api: &PlatformApi) {
    let mut update_signals: Vec<UpdateSignal> = vec![];
    for (eid, entity) in &mut gs.world.entities {
        update_signals.append(&mut entity.sim_update(ms));
    }
    handle_signals(update_signals, gs, es, platform_api);
}

// The render_api is hard-coded here instead of using a trait so that we can support hot reloading
#[no_mangle]
pub fn game_loop_ogl(
    prev_delta_time: f64,
    gs: &mut State,
    es: &mut EngineState,
    input: &mut Input,
    render_api: &OglRenderApi,
    platform_api: &PlatformApi,
) {
    game_loop(prev_delta_time, gs, es, input, render_api, platform_api);
}

// Prev delta time is in seconds. So for 60 fps 0.016666.
pub fn game_loop(
    prev_delta_time: f64,
    gs: &mut State,
    es: &mut EngineState,
    input: &mut Input,
    render_api: &impl RenderApi,
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
            font_nav: gs.font_style_nav.clone(),

            render_commands: vec![],
            button_state: HashMap::new(),

            delta_time: prev_delta_time,
        });

        ui_context.mouse = input.mouse.clone();
        ui_context.delta_time = prev_delta_time;
    }

    let mut ui_frame_state = UIFrameState::new(&input, es.window_resolution);

    // render tile thumbnails.
    // use the hash map to know which tiles to render.
    {
        let mut tiles_to_render: Vec<TileType> = vec![];
        for (key, value) in &mut gs.assets.tile_thumbnails {
            if value.is_none() {
                tiles_to_render.push(*key);
            }
        }

        for tile_type in tiles_to_render {
            gs.assets
                .render_tile_thumbnail(tile_type, None, None, render_api);
        }
    }

    // save game
    {
        // manual save for testing
        #[cfg(feature = "dev")]
        {
            if input.get_key(KeyCode::Q).on_press {
                save_game(&gs.world, &gs.inventory, platform_api).expect("Error saving game.");
                println!("Game manually saved");
            }

            if input.get_key(KeyCode::L).on_press {
                (platform_api.fetch_game_save)();
            }
        }

        // check for data to load
        {
            if !es.game_to_load.is_empty() {
                match load_game(
                    &mut gs.world,
                    &mut gs.inventory,
                    &es.game_to_load,
                    platform_api,
                ) {
                    Ok(mut ms_to_sim) => {
                        while ms_to_sim > 0.0 {
                            // println!("Forward Simulating {}ms remaining", ms_to_sim);
                            let ms_step = ms_to_sim.clamp(0.0, MAX_SIM_MS);
                            sim_world(gs, es, ms_step / 1000.0, platform_api);
                            ms_to_sim -= ms_step;
                        }
                    }
                    Err(error) => {
                        println!("Error loading save file {:?}", error);
                    }
                };
                es.game_to_load.clear();
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
                &mut gs.assets,
                &mut gs.ui_context.as_mut().unwrap(),
                platform_api,
            ));
        }

        // update active page
        match &mut gs.active_page {
            Some(page) => update_signals.append(&mut page.update(
                &mut ui_frame_state,
                &gs.inventory,
                &mut gs.assets,
                &mut gs.ui_context.as_mut().unwrap(),
                platform_api,
            )),
            None => {}
        }

        // Handle signals
        handle_signals(update_signals, gs, es, platform_api);

        // Update input
        input.mouse.button_left.on_press = ui_frame_state.mouse_left;
    }

    // debug developer stuff
    #[cfg(feature = "dev")]
    {
        let g = 0.3;

        // draw fps
        {
            let fps = 1.0 / prev_delta_time;
            draw_text(
                &format!(
                    "{:?}fps {:?}ms",
                    fps as i32,
                    (prev_delta_time * 1000.0) as i32
                ),
                VecTwo::new(
                    es.window_resolution.x - 140.0,
                    es.window_resolution.y - 60.0,
                ),
                Color::new(g, g, g, 1.0),
                &gs.font_style_body,
                &mut ui_frame_state,
                &mut gs.ui_context.as_mut().unwrap(),
            );
        }

        // render commands
        {
            draw_text(
                &format!("{:?} rc", es.render_commands_len),
                VecTwo::new(
                    es.window_resolution.x - 140.0,
                    es.window_resolution.y - 80.0,
                ),
                Color::new(g, g, g, 1.0),
                &gs.font_style_body,
                &mut ui_frame_state,
                &mut gs.ui_context.as_mut().unwrap(),
            );
        }

        // debug panel
        {
            if input.get_key(KeyCode::Tab).on_press {
                gs.debug_state.showing_debug_panel = !gs.debug_state.showing_debug_panel;
            }

            if gs.debug_state.showing_debug_panel {
                if let Some(panel) = &mut gs.debug_state.debug_panel {
                    let sigs = panel.update(
                        &mut ui_frame_state,
                        &gs.inventory,
                        &mut gs.assets,
                        &mut gs.ui_context.as_mut().unwrap(),
                        platform_api,
                    );

                    handle_signals(sigs, gs, es, platform_api);
                }
            }
        }

        // tile thumbnail testing
        if false {
            let spd = 0.05;

            if input.get_key(KeyCode::U).pressing {
                gs.debug_state.thumbnail_dist -= spd;
            }
            if input.get_key(KeyCode::M).pressing {
                gs.debug_state.thumbnail_dist += spd;
            }
            if input.get_key(KeyCode::Y).pressing {
                gs.debug_state.thumbnail_height += spd;
            }
            if input.get_key(KeyCode::N).pressing {
                gs.debug_state.thumbnail_height -= spd;
            }

            let tile_type = TileType::Reed;
            gs.assets.render_tile_thumbnail(
                tile_type,
                Some(gs.debug_state.thumbnail_dist),
                Some(gs.debug_state.thumbnail_height),
                render_api,
            );

            println!("dist {:?}", gs.debug_state.thumbnail_dist);
            println!("height {:?}", gs.debug_state.thumbnail_height);
        }
    }

    // update tiles
    {
        let mut frame_delta: f64 = prev_delta_time;

        #[cfg(feature = "dev")]
        if input.get_key(KeyCode::One).on_press {
            frame_delta = 100.0;
        }

        sim_world(gs, es, frame_delta, platform_api);
    }

    // camera controls
    {
        /*
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
        */

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
            update_sigs.append(&mut tile_inst.update(prev_delta_time, platform_api));
        }
        handle_signals(update_sigs, gs, es, platform_api);
    }

    match gs.world_status {
        WorldStatus::World => {
            // camera controls
            {
                es.render_packs
                    .get_mut(&RenderPackID::NewWorld)
                    .unwrap()
                    .camera
                    .move_fly(0.3, input);
            }

            // render tiles
            {
                // TODO chagne this to use delta_time
                gs.rotate_time += 0.08;

                for (grid_pos, world_cell) in &gs.world.entity_map {
                    for (layer, eid) in &world_cell.layers {
                        // Skip ground tils if there is a floor
                        if *layer == WorldLayer::Ground {
                            if world_cell.layers.contains_key(&WorldLayer::Floor) {
                                continue;
                            }
                        }

                        let entity = &gs.world.get_entity(&eid);

                        entity.render(
                            gs.rotate_time,
                            &entity.grid_pos,
                            es.color_texture_shader,
                            es.render_packs.get_mut(&RenderPackID::NewWorld).unwrap(),
                            &gs.assets,
                        );
                    }
                }
            }

            // Get mouse grid position
            let mouse_grid: GridPos = {
                let mut val = GridPos::new(0, 0);

                let cam: &Camera = &es.render_packs.get(&RenderPackID::NewWorld).unwrap().camera;
                let pos = cam.screen_to_world(input.mouse.pos);

                let dir = (pos - cam.transform.local_position).normalize();

                if let Some(len) = plane_intersection_distance(
                    cam.transform.local_position,
                    dir,
                    VecThreeFloat::new(0.0, 0.0, 0.0),
                    VecThreeFloat::new(0.0, -1.0, 0.0),
                ) {
                    let world_pos = cam.transform.local_position + (dir * len);
                    val = world_to_grid(&world_pos.xz());
                }

                val
            };

            // placing tiles
            if let Some(tile) = gs.tile_placing {
                // escape key reseting
                if input.get_key(KeyCode::Escape).on_press {
                    gs.tile_placing = None;
                }

                // render tile placing
                if tile.get_definition().placing_draw_footprint {
                    let footprint = &tile.get_definition().footprint;

                    for p in footprint {
                        let pos = mouse_grid + *p;

                        let can_place = tile.pos_passes_placement_constraints(pos, &gs.world);

                        draw_tile_grid_pos(
                            tile,
                            0.0,
                            &pos,
                            can_place,
                            es.render_packs.get_mut(&RenderPackID::NewWorld).unwrap(),
                            &gs.assets,
                        );
                    }
                } else {
                    let can_place = tile.can_place_here(mouse_grid, &gs.world);

                    draw_tile_grid_pos(
                        tile,
                        0.0,
                        &mouse_grid,
                        can_place,
                        es.render_packs.get_mut(&RenderPackID::NewWorld).unwrap(),
                        &gs.assets,
                    );
                }

                // place tile
                let can_place = tile.can_place_here(mouse_grid, &gs.world);
                if input.mouse.button_left.on_press && can_place {
                    if let Ok(update_sigs) = gs.world.try_place_tile(mouse_grid, tile) {
                        let count = gs.inventory.give_item(ItemType::Tile(tile), -1).unwrap();
                        if count == 0 {
                            gs.tile_placing = None;
                        }

                        handle_signals(update_sigs, gs, es, platform_api);
                    }
                }
            }

            // tile hovering
            {
                let mut update_signals: Vec<UpdateSignal> = vec![];
                let world_snapshot = gs.world.get_world_snapshot();

                if gs.tile_placing.is_none() {
                    let world_cell: WorldCell = gs.world.get_entities(mouse_grid);

                    for (i, (layer, eid)) in world_cell.layers.iter().enumerate() {
                        let tile = gs.world.get_entity_mut(eid);

                        // Harvesting
                        if input.mouse.button_left.pressing && tile.can_harvest() {
                            tile.harvest(&world_snapshot, platform_api);
                        }

                        // render hover rect
                        {
                            let mut mat = Material::new();
                            mat.shader = Some(es.shader_color);
                            mat.set_color(Color::new(1.0, 1.0, 1.0, 0.8));

                            let mut trans = Transform::new();
                            trans.local_position = grid_to_world(&mouse_grid);
                            trans.update_global_matrix(&M44::new_identity());

                            es.render_packs
                                .get_mut(&RenderPackID::NewWorld)
                                .unwrap()
                                .commands
                                .push(RenderCommand::new_model(
                                    &trans,
                                    gs.assets.asset_library.get_model("tile_outline"),
                                    &mat,
                                ));
                        }

                        // render info
                        {
                            let mut ui_frame_state =
                                UIFrameState::new(&input, es.window_resolution);

                            let y = layer.to_index() as f64 * 40.0;

                            draw_text(
                                &format!("{:?}", tile.tile_type),
                                VecTwo::new(450.0, 100.0 + y),
                                COLOR_WHITE,
                                &gs.font_style_body,
                                &mut ui_frame_state,
                                &mut gs.ui_context.as_mut().unwrap(),
                            );

                            tile.render_hover_info(
                                tile.get_component_harvestable(),
                                y,
                                es.shader_color.clone(),
                                es.render_packs.get_mut(&RenderPackID::UI).unwrap(),
                            );
                        }
                    }
                }

                handle_signals(update_signals, gs, es, platform_api);
            }
        }
        WorldStatus::Shop => {
            let mouse_world: VecThreeFloat = {
                let mut val = VecThreeFloat::new_zero();

                let cam: &Camera = &es.render_packs.get(&RenderPackID::Shop).unwrap().camera;
                let pos = cam.screen_to_world(input.mouse.pos);

                let dir = (pos - cam.transform.local_position).normalize();

                if let Some(len) = plane_intersection_distance(
                    cam.transform.local_position,
                    dir,
                    VecThreeFloat::new(0.0, 0.0, 0.0),
                    VecThreeFloat::new(0.0, -1.0, 0.0),
                ) {
                    val = cam.transform.local_position + (dir * len);
                }

                val
            };

            // camera controls
            if false {
                let cam_pack = es.render_packs.get_mut(&RenderPackID::Shop).unwrap();

                let keyboard_speed = 30.0;
                let mouse_scroll_speed = 400.0;
                let drag_speed = gengar_engine::math::lerp(
                    0.02,
                    0.08,
                    cam_pack.camera.transform.local_position.y / 100.0,
                );

                if input.get_key(KeyCode::W).pressing {
                    cam_pack.camera.transform.local_position.z -= keyboard_speed * prev_delta_time;
                }
                if input.get_key(KeyCode::S).pressing {
                    cam_pack.camera.transform.local_position.z += keyboard_speed * prev_delta_time;
                }
                if input.get_key(KeyCode::A).pressing {
                    cam_pack.camera.transform.local_position.x -= keyboard_speed * prev_delta_time;
                }
                if input.get_key(KeyCode::D).pressing {
                    cam_pack.camera.transform.local_position.x += keyboard_speed * prev_delta_time;
                }
                if input.mouse.scroll_delta > 0 {
                    cam_pack.camera.transform.local_position.y -=
                        mouse_scroll_speed * prev_delta_time;
                } else if input.mouse.scroll_delta < 0 {
                    cam_pack.camera.transform.local_position.y +=
                        mouse_scroll_speed * prev_delta_time
                }

                // camera click dragging
                {
                    if input.mouse.button_left.pressing {
                        if input.mouse.pos_delta.dist_from(VecTwo::new(0.0, 0.0)) > 1.0 {
                            cam_pack.camera.transform.local_position.x +=
                                input.mouse.pos_delta.x * drag_speed;
                            cam_pack.camera.transform.local_position.z +=
                                input.mouse.pos_delta.y * drag_speed;
                        }
                    }
                }
            } else {
                // fly cam for testing
                es.render_packs
                    .get_mut(&RenderPackID::Shop)
                    .unwrap()
                    .camera
                    .move_fly(0.3, input);
            }

            // pack layout rendering
            {
                let packs: Vec<PackID> = vec![PackID::Starter];

                for (i, pack_id) in packs.iter().enumerate() {
                    let pack_info = pack_id.get_pack_info();

                    let world_origin = VecThreeFloat::new(0.0, 0.0, i as f64 * 9.0);

                    let cam: &Camera = &es.render_packs.get(&RenderPackID::Shop).unwrap().camera;
                    let screen_origin = cam.world_to_screen(world_origin, es.window_resolution);

                    // ui
                    let ui_context = &mut gs.ui_context.as_mut().unwrap();

                    let info_rect =
                        Rect::new_top_size(screen_origin + VecTwo::new(50.0, 0.0), 100.0, 100.0);
                    begin_panel(
                        info_rect,
                        Color::new(0.0, 0.0, 0.0, 0.0),
                        &mut ui_frame_state,
                        ui_context,
                    );
                    {
                        draw_text(
                            &pack_info.display_name,
                            VecTwo::new(00.0, 0.0),
                            COLOR_WHITE,
                            &gs.font_style_header,
                            &mut ui_frame_state,
                            ui_context,
                        );

                        // cost
                        {
                            draw_text(
                                "Cost",
                                VecTwo::new(0.0, 30.0),
                                COLOR_WHITE,
                                &ui_context.font_body.clone(),
                                &mut ui_frame_state,
                                ui_context,
                            );
                            for (j, cost) in pack_info.cost.iter().enumerate() {
                                let cost_origin = VecTwo::new(80.0 * j as f64, 35.0);
                                let icon_size = 40.0;

                                let icon = gs.assets.get_item_icon(&cost.0);
                                let r = Rect::new_top_size(cost_origin, icon_size, icon_size);

                                let mut color = COLOR_WHITE;
                                if !gs.inventory.has_atleast(cost.0, cost.1) {
                                    color = COLOR_RED;
                                }

                                draw_image(r, icon, color, &mut ui_frame_state, ui_context);

                                draw_text(
                                    &format!("{}", cost.1),
                                    cost_origin + VecTwo::new(40.0, 30.0),
                                    color,
                                    &ui_context.font_body.clone(),
                                    &mut ui_frame_state,
                                    ui_context,
                                );
                            }
                        }

                        /*
                        if draw_text_button_id(
                            i as i32,
                            "Show Drop List",
                            desc_origin + VecTwo::new(10.0, 110.0),
                            &ui_context.font_body.clone(),
                            false,
                            Some(crate::BUTTON_BG),
                            ui_state,
                            std::line!(),
                            ui_context,
                        ) {
                            let new_panel_data = CreatePanelData::PackDetails { pack_id: *pack_id };
                            sigs.push(UpdateSignal::SetActivePage(new_panel_data));
                        }
                        */
                    }
                    end_panel(&mut ui_frame_state, &mut gs.ui_context.as_mut().unwrap());

                    let hovering = point_within_circle(
                        VecTwo::new(mouse_world.x, mouse_world.z),
                        VecTwo::new(world_origin.x, world_origin.z),
                        2.0,
                    );

                    // pack model
                    {
                        let tile_asset_id = pack_id.to_string_id();

                        let mut trans = Transform::new();
                        trans.local_position = world_origin;
                        trans.local_rotation =
                            VecThreeFloat::new(0.0, -90.0_f64.to_radians(), -90.0_f64.to_radians());
                        trans.update_global_matrix(&M44::new_identity());

                        let mut mat = gs.assets.get_pack_material(*pack_id).clone();
                        if hovering {
                            mat.uniforms
                                .insert("ambientRed".to_string(), UniformData::Float(10.0));
                        }

                        es.render_packs
                            .get_mut(&RenderPackID::Shop)
                            .unwrap()
                            .commands
                            .push(RenderCommand::new_model(
                                &trans,
                                gs.assets.asset_library.get_model(&tile_asset_id),
                                &mat,
                            ));
                    }
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
                es.render_packs.get_mut(&RenderPackID::UI).unwrap(),
                &mut gs.assets,
            );

            if h.is_finished() {
                sigs.push(UpdateSignal::GiveDrop(h.drop));
            }
        }

        // remove fnished
        gs.harvest_drops.retain(|h| !h.is_finished());

        handle_signals(sigs, gs, es, platform_api);
    }

    // draw sphere for light
    {
        let ct: &mut Transform = &mut es.transforms[gs.light_trans.unwrap()];
        ct.local_position.x = -2.0;
        ct.local_position.z = 10.0;
        ct.local_position.y = 15.0;
        // draw_sphere(ct.global_matrix.get_position(), 0.1, COLOR_WHITE);
    }

    es.render_packs
        .get_mut(&RenderPackID::UI)
        .unwrap()
        .commands
        .append(&mut gs.ui_context.as_mut().unwrap().render_commands);

    // get draw calls
    #[cfg(feature = "dev")]
    {
        es.render_commands_len = 0;
        for (key, value) in &es.render_packs {
            es.render_commands_len += value.commands.len() as i32;
        }
    }

    es.game_ui_debug_render_commands = gengar_engine::debug::get_ui_render_list().clone();
    es.game_debug_render_commands = gengar_engine::debug::get_render_list().clone();
}
