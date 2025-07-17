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
    title: "Clam",
    description: "Drops trash and occasionally pearls.",
    world_layer: WorldLayer::Walker,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Water)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = 20.0;

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Clam, grid_pos, TileMethods::Clam);

    inst.comp_harvest = Some(TileCompHarvest::new(
        HARVEST_SECONDS,
        FixedTableID::Clam,
        false,
    ));

    inst
}
