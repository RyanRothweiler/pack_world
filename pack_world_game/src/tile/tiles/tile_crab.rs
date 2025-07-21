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
    title: "Crab",
    description: "Automatically harvests anything adjacent to myself. Must be adjacent to a kelp.",
    world_layer: WorldLayer::Walker,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![
        WorldCondition::OriginContains(TileSnapshot::Water),
        WorldCondition::AdjacentTo(TileSnapshot::Kelp),
    ],
    new_instance: new_instance,
    placement_global_mod: vec![],
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Crab, grid_pos, TileMethods::Crab);

    let harvest_time = Time::new(TimeUnit::Seconds(10.0));
    let positions: Vec<GridPos> = GridPos::new(0, 0).to_adjacents_iter().collect();
    inst.comp_harvest_others = Some(TileCompHarvestOthers::new(harvest_time, positions));

    inst
}
