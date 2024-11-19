use crate::{item::*, state::*, ui_panels::*, UpdateSignal};
use gengar_engine::{font::*, rect::*, ui::*, vectors::*};

pub struct MiningPanel {}

impl MiningPanel {
    pub fn update(&mut self, common: &UIPanelCommon) -> Vec<UpdateSignal> {
        let r = Rect::new_top_size(VecTwo::new(600.0, 100.0), 200.0, 200.0);

        if draw_button(
            "first asteroid",
            std::line!(),
            &r,
            &common.button_font_style,
        ) {}

        vec![]
    }
}
