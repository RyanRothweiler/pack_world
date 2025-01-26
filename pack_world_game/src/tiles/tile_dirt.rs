use crate::tiles::*;

pub struct TileDirt {}

impl TileMethods for TileDirt {
    fn update(&mut self, time_step: f64) -> Vec<UpdateSignal> {
        vec![]
    }

    fn can_harvest(&self) -> bool {
        false
    }

    fn harvest(&mut self) -> Vec<UpdateSignal> {
        vec![]
    }

    fn render_hover_info(&self, shader_color: Shader, render_pack: &mut RenderPack) {}
}

impl TileDirt {
    pub fn new() -> TileInstance {
        TileInstance {
            tile_type: TileType::Dirt,
            methods: Box::new(TileDirt {}),
        }
    }
}
