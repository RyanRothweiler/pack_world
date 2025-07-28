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

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| {
    let mod_positions: Vec<GridPos> = GridPos::new(0, 0).to_adjacents_iter().collect();

    TileDefinition {
        title: "Spring",
        description: "Doubles the drops of all adjacent tiles.",
        world_layer: WorldLayer::Floor,
        footprint: vec![GridPos::new(0, 0)],
        placing_draw_footprint: false,

        placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],
        placement_global_mod: vec![GlobalMod::new(GlobalModKind::DropCount(2.0), mod_positions)],

        new_instance: new_instance,
    }
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Spring, grid_pos, TileMethods::Spring);
    inst
}
