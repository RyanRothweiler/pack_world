use crate::{
    drop_table::*,
    grid::*,
    item::*,
    save_file::*,
    state::{inventory::*, *},
    tile::*,
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
    title: "Shrub",
    description: "Drops basic food.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = 40.0;

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Shrub, grid_pos, TileMethods::Shrub);

    inst.comp_harvest = Some(TileCompHarvest::new(
        HARVEST_SECONDS,
        FixedTableID::Shrub,
        false,
    ));

    inst
}
