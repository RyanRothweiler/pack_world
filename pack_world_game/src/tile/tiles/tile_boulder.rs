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
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Boulder",
    description: "Drops basic resources.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    build_methods: new_methods,
    add_components: add_components,
});

const HARVEST_SECONDS: f64 = 120.0;

pub fn new_methods(origin: GridPos) -> TileMethods {
    TileMethods::Boulder
}

fn add_components(inst: &mut TileInstance, origin: GridPos) {
    inst.components.push(TileComponent::Harvestable {
        timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Boulder),
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
    todo!();

    let key = format!("{}.h", key_parent);
    // let tm = TileMethods::Boulder(TileBoulder {});

    // Ok(tm)
}
