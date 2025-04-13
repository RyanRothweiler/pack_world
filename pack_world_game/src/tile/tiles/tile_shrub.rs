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

    build_methods: TileShrub::new_methods,
    add_components: TileShrub::add_components,
});

const HARVEST_SECONDS: f64 = 40.0;

#[derive(Debug)]
pub struct TileShrub {}

impl TileShrub {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        TileMethods::Shrub(TileShrub {})
    }

    pub fn add_components(inst: &mut TileInstance, origin: GridPos) {
        inst.components.push(TileComponent::Harvestable {
            timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Shrub),
        });
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let key = format!("{}.h", key_parent);
        // self.harvest_timer.save_file_write(key, save_file)?;

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<TileMethods, Error> {
        let key = format!("{}.h", key_parent);
        let tm = TileMethods::Shrub(TileShrub {
            // harvest_timer: HarvestTimer::save_file_load(key, save_file)?,
        });

        Ok(tm)
    }
}
