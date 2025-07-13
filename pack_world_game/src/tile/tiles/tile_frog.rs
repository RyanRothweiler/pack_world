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
    title: "Frog",
    description: "Must be placed in tall grass. Drops potion resourcs.",
    world_layer: WorldLayer::Walker,
    footprint: GridPos::new(0, 0).to_rect_iter(4, 4).collect(),
    placing_draw_footprint: true,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::TallGrass)],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Frog, grid_pos, TileMethods::Frog);

    inst.components.push(TileComponent::Harvestable {
        timer: HarvestTimer::new(10800.0, FixedTableID::Frog, false),
    });

    inst.components.push(TileComponent::Wander {
        state: WanderState {
            target_grid_offset: GridPos::new(1, 1),
            curr_world_pos: grid_to_world(&grid_pos),
        },
    });

    inst
}
