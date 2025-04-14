use crate::{grid::*, tile::*};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*},
    vectors::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Water",
    description: "Placed on empty space. Creates water for other tiles.",
    world_layer: WorldLayer::Ground,
    footprint: vec![GridPos::new(0, 0)],

    placement_constraints: vec![WorldCondition::ValidPosition()],

    build_methods: new_methods,
    add_components: add_components,
});

pub fn new_methods(origin: GridPos) -> TileMethods {
    TileMethods::Water
}

pub fn add_components(inst: &mut TileInstance, origin: GridPos) {}
