use crate::{state::*, tiles::*, ui_panels::*};

// state update signals
pub enum UpdateSignal {
    // set the active_page var
    SetActivePage(PanelID),

    SetPlacingTile(Option<TileType>),
    GiveItem { item_type: ItemType, count: i32 },
}

pub fn handle_signals(signals: Vec<UpdateSignal>, gs: &mut State) {
    for us in signals {
        match us {
            UpdateSignal::SetActivePage(panel_id) => {
                let page = panel_id.create_page(gs);
                gs.active_page = Some(page);
            }
            UpdateSignal::SetPlacingTile(tile) => {
                gs.tile_placing = tile;
            }
            UpdateSignal::GiveItem { item_type, count } => {
                *gs.inventory.items.entry(item_type).or_insert(0) += count;
            }
        }
    }
}
