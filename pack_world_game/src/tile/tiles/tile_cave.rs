use crate::{
    drop_table::*,
    grid::*,
    save_file::*,
    state::{inventory::*, *},
    tile::*,
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
    title: "Cave",
    description: "Drops babies and eggs.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],

    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Cave, grid_pos, TileMethods::Cave);

    inst.comp_harvest = Some(TileCompHarvest::new(
        Time::new(TimeUnit::Days(1.5)),
        FixedTableID::Cave,
        false,
    ));

    inst
}
