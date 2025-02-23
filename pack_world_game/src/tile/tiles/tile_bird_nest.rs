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

pub const TITLE: &str = "Bird Nest";

#[derive(Debug)]
pub struct TileBirdNest {}

impl TileBirdNest {
    pub fn new_methods() -> TileMethods {
        TileMethods::BirdNest(TileBirdNest {})
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
        // render tree
        {
            let mut r = Rect::new_square(GRID_SIZE);
            let pos_world = grid_to_world(pos) + VecTwo::new(GRID_SIZE * 0.5, GRID_SIZE * 0.1);
            r.set_center(pos_world);

            let mut mat = Material::new();
            mat.shader = Some(shader_color);

            mat.uniforms.insert(
                "tex".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: assets.get_tile_icon(&TileType::BirdNest),
                    texture_slot: 0,
                }),
            );

            mat.uniforms.insert(
                "color".to_string(),
                UniformData::VecFour(COLOR_WHITE.into()),
            );

            render_pack
                .commands
                .push(RenderCommand::new_rect(&r, -1.0, 0.0, &mat));
        }
    }
}
