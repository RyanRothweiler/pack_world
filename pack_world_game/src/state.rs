use std::collections::HashMap;

use crate::{item::*, tiles::*, ui_panels::*, world::*};
use gengar_engine::{
    font::*,
    model::*,
    render::{image::Image, material::*, vao::*},
    transform::*,
    vectors::*,
};

pub struct State {
    pub image_dirt: Image,
    pub image_grass: Image,

    pub light_trans: Option<usize>,

    pub font_style_button: FontStyle,

    pub active_ui_panels: Vec<UIPanelState>,
    pub active_page: Option<UIPanelState>,

    pub items: Vec<Item>,

    pub ui_panel_common: Option<UIPanelCommon>,

    pub tile_placing: Option<Tile>,

    pub world: World,
}

impl State {
    pub fn new() -> Self {
        State {
            active_ui_panels: vec![],
            items: vec![],

            image_dirt: Image::new(),
            image_grass: Image::new(),

            light_trans: None,

            ui_panel_common: None,
            font_style_button: Default::default(),
            active_page: None,

            world: World {
                tiles: HashMap::new(),
                valids: HashMap::new(),
            },
            tile_placing: None,
        }
    }

    pub fn get_tile_icon(&self, tile: &Tile) -> u32 {
        let image_id = match tile {
            Tile::Dirt => return self.image_dirt.gl_id.unwrap(),
            Tile::Grass => return self.image_grass.gl_id.unwrap(),
        };
    }
}
