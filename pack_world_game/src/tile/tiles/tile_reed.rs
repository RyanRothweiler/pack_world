#![allow(dead_code)]

use crate::{
    drop_table::*,
    grid::*,
    item::*,
    save_file::*,
    state::{inventory::*, *},
    tile::{harvest_timer::*, *},
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
    title: "Reed",
    description: "Must be placed in mud. Drops potion resources",
    world_layer: WorldLayer::Planted,
    footprint: vec![GridPos::new(0, 0)],

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::MudPit)],

    build_methods: new_methods,
    add_components: add_components,
});

const HARVEST_SECONDS: f64 = 20.0;

pub fn new_methods(origin: GridPos) -> TileMethods {
    TileMethods::Reed
}

pub fn add_components(inst: &mut TileInstance, origin: GridPos) {
    inst.components.push(TileComponent::Harvestable {
        timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::OakTree),
    });
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
    let tm = TileMethods::Reed;

    Ok(tm)
}
