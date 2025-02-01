use crate::{
    state::{assets::*, inventory::*, *},
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

pub trait UIPanelLifecycle {
    fn update(
        &mut self,
        common: &UIPanelCommon,
        ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
    ) -> Vec<UpdateSignal>;
}

pub struct UIPanel {
    pub panel_id: PanelID,
    pub lifecycle: Box<dyn UIPanelLifecycle>,
}

#[derive(Clone, Copy)]
pub enum PanelID {
    NavTabs,
    TileLibrary,
    Shop,
}

impl PanelID {
    pub fn create_panel(&self) -> UIPanel {
        match self {
            PanelID::NavTabs => UIPanel {
                panel_id: *self,
                lifecycle: Box::new(NavTabsPanel {}),
            },
            PanelID::TileLibrary => UIPanel {
                panel_id: *self,
                lifecycle: Box::new(TileLibraryPanel {}),
            },
            PanelID::Shop => UIPanel {
                panel_id: *self,
                lifecycle: Box::new(ShopPanel {}),
            },
        }
    }
}

#[derive(Clone)]
pub struct UIPanelCommon {
    pub button_font_style: FontStyle,
}
