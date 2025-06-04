use crate::{
    drop_table::*,
    grid::*,
    save_file::*,
    state::{inventory::*, *},
    tile::{harvest_timer::*, *},
};
use gengar_engine::{
    color::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::*,
    vectors::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Newt",
    description: "Must be placed in water. Drops potion resources.",
    world_layer: WorldLayer::Walker,
    footprint: GridPos::new(0, 0).to_rect_iter(4, 4).collect(),
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Water)],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = 10800.0;

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Newt, grid_pos, TileMethods::Newt);

    inst.components.push(TileComponent::Harvestable {
        timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::SmallGold),
    });

    todo!("add third dimension");

    /*
    inst.components.push(TileComponent::Wander {
        state: WanderState {
            target_grid_offset: GridPos::new(1, 1),
            curr_world_pos: grid_to_world(&grid_pos),
        },
    });
    */

    inst
}
