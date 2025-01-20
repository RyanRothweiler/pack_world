use gengar_engine::{color::*, font::*, ui::*};

pub mod tile_library_panel;

use tile_library_panel::*;

pub const BG_COLOR: Color = COLOR_BLUE;

pub enum UpdateSignal {
    SetActivePage(PanelID),
    ConsumeInput,
}
pub enum PanelID {
    TileLibrary,
}

pub enum UIPanelState {
    TileLibrary(UIPanelCommon, TileLibraryPanel),
}

#[derive(Clone)]
pub struct UIPanelCommon {
    pub button_font_style: FontStyle,
}

pub fn update_panel(panel: &mut UIPanelState, ui_state: &mut UIFrameState) -> Vec<UpdateSignal> {
    let mut update_signals: Vec<UpdateSignal> = vec![];

    match panel {
        UIPanelState::TileLibrary(common, panel_state) => {
            update_signals.append(&mut panel_state.update(common, ui_state));
        }
    }

    update_signals
}
