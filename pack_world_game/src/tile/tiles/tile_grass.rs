#![allow(dead_code)]

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
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Grass",
    description: "Drops basic resources. Reduce cooldown by 10% if adjacent to water.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = 18.0;

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Grass, grid_pos, TileMethods::Grass);

    let mut ht = HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Grass);
    ht.add_length_condition(-0.1, WorldCondition::AdjacentTo(TileSnapshot::Water));
    ht.add_drop_condition(
        (EntryOutput::new_item(ItemType::Acorn, 1), 10.0),
        WorldCondition::AdjacentTo(TileSnapshot::OakTree { has_nest: true }),
    );

    inst.components
        .push(TileComponent::Harvestable { timer: ht });

    inst
}
