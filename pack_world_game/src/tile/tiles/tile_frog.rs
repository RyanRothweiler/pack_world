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

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::TallGrass)],

    build_methods: TileFrog::new_methods,
    add_components: TileFrog::add_components,
});

const HARVEST_SECONDS: f64 = 10800.0;

#[derive(Debug)]
pub struct TileFrog {}

impl TileFrog {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        TileMethods::Frog(TileFrog {})
    }

    pub fn add_components(inst: &mut TileInstance, origin: GridPos) {
        inst.components.push(TileComponent::Harvestable {
            timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Frog),
        });

        inst.components.push(TileComponent::Wander {
            state: WanderState {
                target_grid_offset: GridPos::new(1, 1),
                curr_world_pos: grid_to_world(&origin),
            },
        });
    }

    pub fn render(
        &self,
        time_comp: &HarvestTimer,
        wander_comp: &WanderState,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        let mut rotation: f64 = 0.0;
        if time_comp.can_harvest() {
            rotation = f64::sin(rot_time) * 7.0;
        }

        draw_tile_world_pos(
            TileType::Frog,
            rotation,
            &wander_comp.curr_world_pos,
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

    pub fn save_file_load(
        key_parent: String,
        grid_pos: GridPos,
        save_file: &SaveFile,
    ) -> Result<TileMethods, Error> {
        let key = format!("{}.h", key_parent);
        let tm = TileMethods::Frog(TileFrog {
            // harvest_timer: HarvestTimer::save_file_load(key, save_file)?,
            // target_grid_offset: GridPos::new(1, 1),
            // curr_world_pos: grid_to_world(&grid_pos),
        });

        Ok(tm)
    }
}
