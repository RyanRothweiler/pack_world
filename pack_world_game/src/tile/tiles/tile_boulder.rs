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

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = 120.0;

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Boulder, grid_pos, TileMethods::Boulder);

    inst.components.push(TileComponent::Harvestable {
        timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Boulder),
    });

    inst
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
