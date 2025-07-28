use crate::{
    drop_table::*,
    grid::*,
    save_file::*,
    state::{inventory::*, *},
    tile::*,
};
use elara_engine::{
    color::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    time::*,
    ui::*,
    vectors::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Newt",
    description: "Must be placed in water. Drops potion resources.",
    world_layer: WorldLayer::Walker,
    footprint: GridPos::new(0, 0).to_rect_iter(4, 4).collect(),
    placing_draw_footprint: true,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Water)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Newt, grid_pos, TileMethods::Newt);

    inst.comp_harvest = Some(TileCompHarvest::new(
        Time::new(TimeUnit::Hours(3.0)),
        FixedTableID::Newt,
        false,
    ));

    inst.comp_wander = Some(TileCompWander {
        range: 4,
        target_grid_offset: GridPos::new(1, 1),
        curr_world_pos: grid_to_world(&grid_pos),
    });

    inst
}
