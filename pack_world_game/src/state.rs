use crate::{
    harvest_drop::*,
    tile::*,
    ui_panels::{debug_panel::*, *},
    world::*,
};
use gengar_engine::{
    font::*,
    model::*,
    render::{image::Image, material::*, vao::*},
    state::State as EngineState,
    transform::*,
    vectors::*,
};
use std::collections::HashMap;

pub mod assets;
pub mod inventory;
pub mod player_state;

use assets::*;
use inventory::*;
use player_state::*;

#[cfg(feature = "dev")]
pub struct DebugState {
    pub showing_debug_panel: bool,
    pub debug_panel: Option<UIPanel>,
}

pub struct State {
    #[cfg(feature = "dev")]
    pub debug_state: DebugState,

    pub assets: Assets,

    pub light_trans: Option<usize>,

    pub font_style_button: FontStyle,

    pub active_ui_panels: Vec<UIPanel>,
    pub active_page: Option<UIPanel>,

    pub tile_placing: Option<TileType>,

    pub world: World,

    pub inventory: Inventory,
    pub player_state: PlayerState,

    pub harvest_drops: Vec<HarvestDrop>,

    // time for rotation animation
    pub rotate_time: f64,
}

impl State {
    pub fn new() -> Self {
        State {
            #[cfg(feature = "dev")]
            debug_state: DebugState {
                showing_debug_panel: false,
                debug_panel: None,
            },

            active_ui_panels: vec![],

            assets: Assets::new(),

            harvest_drops: vec![],

            light_trans: None,

            font_style_button: Default::default(),
            active_page: None,

            world: World {
                entity_map: HashMap::new(),
                valids: HashMap::new(),
                entities: vec![],
            },
            tile_placing: None,
            inventory: Inventory::new(),
            player_state: PlayerState::new(),

            rotate_time: 0.0,
        }
    }
}
