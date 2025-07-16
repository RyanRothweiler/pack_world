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
    time::*,
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Mud Pit",
    description: "Drops mud babies and ground tiles.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = Time::new(TimeUnit::Minutes(4.0)).as_seconds().value();

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::MudPit, grid_pos, TileMethods::MudPit);

    inst.harvest = Some(TileHarvest::new(
        HARVEST_SECONDS,
        FixedTableID::MudPit,
        false,
    ));

    inst
}
