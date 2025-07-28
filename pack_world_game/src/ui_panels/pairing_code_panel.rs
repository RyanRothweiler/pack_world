use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use elara_engine::{
    account_call::*, networking::*, rect::*, render::material::*, typeface::*, ui::*, vectors::*,
};

pub struct PairingCodePanel {
    email: String,

    pairing_code: String,
    network_call: Option<usize>,
}

impl PairingCodePanel {
    pub fn new(email: String) -> Self {
        Self {
            email,
            network_call: None,
            pairing_code: String::new(),
        }
    }

    pub fn update(
        &mut self,
        networking_system: &mut NetworkingSystem,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let panel_r = Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(500.0, 600.0));
        begin_panel(panel_r, *THEME_PANEL_BG, &mut ui_state, ui_context);

        let margin_l = 30.0;

        draw_text(
            "Enter Pairing Code",
            VecTwo::new(margin_l, 50.0),
            COLOR_WHITE,
            &&ui_context.font_header.clone(),
            ui_state,
            ui_context,
        );

        draw_paragraph(
            "A pairing code email has been sent. Check your email and input the pairing code here.",
            Rect::new_top_size(VecTwo::new(margin_l, 75.0), 300.0, 1000.0),
            COLOR_WHITE,
            &ui_context.font_body.clone(),
            ui_state,
            ui_context,
        );

        InputField::draw(
            "Pairing Code",
            "pairing code",
            &mut self.pairing_code,
            VecTwo::new(margin_l, 200.0),
            300.0,
            &ui_context.font_nav.clone(),
            &ui_context.font_body.clone(),
            ui_state,
            ui_context,
            std::line!(),
        );

        if draw_text_button(
            "Submit",
            VecTwo::new(margin_l, 300.0),
            &ui_context.font_nav.clone(),
            false,
            Some(crate::BUTTON_BG),
            ui_state,
            std::line!(),
            ui_context,
        ) {
            self.network_call = Some(networking_system.start_call(
                AccountCall::VerifyPairingCode {
                    email: self.email.clone(),
                    pairing_code: self.pairing_code.clone(),
                },
            ));
        }

        if draw_text_button(
            "Back",
            VecTwo::new(margin_l, 360.0),
            &ui_context.font_nav.clone(),
            false,
            Some(crate::BUTTON_BG),
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::PreviousPanel());
            ret.push(UpdateSignal::PushPanel(CreatePanelData::CreateAccount));
        }

        if let Some(call_id) = self.network_call {
            let status = networking_system.get_status(call_id);
            let mut col = COLOR_WHITE;

            match &status {
                NetworkCallStatus::Error { error } => col = COLOR_RED,
                NetworkCallStatus::Waiting | NetworkCallStatus::Sending => {
                    // do nothing
                }
                NetworkCallStatus::Success { response } => {
                    ret.push(UpdateSignal::PreviousPanel());
                    ret.push(UpdateSignal::LoginUserFromSupabase {
                        user_json: response.clone(),
                    });
                }
            }

            draw_text(
                &status.display(),
                VecTwo::new(margin_l, 450.0),
                col,
                &ui_context.font_body.clone(),
                ui_state,
                ui_context,
            );
        }

        end_panel(&mut ui_state, ui_context);

        return ret;
    }
}
