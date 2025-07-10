use crate::{
    account_system::*,
    game_mode::*,
    harvest_drop::*,
    pack_shop_display::*,
    tile::*,
    ui_panels::{debug_panel::*, *},
    user_account::*,
    world::*,
    PackID,
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

pub struct DebugState {
    pub showing_debug_panel: bool,
    pub debug_panel: Option<UIPanel>,

    pub thumbnail_dist: f64,
    pub thumbnail_height: f64,
}

pub struct State {
    pub debug_state: DebugState,

    pub assets: Assets,

    pub font_style_body: FontStyle,
    pub font_style_header: FontStyle,
    pub font_style_nav: FontStyle,

    pub active_ui_panels: Vec<UIPanel>,
    pub active_page: Option<UIPanel>,
    pub ui_panel_stack: Vec<UIPanel>,

    // these things need to be saved and loaded between runs
    pub inventory: Inventory,
    pub world: World,

    pub harvest_drops: Vec<HarvestDrop>,

    // time for rotation animation
    pub rotate_time: f64,

    pub ui_context: Option<UIContext>,

    pub current_mode: GameModeKind,
    pub game_mode_world: Option<GameModeWorld>,
    pub game_mode_shop: Option<GameModeShop>,
    pub game_mode_inventory: Option<GameModeInventory>,

    pub pack_light_origin: usize,
    pub pack_light_trans: usize,
    pub pack_light_trans_second: usize,

    pub pack_display_state: HashMap<PackID, PackShopDisplay>,

    pub pack_selected: Option<PackID>,
    pub opening_pack: bool,

    pub account_system: AccountSystem,

    pub save_queued: bool,
    pub save_timer_check: f64,
}

impl State {
    pub fn new() -> Self {
        State {
            debug_state: DebugState {
                showing_debug_panel: false,
                debug_panel: None,
                thumbnail_dist: 0.0,
                thumbnail_height: 0.0,
            },

            active_ui_panels: vec![],
            ui_panel_stack: vec![],

            assets: Assets::new(),

            harvest_drops: vec![],

            font_style_body: Default::default(),
            font_style_header: Default::default(),
            font_style_nav: Default::default(),

            active_page: None,

            world: World::new(),
            inventory: Inventory::new(),

            rotate_time: 0.0,

            ui_context: None,

            current_mode: GameModeKind::World,
            game_mode_world: None,
            game_mode_inventory: None,
            game_mode_shop: None,

            pack_light_origin: 0,
            pack_light_trans: 0,
            pack_light_trans_second: 0,

            pack_display_state: HashMap::new(),

            pack_selected: None,
            opening_pack: false,

            account_system: AccountSystem::new(),

            save_queued: false,
            save_timer_check: 0.0,
        }
    }
}
