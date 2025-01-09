use crate::{item::*, state::*, ui_panels::*, UpdateSignal};
use gengar_engine::{font::*, rect::*, ui::*, vectors::*};

// TODO rename to GlobalNavigation
pub struct SkillButtonsPanel {}

impl SkillButtonsPanel {
    pub fn update(&mut self, common: &UIPanelCommon) -> Vec<UpdateSignal> {
        let r = Rect::new(VecTwo::new(100.0, 100.0), VecTwo::new(400.0, 150.0));

        if draw_button("Mining", std::line!(), &r, &common.button_font_style) {
            return vec![UpdateSignal::SetActivePage(PanelID::Mining)];
        }

        vec![]
    }
}
