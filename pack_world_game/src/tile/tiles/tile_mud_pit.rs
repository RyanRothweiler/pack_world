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
    time::*,
    ui::*,
};
use std::sync::LazyLock;

pub static DEF: LazyLock<TileDefinition> = LazyLock::new(|| TileDefinition {
    title: "Mud Pit",
    description: "Drops mud babies and ground tiles.",
    world_layer: WorldLayer::Floor,
    footprint: vec![GridPos::new(0, 0)],

    placement_constraints: vec![WorldCondition::OriginContains(TileSnapshot::Dirt)],

    build_methods: TileMudPit::new_methods,
    add_components: TileMudPit::add_components,
});

const HARVEST_SECONDS: f64 = minutes_to_seconds(4.0);

#[derive(Debug)]
pub struct TileMudPit {}

impl TileMudPit {
    pub fn new_methods(origin: GridPos) -> TileMethods {
        TileMethods::MudPit(TileMudPit {})
    }

    pub fn add_components(inst: &mut TileInstance, origin: GridPos) {
        inst.components.push(TileComponent::Harvestable {
            timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::Frog),
        });
    }

    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
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
        draw_tile(TileType::Dirt, 0.0, pos, shader_color, render_pack, assets);

        let mut rotation: f64 = 0.0;
        if time_comp.can_harvest() {
            rotation = f64::sin(rot_time) * 7.0;
        }

        draw_tile(
            TileType::MudPit,
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
        let tm = TileMethods::MudPit(TileMudPit {
            // harvest_timer: HarvestTimer::save_file_load(key, save_file)?,
        });

        Ok(tm)
    }
}
