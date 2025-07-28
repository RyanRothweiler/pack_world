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
    title: "Mud Chicken",
    description: "Automatically harvests all tiles within its area. Dies after 3 days.",
    world_layer: WorldLayer::Walker,
    footprint: GridPos::new(0, 0).to_rect_iter(4, 4).collect(),
    placing_draw_footprint: true,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],
    new_instance: new_instance,
    placement_global_mod: vec![],
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::MudChicken, grid_pos, TileMethods::MudChicken);

    inst.comp_wander = Some(TileCompWander {
        range: 4,
        target_grid_offset: GridPos::new(1, 1),
        curr_world_pos: grid_to_world(&grid_pos),
    });

    let harvest_time = Time::new(TimeUnit::Seconds(10.0));
    let positions: Vec<GridPos> = DEF.footprint.clone();
    inst.comp_harvest_others = Some(TileCompHarvestOthers::new(harvest_time, positions));

    inst.comp_auto_death = Some(TileCompAutoDeath::new(Time::new(TimeUnit::Days(3.0))));

    inst
}
