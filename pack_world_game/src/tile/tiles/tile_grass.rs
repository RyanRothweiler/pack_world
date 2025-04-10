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
    title: "Grass",
    description: "Drops basic resources. Reduce cooldown by 10% if adjacent to water.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    build_methods: TileGrass::new_methods,

    components: vec![],
});

const HARVEST_SECONDS: f64 = 18.0;

#[derive(Debug)]
pub struct TileGrass {
    pub harvest_timer: HarvestTimer,
}

impl TileGrass {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        let mut ht = HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Grass);
        ht.add_length_condition(-0.1, WorldCondition::AdjacentTo(TileSnapshot::Water));
        ht.add_drop_condition(
            (EntryOutput::new_item(ItemType::Acorn, 1), 10.0),
            WorldCondition::AdjacentTo(TileSnapshot::OakTree { has_nest: true }),
        );

        TileMethods::Grass(TileGrass { harvest_timer: ht })
    }

    pub fn build_instance(origin: GridPos) -> TileInstance {
        let methods = (DEF.build_methods)(origin);
        let mut inst = TileInstance::new(TileType::Grass, origin, methods);

        inst
    }

    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        self.harvest_timer.inc(time_step);
        vec![]
    }

    pub fn update_world_conditions(&mut self, grid_pos: GridPos, world_snapshot: &WorldSnapshot) {
        self.harvest_timer
            .update_world_conditions(grid_pos, world_snapshot);
    }

    pub fn can_harvest(&self) -> bool {
        self.harvest_timer.can_harvest()
    }

    pub fn harvest(
        &mut self,
        grid_pos: GridPos,
        world_snapshot: &WorldSnapshot,
        platform_api: &PlatformApi,
    ) -> Drop {
        return self.harvest_timer.harvest(platform_api);
    }

    pub fn render_hover_info(
        &self,
        y_offset: f64,
        shader_color: Shader,
        render_pack: &mut RenderPack,
    ) {
        let base: VecTwo = VecTwo::new(450.0, 110.0 + y_offset);
        let mut r = Rect::new_top_size(base, 200.0, 10.0);

        draw_progress_bar(
            self.harvest_timer.percent_done(),
            &r,
            shader_color,
            render_pack,
        );
    }

    pub fn render(
        &self,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        draw_tile(TileType::Dirt, 0.0, pos, shader_color, render_pack, assets);

        let mut rotation: f64 = 0.0;
        if self.can_harvest() {
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
        self.harvest_timer.save_file_write(key, save_file)?;

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<TileMethods, Error> {
        let key = format!("{}.h", key_parent);
        let tm = TileMethods::Grass(TileGrass {
            harvest_timer: HarvestTimer::save_file_load(key, save_file)?,
        });

        Ok(tm)
    }
}

mod test {
    use super::*;

    #[test]
    fn adj_water() {
        let mut world = World::new();

        let _ = world.insert_tile(GridPos::new(0, 0), TileType::Grass);
        let _ = world.insert_tile(GridPos::new(1, 0), TileType::Water);

        let geid = EntityID { id: 0 };

        {
            let mut grass_inst = world.get_entity_mut(&geid);

            match &mut grass_inst.methods {
                TileMethods::Grass(state) => {
                    state.update(10.0);

                    assert_eq!(state.harvest_timer.length(), HARVEST_SECONDS * 0.9);
                }
                _ => panic!("Invalid tile type"),
            }
        }
    }
}
