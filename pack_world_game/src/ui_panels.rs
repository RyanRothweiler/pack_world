use crate::{
    pack::*,
    state::{assets::*, inventory::*, *},
    tile::*,
    UpdateSignal,
};
use gengar_engine::{color::*, font::*, ui::*, vectors::*};

pub mod debug_panel;
pub mod home_panel;
pub mod nav_tabs_panel;
pub mod open_pack_panel;
pub mod shop_panel;
pub mod tile_library_panel;

use debug_panel::*;
use home_panel::*;
use nav_tabs_panel::*;
use open_pack_panel::*;
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
        ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
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
    Home,
    OpenPack,
    DebugPanel,
}

#[derive(Clone, Copy)]
pub enum CreatePanelData {
    NavTabs,
    TileLibrary,
    Shop,
    Home,
    OpenPack { pack_id: PackID },
}

impl CreatePanelData {
    pub fn create_panel(&self) -> UIPanel {
        match self {
            CreatePanelData::NavTabs => UIPanel {
                panel_id: PanelID::NavTabs,
                lifecycle: Box::new(NavTabsPanel {}),
            },
            CreatePanelData::OpenPack { pack_id } => UIPanel {
                panel_id: PanelID::OpenPack,
                lifecycle: Box::new(OpenPackPanel::new(*pack_id)),
            },
            CreatePanelData::TileLibrary => UIPanel {
                panel_id: PanelID::TileLibrary,
                lifecycle: Box::new(TileLibraryPanel {}),
            },
            CreatePanelData::Shop => UIPanel {
                panel_id: PanelID::Shop,
                lifecycle: Box::new(ShopPanel {}),
            },
            CreatePanelData::Home => UIPanel {
                panel_id: PanelID::Home,
                lifecycle: Box::new(HomePanel {
                    tab: home_panel::Tab::Inventory,

                    ui_nav_tabs: CreatePanelData::NavTabs.create_panel(),
                    ui_shop: CreatePanelData::Shop.create_panel(),
                    ui_inventory: CreatePanelData::TileLibrary.create_panel(),
                }),
            },
        }
    }
}
