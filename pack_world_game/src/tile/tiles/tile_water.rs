use crate::{grid::*, tile::*};
use gengar_engine::{
    color::*,
    rect::*,
    render::{material::*, render_command::*},
    vectors::*,
};

pub const TITLE: &str = "Water";
pub const DESC: &str = "Placed on empty space. Creates water for other tiles.";

#[derive(Debug)]
pub struct TileWater {}

impl TileWater {
    pub fn new_methods() -> TileMethods {
        TileMethods::Water(TileWater {})
    }

    pub fn can_place(pos: GridPos, world: &World) -> bool {
        if !world.pos_valid(pos) {
            return false;
        }

        true
    }

    pub fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        vec![]
    }

    pub fn can_harvest(&self) -> bool {
        false
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
        draw_tile(TileType::Water, 0.0, pos, shader_color, render_pack, assets);
    }
}
