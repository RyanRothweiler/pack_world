use crate::{grid::*, tile::*};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*},
    vectors::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Dirt",
    description: "Placed on empty space. Creates ground for other tiles.",
    world_layer: WorldLayer::Ground,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::ValidPosition()],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    TileInstance::new(TileType::Dirt, grid_pos, TileMethods::Dirt)
}
