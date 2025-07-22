pub use crate::{
    account_system::*, assets::*, grid::*, inventory::*, item::*, pack::*, pack_shop_display::*,
    pack_shop_signals::*, save_file::*, state::*, tile::*, update_signal::*, world::*,
};
pub use gengar_engine::{
    collisions::*,
    color::*,
    input::{Input, KeyCode},
    matricies::*,
    platform_api::*,
    rect::*,
    render::{camera::*, light::*, material::*, render_command::*, render_pack::*, *},
    state::{components::*, State as EngineState},
    transform::*,
    ui::*,
    vectors::*,
};
use std::collections::HashMap;

pub struct GameModeShop {
    light_origin: usize,

    light_trans: usize,
    light_trans_second: usize,

    pub pack_display_state: HashMap<PackID, PackShopDisplay>,
    pub opening_pack: bool,
    pub pack_selected: Option<PackID>,
}

impl GameModeShop {
    pub fn new(es: &mut EngineState) -> Self {
        let mut sel = Self {
            light_origin: 0,
            light_trans: 0,
            light_trans_second: 0,

            pack_display_state: HashMap::new(),
            opening_pack: false,
            pack_selected: None,
        };

        // setup lights
        {
            sel.light_origin = es.components.new_transform();
            // let origin_trans: &mut Transform = &mut es.components.transforms[light.transform];

            let rad = 10.0;
            let y = 20.0;

            // first light
            {
                sel.light_trans = es.components.new_transform();

                let light = Light::new(sel.light_trans);

                let ct: &mut Transform = &mut es.components.transforms[light.transform];
                ct.parent = Some(sel.light_origin);
                ct.local_position.x = rad;
                ct.local_position.z = rad;
                ct.local_position.y = y;

                es.render_system
                    .get_pack(RenderPackID::Shop)
                    .lights
                    .push(light);
            }

            // second light
            {
                sel.light_trans_second = es.components.new_transform();

                let light = Light::new(sel.light_trans_second);

                let ct: &mut Transform = &mut es.components.transforms[light.transform];
                ct.parent = Some(sel.light_origin);
                ct.local_position.x = -rad;
                ct.local_position.z = -rad;
                ct.local_position.y = y;

                es.render_system
                    .get_pack(RenderPackID::Shop)
                    .lights
                    .push(light);
            }

            // third whilte light
            {
                let light = Light::new(es.components.new_transform());

                let ct: &mut Transform = &mut es.components.transforms[light.transform];
                ct.parent = Some(sel.light_origin);
                ct.local_position.x = rad;
                ct.local_position.z = -rad;
                ct.local_position.y = 0.0;

                es.render_system
                    .get_pack(RenderPackID::Shop)
                    .lights
                    .push(light);
            }
        }

        return sel;
    }

    pub fn update(
        &mut self,
        prev_delta_time: f64,
        es: &mut EngineState,
        mut ui_frame_state: &mut UIFrameState,
        input: &mut Input,
        render_api: &mut impl RenderApi,
        platform_api: &PlatformApi,
        inventory: &mut Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
        account_system: &mut AccountSystem,
    ) -> Vec<UpdateSignal> {
        let mut sigs: Vec<UpdateSignal> = vec![];

        let mouse_world: VecThreeFloat = {
            let mut val = VecThreeFloat::new_zero();

            let cam: &Camera = &es
                .render_system
                .render_packs
                .get(&RenderPackID::Shop)
                .unwrap()
                .camera;
            let pos = cam.screen_to_world(input.mouse.pos);

            let dir = (pos - cam.transform.local_position).normalize();

            if let Some(len) = plane_intersection_distance(
                cam.transform.local_position,
                dir,
                VecThreeFloat::new(0.0, 0.0, 0.0),
                VecThreeFloat::new(0.0, -1.0, 0.0),
            ) {
                val = cam.transform.local_position + (dir * len);
            }

            val
        };

        // premium shop UI
        if !account_system.user_purchased_base() {
            let panel_w = 400.0;
            let margin_l = 10.0;
            // let premium_marin_l = margin_l + 140.0;

            begin_panel(
                Rect::new_top_size(VecTwo::new(10.0, 70.0), panel_w, 230.0),
                *THEME_PANEL_BG,
                ui_frame_state,
                ui_context,
            );

            draw_paragraph(
                "Support Development",
                Rect::new_top_size(VecTwo::new(margin_l, 0.0), panel_w, 600.0),
                COLOR_WHITE,
                &ui_context.font_body.clone(),
                ui_frame_state,
                ui_context,
            );
            draw_text(
                "$0.99",
                VecTwo::new(margin_l, 50.0),
                COLOR_GREEN,
                &ui_context.font_body.clone(),
                ui_frame_state,
                ui_context,
            );
            draw_paragraph(
                "Price may vary at checkout.",
                Rect::new_top_size(VecTwo::new(margin_l, 50.0), panel_w, 500.0),
                Color::new(1.0, 1.0, 1.0, 0.4),
                &ui_context.font_body.clone(),
                ui_frame_state,
                ui_context,
            );

            draw_paragraph(
                &crate::constants::OFFLINE_PROGRESS_DESC,
                Rect::new_top_size(VecTwo::new(margin_l, 75.0), panel_w, 600.0),
                *THEME_TEXT_MUT,
                &ui_context.font_body.clone(),
                &mut ui_frame_state,
                ui_context,
            );

            if account_system.purchase_in_progress() {
                draw_text(
                    "Starting Checkout ...",
                    VecTwo::new(margin_l, 200.0),
                    COLOR_WHITE,
                    &ui_context.font_header.clone(),
                    &mut ui_frame_state,
                    ui_context,
                );
            } else {
                if draw_text_button(
                    "Purchase",
                    VecTwo::new(margin_l + 10.0, 200.0),
                    &ui_context.font_header.clone(),
                    false,
                    Some(crate::BUTTON_BG),
                    &mut ui_frame_state,
                    std::line!(),
                    ui_context,
                ) {
                    account_system.start_purchase();
                }
            }

            end_panel(&mut ui_frame_state, ui_context);
        } else {
            draw_text(
                "Account has premium access. Thank you for your support.",
                VecTwo::new(10.0, 80.0),
                Color::new(1.0, 1.0, 1.0, 0.2),
                &ui_context.font_body.clone(),
                &mut ui_frame_state,
                ui_context,
            );
        }

        // lighting
        {
            let spd = 0.007;
            let origin_trans: &mut Transform = &mut es.components.transforms[self.light_origin];
            // origin_trans.local_rotation.x = es.frame as f64 * spd;
            origin_trans.local_rotation.y = es.frame as f64 * spd;
            // origin_trans.local_rotation.x = es.frame as f64 * spd;
            // origin_trans.local_rotation.z = es.frame as f64 * spd;

            let cp = es
                .render_system
                .render_packs
                .get(&RenderPackID::Shop)
                .unwrap()
                .camera
                .transform
                .local_position;

            origin_trans.local_position.x = cp.x;
            origin_trans.local_position.z = cp.z;

            /*
            let light = Light::new(es.components.new_transform());

            let ct: &mut Transform = &mut es.components.transforms[light.transform];
            ct.parent = Some(gs.pack_light_origin);
            ct.local_position.x = -2.0;
            ct.local_position.z = 10.0;
            ct.local_position.y = 15.0;

            es.render_system
                .get_pack(RenderPackID::Shop)
                .lights
                .push(light);
            */
        }

        // camera controls
        if true {
            let cam_pack = es
                .render_system
                .render_packs
                .get_mut(&RenderPackID::Shop)
                .unwrap();

            if self.pack_selected.is_none() && !self.opening_pack {
                cam_pack.camera.move_plane(true, input, prev_delta_time);
            } else {
                if input.keyboard.get_key(KeyCode::W).pressing
                    || input.keyboard.get_key(KeyCode::S).pressing
                    || input.keyboard.get_key(KeyCode::A).pressing
                    || input.keyboard.get_key(KeyCode::D).pressing
                    || input.keyboard.get_key(KeyCode::Escape).pressing
                    || input.mouse.scroll_delta != 0
                {
                    handle_pack_shop_signals(
                        self,
                        vec![PackShopSignals::DeselectAll],
                        es,
                        inventory,
                        platform_api,
                    );
                    self.pack_selected = None;
                }
            }

            let cam_pack = es
                .render_system
                .render_packs
                .get_mut(&RenderPackID::Shop)
                .unwrap();
            cam_pack.camera.update_position(prev_delta_time);
        } else {
            // fly cam for testing
            es.render_system
                .render_packs
                .get_mut(&RenderPackID::Shop)
                .unwrap()
                .camera
                .move_fly(0.3, input);
        }

        // pack layout rendering
        {
            // light testing
            {
                let p = 500.0;
                let white_p = 2000.0;

                es.render_system
                    .render_packs
                    .get_mut(&RenderPackID::Shop)
                    .unwrap()
                    .lights[0]
                    .power = VecThreeFloat::new(3.0 * p, 0.95 * p, 0.9 * p);

                es.render_system
                    .render_packs
                    .get_mut(&RenderPackID::Shop)
                    .unwrap()
                    .lights[1]
                    .power = VecThreeFloat::new(1.0 * p, 0.85 * p, 3.6 * p);

                es.render_system
                    .render_packs
                    .get_mut(&RenderPackID::Shop)
                    .unwrap()
                    .lights[2]
                    .power = VecThreeFloat::new(white_p, white_p, white_p);
            }

            // make sure all packs exist in the hashmap.
            // Really means we don't need a hashmap probably
            for pack_id in crate::pack_id::ALL_PACKS.iter() {
                self.pack_display_state
                    .entry(*pack_id)
                    .or_insert(PackShopDisplay::new());
            }

            for pack_id in crate::pack_id::ALL_PACKS.iter() {
                let signals = self
                    .pack_display_state
                    .entry(*pack_id)
                    .or_insert(PackShopDisplay::new())
                    .update(
                        *pack_id,
                        &input.mouse.button_left,
                        mouse_world,
                        inventory,
                        assets,
                        &mut es.render_system,
                        &mut ui_frame_state,
                        ui_context,
                        es.window_resolution,
                        platform_api,
                    );

                sigs.append(&mut handle_pack_shop_signals(
                    self,
                    signals,
                    es,
                    inventory,
                    platform_api,
                ));
            }
        }

        sigs
    }
}
