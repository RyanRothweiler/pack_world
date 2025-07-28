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

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Goblin",
    description: "Placed in cave. Doubles drops.",
    world_layer: WorldLayer::Walker,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Cave)],
    placement_global_mod: vec![GlobalMod::new(
        GlobalModKind::DropCount(2.0),
        vec![GridPos::new(0, 0)],
    )],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Goblin, grid_pos, TileMethods::Goblin);
    inst
}
