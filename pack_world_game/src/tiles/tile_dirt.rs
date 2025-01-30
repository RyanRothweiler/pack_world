use crate::{grid::*, tiles::*};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*},
    vectors::*,
};

pub struct TileDirt {}

impl TileMethods for TileDirt {
    fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        vec![]
    }

    fn can_harvest(&self) -> bool {
        false
    }

    fn harvest(&mut self, tile_pos: VecTwo) -> Vec<UpdateSignal> {
        vec![]
    }

    fn render_hover_info(&self, shader_color: Shader, render_pack: &mut RenderPack) {}
    fn render(
        &self,
        rot_time: f64,
        pos: &VecTwoInt,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        draw_tile(TileType::Dirt, 0.0, pos, shader_color, render_pack, assets);
    }
}

impl TileDirt {
    pub fn new() -> TileInstance {
        TileInstance {
            tile_type: TileType::Dirt,
            methods: Box::new(TileDirt {}),
        }
    }
}
