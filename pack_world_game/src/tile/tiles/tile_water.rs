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

    build_methods: TileWater::new_methods,
    add_components: TileWater::add_components,
});

#[derive(Debug)]
pub struct TileWater {}

impl TileWater {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        TileMethods::Water(TileWater {})
    }

    pub fn add_components(inst: &mut TileInstance, origin: GridPos) {}

    pub fn render(
        &self,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        draw_tile(TileType::Water, 0.0, pos, shader_color, render_pack, assets);
    }
}
