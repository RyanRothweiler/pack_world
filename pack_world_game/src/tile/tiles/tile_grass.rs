use crate::{
    drop_table::*,
    grid::*,
    item::*,
    save_file::*,
    state::{inventory::*, *},
    tile::*,
    world::*,
};
use gengar_engine::{
    color::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    time::*,
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Grass",
    description: "Drops basic resources. Reduce cooldown by 10% if adjacent to water.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],
    placing_draw_footprint: false,

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],
    placement_global_mod: vec![],

    new_instance: new_instance,
});

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Grass, grid_pos, TileMethods::Grass);

    let mut ht = TileCompHarvest::new(
        Time::new(TimeUnit::Seconds(18.0)),
        FixedTableID::Grass,
        false,
    );
    ht.add_length_condition(-0.1, WorldCondition::AdjacentTo(TileSnapshot::Water));
    ht.add_drop_condition(
        (EntryOutput::new_item(ItemType::Acorn, 1), 10.0),
        WorldCondition::AdjacentTo(TileSnapshot::OakTree { has_nest: true }),
    );

    inst.comp_harvest = Some(ht);

    inst
}
