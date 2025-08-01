#![allow(
    unused_imports,
    unused_variables,
    clippy::all,
    unused_mut,
    unreachable_code
)]

use crate::state::*;
use elara_engine::{
    account_call::*,
    analytics::*,
    ascii::*,
    build_vars::*,
    collisions::*,
    color::*,
    debug::*,
    input::*,
    logging::*,
    matricies::matrix_four_four::*,
    model::*,
    obj,
    platform_api::*,
    rect::*,
    render::{
        camera::*, image::Image, light::*, load_image, load_image_cursor, material::*,
        render_command::RenderCommand, render_pack::*, shader::*, vao::*, RenderApi,
    },
    state::State as EngineState,
    transform::*,
    typeface::*,
    ui::*,
    vectors::*,
};
use elara_render_opengl::*;
use std::ffi::c_void;
use std::{
    collections::HashMap,
    fs::File,
    io::Cursor,
    path::Path,
    sync::{LazyLock, Mutex},
};

pub mod account_system;
pub mod constants;
pub mod drop_table;
pub mod error;
pub mod game_mode;
pub mod grid;
pub mod harvest_drop;
pub mod item;
pub mod pack;
pub mod pack_shop_display;
pub mod pack_shop_signals;
pub mod save_file;
pub mod state;
pub mod tile;
pub mod ui_panels;
pub mod update_signal;
pub mod user_account;
pub mod world;

#[cfg(test)]
pub mod testing_infra;
#[cfg(test)]
pub mod tests;

use account_system::*;
use assets::*;
pub use constants::*;
use game_mode::*;
use grid::*;
use harvest_drop::*;
use item::*;
use pack::*;
use pack_shop_display::*;
use pack_shop_signals::*;
use save_file::*;
use state::inventory::*;
use tile::*;
use ui_panels::{debug_panel::*, nav_tabs_panel::*, tile_library_panel::*, *};
use update_signal::*;
use user_account::*;
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

fn setup_initial(world: &mut World, inventory: &mut Inventory) {
    // setup map
    {
        let init_dirt: Vec<GridPos> = vec![
            GridPos::new(0, 0),
            // GridPos::new(21, 10),
            // GridPos::new(20, 11),
            // GridPos::new(21, 11),
        ];
        for p in init_dirt {
            let _ = world.insert_tile_type(p, TileType::Dirt);
        }
    }

    // setup inventory
    {
        inventory
            .give_item(ItemType::Tile(TileType::Dirt), 4)
            .unwrap();
        inventory
            .give_item(ItemType::Tile(TileType::Grass), 4)
            .unwrap();
    }
}

#[no_mangle]
pub fn game_init(
    game_state_ptr: *mut c_void,
    es: &mut EngineState,
    render_api: &mut OglRenderApi,
    platform_api: &PlatformApi,
) {
    let gs = unsafe { &mut *(game_state_ptr as *mut State) };

    (platform_api.send_event)(AnalyticsEvent::AppStart);

    elara_engine::debug::init_context(
        es.shader_color.clone(),
        es.shader_color_ui,
        es.model_sphere.clone(),
        es.model_plane.clone(),
    );

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
    gs.assets.image_mud_heart =
        load_image_cursor(include_bytes!("../resources/mud_heart.png"), render_api).unwrap();
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
    gs.assets.image_eye_of_newt =
        load_image_cursor(include_bytes!("../resources/eye_of_newt.png"), render_api).unwrap();
    gs.assets.image_frog_leg =
        load_image_cursor(include_bytes!("../resources/frog_leg.png"), render_api).unwrap();
    gs.assets.image_root =
        load_image_cursor(include_bytes!("../resources/root.png"), render_api).unwrap();
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
    gs.assets.image_discord = load_image_cursor(
        include_bytes!("../resources/social_icons/discord.png"),
        render_api,
    )
    .unwrap();

    // init world camera
    {
        let mut cam = &mut es.render_system.get_pack(RenderPackID::NewWorld).camera;

        cam.transform.local_position = VecThreeFloat::new(1.0, 27.0, 20.0);
        cam.pitch = 55.0;
        cam.yaw = 90.0;
        cam.move_target_position = cam.transform.local_position;
    }

    // init shop
    {
        let mut cam = &mut es.render_system.get_pack(RenderPackID::Shop).camera;

        cam.transform.local_position = VecThreeFloat::new(0.0, 23.0, 11.0);
        cam.pitch = 70.0;
        cam.yaw = 90.0;
        cam.move_target_position = cam.transform.local_position;
    }

    // lights
    {
        // new world light
        {
            let light = Light::new(es.components.new_transform());

            let ct: &mut Transform = &mut es.components.transforms[light.transform];
            ct.local_position.x = -2.0;
            ct.local_position.z = 10.0;
            ct.local_position.y = 15.0;

            es.render_system
                .get_pack(RenderPackID::NewWorld)
                .lights
                .push(light);
        }
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

    gs.ui_panel_stack.push(CreatePanelData::Home.create_panel());

    // setup first map
    setup_initial(&mut gs.world, &mut gs.inventory);

    // make debug panel. Needs to happen here so that the memory is in dll space.
    {
        gs.debug_state.debug_panel = Some(UIPanel::DebugPanel(DebugPanel {}));
    }

    gs.account_system
        .start_try_login_existing(platform_api, &mut es.networking_system);

    // setup game modes
    {
        gs.game_mode_world = Some(GameModeWorld::new());
        gs.game_mode_shop = Some(GameModeShop::new(es));
        gs.game_mode_inventory = Some(GameModeInventory::new());
    }
}

fn sim_world(gs: &mut State, es: &mut EngineState, ms: f64, platform_api: &PlatformApi) {
    let world_snapshot = gs.world.get_world_snapshot();

    let mut update_signals: Vec<UpdateSignal> = vec![];

    for (eid, entity) in &mut gs.world.entities {
        update_signals.append(&mut entity.sim_update(ms, &world_snapshot, platform_api));
    }
    handle_signals(update_signals, gs, es, platform_api);
}

// Prev delta time is in seconds. So for 60 fps 0.016666.
#[no_mangle]
pub fn game_loop(
    prev_delta_time: f64,
    game_state_ptr: *mut c_void,
    es: &mut EngineState,
    input: &mut Input,
    render_api: &mut OglRenderApi,
    platform_api: &PlatformApi,
) {
    let gs = unsafe { &mut *(game_state_ptr as *mut State) };

    elara_engine::debug::init_context(
        es.shader_color.clone(),
        es.shader_color_ui.clone(),
        es.model_sphere.clone(),
        es.model_plane.clone(),
    );
    elara_engine::debug::frame_start();

    let account_update_sigs = gs
        .account_system
        .update(platform_api, &mut es.networking_system);
    handle_signals(account_update_sigs, gs, es, platform_api);

    // update ui_context
    {
        let ui_context = gs.ui_context.get_or_insert(UIContext {
            mouse: input.mouse.clone(),
            keyboard: input.keyboard.clone(),
            paste: None,

            color_shader: es.shader_color_ui,
            color_shader_texture: es.color_texture_shader,

            font_body: gs.font_style_body.clone(),
            font_header: gs.font_style_header.clone(),
            font_nav: gs.font_style_nav.clone(),

            render_commands: vec![],

            button_state: HashMap::new(),
            input_fields: HashMap::new(),

            delta_time: prev_delta_time,

            selected_input_field: None,
        });

        ui_context.mouse = input.mouse.clone();
        ui_context.keyboard = input.keyboard.clone();
        ui_context.delta_time = prev_delta_time;
        ui_context.paste = input.paste.clone();
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
                .render_tile_thumbnail(tile_type, None, None, render_api, &mut es.components);
        }
    }

    // save game
    {
        // save timer
        {
            gs.save_timer_check += prev_delta_time;
            if gs.save_timer_check > 3.0 {
                gs.save_timer_check = 0.0;

                if gs.save_queued {
                    gs.save_queued = false;
                    save_game(&gs.world, &gs.inventory, platform_api).expect("Error saving game.");
                    println!("Saving game");
                }
            }
        }

        // manual save for testing
        if build_type_development() {
            if input.keyboard.get_key(KeyCode::Q).on_press {
                save_game(&gs.world, &gs.inventory, platform_api).expect("Error saving game.");
                println!("Game manually saved");
            }

            if input.keyboard.get_key(KeyCode::L).on_press {
                (platform_api.fetch_game_save)();
                println!("Game manually loaded");
            }
        }

        // check for data to load
        {
            if !es.game_to_load.is_empty() && gs.account_system.user_fetches_finished() {
                match load_game(
                    &mut gs.world,
                    &mut gs.inventory,
                    &es.game_to_load,
                    &gs.account_system,
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
                        es.logger.println(&format!(
                            "Error loading save file. Clearing save data. {:?}",
                            error
                        ));
                        setup_initial(&mut gs.world, &mut gs.inventory);
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
                &gs.account_system,
                &mut es.networking_system,
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
                &gs.account_system,
                &mut es.networking_system,
                &mut ui_frame_state,
                &gs.inventory,
                &mut gs.assets,
                &mut gs.ui_context.as_mut().unwrap(),
                platform_api,
            )),
            None => {}
        }

        if let Some(top_panel) = gs.ui_panel_stack.last_mut() {
            update_signals.append(&mut top_panel.update(
                &gs.account_system,
                &mut es.networking_system,
                &mut ui_frame_state,
                &gs.inventory,
                &mut gs.assets,
                &mut gs.ui_context.as_mut().unwrap(),
                platform_api,
            ))
        }

        // Handle signals
        handle_signals(update_signals, gs, es, platform_api);

        // Update input
        input.mouse.button_left.on_press = ui_frame_state.mouse_left;

        // Selected input field updating
        if input.keyboard.get_key(KeyCode::Escape).on_press {
            let ui_cont = gs.ui_context.as_mut().unwrap();

            if ui_cont.selected_input_field.is_some() {
                let id: String = ui_cont.selected_input_field.as_ref().unwrap().clone();
                ui_cont.input_fields.get_mut(&id).unwrap().selected = false;
            }
            ui_cont.selected_input_field = None;
        }
    }

    // debug stats display
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

        // build info
        {
            draw_text(
                &format!("{} {:?}", server_env().display_name, build_type()),
                VecTwo::new(
                    es.window_resolution.x - 180.0,
                    es.window_resolution.y - 100.0,
                ),
                Color::new(g, g, g, 1.0),
                &gs.font_style_body,
                &mut ui_frame_state,
                &mut gs.ui_context.as_mut().unwrap(),
            );
        }
    }

    // debug developer stuff
    if build_type_development() {
        // debug panel
        {
            if input.keyboard.get_key(KeyCode::Tab).on_press {
                gs.debug_state.showing_debug_panel = !gs.debug_state.showing_debug_panel;
            }

            if gs.debug_state.showing_debug_panel {
                if let Some(panel) = &mut gs.debug_state.debug_panel {
                    let sigs = panel.update(
                        &gs.account_system,
                        &mut es.networking_system,
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

            if input.keyboard.get_key(KeyCode::U).pressing {
                gs.debug_state.thumbnail_dist -= spd;
            }
            if input.keyboard.get_key(KeyCode::M).pressing {
                gs.debug_state.thumbnail_dist += spd;
            }
            if input.keyboard.get_key(KeyCode::Y).pressing {
                gs.debug_state.thumbnail_height += spd;
            }
            if input.keyboard.get_key(KeyCode::N).pressing {
                gs.debug_state.thumbnail_height -= spd;
            }

            let tile_type = TileType::MudHenge;
            gs.assets.render_tile_thumbnail(
                tile_type,
                Some(gs.debug_state.thumbnail_dist),
                Some(gs.debug_state.thumbnail_height),
                render_api,
                &mut es.components,
            );

            println!("dist {:?}", gs.debug_state.thumbnail_dist);
            println!("height {:?}", gs.debug_state.thumbnail_height);
        }
    }

    // update tiles
    {
        let mut frame_delta: f64 = prev_delta_time;

        if build_type_development() && input.keyboard.get_key(KeyCode::One).on_press {
            frame_delta = 10000000.0;
        }

        sim_world(gs, es, frame_delta, platform_api);
    }

    // run tile updates
    {
        let mut update_sigs: Vec<UpdateSignal> = vec![];
        for (eid, tile_inst) in &mut gs.world.entities {
            update_sigs.append(&mut tile_inst.update(prev_delta_time, platform_api));
        }
        handle_signals(update_sigs, gs, es, platform_api);
    }

    let show_game = {
        if let Some(top_panel) = gs.ui_panel_stack.last_mut() {
            !top_panel.owns_screen()
        } else {
            true
        }
    };

    if show_game {
        /*
        let update_sigs = gs.current_mode.update(
            prev_delta_time,
            es,
            &mut ui_frame_state,
            input,
            render_api,
            platform_api,
            &mut gs.inventory,
            &mut gs.assets,
            gs.ui_context.as_mut().unwrap(),
            &mut gs.world,
        );
        */
        let update_sigs = match gs.current_mode {
            GameModeKind::Inventory => gs.game_mode_inventory.as_mut().unwrap().update(
                prev_delta_time,
                es,
                &mut ui_frame_state,
                input,
                render_api,
                platform_api,
                &mut gs.inventory,
                &mut gs.assets,
                gs.ui_context.as_mut().unwrap(),
            ),
            GameModeKind::Shop => gs.game_mode_shop.as_mut().unwrap().update(
                prev_delta_time,
                es,
                &mut ui_frame_state,
                input,
                render_api,
                platform_api,
                &mut gs.inventory,
                &mut gs.assets,
                gs.ui_context.as_mut().unwrap(),
                &mut gs.account_system,
            ),
            GameModeKind::World => gs.game_mode_world.as_mut().unwrap().update(
                prev_delta_time,
                es,
                input,
                render_api,
                platform_api,
                &mut gs.world,
                &mut gs.assets,
                &mut gs.inventory,
                gs.ui_context.as_mut().unwrap(),
                &mut ui_frame_state,
            ),
        };

        handle_signals(update_sigs, gs, es, platform_api);
    }

    // update harvest drops
    {
        let mut sigs: Vec<UpdateSignal> = vec![];

        for h in &mut gs.harvest_drops {
            h.update_and_draw(
                0.001,
                es.color_texture_shader,
                es.render_system.get_pack(RenderPackID::UI),
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

    es.render_system
        .render_packs
        .get_mut(&RenderPackID::UI)
        .unwrap()
        .commands
        .append(&mut gs.ui_context.as_mut().unwrap().render_commands);

    // get draw calls
    if build_type_development() {
        es.render_commands_len = 0;
        for (key, value) in &es.render_system.render_packs {
            es.render_commands_len += value.commands.len() as i32;
        }
    }

    es.game_ui_debug_render_commands = elara_engine::debug::get_ui_render_list().clone();
    es.game_debug_render_commands = elara_engine::debug::get_render_list().clone();
}
