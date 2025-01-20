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

pub mod item;
pub mod state;
pub mod tiles;
pub mod ui_panels;

use item::*;
use tiles::*;
use ui_panels::{tile_library_panel::*, *};

// Used for windows platform loading dlls
pub const PACKAGE_NAME: &str = "pack_world_game";

pub const GRID_SIZE: f64 = 50.0;

pub fn grid_snap(pos: VecTwo) -> VecTwo {
    VecTwo::new(
        (pos.x / GRID_SIZE).round() * GRID_SIZE,
        (pos.y / GRID_SIZE).round() * GRID_SIZE,
    )
}

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

    gs.image_dirt = load_image_cursor(include_bytes!("../resources/dirt.png"), render_api).unwrap();
    gs.image_grass =
        load_image_cursor(include_bytes!("../resources/grass.png"), render_api).unwrap();

    gs.light_trans = Some(es.new_transform());

    /*
    let mt: &mut Transform = &mut es.transforms[gs.monkey_trans.unwrap()];

    let ct: &mut Transform = &mut es.transforms[gs.center_trans.unwrap()];
    ct.local_rotation.y = 90.0;

    let lt: &mut Transform = &mut es.transforms[gs.light_trans.unwrap()];
    lt.local_position.x = 3.5;
    lt.local_position.y = 3.5;
    lt.parent = gs.center_trans;
    */

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

    // setup first ui
    {
        gs.active_ui_panels.push(UIPanelState::TileLibrary(
            gs.ui_panel_common.as_mut().unwrap().clone(),
            TileLibraryPanel {},
        ))
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
    gengar_engine::ui::frame_start(&input, es.shader_color_ui);

    // update UI
    {
        let mut ui_frame_state = UIFrameState::new(&input);

        let mut update_signals: Vec<UpdateSignal> = vec![];

        // Render and update active UI
        for panel in &mut gs.active_ui_panels {
            update_signals.append(&mut ui_panels::update_panel(panel, &mut ui_frame_state));
        }

        // update active page
        match &mut gs.active_page {
            Some(page) => {
                update_signals.append(&mut ui_panels::update_panel(page, &mut ui_frame_state))
            }
            None => {}
        }

        // Handle signals
        for us in update_signals {
            match us {
                UpdateSignal::SetActivePage(panel_id) => match panel_id {
                    ui_panels::PanelID::TileLibrary => {
                        gs.active_page = Some(UIPanelState::TileLibrary(
                            gs.ui_panel_common.as_mut().unwrap().clone(),
                            ui_panels::tile_library_panel::TileLibraryPanel {},
                        ))
                    }
                },
                UpdateSignal::ConsumeInput => {
                    input.mouse_left.on_press = false;
                }
            }
        }

        // Update input
        input.mouse_left.on_press = ui_frame_state.mouse_left;
    }

    // test square render
    {
        let mut r = Rect::new_square(GRID_SIZE * 0.5);
        r.set_center(grid_snap(input.mouse_pos));

        let mut mat = Material::new();
        mat.shader = Some(es.color_texture_shader);

        mat.uniforms.insert(
            "tex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: gs.image_dirt.gl_id.unwrap(),
                texture_slot: 0,
            }),
        );

        mat.uniforms.insert(
            "color".to_string(),
            UniformData::VecFour(COLOR_WHITE.into()),
        );

        es.render_packs
            .get_mut(&RenderPackID::UI)
            .unwrap()
            .commands
            .push(RenderCommand::new_rect(&r, -1.0, &mat));
    }

    // place tile
    if input.mouse_left.on_press {
        let mp = grid_snap(input.mouse_pos);
        let mpi: VecTwoInt = VecTwoInt {
            x: mp.x as i32,
            y: mp.y as i32,
        };
        gs.tiles.entry(mpi).or_insert(Tile {
            image_id: gs.image_dirt.gl_id.unwrap(),
        });
    }

    // render tiles
    {
        for (pos, tile) in &gs.tiles {
            let mut r = Rect::new_square(GRID_SIZE * 0.5);

            r.set_center(VecTwo {
                x: pos.x as f64,
                y: pos.y as f64,
            });

            let mut mat = Material::new();
            mat.shader = Some(es.color_texture_shader);

            mat.uniforms.insert(
                "tex".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: tile.image_id,
                    texture_slot: 0,
                }),
            );

            mat.uniforms.insert(
                "color".to_string(),
                UniformData::VecFour(COLOR_WHITE.into()),
            );

            es.render_packs
                .get_mut(&RenderPackID::UI)
                .unwrap()
                .commands
                .push(RenderCommand::new_rect(&r, -1.0, &mat));
        }
    }

    es.render_packs
        .get_mut(&RenderPackID::UI)
        .unwrap()
        .commands
        .append(&mut gengar_engine::ui::get_render_commands());

    es.game_ui_debug_render_commands = gengar_engine::debug::get_ui_render_list().clone();
    es.game_debug_render_commands = gengar_engine::debug::get_render_list().clone();
}
