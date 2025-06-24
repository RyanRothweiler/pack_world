use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{
    account_call::*, networking::*, rect::*, render::material::*, typeface::*, ui::*, vectors::*,
};

pub struct CreateAccountPanel {
    email: String,

    create_account_call: Option<usize>,
}

impl CreateAccountPanel {
    pub fn new() -> Self {
        Self {
            create_account_call: None,
            email: String::new(),
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

        let panel_r = Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(500.0, 800.0));
        begin_panel(panel_r, BG_COLOR, &mut ui_state, ui_context);

        let margin_l = 30.0;

        draw_text(
            "Create Account",
            VecTwo::new(margin_l, 50.0),
            COLOR_WHITE,
            &&ui_context.font_header.clone(),
            ui_state,
            ui_context,
        );

        InputField::draw(
            "Email",
            "email",
            &mut self.email,
            VecTwo::new(margin_l, 100.0),
            300.0,
            &ui_context.font_nav.clone(),
            &ui_context.font_body.clone(),
            ui_state,
            ui_context,
            std::line!(),
        );

        if draw_text_button(
            "Submit",
            VecTwo::new(margin_l, 200.0),
            &ui_context.font_nav.clone(),
            false,
            Some(crate::BUTTON_BG),
            ui_state,
            std::line!(),
            ui_context,
        ) {
            self.create_account_call = Some(networking_system.start_call(AccountCall::SendOTP {
                email: self.email.clone(),
            }));
        }

        if draw_text_button(
            "Close",
            VecTwo::new(margin_l, 260.0),
            &ui_context.font_nav.clone(),
            false,
            Some(crate::BUTTON_BG),
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::PreviousPanel());
        }

        if let Some(call_id) = self.create_account_call {
            let status = networking_system.get_status(call_id);

            draw_text(
                &format!("call status {:?}", status),
                VecTwo::new(margin_l, 300.0),
                COLOR_WHITE,
                &ui_context.font_body.clone(),
                ui_state,
                ui_context,
            );
        }

        end_panel(&mut ui_state, ui_context);

        return ret;
    }
}
