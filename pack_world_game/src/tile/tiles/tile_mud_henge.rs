use crate::{grid::*, tile::*};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*},
    vectors::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Mud Henge",
    description: "todo",
    world_layer: WorldLayer::Floor,
    footprint: GridPos::new(0, 0).to_rect_iter(2, 2).collect(),
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    TileInstance::new(TileType::MudHenge, grid_pos, TileMethods::MudHenge)
}
