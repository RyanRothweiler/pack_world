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

    placement_constraints: vec![WorldCondition::ValidPosition()],

    build_methods: TileDirt::new_methods,
    add_components: TileDirt::add_components,
});

#[derive(Debug)]
pub struct TileDirt {}

impl TileDirt {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        TileMethods::Dirt(TileDirt {})
    }

    pub fn add_components(inst: &mut TileInstance, origin: GridPos) {}
}
