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
use std::{collections::HashMap, fs::File, io::Cursor, path::Path};

pub mod constants;
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

pub use constants::*;
use grid::*;
use harvest_drop::*;
use item::*;
use state::inventory::*;
use tile::*;
use ui_panels::{debug_panel::*, nav_tabs_panel::*, tile_library_panel::*, *};
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

    gs.light_trans = Some(es.new_transform());

    // setup font styles
    {
        gs.font_style_button = FontStyle {
            size: 2.0,
            typeface: es.roboto_font.clone(),
        };
    }

    // setup initial UI
    {
        // gs.active_ui_panels.push(PanelID::Home.create_panel());
        gs.active_page = Some(CreatePanelData::Home.create_panel());
    }

    // setup first map
    {
        let init_dirt: Vec<GridPos> = vec![
            GridPos::new(20, 10),
            GridPos::new(21, 10),
            GridPos::new(20, 11),
            GridPos::new(21, 11),
        ];
        for p in init_dirt {
            let _ = gs.world.force_insert_tile(p, TileType::Dirt);
        }
    }

    // setup initial inventory
    {
        gs.inventory
            .give_item(ItemType::Tile(TileType::Dirt), 10)
            .unwrap();
        gs.inventory
            .give_item(ItemType::Tile(TileType::Grass), 10)
            .unwrap();
    }

    // make debug panel. Needs to happen here so that the memory is in dll space.
    {
        gs.debug_state.debug_panel = Some(UIPanel::DebugPanel(DebugPanel {}));
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

    let mut ui_context = UIContext {
        mouse_pos: input.mouse_pos,
        mouse_down: input.mouse_left.on_press,

        color_shader: es.shader_color_ui,
        color_shader_texture: es.color_texture_shader,

        button_font_style: gs.font_style_button.clone(),

        render_commands: vec![],
        button_state: HashMap::new(),
    };

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
            &mut ui_frame_state,
            &mut ui_context,
        );
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
                &gs.player_state,
                &mut ui_context,
            ));
        }

        // update active page
        match &mut gs.active_page {
            Some(page) => update_signals.append(&mut page.update(
                &mut ui_frame_state,
                &gs.inventory,
                &gs.assets,
                &gs.player_state,
                &mut ui_context,
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
                let sigs = panel.update(
                    &mut ui_frame_state,
                    &gs.inventory,
                    &gs.assets,
                    &gs.player_state,
                    &mut ui_context,
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
        for (eid, entity) in &mut gs.world.entities {
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
        for h in &mut gs.harvest_drops {
            h.update_and_draw(
                0.001,
                es.color_texture_shader,
                es.render_packs.get_mut(&RenderPackID::World).unwrap(),
                &gs.assets,
            );

            if h.is_finished() {
                gs.inventory.give_drop(h.drop).unwrap();
            }
        }

        // remove fnished
        gs.harvest_drops.retain(|h| !h.is_finished());
    }

    let mouse_grid: GridPos = {
        let cam_pack = es.render_packs.get_mut(&RenderPackID::World).unwrap();
        let mouse_world = cam_pack.camera.screen_to_world(input.mouse_pos);
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
        if input.mouse_left.on_press && can_place {
            if let Ok(update_sigs) = gs.world.try_place_tile(mouse_grid, tile) {
                let count = gs.inventory.give_item(ItemType::Tile(tile), -1).unwrap();
                if count == 0 {
                    gs.tile_placing = None;
                }

                handle_signals(update_sigs, gs);
            }
        }
    }

    // tile hovering
    {
        let world_snapshot = gs.world.get_world_snapshot();

        let mut update_signals: Vec<UpdateSignal> = vec![];

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
                if input.mouse_left.pressing && tile.methods.can_harvest() {
                    update_signals.append(&mut tile.methods.harvest(mouse_grid, &world_snapshot));
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
                        &mut ui_frame_state,
                        &mut ui_context,
                    );

                    tile.methods.render_hover_info(
                        y,
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
        .append(&mut ui_context.render_commands);

    es.game_ui_debug_render_commands = gengar_engine::debug::get_ui_render_list().clone();
    es.game_debug_render_commands = gengar_engine::debug::get_render_list().clone();
}
