use crate::{
    state::{inventory::*, *},
    tiles::*,
    UpdateSignal,
};
use gengar_engine::{color::*, font::*, ui::*, vectors::*};

pub mod nav_tabs_panel;
pub mod shop_panel;
pub mod tile_library_panel;

use nav_tabs_panel::*;
use shop_panel::*;
use tile_library_panel::*;

pub const BG_COLOR: Color = Color {
    r: 0.32,
    g: 0.32,
    b: 0.32,
    a: 1.0,
};

pub enum PanelID {
    TileLibrary,
    Shop,
}

impl PanelID {
    pub fn create_page(&self, gs: &mut State) -> UIPanelState {
        match self {
            PanelID::TileLibrary => {
                return UIPanelState::TileLibrary(
                    gs.ui_panel_common.as_mut().unwrap().clone(),
                    tile_library_panel::TileLibraryPanel {},
                )
            }
            PanelID::Shop => {
                return UIPanelState::Shop(
                    gs.ui_panel_common.as_mut().unwrap().clone(),
                    shop_panel::ShopPanel {},
                )
            }
        };
    }
}

pub enum UIPanelState {
    TileLibrary(UIPanelCommon, TileLibraryPanel),
    NavTabs(UIPanelCommon, NavTabsPanel),
    Shop(UIPanelCommon, ShopPanel),
}

#[derive(Clone)]
pub struct UIPanelCommon {
    pub button_font_style: FontStyle,
}

pub fn update_panel(
    panel: &mut UIPanelState,
    ui_state: &mut UIFrameState,
    inventory: &Inventory,
) -> Vec<UpdateSignal> {
    let mut update_signals: Vec<UpdateSignal> = vec![];

    match panel {
        UIPanelState::TileLibrary(common, panel_state) => {
            update_signals.append(&mut panel_state.update(common, ui_state, inventory));
        }
        UIPanelState::NavTabs(common, panel_state) => {
            update_signals.append(&mut panel_state.update(common, ui_state, inventory));
        }
        UIPanelState::Shop(common, panel_state) => {
            update_signals.append(&mut panel_state.update(common, ui_state, inventory));
        }
    }

    update_signals
}
