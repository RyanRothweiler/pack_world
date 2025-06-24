use crate::{
    pack::*,
    state::{assets::*, inventory::*, *},
    tile::*,
    UpdateSignal,
};
use gengar_engine::{color::*, networking::*, platform_api::*, typeface::*, ui::*, vectors::*};

pub mod create_account_panel;
pub mod debug_panel;
pub mod home_panel;
pub mod nav_tabs_panel;
pub mod open_pack_panel;
pub mod pack_details_panel;
pub mod shop_panel;
pub mod tile_library_panel;

use create_account_panel::*;
use debug_panel::*;
use home_panel::*;
use nav_tabs_panel::*;
use open_pack_panel::*;
use pack_details_panel::*;
use shop_panel::*;
use tile_library_panel::*;

pub const BG_COLOR: Color = Color {
    r: 0.32,
    g: 0.32,
    b: 0.32,
    a: 1.0,
};

pub enum UIPanel {
    NavTabs(NavTabsPanel),
    TileLibrary(TileLibraryPanel),
    Shop(ShopPanel),
    Home(HomePanel),
    OpenPack(OpenPackPanel),
    DebugPanel(DebugPanel),
    PackDetails(PackDetailsData),
    CreateAccount(CreateAccountPanel),
}

impl UIPanel {
    pub fn update(
        &mut self,
        networking_system: &mut NetworkingSystem,
        ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        match self {
            UIPanel::NavTabs(state) => {
                panic!("Nav tabs need to be updated manually");
            }
            UIPanel::TileLibrary(state) => state.update(ui_state, inventory, assets, ui_context),
            UIPanel::Shop(state) => state.update(ui_state, inventory, assets, ui_context),
            UIPanel::Home(state) => state.update(
                networking_system,
                ui_state,
                inventory,
                assets,
                ui_context,
                platform_api,
            ),
            UIPanel::OpenPack(state) => {
                state.update(ui_state, inventory, assets, ui_context, platform_api)
            }
            UIPanel::DebugPanel(state) => state.update(ui_state, inventory, assets, ui_context),
            UIPanel::PackDetails(state) => state.update(ui_state, inventory, assets, ui_context),
            UIPanel::CreateAccount(state) => {
                state.update(networking_system, ui_state, inventory, assets, ui_context)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PanelID {
    NavTabs,
    TileLibrary,
    Shop,
    Home,
    OpenPack,
    DebugPanel,
    PackDetails,
}

#[derive(Clone, Copy, Debug)]
pub enum CreatePanelData {
    NavTabs,
    TileLibrary,
    Shop,
    Home,
    OpenPack { pack_id: PackID },
    PackDetails { pack_id: PackID },
    CreateAccount,
}

impl CreatePanelData {
    pub fn create_panel(&self) -> UIPanel {
        match self {
            CreatePanelData::NavTabs => UIPanel::NavTabs(NavTabsPanel { user_account: None }),
            CreatePanelData::TileLibrary => UIPanel::TileLibrary(TileLibraryPanel::new()),
            CreatePanelData::Shop => UIPanel::Shop(ShopPanel {}),
            CreatePanelData::Home => UIPanel::Home(HomePanel {
                tab: WorldStatus::World,

                ui_nav_tabs: Box::new(CreatePanelData::NavTabs.create_panel()),
                ui_shop: Box::new(CreatePanelData::Shop.create_panel()),
                ui_inventory: Box::new(CreatePanelData::TileLibrary.create_panel()),
            }),
            CreatePanelData::OpenPack { pack_id } => {
                UIPanel::OpenPack(OpenPackPanel::new(*pack_id))
            }
            CreatePanelData::PackDetails { pack_id } => {
                UIPanel::PackDetails(PackDetailsData::new(*pack_id))
            }
            CreatePanelData::CreateAccount => UIPanel::CreateAccount(CreateAccountPanel::new()),
        }
    }
}
