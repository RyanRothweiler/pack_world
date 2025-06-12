use crate::{pack::*, pack::*, pack_shop_signals::*, state::assets::*};
use gengar_engine::{
    collisions::*,
    input::*,
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
    Idle,
    Hidden,
    Hover,
    Selected,
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
        let mut ret = Self {
            hover_time: 0.0,
            rotation: VecThreeFloat::new_zero(),
            scale: 0.0,
            rot_time: 0.0,

            state: PackShopDisplayState::Idle,
        };

        ret.set_state(PackShopDisplayState::Idle);

        ret
    }

    pub fn set_state(&mut self, new_state: PackShopDisplayState) {
        self.state = new_state;

        /*
        match new_state {
            PackShopDisplayState::Idle => {
                self.target_scale = 1.0;
            }
            PackShopDisplayState::Hidden => {
                self.target_scale = 0.0;
            }
            PackShopDisplayState::Hover => {
                self.target_scale = 1.5;
            }
            PackShopDisplayState::Selected => {
                self.target_scale = 1.2;
            }
        }
        */
    }

    pub fn update(
        &mut self,
        pack_id: PackID,
        self_selected: bool,
        something_selected: bool,
        mouse_left: &ButtonState,
        mouse_world: VecThreeFloat,
        assets: &Assets,
        render_system: &mut RenderSystem,
    ) -> Vec<PackShopSignals> {
        let mut ret: Vec<PackShopSignals> = vec![];

        let pack_info = pack_id.get_pack_info();

        let mut hovering = point_within_circle(
            VecTwo::new(mouse_world.x, mouse_world.z),
            VecTwo::new(pack_info.shop_position.x, pack_info.shop_position.z),
            3.0,
        );

        match self.state {
            PackShopDisplayState::Idle => {
                if hovering {
                    ret.push(PackShopSignals::Hover { pack_id });
                }
            }
            PackShopDisplayState::Selected => {
                /*
                if !hovering {
                    ret.push(PackShopSignals::Idle { pack_id });
                } else if !mouse_left.pressing {
                    ret.push(PackShopSignals::Hover { pack_id });
                }
                */
            }
            PackShopDisplayState::Hidden => {}
            PackShopDisplayState::Hover => {
                if !hovering {
                    ret.push(PackShopSignals::Idle { pack_id });
                } else if mouse_left.pressing {
                    ret.push(PackShopSignals::Select { pack_id });
                }
            }
        }

        let scale_target = match self.state {
            PackShopDisplayState::Hidden => 0.0,
            PackShopDisplayState::Hover => 1.5,
            PackShopDisplayState::Selected => 1.2,
            PackShopDisplayState::Idle => 1.0,
        };

        let rot_max = match self.state {
            PackShopDisplayState::Hover | PackShopDisplayState::Selected => 0.05,
            _ => 0.45,
        };

        /*
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
        */

        self.scale = gengar_engine::math::lerp(self.scale, scale_target, 0.35);

        let target_rot = VecThreeFloat::new(
            f64::sin(self.rot_time) * rot_max,
            -90.0_f64.to_radians() + (f64::sin(self.rot_time + 2.0) * rot_max),
            -70.0_f64.to_radians() + (f64::sin(self.rot_time + 1.0) * rot_max),
        );
        self.rotation = VecThreeFloat::lerp(self.rotation, target_rot, 0.1);

        if self.scale > 0.01 {
            self.render(pack_id, assets, render_system);
        }

        ret
    }

    pub fn render(&mut self, pack_id: PackID, assets: &Assets, render_system: &mut RenderSystem) {
        self.rot_time += 0.04;

        let mut trans = Transform::new();
        trans.local_scale = VecThreeFloat::new(self.scale, self.scale, self.scale);

        trans.local_position = pack_id.get_pack_info().shop_position;
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
