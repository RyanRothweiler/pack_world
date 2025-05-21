use crate::{
    drop_table::*,
    grid::*,
    harvest_drop::*,
    item::*,
    pack::*,
    save_file::*,
    state::{inventory::*, *},
    tile::*,
    ui_panels::{home_panel::*, *},
    world::world_layer::*,
};
use gengar_engine::{
    analytics::*,
    platform_api::*,
    render::{camera::*, render_pack::*},
    state::State as EngineState,
    vectors::*,
};

// TODO maybe consolidate shot purchases into one enum.
// with state for the type of purchase

// state update signals
#[derive(Debug)]
pub enum UpdateSignal {
    /// set the active_page var
    SetActivePage(CreatePanelData),

    /// Add an item to inventory
    GiveItem { item_type: ItemType, count: i64 },

    /// Update the tile that we're currently placing
    SetPlacingTile(Option<TileType>),

    /// Open a pack
    OpenPack(PackID),

    /// For the home panel. Not good that this is here.
    /// This is suggesting a different architecture.
    HomePanelTabChange(home_panel::Tab),

    /// Purchase a bank slot
    PurchaseBankSlot,

    /// Give gold
    GiveGold { amount: i64 },

    /// Give a drop
    GiveDrop(Drop),

    /// Setup a harvest drop
    AddHarvestDrop { drop: Drop, origin: GridPos },

    /// Destroy a tile
    DestroyTile { pos: GridPos, layer: WorldLayer },

    /// Trigger a game save
    SaveGame,

    /// Open url
    OpenURL { url: String },

    /// Trigger rendering a tile thumbnail
    TriggerRenderTileThumbnail { tile_type: TileType },
}

pub fn handle_signals(
    mut signals: Vec<UpdateSignal>,
    gs: &mut State,
    es: &EngineState,
    platform_api: &PlatformApi,
) {
    let mut curr_signals: Vec<UpdateSignal> = vec![];
    curr_signals.append(&mut signals);

    while curr_signals.len() > 0 {
        let mut new_signals: Vec<UpdateSignal> = vec![];

        // handle current signals
        for us in &curr_signals {
            println!("UpdateSignal: {:?}", us);

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
                    gs.inventory.give_item(*item_type, *count).unwrap();
                    vec![UpdateSignal::SaveGame]
                }

                UpdateSignal::GiveDrop(drop) => {
                    gs.inventory.give_drop(*drop).unwrap();
                    vec![UpdateSignal::SaveGame]
                }

                UpdateSignal::AddHarvestDrop { drop, origin } => {
                    let cam: &Camera =
                        &es.render_packs.get(&RenderPackID::NewWorld).unwrap().camera;

                    let world_pos = grid_to_world(origin);
                    let screen_pos_origin: VecTwo =
                        cam.world_to_screen(world_pos, es.window_resolution);

                    gs.harvest_drops
                        .push(HarvestDrop::new(*drop, screen_pos_origin, platform_api));
                    vec![]
                }

                UpdateSignal::OpenPack(pack_id) => {
                    let pack_info: &Pack = pack_id.get_pack_info();

                    if !pack_info.can_afford(&gs.inventory) {
                        println!("Cannot afford that pack.");
                        continue;
                    }

                    (platform_api.send_event)(AnalyticsEvent::PackOpen(format!("{:?}", pack_id)));
                    pack_info.spend(&mut gs.inventory);

                    let new_panel_data = CreatePanelData::OpenPack { pack_id: *pack_id };
                    vec![
                        UpdateSignal::SetActivePage(new_panel_data),
                        UpdateSignal::SaveGame,
                    ]
                }

                UpdateSignal::HomePanelTabChange(_) => {
                    panic!("Home panel needs to consume this");
                    vec![]
                }

                UpdateSignal::PurchaseBankSlot => {
                    if gs.inventory.gold >= gs.inventory.next_slot_cost() {
                        gs.inventory.gold -= gs.inventory.next_slot_cost();
                        gs.inventory.limit += 1;
                    }
                    vec![UpdateSignal::SaveGame]
                }

                UpdateSignal::GiveGold { amount } => {
                    let _ = gs.inventory.give_gold(*amount);
                    vec![UpdateSignal::SaveGame]
                }

                UpdateSignal::SaveGame => {
                    match save_game(&gs.world, &gs.inventory, platform_api) {
                        Ok(()) => println!("Game saved successfully"),
                        Err(error) => println!("Error saving game {:?}", error),
                    }

                    vec![]
                }

                UpdateSignal::DestroyTile { pos, layer } => {
                    gs.world.destroy_tile(*pos, *layer);
                    vec![UpdateSignal::SaveGame]
                }

                UpdateSignal::OpenURL { url } => {
                    (platform_api.open_url)(url.clone());

                    vec![]
                }

                UpdateSignal::TriggerRenderTileThumbnail { tile_type } => {
                    gs.assets.tile_thumbnails.insert(*tile_type, None);

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
