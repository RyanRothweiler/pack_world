pub use crate::{
    pack_shop_display::*,
    grid::*, 
    item::*, 
    purchase_flow::*, 
    save_file::*, 
    state::*, tile::*, 
    update_signal::*, 
    world::*, 
    pack_shop_signals::*, 
    pack::*,
};
pub use gengar_engine::{
    collisions::*,
    color::*,
    input::{Input, KeyCode},
    matricies::*,
    platform_api::*,
    rect::*,
    render::{camera::*, material::*, render_command::*, render_pack::*, *},
    state::State as EngineState,
    transform::*,
    ui::*,
    vectors::*,
};

pub fn game_mode_shop(
    prev_delta_time: f64,
    gs: &mut State,
    es: &mut EngineState,
    mut ui_frame_state: &mut UIFrameState,
    input: &mut Input,
    render_api: &mut impl RenderApi,
    platform_api: &PlatformApi,
) {


    if gs.active_page.is_none() {
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
        let ui_context = &mut gs.ui_context.as_mut().unwrap();
        if !gs.account_system.user_purchased_base() {
            let panel_w = 400.0;
            let margin_l = 10.0;
            // let premium_marin_l = margin_l + 140.0;

            begin_panel(
                Rect::new_top_size(VecTwo::new(50.0, 100.0), panel_w, 280.0),
                *THEME_PANEL_BG,
                ui_frame_state,
                ui_context,
            );

            draw_paragraph(
                "Purchase Base Game",
                Rect::new_top_size(VecTwo::new(margin_l, 0.0), panel_w, 600.0),
                COLOR_WHITE,
                &ui_context.font_header.clone(),
                ui_frame_state,
                ui_context,
            );
            draw_text(
                "$2.99",
                VecTwo::new(margin_l, 100.0),
                COLOR_GREEN,
                &ui_context.font_header.clone(),
                ui_frame_state,
                ui_context,
            );
            draw_paragraph(
                "Price may vary at checkout.",
                Rect::new_top_size(VecTwo::new(margin_l, 100.0), panel_w, 600.0),
                Color::new(1.0, 1.0, 1.0, 0.4),
                &ui_context.font_body.clone(),
                ui_frame_state,
                ui_context,
            );

            draw_paragraph(
                            &format!("Increase offline progress from {} hour to {} hours. More features coming in the future!", 
                                crate::save_file::SIM_LIMIT_H_FREE, 
                                crate::save_file::SIM_LIMIT_H_PREMIUM
                            ),
                            Rect::new_top_size(VecTwo::new(margin_l, 140.0), panel_w, 600.0),
                            *THEME_TEXT_MUT,
                            &ui_context.font_body.clone(),
                            &mut ui_frame_state,
                            ui_context,
                        );

            if let Some(purchase_flow) = &gs.purchase_flow {
                match purchase_flow {
                    PurchaseFlow::StartingCheckout { network_call } => {
                        draw_text(
                            "Starting Checkout ...",
                            VecTwo::new(margin_l, 250.0),
                            COLOR_WHITE,
                            &ui_context.font_header.clone(),
                            &mut ui_frame_state,
                            ui_context,
                        );
                    }
                    PurchaseFlow::RunningCheckout
                    | PurchaseFlow::Initiate
                    | PurchaseFlow::Register => {}
                }
            } else {
                if draw_text_button(
                    "Purchase",
                    VecTwo::new(margin_l + 10.0, 250.0),
                    &ui_context.font_header.clone(),
                    false,
                    Some(crate::BUTTON_BG),
                    &mut ui_frame_state,
                    std::line!(),
                    ui_context,
                ) {
                    gs.purchase_flow = Some(PurchaseFlow::Initiate);
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
            let light_trans: &mut Transform = &mut es.components.transforms[gs.pack_light_trans];

            /*
            draw_tile_world_pos(
                TileType::Dirt,
                0.0,
                &light_trans.global_matrix.get_position(),
                true,
                es.render_system
                    .render_packs
                    .get_mut(&RenderPackID::Shop)
                    .unwrap(),
                &gs.assets,
            );
            */

            let spd = 0.007;
            let origin_trans: &mut Transform = &mut es.components.transforms[gs.pack_light_origin];
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

            if gs.pack_selected.is_none() && !gs.opening_pack {
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
                        vec![PackShopSignals::DeselectAll],
                        gs,
                        es,
                        platform_api,
                    );
                    gs.pack_selected = None;
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

            let packs: Vec<PackID> =
                vec![PackID::Starter, PackID::Mud, PackID::Stick, PackID::Water];

            // make sure all packs exist in the hashmap.
            // Really means we don't need a hashmap probably
            for pack_id in &packs {
                gs.pack_display_state
                    .entry(*pack_id)
                    .or_insert(PackShopDisplay::new());
            }

            for pack_id in &packs {
                let signals = gs
                    .pack_display_state
                    .entry(*pack_id)
                    .or_insert(PackShopDisplay::new())
                    .update(
                        *pack_id,
                        &input.mouse.button_left,
                        mouse_world,
                        &gs.inventory,
                        &mut gs.assets,
                        &mut es.render_system,
                        &mut ui_frame_state,
                        &mut gs.ui_context.as_mut().unwrap(),
                        es.window_resolution,
                        platform_api,
                    );

                handle_pack_shop_signals(signals, gs, es, platform_api);
            }
        }
    }
}
