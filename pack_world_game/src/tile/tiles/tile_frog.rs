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

pub const DEF: TileDefinition<'static> = TileDefinition {
    title: "Frog",
    description: "Must be placed in tall grass. Drops potion resourcs.",
    world_layer: WorldLayer::Walker,
};

const HARVEST_SECONDS: f64 = 10800.0;
const MOVE_SPEED: f64 = 0.5;

#[derive(Debug)]
pub struct TileFrog {
    harvest_timer: HarvestTimer,

    curr_world_pos: VecTwo,
    target_grid_offset: GridPos,
}

impl TileFrog {
    pub fn new_methods(pos: GridPos) -> TileMethods {
        TileMethods::Frog(TileFrog {
            harvest_timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Frog),
            target_grid_offset: GridPos::new(1, 1),
            curr_world_pos: grid_to_world(&pos),
        })
    }

    pub fn can_place(pos: GridPos, world: &World) -> bool {
        if !world.pos_valid(pos) {
            return false;
        }

        if !world.cell_contains_type(pos, TileType::Grass) {
            return false;
        }

        true
    }

    pub fn update(
        &mut self,
        origin: GridPos,
        time_step: f64,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        self.harvest_timer.inc(time_step);

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

    pub fn can_harvest(&self) -> bool {
        self.harvest_timer.can_harvest()
    }

    pub fn harvest(&mut self, grid_pos: GridPos, platform_api: &PlatformApi) -> Drop {
        self.harvest_timer.harvest(platform_api)
    }

    pub fn render_hover_info(
        &self,
        y_offset: f64,
        shader_color: Shader,
        render_pack: &mut RenderPack,
    ) {
        let base: VecTwo = VecTwo::new(450.0, 110.0 + y_offset);
        let r = Rect::new_top_size(base, 200.0, 10.0);

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
        let mut rotation: f64 = 0.0;
        if self.can_harvest() {
            rotation = f64::sin(rot_time) * 7.0;
        }

        draw_tile_world_pos(
            TileType::Frog,
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
        self.harvest_timer.save_file_write(key, save_file)?;

        Ok(())
    }

    pub fn save_file_load(
        key_parent: String,
        grid_pos: GridPos,
        save_file: &SaveFile,
    ) -> Result<TileMethods, Error> {
        let key = format!("{}.h", key_parent);
        let tm = TileMethods::Frog(TileFrog {
            harvest_timer: HarvestTimer::save_file_load(key, save_file)?,
            target_grid_offset: GridPos::new(1, 1),
            curr_world_pos: grid_to_world(&grid_pos),
        });

        Ok(tm)
    }
}
