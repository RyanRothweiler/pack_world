#![allow(dead_code)]

use crate::{
    drop_table::*,
    grid::*,
    item::*,
    save_file::*,
    state::{inventory::*, *},
    tile::{harvest_timer::*, tile_methods::tile_component::TileComponent, *},
    world::*,
};
use gengar_engine::{
    color::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Grass",
    description: "Drops basic resources. Reduce cooldown by 10% if adjacent to water.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    build_methods: new_methods,
    add_components: add_components,
});

const HARVEST_SECONDS: f64 = 18.0;

pub fn new_methods(origin: GridPos) -> TileMethods {
    let mut ht = HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Grass);
    ht.add_length_condition(-0.1, WorldCondition::AdjacentTo(TileSnapshot::Water));
    ht.add_drop_condition(
        (EntryOutput::new_item(ItemType::Acorn, 1), 10.0),
        WorldCondition::AdjacentTo(TileSnapshot::OakTree { has_nest: true }),
    );

    TileMethods::Grass
}

fn add_components(inst: &mut TileInstance, origin: GridPos) {
    let mut ht = HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Grass);
    ht.add_length_condition(-0.1, WorldCondition::AdjacentTo(TileSnapshot::Water));
    ht.add_drop_condition(
        (EntryOutput::new_item(ItemType::Acorn, 1), 10.0),
        WorldCondition::AdjacentTo(TileSnapshot::OakTree { has_nest: true }),
    );

    inst.components
        .push(TileComponent::Harvestable { timer: ht });
}

/*
pub fn save_file_write(&self, key_parent: String, save_file: &mut SaveFile) -> Result<(), Error> {
    let key = format!("{}.h", key_parent);
    // self.harvest_timer.save_file_write(key, save_file)?;

    Ok(())
}
*/

pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<TileMethods, Error> {
    let key = format!("{}.h", key_parent);
    /*
    let tm = TileMethods::Grass(TileGrass {
        harvest_timer: HarvestTimer::save_file_load(key, save_file)?,
    });
    */

    todo!();
    // Ok(tm)
}
