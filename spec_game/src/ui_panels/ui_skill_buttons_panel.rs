use crate::{item::*, state::*, ui_panels::*, UpdateSignal};
use gengar_engine::{font::*, rect::*, ui::*, vectors::*};

pub struct SkillButtonsPanel {}

impl SkillButtonsPanel {
    pub fn update(&mut self, common: &UIPanelCommon, items: &Vec<Item>) -> Vec<UpdateSignal> {
        let r = Rect::new(VecTwo::new(100.0, 100.0), VecTwo::new(200.0, 200.0));

        let disp = format!("items {:?}", items.len());
        if draw_button(&disp, std::line!(), &r, &common.button_font_style) {
            return vec![UpdateSignal::CreateItem];
        }

        return vec![];
    }
}
