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
    title: "Shrub",
    description: "Drops basic food.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    new_instance: new_instance,
});

const HARVEST_SECONDS: f64 = 40.0;

pub fn new_instance(grid_pos: GridPos) -> TileInstance {
    let mut inst = TileInstance::new(TileType::Shrub, grid_pos, TileMethods::Shrub);

    inst.components.push(TileComponent::Harvestable {
        timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Shrub),
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
    let key = format!("{}.h", key_parent);
    let tm = TileMethods::Shrub;

    Ok(tm)
}
