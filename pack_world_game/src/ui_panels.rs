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
        common: &UIPanelCommon,
        ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
    ) -> Vec<UpdateSignal>;
}

pub struct UIPanel<T: UIPanelLifecycle> {
    pub panel_id: PanelID,
    pub lifecycle: T,
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

impl UIPanel<NavTabsPanel> {
    pub fn new() -> UIPanel<NavTabsPanel> {
        UIPanel {
            panel_id: PanelID::NavTabs,
            lifecycle: NavTabsPanel {},
        }
    }
}

impl UIPanel<HomePanel> {
    pub fn new() -> UIPanel<HomePanel> {
        UIPanel {
            panel_id: PanelID::Home,
            lifecycle: HomePanel {},
        }
    }
}

/*
impl CreatePanelData {
    pub fn create_panel<T>(&self) -> UIPanel<T>
    where
        T: UIPanelLifecycle,
    {
        match self {
            CreatePanelData::NavTabs => UIPanel::new(),
            /*
            CreatePanelData::OpenPack { pack_id } => UIPanel {
                panel_id: PanelID::OpenPack,
                lifecycle: OpenPackPanel::new(*pack_id),
            },
            CreatePanelData::TileLibrary => UIPanel {
                panel_id: PanelID::TileLibrary,
                lifecycle: TileLibraryPanel {},
            },
            CreatePanelData::Shop => UIPanel {
                panel_id: PanelID::Shop,
                lifecycle: ShopPanel {},
            },
            CreatePanelData::Home => UIPanel {
                panel_id: PanelID::Home,
                lifecycle: HomePanel {
                    tab: home_panel::Tab::Inventory,

                    ui_nav_tabs: CreatePanelData::NavTabs.create_panel(),
                    ui_shop: CreatePanelData::Shop.create_panel(),
                    ui_inventory: CreatePanelData::TileLibrary.create_panel(),
                },
            },
            */
            _ => todo!(),
        }
    }
}
*/

#[derive(Clone)]
pub struct UIPanelCommon {
    pub button_font_style: FontStyle,
}
