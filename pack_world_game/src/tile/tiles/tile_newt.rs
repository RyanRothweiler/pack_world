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
    title: "Newt",
    description: "Must be placed in water. Drops potion resources.",
    world_layer: WorldLayer::Walker,
    footprint: GridPos::new(0, 0).to_rect_iter(4, 4).collect(),

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Water)],

    build_methods: TileNewt::new_methods,
    add_components: TileNewt::add_components,
});

const HARVEST_SECONDS: f64 = 10800.0;
const MOVE_SPEED: f64 = 0.5;

#[derive(Debug)]
pub struct TileNewt {
    curr_world_pos: VecTwo,
    target_grid_offset: GridPos,
}

impl TileNewt {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        TileMethods::Newt(TileNewt {
            target_grid_offset: GridPos::new(1, 1),
            curr_world_pos: grid_to_world(&origin),
        })
    }

    pub fn add_components(inst: &mut TileInstance, origin: GridPos) {
        inst.components.push(TileComponent::Harvestable {
            timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::SmallGold),
        });
    }

    pub fn update(
        &mut self,
        origin: GridPos,
        time_step: f64,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        // move frog around
        {
            let target_world = grid_to_world(&(origin + self.target_grid_offset));
            let mut dir = target_world - self.curr_world_pos;
            dir.normalize();

            self.curr_world_pos = self.curr_world_pos + (dir * MOVE_SPEED);

            if self.curr_world_pos.dist_from(target_world) < 1.0 {
                self.target_grid_offset.x = ((platform_api.rand)() * 4.0) as i32;
                self.target_grid_offset.y = ((platform_api.rand)() * 4.0) as i32;
            }
        }

        vec![]
    }

    pub fn render_hover_info(
        &self,
        time_comp: &HarvestTimer,
        y_offset: f64,
        shader_color: Shader,
        render_pack: &mut RenderPack,
    ) {
        let base: VecTwo = VecTwo::new(450.0, 110.0 + y_offset);
        let r = Rect::new_top_size(base, 200.0, 10.0);

        draw_progress_bar(time_comp.percent_done(), &r, shader_color, render_pack);
    }

    pub fn render(
        &self,
        time_comp: &HarvestTimer,
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
            TileType::Newt,
            rotation,
            &self.curr_world_pos,
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
        let tm = TileMethods::Newt(TileNewt {
            // harvest_timer: HarvestTimer::save_file_load(key, save_file)?,
            target_grid_offset: GridPos::new(1, 1),
            curr_world_pos: grid_to_world(&grid_pos),
        });

        Ok(tm)
    }
}
