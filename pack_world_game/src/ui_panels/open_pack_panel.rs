use crate::{
    pack::*,
    state::{assets, *},
    ui_panels::{nav_tabs_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct OpenPackPanel {}

impl UIPanelLifecycle for OpenPackPanel {
    fn update(
        &mut self,
        common: &UIPanelCommon,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
    ) -> Vec<UpdateSignal> {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        begin_panel(
            Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(1400.0, 800.0)),
            BG_COLOR,
            &mut ui_state,
        );

        end_panel(&mut ui_state);

        update_signals
    }
}
