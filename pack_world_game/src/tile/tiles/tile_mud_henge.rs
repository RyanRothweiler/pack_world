use crate::{grid::*, tile::*};
use elara_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*},
    time::*,
    vectors::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Mud Henge",
    description: "Gives mud hearts. Destroyed after harvesting.",
    world_layer: WorldLayer::Floor,
    footprint: GridPos::new(0, 0).to_rect_iter(2, 2).collect(),
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::MudHenge, grid_pos, TileMethods::MudHenge);

    inst.comp_harvest = Some(TileCompHarvest::new(
        Time::new(TimeUnit::Days(1.0)),
        FixedTableID::MudHenge,
        false,
    ));
    inst.comp_harvest.as_mut().unwrap().destroy_after_harvest = true;

    inst
}
