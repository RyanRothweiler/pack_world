use crate::{
    harvest_drop::*,
    pack::*,
    state::{inventory::*, *},
    tiles::*,
    ui_panels::*,
};
use gengar_engine::vectors::*;

// state update signals
pub enum UpdateSignal {
    // set the active_page var
    SetActivePage(PanelID),

    // Start the harvest anim to give item at end of anim
    HarvestItem { item_type: ItemType, origin: VecTwo },

    // Add an item to inventory
    GiveItem { item_type: ItemType, count: i32 },

    // Update the tile that we're currently placing
    SetPlacingTile(Option<TileType>),

    // Open a pack
    OpenPack(PackID),
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
            UpdateSignal::HarvestItem { item_type, origin } => {
                gs.harvest_drops.push(HarvestDrop::new(item_type, origin));
            }
            UpdateSignal::OpenPack(pack_id) => {
                let pack_info: &Pack = get_pack_info(PackID::Starter);

                if !pack_info.can_afford(&gs.inventory) {
                    println!("Cannot afford that pack.");
                    continue;
                }

                for i in 0..4 {
                    let pull_item = pack_info.pull(&gs.inventory).unwrap();
                    println!("Gave item {:?}", pull_item);

                    gs.inventory.add_item(pull_item, 1).unwrap();
                }
            }
        }
    }
}
