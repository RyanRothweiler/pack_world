use crate::{
    account_system::*,
    state::*,
    ui_panels::{home_panel::*, *},
    user_account::*,
    UpdateSignal,
};
use gengar_engine::{
    rect::*,
    render::{material::*, render_command::*},
    typeface::*,
    ui::*,
    vectors::*,
};

pub struct NavTabsPanel {}

impl NavTabsPanel {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        account_system: &AccountSystem,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
        current_mode: GameMode,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        if draw_text_button(
            "World",
            VecTwo::new(20.0, 40.0),
            &ui_context.font_nav.clone(),
            current_mode == GameMode::World,
            None,
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::SetGameMode {
                new_mode: GameMode::World,
            });
        }
        if draw_text_button(
            "Shop",
            VecTwo::new(125.0, 40.0),
            &ui_context.font_nav.clone(),
            current_mode == GameMode::Shop,
            None,
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::SetGameMode {
                new_mode: GameMode::Shop,
            });
        }
        if draw_text_button(
            "Inventory",
            VecTwo::new(225.0, 40.0),
            &ui_context.font_nav.clone(),
            current_mode == GameMode::Inventory,
            None,
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::SetGameMode {
                new_mode: GameMode::Inventory,
            });
        }

        {
            let twitter =
                Rect::new_top_size(VecTwo::new(ui_state.resolution.x - 60.0, 15.0), 25.0, 25.0);
            let bluesky =
                Rect::new_top_size(VecTwo::new(ui_state.resolution.x - 100.0, 15.0), 25.0, 25.0);
            let discord =
                Rect::new_top_size(VecTwo::new(ui_state.resolution.x - 150.0, 15.0), 35.0, 25.0);

            if draw_button_id(
                0,
                "",
                ButtonStyleData::new_shrink(Some(assets.image_twitter.gl_id.unwrap()), None, 0.2),
                &twitter,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::OpenURL {
                    url: "https://x.com/RyanRothweiler".into(),
                })
            }
            if draw_button_id(
                1,
                "",
                ButtonStyleData::new_shrink(Some(assets.image_bluesky.gl_id.unwrap()), None, 0.2),
                &bluesky,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::OpenURL {
                    url: "https://bsky.app/profile/ryanrothweiler.bsky.social".into(),
                })
            }

            if draw_button_id(
                2,
                "",
                ButtonStyleData::new_shrink(Some(assets.image_discord.gl_id.unwrap()), None, 0.2),
                &discord,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::OpenURL {
                    url: "https://discord.gg/FUmeVUeX".into(),
                })
            }

            if let Some(user_account) = &account_system.user_account {
                let mut col = if user_account.did_purchase_base() {
                    COLOR_GREEN
                } else {
                    *THEME_TEXT
                };
                col.a = 0.5;

                draw_text(
                    &user_account.email,
                    VecTwo::new(ui_state.resolution.x - 690.0, 35.0),
                    col,
                    &ui_context.font_body.clone(),
                    ui_state,
                    ui_context,
                );

                if draw_text_button(
                    "Logout",
                    VecTwo::new(ui_state.resolution.x - 360.0, 35.0),
                    &ui_context.font_body.clone(),
                    false,
                    Some(crate::BUTTON_BG),
                    ui_state,
                    std::line!(),
                    ui_context,
                ) {
                    ret.push(UpdateSignal::Logout);
                }
            } else {
                /*
                draw_text(
                    "!! No cloud save !!",
                    VecTwo::new(ui_state.resolution.x - 690.0, 35.0),
                    Color::new(1.0, 0.0, 0.0, 0.7),
                    &ui_context.font_body.clone(),
                    ui_state,
                    ui_context,
                );
                */

                if draw_text_button(
                    "Create Account / Sign in",
                    VecTwo::new(ui_state.resolution.x - 510.0, 35.0),
                    &ui_context.font_body.clone(),
                    false,
                    Some(crate::BUTTON_BG),
                    ui_state,
                    std::line!(),
                    ui_context,
                ) {
                    ret.push(UpdateSignal::PushPanel(CreatePanelData::CreateAccount));
                }
            }
        }

        // underline separator
        {
            let mut r = Rect::new_zero();
            r.top_left = VecTwo::new(0.0, 55.0);
            r.bottom_right = VecTwo::new(20000.0, r.top_left.y + 2.0);

            let mut mat = Material::new();
            mat.shader = Some(ui_context.color_shader);
            mat.set_color(Color::new(0.0, 0.51, 0.75, 0.25));
            ui_context
                .render_commands
                .push(RenderCommand::new_rect(&r, -1.0, 0.0, &mat));
        }

        return ret;
    }
}
