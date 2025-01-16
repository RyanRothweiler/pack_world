use std::collections::HashMap;

use crate::{item::*, tiles::*, ui_panels::*};
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

    pub tiles: HashMap<VecTwoInt, Tile>,
}

impl State {
    pub fn new() -> Self {
        State {
            active_ui_panels: vec![],
            items: vec![],

            image_dirt: Image::new(),
            image_grass: Image::new(),

            light_trans: None,

            font_style_button: Default::default(),
            ui_panel_common: None,
            active_page: None,

            tiles: HashMap::new(),
        }
    }
}
