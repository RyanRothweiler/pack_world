use crate::{state::*, tiles::*, ui_panels::*};

// state update signals
pub enum UpdateSignal {
    SetActivePage(PanelID),
    SetPlacingTile(Option<TileType>),
    GiveItem { item_type: ItemType, count: i32 },
}

pub fn handle_signals(signals: Vec<UpdateSignal>, gs: &mut State) {
    for us in signals {
        match us {
            UpdateSignal::SetActivePage(panel_id) => match panel_id {
                PanelID::TileLibrary => {
                    gs.active_page = Some(UIPanelState::TileLibrary(
                        gs.ui_panel_common.as_mut().unwrap().clone(),
                        tile_library_panel::TileLibraryPanel {},
                    ))
                }
            },
            UpdateSignal::SetPlacingTile(tile) => {
                gs.tile_placing = tile;
            }
            UpdateSignal::GiveItem { item_type, count } => {
                *gs.inventory.items.entry(item_type).or_insert(0) += count;
            }
        }
    }
}
