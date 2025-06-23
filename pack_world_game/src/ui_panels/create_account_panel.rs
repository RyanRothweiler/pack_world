use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{rect::*, render::material::*, typeface::*, ui::*, vectors::*};

pub struct CreateAccountPanel {
    email: String,
}

impl CreateAccountPanel {
    pub fn new() -> Self {
        Self {
            email: String::new(),
        }
    }

    pub fn update(
        &mut self,
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
        ) {}

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

        end_panel(&mut ui_state, ui_context);

        return ret;
    }
}
