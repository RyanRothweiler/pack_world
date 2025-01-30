use std::f32::consts::PI;

use crate::state::{assets::*, inventory::*};
use gengar_engine::{
    color::*,
    math::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*, *},
    vectors::*,
};
use rand::prelude::*;

pub static DROP_RADIUS: f64 = 50.0;
pub static CIRCLE_TIME: f64 = 0.01;
pub static ICON_SIZE: f64 = 18.0;
pub static SIN_HEIGHT: f64 = 20.0;

pub struct HarvestDrop {
    pub item_type: ItemType,
    pub time: f64,
    pub origin: VecTwo,

    pub circle_target: VecTwo,
}

impl HarvestDrop {
    pub fn new(item_type: ItemType, origin: VecTwo) -> Self {
        // not a uniform randomness here
        let x: f64 = rand::random_range(-DROP_RADIUS..DROP_RADIUS);
        let y: f64 = rand::random_range(-DROP_RADIUS..DROP_RADIUS);

        Self {
            item_type,
            origin,
            time: 0.0,

            circle_target: origin + VecTwo::new(x, y),
        }
    }

    pub fn update_and_draw(
        &mut self,
        step: f64,
        shader: Shader,
        render_pack: &mut RenderPack,
        assets: &Assets,
    ) {
        self.time += step;

        let mut circle_t: f64 = (self.time / CIRCLE_TIME).clamp(0.0, 1.0);
        circle_t = eas_out_quint(circle_t);

        let mut pos = VecTwo::lerp(self.origin, self.circle_target, circle_t);

        let h_rad = circle_t * std::f64::consts::PI;
        pos.y -= f64::sin(h_rad) * SIN_HEIGHT;

        // draw
        {
            let mut rect = Rect::new_size(ICON_SIZE, ICON_SIZE);
            rect.set_center(pos);

            let mut mat = Material::new();
            mat.shader = Some(shader);
            mat.set_image(assets.get_item_icon(&self.item_type));
            mat.set_color(COLOR_WHITE);

            render_pack
                .commands
                .push(RenderCommand::new_rect(&rect, -1.0, 0.0, &mat));
        }
    }
}
