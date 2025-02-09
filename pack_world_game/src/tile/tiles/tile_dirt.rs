use crate::{grid::*, tile::*};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*},
    vectors::*,
};

#[derive(Debug)]
pub struct TileDirt {}

impl TileDirt {
    pub fn new(placement_type: TilePlacementType) -> TileInstance {
        TileInstance {
            placement_type,
            tile_type: TileType::Dirt,
            methods: TileMethods::Dirt(TileDirt {}),
        }
    }
}

impl TileDirt {
    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        vec![]
    }

    pub fn can_harvest(&self) -> bool {
        false
    }

    pub fn harvest(&mut self, grid_pos: GridPos) -> Vec<UpdateSignal> {
        vec![]
    }

    pub fn render_hover_info(&self, shader_color: Shader, render_pack: &mut RenderPack) {}

    pub fn render(
        &self,
        rot_time: f64,
        pos: &GridPos,
        shader_color: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        draw_tile(TileType::Dirt, 0.0, pos, shader_color, render_pack, assets);
    }
}
