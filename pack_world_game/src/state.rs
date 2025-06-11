use crate::{
    harvest_drop::*,
    tile::*,
    ui_panels::{debug_panel::*, *},
    world::*,
};
use gengar_engine::{
    model::*,
    render::{frame_buffer_pack::*, image::Image, material::*, vao::*},
    state::State as EngineState,
    transform::*,
    typeface::*,
    ui::*,
    vectors::*,
};
use std::collections::HashMap;

pub mod assets;
pub mod inventory;

use assets::*;
use inventory::*;

#[cfg(feature = "dev")]
pub struct DebugState {
    pub showing_debug_panel: bool,
    pub debug_panel: Option<UIPanel>,

    pub thumbnail_dist: f64,
    pub thumbnail_height: f64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WorldStatus {
    World,
    Shop,
}

#[derive(Copy, Clone)]
pub struct PackDisplayState {
    pub hover_time: f64,
    pub rotation: VecThreeFloat,
}

pub struct State {
    #[cfg(feature = "dev")]
    pub debug_state: DebugState,

    pub assets: Assets,

    pub font_style_body: FontStyle,
    pub font_style_header: FontStyle,
    pub font_style_nav: FontStyle,

    pub active_ui_panels: Vec<UIPanel>,
    pub active_page: Option<UIPanel>,

    pub tile_placing: Option<TileType>,

    // these things need to be saved and loaded between runs
    pub inventory: Inventory,
    pub world: World,

    pub harvest_drops: Vec<HarvestDrop>,

    // time for rotation animation
    pub rotate_time: f64,

    pub ui_context: Option<UIContext>,

    pub world_status: WorldStatus,

    pub pack_light_origin: usize,
    pub pack_light_trans: usize,
    pub pack_light_trans_second: usize,

    pub pack_display_state: [PackDisplayState; 20],
}

impl State {
    pub fn new() -> Self {
        State {
            #[cfg(feature = "dev")]
            debug_state: DebugState {
                showing_debug_panel: false,
                debug_panel: None,
                thumbnail_dist: 0.0,
                thumbnail_height: 0.0,
            },

            active_ui_panels: vec![],

            assets: Assets::new(),

            harvest_drops: vec![],

            font_style_body: Default::default(),
            font_style_header: Default::default(),
            font_style_nav: Default::default(),

            active_page: None,

            world: World::new(),
            tile_placing: None,
            inventory: Inventory::new(),

            rotate_time: 0.0,

            ui_context: None,

            world_status: WorldStatus::World,

            pack_light_origin: 0,
            pack_light_trans: 0,
            pack_light_trans_second: 0,

            pack_display_state: [PackDisplayState {
                hover_time: 0.0,
                rotation: VecThreeFloat::new_zero(),
            }; 20],
        }
    }
}
