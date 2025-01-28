use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*},
    ui::*,
};

use crate::{grid::*, state::*, tiles::*};

const HARVEST_SECONDS: f64 = 50.0;

pub struct TileGrass {
    pub time: f64,
}

impl TileMethods for TileGrass {
    fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        self.time += time_step;
        self.time = self.time.clamp(0.0, HARVEST_SECONDS);

        vec![]
    }

    fn can_harvest(&self) -> bool {
        self.time >= HARVEST_SECONDS
    }

    fn harvest(&mut self) -> Vec<UpdateSignal> {
        return vec![UpdateSignal::GiveItem {
            item_type: ItemType::DirtClod,
            count: 1,
        }];
    }

    fn render_hover_info(&self, shader_color: Shader, render_pack: &mut RenderPack) {
        let base: VecTwo = VecTwo::new(450.0, 120.0);
        let r = Rect::new_top_size(base, 200.0, 10.0);

        let prog = self.time / HARVEST_SECONDS;

        draw_progress_bar(prog, &r, shader_color, render_pack);
    }

    fn render(
        &self,
        rot_time: f64,
        pos: &VecTwoInt,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
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
}

impl TileGrass {
    pub fn new() -> TileInstance {
        TileInstance {
            tile_type: TileType::Grass,
            methods: Box::new(TileGrass { time: 0.0 }),
        }
    }
}
