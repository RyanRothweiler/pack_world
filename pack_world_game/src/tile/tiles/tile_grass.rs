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

    build_methods: TileGrass::new_methods,
    add_components: TileGrass::add_components,
});

const HARVEST_SECONDS: f64 = 18.0;

#[derive(Debug)]
pub struct TileGrass {}

impl TileGrass {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        let mut ht = HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Grass);
        ht.add_length_condition(-0.1, WorldCondition::AdjacentTo(TileSnapshot::Water));
        ht.add_drop_condition(
            (EntryOutput::new_item(ItemType::Acorn, 1), 10.0),
            WorldCondition::AdjacentTo(TileSnapshot::OakTree { has_nest: true }),
        );

        TileMethods::Grass(TileGrass {})
    }

    pub fn add_components(inst: &mut TileInstance, origin: GridPos) {
        let mut ht = HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Grass);
        ht.add_length_condition(-0.1, WorldCondition::AdjacentTo(TileSnapshot::Water));
        ht.add_drop_condition(
            (EntryOutput::new_item(ItemType::Acorn, 1), 10.0),
            WorldCondition::AdjacentTo(TileSnapshot::OakTree { has_nest: true }),
        );

        inst.components
            .push(TileComponent::Harvestable { timer: ht });
    }

    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        vec![]
    }

    pub fn update_world_conditions(&mut self, grid_pos: GridPos, world_snapshot: &WorldSnapshot) {}

    pub fn render(
        &self,
        time_comp: &HarvestTimer,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        draw_tile(TileType::Dirt, 0.0, pos, shader_color, render_pack, assets);

        let mut rotation: f64 = 0.0;
        if time_comp.can_harvest() {
            rotation = f64::sin(rot_time) * 7.0;
        }

        draw_tile(
            TileType::Grass,
            rotation,
            pos,
            shader_color,
            render_pack,
            assets,
        );
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
        /*
        let tm = TileMethods::Grass(TileGrass {
            harvest_timer: HarvestTimer::save_file_load(key, save_file)?,
        });
        */

        todo!();
        // Ok(tm)
    }
}
