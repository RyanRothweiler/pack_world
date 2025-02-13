use crate::{
    drop_table::*,
    grid::*,
    state::{inventory::*, *},
    tile::{harvest_timer::*, *},
};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::*,
};

pub const TITLE: &str = "Oak Tree";

const HARVEST_SECONDS: f64 = 360.0;

pub struct TileOakTree {
    harvest_timer: HarvestTimer,
}

impl TileOakTree {
    pub fn new(grid_pos: GridPos) -> TileInstance {
        TileInstance {
            grid_pos,
            tile_type: TileType::OakTree,
            methods: TileMethods::OakTree(TileOakTree {
                harvest_timer: HarvestTimer::new(HARVEST_SECONDS, FixedTableID::OakTree),
            }),
        }
    }
}

impl TileOakTree {
    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        self.harvest_timer.inc(time_step);
        vec![]
    }

    pub fn can_harvest(&self) -> bool {
        self.harvest_timer.can_harvest()
    }

    pub fn harvest(&mut self, grid_pos: GridPos) -> Vec<UpdateSignal> {
        self.harvest_timer.harvest(grid_pos)
    }

    pub fn render_hover_info(&self, shader_color: Shader, render_pack: &mut RenderPack) {
        let base: VecTwo = VecTwo::new(450.0, 120.0);
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
        draw_tile(TileType::Dirt, 0.0, pos, shader_color, render_pack, assets);

        let mut rotation: f64 = 0.0;
        if self.can_harvest() {
            rotation = f64::sin(rot_time) * 7.0;
        }

        // render tree
        {
            let mut r = Rect::new_square(GRID_SIZE * 2.0);
            let pos_world = grid_to_world(pos) + VecTwo::new(GRID_SIZE * 0.5, GRID_SIZE * 0.5);
            r.set_center(pos_world);

            let mut mat = Material::new();
            mat.shader = Some(shader_color);

            mat.uniforms.insert(
                "tex".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: assets.get_tile_icon(&TileType::OakTree),
                    texture_slot: 0,
                }),
            );

            mat.uniforms.insert(
                "color".to_string(),
                UniformData::VecFour(COLOR_WHITE.into()),
            );

            render_pack
                .commands
                .push(RenderCommand::new_rect(&r, -1.0, rotation, &mat));
        }
    }
}
