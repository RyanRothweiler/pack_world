use crate::{pack::*, pack_shop_display::PackShopDisplayState, state::assets::*};
use elara_engine::{
    math::*,
    matricies::*,
    platform_api::*,
    render::{render_command::*, render_pack::*, shader::*, *},
    state::render_system::*,
    transform::*,
    vectors::*,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PackInstanceState {
    Idle,
    Exiting,
    Opening,

    /// This one is dead and needs to be removed.
    Remove,
}

/// Handles the display of the pack itself.
/// This is different from the node because packs are consumed and created when opened.
/// The node is always unchanged.
pub struct PackRenderInstance {
    rotation: VecThreeFloat,

    state: PackInstanceState,
    position_offset: VecThreeFloat,

    scale: f64,
    rot_time: f64,
    time: f64,
}

impl PackRenderInstance {
    pub fn new() -> Self {
        Self {
            state: PackInstanceState::Idle,
            position_offset: VecThreeFloat::new_zero(),

            rotation: VecThreeFloat::new_zero(),
            scale: 0.0,
            rot_time: 0.0,
            time: 0.0,
        }
    }

    pub fn change_state(&mut self, new_state: PackInstanceState) {
        self.state = new_state;
        self.time = 0.0;
    }

    pub fn update_and_render(
        &mut self,
        hovering: bool,
        can_afford: bool,
        pack_id: PackID,
        node_state: PackShopDisplayState,
        render_system: &mut RenderSystem,
        assets: &Assets,
        platform_api: &PlatformApi,
    ) {
        let pack_info = pack_id.get_pack_info();

        match self.state {
            PackInstanceState::Exiting => {
                /*
                self.position_offset = VecThreeFloat::lerp(
                    VecThreeFloat::new_zero(),
                    VecThreeFloat::new(30.0, 0.0, 0.0),
                    ease_in_quint(self.time.clamp(0.0, 1.0)),
                );
                */
                self.scale = elara_engine::math::lerp(
                    self.scale,
                    0.0,
                    ease_out_quint(self.time.clamp(0.0, 1.0)),
                );

                self.time += 0.02;

                if self.time > 1.5 {
                    self.change_state(PackInstanceState::Remove);
                }
            }
            PackInstanceState::Idle | PackInstanceState::Opening => {
                let mut scale_target = match node_state {
                    PackShopDisplayState::Hidden => 0.0,
                    // PackShopDisplayState::Hover => 1.0,
                    PackShopDisplayState::Selected => 1.2,
                    PackShopDisplayState::Idle => {
                        if !can_afford {
                            0.9
                        } else {
                            1.2
                        }
                    }
                    PackShopDisplayState::Opening => 1.0,
                };

                match node_state {
                    PackShopDisplayState::Idle | PackShopDisplayState::Opening => {
                        if hovering {
                            scale_target = 1.7;
                        }
                    }
                    _ => {
                        // no hovering for all other states
                    }
                }

                self.scale = elara_engine::math::lerp(self.scale, scale_target, 0.35);
            }
            PackInstanceState::Remove => {
                self.scale = 0.0;
            }
        };
        let position = pack_info.shop_position + self.position_offset;

        let mut rot_max = match node_state {
            PackShopDisplayState::Selected => 0.05,
            PackShopDisplayState::Opening => 0.2,
            PackShopDisplayState::Idle => {
                if hovering {
                    0.1
                } else {
                    0.45
                }
            }
            _ => 0.45,
        };

        self.rot_time += match self.state {
            PackInstanceState::Opening => 0.7,
            _ => {
                if !can_afford {
                    0.01
                } else {
                    0.04
                }
            }
        };

        // disable any rotation animation. I kinda like it without the animation
        self.rot_time = 1.1;

        let target_rot = VecThreeFloat::new(
            f64::sin(self.rot_time) * rot_max,
            -90.0_f64.to_radians() + (f64::sin(self.rot_time + 2.0) * rot_max),
            -70.0_f64.to_radians() + (f64::sin(self.rot_time + 1.0) * rot_max),
        );
        self.rotation = VecThreeFloat::lerp(self.rotation, target_rot, 0.1);

        if self.scale > 0.01 {
            let mut trans = Transform::new();
            trans.local_scale = VecThreeFloat::new(self.scale, self.scale, self.scale);

            trans.local_position = pack_info.shop_position + self.position_offset;
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
}
