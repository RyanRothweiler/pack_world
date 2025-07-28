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
    title: "Tall Grass",
    description: "Tall Grass",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::TallGrass, grid_pos, TileMethods::TallGrass);

    inst.comp_harvest = Some(TileCompHarvest::new(
        Time::new(TimeUnit::Seconds(60.0)),
        FixedTableID::TallGrass,
        false,
    ));

    inst
}
