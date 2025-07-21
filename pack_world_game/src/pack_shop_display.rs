use crate::{
    drop_table::*, inventory::*, item::*, pack::*, pack::*, pack_shop_signals::*, state::assets::*,
    ui_panels::*, update_signal::*,
};
use gengar_engine::{
    collisions::*,
    color::*,
    input::*,
    matricies::*,
    platform_api::*,
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

mod pack_render_instance;

use pack_render_instance::*;

struct PullDisplay {
    pub drop: Drop,
    pub pos: VecTwo,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PackShopDisplayState {
    Idle,
    Hidden,
    Selected,
    Opening,
}

/// Handles the display of a pack "node"
/// Should be rename to ShopPackNode
pub struct PackShopDisplay {
    state: PackShopDisplayState,
    pack_instances: Vec<PackRenderInstance>,

    pulls: Vec<PullDisplay>,
    items_remaining: i32,
}

impl PackShopDisplay {
    pub fn new() -> Self {
        let mut ret = Self {
            state: PackShopDisplayState::Idle,
            pack_instances: vec![],

            items_remaining: 0,
            pulls: vec![],
        };

        ret.set_state(PackShopDisplayState::Idle);
        ret.pack_instances.push(PackRenderInstance::new());

        ret
    }

    pub fn set_state(&mut self, new_state: PackShopDisplayState) {
        self.state = new_state;

        match new_state {
            PackShopDisplayState::Idle => {
                self.pulls.clear();
            }
            PackShopDisplayState::Opening => {
                self.items_remaining = 4;
                self.pulls.clear();
            }
            _ => {}
        }
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
        platform_api: &PlatformApi,
    ) -> Vec<PackShopSignals> {
        let mut ret: Vec<PackShopSignals> = vec![];

        let pack_info = pack_id.get_pack_info();
        let cam = &render_system
            .render_packs
            .get(&RenderPackID::Shop)
            .unwrap()
            .camera;

        let screen_origin = cam.world_to_screen(
            pack_info.shop_position + VecThreeFloat::new(2.5, 0.0, 0.0),
            window_resolution,
        );
        let pack_center_screen = cam.world_to_screen(pack_info.shop_position, window_resolution);

        let mut hovering = point_within_circle(
            VecTwo::new(mouse_world.x, mouse_world.z),
            VecTwo::new(pack_info.shop_position.x, pack_info.shop_position.z),
            3.0,
        );

        match self.state {
            PackShopDisplayState::Idle => {
                if hovering && mouse_left.on_press {
                    ret.push(PackShopSignals::Select { pack_id });
                }
            }
            PackShopDisplayState::Selected => {}
            PackShopDisplayState::Hidden => {}
            PackShopDisplayState::Opening => {
                if hovering && mouse_left.on_press && self.items_remaining > 0 {
                    // pull item from pack and give
                    let pull = pack_info.pull(platform_api);
                    ret.push(PackShopSignals::StandardUpateSignal {
                        sigs: vec![UpdateSignal::GiveDrop(pull)],
                    });

                    self.items_remaining -= 1;

                    self.pulls.push(PullDisplay {
                        drop: pull,
                        pos: pack_center_screen,
                    });

                    if self.items_remaining == 0 {
                        self.pack_instances[0].change_state(PackInstanceState::Exiting);
                    }
                }
            }
        }

        // ui
        {
            let info_rect = Rect::new_top_size(screen_origin, 100.0, 100.0);
            begin_panel(
                info_rect,
                Color::new(0.0, 0.0, 0.0, 0.0),
                ui_frame_state,
                ui_context,
            );
            {
                // cost
                {
                    let show_cost: bool = match self.state {
                        PackShopDisplayState::Selected => true,
                        PackShopDisplayState::Idle => hovering,
                        _ => false,
                    };

                    if show_cost {
                        draw_text(
                            "Cost",
                            VecTwo::new(0.0, 50.0),
                            COLOR_WHITE,
                            &ui_context.font_body.clone(),
                            &mut ui_frame_state,
                            ui_context,
                        );
                        for (j, cost) in pack_info.cost.iter().enumerate() {
                            let cost_origin = VecTwo::new(70.0 * j as f64, 60.0);
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
                }

                // drop and open buttons
                if self.state == PackShopDisplayState::Selected {
                    if pack_info.can_afford(inventory) {
                        if draw_text_button(
                            "Open Pack",
                            VecTwo::new(10.0, 0.0),
                            &&ui_context.font_header.clone(),
                            false,
                            Some(crate::BUTTON_BG),
                            &mut ui_frame_state,
                            std::line!(),
                            ui_context,
                        ) {
                            ret.push(PackShopSignals::Open { pack_id: pack_id });
                            self.pack_instances[0].change_state(PackInstanceState::Opening);
                        }
                    } else {
                        draw_text(
                            "Can't Afford",
                            VecTwo::new(0.0, 0.0),
                            COLOR_WHITE,
                            &ui_context.font_body.clone(),
                            &mut ui_frame_state,
                            ui_context,
                        );
                    }

                    if draw_text_button(
                        "Show Drop List",
                        VecTwo::new(10.0, 140.0),
                        &ui_context.font_body.clone(),
                        false,
                        Some(crate::BUTTON_BG),
                        &mut ui_frame_state,
                        std::line!(),
                        ui_context,
                    ) {
                        let new_panel_data = CreatePanelData::PackDetails { pack_id: pack_id };
                        ret.push(PackShopSignals::StandardUpateSignal {
                            sigs: vec![UpdateSignal::PushPanel(new_panel_data)],
                        });
                    }

                    if draw_text_button(
                        "Back",
                        VecTwo::new(10.0, 190.0),
                        &ui_context.font_body.clone(),
                        false,
                        Some(crate::BUTTON_BG),
                        &mut ui_frame_state,
                        std::line!(),
                        ui_context,
                    ) {
                        ret.push(PackShopSignals::DeselectAll);
                    }
                }

                // Open again buttons
                if self.state == PackShopDisplayState::Opening && self.items_remaining == 0 {
                    if draw_text_button(
                        "Open Another",
                        VecTwo::new(10.0, 0.0),
                        &&ui_context.font_header.clone(),
                        false,
                        Some(crate::BUTTON_BG),
                        &mut ui_frame_state,
                        std::line!(),
                        ui_context,
                    ) {
                        ret.push(PackShopSignals::OpenFinished);
                        ret.push(PackShopSignals::Select { pack_id: pack_id });
                        self.pack_instances[0].change_state(PackInstanceState::Idle);
                    }
                }

                end_panel(&mut ui_frame_state, ui_context);
            }
        }

        // draw pulls
        {
            let icon_size: f64 = 160.0;

            let deg_between: f64 = 40.0;
            let deg_needed: f64 = deg_between * (self.pulls.len() as f64 - 1.0);
            let deg_start: f64 = deg_needed * 0.5;

            let radius = 300.0;

            let mut i: i32 = 0;

            for p in &mut self.pulls {
                let deg: f64 = (deg_between * i as f64) - deg_start;
                let target_pos = VecTwo::new(
                    radius * f64::sin(deg.to_radians()),
                    radius * f64::cos(deg.to_radians()),
                ) + pack_center_screen;

                p.pos = VecTwo::lerp(p.pos, target_pos, 0.25);

                draw_drop_icon(
                    icon_size,
                    p.pos,
                    &p.drop,
                    ui_frame_state,
                    ui_context,
                    assets,
                    inventory,
                );

                i += 1;
            }
        }

        for (i, inst) in &mut self.pack_instances.iter_mut().enumerate() {
            inst.update_and_render(
                hovering,
                pack_info.can_afford(inventory),
                pack_id,
                self.state,
                render_system,
                assets,
                platform_api,
            );
        }

        ret
    }
}
