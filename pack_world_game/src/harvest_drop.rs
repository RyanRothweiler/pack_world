use crate::{
    drop_table::*,
    item::*,
    state::{assets::*, inventory::*},
};
use elara_engine::{
    color::*,
    math::*,
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*, render_pack::*, shader::*, *},
    vectors::*,
};
use std::f32::consts::PI;

pub static CIRCLE_LEN: f64 = 0.01;
pub static GROUND_LEN: f64 = 0.035;
pub static INVENTORY_LEN: f64 = 0.02;

pub static DROP_RADIUS: f64 = 55.0;
pub static ICON_SIZE: f64 = 35.0;
pub static SIN_HEIGHT: f64 = 50.0;

pub struct HarvestDrop {
    pub drop: Drop,

    time: f64,
    origin: VecTwo,

    circle_target: VecTwo,

    pos: VecTwo,
    ground_pos: VecTwo,
}

impl HarvestDrop {
    pub fn new(drop: Drop, origin: VecTwo, platform_api: &PlatformApi) -> Self {
        // not a uniform randomness here
        let x: f64 = ((platform_api.rand)() * DROP_RADIUS * 2.0) - DROP_RADIUS;
        let y: f64 = ((platform_api.rand)() * DROP_RADIUS * 2.0) - DROP_RADIUS;

        Self {
            drop,
            origin,
            time: 0.0,

            pos: VecTwo::new(0.0, 0.0),
            ground_pos: VecTwo::new(0.0, 0.0),

            circle_target: origin + VecTwo::new(x, y),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.time >= CIRCLE_LEN + GROUND_LEN + INVENTORY_LEN
    }

    pub fn update_and_draw(
        &mut self,
        step: f64,
        shader: Shader,
        render_pack: &mut RenderPack,
        assets: &mut Assets,
    ) {
        self.time += step;

        if self.time < CIRCLE_LEN {
            // move to inventory

            let mut circle_t: f64 = (self.time / CIRCLE_LEN).clamp(0.0, 1.0);
            circle_t = ease_out_quint(circle_t);

            self.pos = VecTwo::lerp(self.origin, self.circle_target, circle_t);

            let h_rad = circle_t * std::f64::consts::PI;
            self.pos.y -= f64::sin(h_rad) * SIN_HEIGHT;

            self.ground_pos = self.pos;
        } else if self.time < GROUND_LEN {
            // leave on ground for a bit
        } else {
            // move to inventory

            let t_step = self.time - CIRCLE_LEN - GROUND_LEN;
            let mut t: f64 = (t_step / INVENTORY_LEN).clamp(0.0, 1.0);

            self.pos = VecTwo::lerp(self.ground_pos, VecTwo::new(0.0, 0.0), t);
        }

        // draw glow
        {
            let s = 2.5;
            let mut rect = Rect::new_size(ICON_SIZE * s, ICON_SIZE * s);
            rect.shrink(lerp(ICON_SIZE * 0.5, 0.0, self.time / CIRCLE_LEN));

            rect.set_center(self.pos);

            let mut mat = Material::new();
            mat.shader = Some(shader);
            mat.set_image(assets.image_glow.gl_id.unwrap());
            mat.set_color(Color::new(1.0, 1.0, 1.0, 0.4));

            render_pack
                .commands
                .push(RenderCommand::new_rect(&rect, -1.0, 0.0, &mat));
        }

        // draw
        {
            let mut rect = Rect::new_size(ICON_SIZE, ICON_SIZE);
            rect.shrink(lerp(ICON_SIZE * 0.5, 0.0, self.time / CIRCLE_LEN));

            rect.set_center(self.pos);

            let mut mat = Material::new();
            mat.shader = Some(shader);
            mat.set_image(assets.get_drop_icon(&self.drop.drop_type));
            mat.set_color(COLOR_WHITE);

            render_pack
                .commands
                .push(RenderCommand::new_rect_flipped(&rect, -1.0, 0.0, &mat));
        }
    }
}
