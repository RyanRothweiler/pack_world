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
    description:
        "Automatically drops dirt tiles. Drops double if adjacent to a MudPit. Dies after 3 days.",
    world_layer: WorldLayer::Walker,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Water)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::MudFish, grid_pos, TileMethods::MudFish);

    let mut tch = TileCompHarvest::new(Time::new(TimeUnit::Hours(6.0)), FixedTableID::Dirt, true);
    tch.add_drop_count_condition(2.0, WorldCondition::AdjacentTo(TileSnapshot::MudPit));
    inst.comp_harvest = Some(tch);

    inst.comp_auto_death = Some(TileCompAutoDeath::new(Time::new(TimeUnit::Days(3.0))));

    inst
}
