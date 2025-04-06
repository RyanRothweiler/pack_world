use crate::{
    state::*,
    ui_panels::{home_panel::*, *},
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
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
        current_tab: Tab,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        if draw_text_button(
            "Inventory",
            VecTwo::new(20.0, 80.0),
            &ui_context.font_nav.clone(),
            current_tab == Tab::Inventory,
            None,
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::HomePanelTabChange(home_panel::Tab::Inventory));
            ret.push(UpdateSignal::SetPlacingTile(None));
        }
        if draw_text_button(
            "Shop",
            VecTwo::new(175.0, 80.0),
            &ui_context.font_nav.clone(),
            current_tab == Tab::Shop,
            None,
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::HomePanelTabChange(home_panel::Tab::Shop));
            ret.push(UpdateSignal::SetPlacingTile(None));
        }

        {
            let twitter =
                Rect::new_top_size(VecTwo::new(ui_state.resolution.x - 60.0, 55.0), 25.0, 25.0);
            let bluesky =
                Rect::new_top_size(VecTwo::new(ui_state.resolution.x - 100.0, 55.0), 25.0, 25.0);

            if draw_button_id(
                0,
                "",
                ButtonStyleData::new_shrink(Some(assets.image_twitter.gl_id.unwrap()), 0.2),
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
                ButtonStyleData::new_shrink(Some(assets.image_bluesky.gl_id.unwrap()), 0.2),
                &bluesky,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::OpenURL {
                    url: "https://bsky.app/profile/ryanrothweiler.bsky.social".into(),
                })
            }

            if draw_text_button(
                "Join Email List",
                VecTwo::new(ui_state.resolution.x - 250.0, 75.0),
                &ui_context.font_body.clone(),
                false,
                Some(crate::BUTTON_BG),
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::OpenURL {
                    url: "https://mailchi.mp/932d23a45465/packworld".into(),
                })
            }
        }

        // underline separator
        {
            let mut r = Rect::new_zero();
            r.top_left = VecTwo::new(0.0, 95.0);
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
