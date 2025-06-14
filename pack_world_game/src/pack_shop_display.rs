use crate::{inventory::*, pack::*, pack::*, pack_shop_signals::*, state::assets::*};
use gengar_engine::{
    collisions::*,
    color::*,
    input::*,
    matricies::*,
    rect::*,
    state::render_system::*,
    transform::*,
    ui::*,
    {
        render::{
            render_command::*,
            render_pack::{RenderPackID::*, *},
            shader::*,
            *,
        },
        vectors::*,
    },
};

#[derive(Copy, Clone, Eq, PartialEq)]
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
        mouse_left: &gengar_engine::input::ButtonState,
        mouse_world: VecThreeFloat,
        inventory: &Inventory,
        assets: &mut Assets,
        render_system: &mut RenderSystem,
        mut ui_frame_state: &mut UIFrameState,
        ui_context: &mut UIContext,
        window_resolution: VecTwo,
    ) -> Vec<PackShopSignals> {
        let mut ret: Vec<PackShopSignals> = vec![];

        let pack_info = pack_id.get_pack_info();
        let cam = &render_system
            .render_packs
            .get(&RenderPackID::Shop)
            .unwrap()
            .camera;

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

        // ui
        {
            let screen_origin = cam.world_to_screen(
                pack_info.shop_position + VecThreeFloat::new(2.5, 0.0, 0.0),
                window_resolution,
            );

            if hovering || self.state == PackShopDisplayState::Hover {
                let info_rect = Rect::new_top_size(screen_origin, 100.0, 100.0);
                begin_panel(
                    info_rect,
                    Color::new(0.0, 0.0, 0.0, 0.0),
                    ui_frame_state,
                    ui_context,
                );
                {
                    /*
                    draw_text(
                        &pack_info.display_name,
                        VecTwo::new(00.0, 0.0),
                        COLOR_WHITE,
                        &gs.font_style_header,
                        &mut ui_frame_state,
                        ui_context,
                    );
                    */

                    // cost
                    {
                        draw_text(
                            "Cost",
                            VecTwo::new(0.0, 30.0),
                            COLOR_WHITE,
                            &ui_context.font_body.clone(),
                            &mut ui_frame_state,
                            ui_context,
                        );
                        for (j, cost) in pack_info.cost.iter().enumerate() {
                            let cost_origin = VecTwo::new(80.0 * j as f64, 35.0);
                            let icon_size = 40.0;

                            let icon = assets.get_item_icon(&cost.0);
                            let r = Rect::new_top_size(cost_origin, icon_size, icon_size);

                            let mut color = COLOR_WHITE;
                            if !inventory.has_atleast(cost.0, cost.1) {
                                color = COLOR_RED;
                            }

                            draw_image(r, icon, color, &mut ui_frame_state, ui_context);

                            draw_text(
                                &format!("{}", cost.1),
                                cost_origin + VecTwo::new(40.0, 30.0),
                                color,
                                &ui_context.font_body.clone(),
                                &mut ui_frame_state,
                                ui_context,
                            );
                        }
                    }

                    if self.state == PackShopDisplayState::Selected {
                        if draw_text_button(
                            "Show Drop List",
                            VecTwo::new(10.0, 110.0),
                            &ui_context.font_body.clone(),
                            false,
                            Some(crate::BUTTON_BG),
                            &mut ui_frame_state,
                            std::line!(),
                            ui_context,
                        ) {
                            /*
                            handle_signals(
                                vec![UpdateSignal::SetActivePage(
                                    CreatePanelData::PackDetails { pack_id: *pack_id },
                                )],
                                gs,
                                es,
                                platform_api,
                            );
                            */
                        }

                        if draw_text_button(
                            "Open Pack",
                            VecTwo::new(10.0, 180.0),
                            &&ui_context.font_header.clone(),
                            false,
                            Some(crate::BUTTON_BG),
                            &mut ui_frame_state,
                            std::line!(),
                            ui_context,
                        ) {
                            /*
                            handle_signals(
                                vec![UpdateSignal::SetActivePage(
                                    CreatePanelData::PackDetails { pack_id: *pack_id },
                                )],
                                gs,
                                es,
                                platform_api,
                            );
                            */
                        }
                    }
                }
                end_panel(&mut ui_frame_state, ui_context);
            }
        }

        // pack rendering
        {
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
