#![allow(dead_code)]

use crate::{
    drop_table::*,
    grid::*,
    item::*,
    save_file::*,
    state::{inventory::*, *},
    tile::*,
    world::*,
};
use elara_engine::{
    color::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    time::*,
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Reed",
    description: "Must be placed in mud. Drops potion resources",
    world_layer: WorldLayer::Planted,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::MudPit)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});
const HARVEST_SECONDS: f64 = 20.0;

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Reed, grid_pos, TileMethods::Reed);

    inst.comp_harvest = Some(TileCompHarvest::new(
        Time::new(TimeUnit::Seconds(20.0)),
        FixedTableID::SmallGold,
        false,
    ));
    inst.comp_harvest.as_mut().unwrap().destroy_after_harvest = true;

    inst
}
