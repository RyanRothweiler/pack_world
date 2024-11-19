use crate::UpdateSignal;
use gengar_engine::font::*;

pub mod ui_mining_panel;
pub mod ui_skill_buttons_panel;

use ui_mining_panel::*;
use ui_skill_buttons_panel::*;

pub enum PanelID {
    Mining,
}

pub enum UIPanelState {
    SkillButtons(UIPanelCommon, SkillButtonsPanel),
    Mining(UIPanelCommon, MiningPanel),
}

#[derive(Clone)]
pub struct UIPanelCommon {
    pub button_font_style: FontStyle,
}

pub fn update_panel(panel: &mut UIPanelState) -> Vec<UpdateSignal> {
    let mut update_signals: Vec<UpdateSignal> = vec![];

    match panel {
        UIPanelState::SkillButtons(common, panel_state) => {
            update_signals.append(&mut panel_state.update(common));
        }
        UIPanelState::Mining(common, panel_state) => {
            update_signals.append(&mut panel_state.update(common));
        }
    }

    update_signals
}
