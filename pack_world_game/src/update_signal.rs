use crate::{
    drop_table::*,
    harvest_drop::*,
    pack::*,
    state::{inventory::*, *},
    tiles::*,
    ui_panels::{home_panel::*, *},
};
use gengar_engine::vectors::*;

// state update signals
pub enum UpdateSignal {
    // set the active_page var
    SetActivePage(CreatePanelData),

    // Start the harvest anim to give item at end of anim
    HarvestItem { item_type: ItemType, origin: VecTwo },

    // Run harvest, pulling randomly from a table
    HarvestItemPullTable { table: DropTableID, origin: VecTwo },

    // Add an item to inventory
    GiveItem { item_type: ItemType, count: i32 },

    // Update the tile that we're currently placing
    SetPlacingTile(Option<TileType>),

    // Open a pack
    OpenPack(PackID),

    // For the home panel. Not good that this is here.
    // This is suggesting a different architecture.
    HomePanelTabChange(home_panel::Tab),
}

pub fn handle_signals(mut signals: Vec<UpdateSignal>, gs: &mut State) {
    let mut curr_signals: Vec<UpdateSignal> = vec![];
    curr_signals.append(&mut signals);

    while curr_signals.len() > 0 {
        let mut new_signals: Vec<UpdateSignal> = vec![];

        // handle current signals
        for us in &curr_signals {
            let mut sigs: Vec<UpdateSignal> = match us {
                UpdateSignal::SetActivePage(new_panel_data) => {
                    let panel = new_panel_data.create_panel();
                    gs.active_page = Some(panel);
                    vec![]
                }
                UpdateSignal::SetPlacingTile(tile) => {
                    gs.tile_placing = *tile;
                    vec![]
                }
                UpdateSignal::GiveItem { item_type, count } => {
                    *gs.inventory.items.entry(*item_type).or_insert(0) += count;
                    vec![]
                }
                UpdateSignal::HarvestItem { item_type, origin } => {
                    gs.harvest_drops.push(HarvestDrop::new(*item_type, *origin));
                    vec![]
                }
                UpdateSignal::HarvestItemPullTable { table, origin } => {
                    let item_type = get_drop(*table);
                    gs.harvest_drops.push(HarvestDrop::new(item_type, *origin));
                    vec![]
                }
                UpdateSignal::OpenPack(pack_id) => {
                    let pack_info: &Pack = get_pack_info(PackID::Starter);

                    if !pack_info.can_afford(&gs.inventory) {
                        println!("Cannot afford that pack.");
                        // continue;
                    }

                    let new_panel_data = CreatePanelData::OpenPack { pack_id: *pack_id };
                    vec![UpdateSignal::SetActivePage(new_panel_data)]
                }
                UpdateSignal::HomePanelTabChange(_) => {
                    panic!("Home panel needs to consume this");
                    vec![]
                }
            };

            new_signals.append(&mut sigs);
        }

        // update curr_signals with any new signals
        curr_signals.clear();
        curr_signals.append(&mut new_signals);
    }
}
