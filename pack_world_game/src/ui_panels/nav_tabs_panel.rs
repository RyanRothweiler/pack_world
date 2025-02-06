use crate::{
    state::*,
    ui_panels::{home_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct NavTabsPanel {}

impl UIPanelLifecycle for NavTabsPanel {
    fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let y_offset: f64 = 80.0;

        if draw_button(
            "Inventory",
            None,
            &Rect::new_top_size(VecTwo::new(10.0, 50.0), 50.0, 50.0),
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::HomePanelTabChange(home_panel::Tab::Inventory));
        }

        if draw_button(
            "Shop",
            None,
            &Rect::new_top_size(VecTwo::new(150.0, 50.0), 50.0, 50.0),
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::HomePanelTabChange(home_panel::Tab::Shop));
        }

        return ret;
    }
}
