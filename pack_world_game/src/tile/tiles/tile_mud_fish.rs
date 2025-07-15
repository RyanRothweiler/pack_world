use crate::{
    drop_table::*,
    grid::*,
    item::*,
    save_file::*,
    state::{inventory::*, *},
    tile::{harvest_timer::*, tile_methods::tile_component::TileComponent, *},
    world::*,
};
use gengar_engine::{
    color::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    time::*,
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "MudFish",
    description: "Placed on water. Gives dirt tiles. Auto harvests itself.",
    world_layer: WorldLayer::Walker,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Water)],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = Time::new(TimeUnit::Minutes(45.0)).as_seconds().value();

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::MudFish, grid_pos, TileMethods::Grass);

    let mut ht = HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Dirt, true);

    inst.components
        .push(TileComponent::Harvestable { timer: ht });

    inst
}
