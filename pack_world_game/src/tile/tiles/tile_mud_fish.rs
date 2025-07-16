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
    title: "MudFish",
    description: "Automatically drops dirt tiles. Dies after 3 days.",
    world_layer: WorldLayer::Walker,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Water)],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = Time::new(TimeUnit::Minutes(45.0)).as_seconds().value();

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::MudFish, grid_pos, TileMethods::Grass);

    inst.comp_harvest = Some(TileCompHarvest::new(
        HARVEST_SECONDS,
        FixedTableID::Dirt,
        true,
    ));

    inst.comp_auto_death = Some(TileCompAutoDeath::new(Time::new(TimeUnit::Days(3.0))));

    inst
}
