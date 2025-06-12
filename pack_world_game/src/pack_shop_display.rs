use crate::{pack::*, pack::*, state::assets::*};
use gengar_engine::{
    matricies::*,
    state::render_system::*,
    transform::*,
    {
        render::{render_command::*, render_pack::*, shader::*, *},
        vectors::*,
    },
};

#[derive(Copy, Clone)]
pub enum PackShopDisplayState {
    Exiting,
    Incoming,
    Selected,
    Hovering,
    Idle,
    Hidden,
}

#[derive(Copy, Clone)]
pub struct PackShopDisplay {
    pub hover_time: f64,
    pub rotation: VecThreeFloat,
    pub scale: f64,

    pub rot_time: f64,
    state: PackShopDisplayState,
}

impl PackShopDisplay {
    pub fn new() -> Self {
        Self {
            hover_time: 0.0,
            rotation: VecThreeFloat::new_zero(),
            scale: 0.0,
            rot_time: 0.0,
            state: PackShopDisplayState::Idle,
        }
    }

    pub fn set_state(&mut self, new_state: PackShopDisplayState) {
        self.state = new_state;
    }

    pub fn update(
        &mut self,
        pack_id: PackID,
        hovering: bool,
        self_selected: bool,
        something_selected: bool,
        mouse_down: bool,
        world_origin: VecThreeFloat,
        assets: &Assets,
        render_system: &mut RenderSystem,
    ) {
        let hover_scale: f64 = 1.5;
        let click_scale: f64 = 1.2;
        let mut scale_target: f64 = 1.0;

        let hover_speed: f64 = 35.0;

        if hovering {
            scale_target = hover_scale;
            if mouse_down {
                scale_target = click_scale;
            }
        }

        if self_selected {
            scale_target = hover_scale;
        }

        if something_selected && !self_selected {
            scale_target = 0.0;
        }

        self.scale = gengar_engine::math::lerp(self.scale, scale_target, 0.35);

        let mut rot_max = 0.45;
        if hovering || self_selected {
            rot_max = 0.05;
        }

        let target_rot = VecThreeFloat::new(
            f64::sin(self.rot_time) * rot_max,
            -90.0_f64.to_radians() + (f64::sin(self.rot_time + 2.0) * rot_max),
            -70.0_f64.to_radians() + (f64::sin(self.rot_time + 1.0) * rot_max),
        );
        self.rotation = VecThreeFloat::lerp(self.rotation, target_rot, 0.1);

        if self.scale > 0.01 {
            self.render(pack_id, world_origin, assets, render_system);
        }
    }

    pub fn render(
        &mut self,
        pack_id: PackID,
        world_origin: VecThreeFloat,
        assets: &Assets,
        render_system: &mut RenderSystem,
    ) {
        self.rot_time += 0.04;

        let mut trans = Transform::new();
        trans.local_scale = VecThreeFloat::new(self.scale, self.scale, self.scale);

        trans.local_position = world_origin;
        trans.local_rotation = self.rotation;
        trans.update_global_matrix(&M44::new_identity());

        let mut mat = assets.get_pack_material(pack_id).clone();

        mat.uniforms.insert(
            "ambientColor".to_string(),
            UniformData::VecThree(VecThreeFloat::new_zero()),
        );

        render_system.add_command(
            RenderCommand::new_model(
                &trans,
                assets.asset_library.get_model(&pack_id.to_string_id()),
                &mat,
            ),
            RenderPackID::Shop,
        );
    }
}
