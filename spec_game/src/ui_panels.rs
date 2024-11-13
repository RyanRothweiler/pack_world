use crate::UpdateSignal;
use gengar_engine::font::*;

pub mod ui_skill_buttons_panel;

use ui_skill_buttons_panel::*;

pub enum UIPanel {
    SkillButtons(UIPanelCommon, SkillButtonsPanel),
}

pub struct UIPanelCommon {
    pub button_font_style: FontStyle,
}

/*
impl UIPanel {
    pub fn update(&mut self) -> Vec<UpdateSignal> {
        match self {
            UIPanel::SkillButtons(common, panel_state) => panel_state.update(common),
        }
    }
}
*/
