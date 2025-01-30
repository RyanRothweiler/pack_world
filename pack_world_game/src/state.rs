use crate::{harvest_drop::*, tiles::*, ui_panels::*, world::*};
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

use assets::*;
use inventory::*;

pub struct State {
    pub assets: Assets,

    pub light_trans: Option<usize>,

    pub font_style_button: FontStyle,

    pub active_ui_panels: Vec<UIPanelState>,
    pub active_page: Option<UIPanelState>,

    pub ui_panel_common: Option<UIPanelCommon>,

    pub tile_placing: Option<TileType>,

    pub world: World,

    pub inventory: Inventory,

    pub harvest_drops: Vec<HarvestDrop>,

    // time for rotation animation
    pub rotate_time: f64,
}

impl State {
    pub fn new() -> Self {
        State {
            active_ui_panels: vec![],

            assets: Assets {
                image_dirt: Image::new(),
                image_grass: Image::new(),
                image_stick: Image::new(),
                image_dirt_clod: Image::new(),
            },

            harvest_drops: vec![],

            light_trans: None,

            ui_panel_common: None,
            font_style_button: Default::default(),
            active_page: None,

            world: World {
                tiles: HashMap::new(),
                valids: HashMap::new(),
            },
            tile_placing: None,
            inventory: Inventory {
                items: HashMap::new(),
            },

            rotate_time: 0.0,
        }
    }
}
