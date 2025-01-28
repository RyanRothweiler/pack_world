use std::collections::HashMap;

use crate::{item::*, tiles::*, ui_panels::*, world::*};
use gengar_engine::{
    font::*,
    model::*,
    render::{image::Image, material::*, vao::*},
    transform::*,
    vectors::*,
};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ItemType {
    DirtClod,
}

pub struct Inventory {
    pub items: HashMap<ItemType, i32>,
}

pub struct Assets {
    pub image_dirt: Image,
    pub image_grass: Image,
}

impl Assets {
    pub fn get_tile_icon(&self, tile: &TileType) -> u32 {
        let image_id = match tile {
            TileType::Dirt => return self.image_dirt.gl_id.unwrap(),
            TileType::Grass => return self.image_grass.gl_id.unwrap(),
        };
    }
}

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
            },

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
