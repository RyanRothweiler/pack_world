use crate::{
    state::{player_state::*, *},
    ui_panels::{home_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{rect::*, render::material::*, typeface::*, ui::*, vectors::*};

pub struct NavTabsPanel {}

impl NavTabsPanel {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        player_state: &PlayerState,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let y_offset: f64 = 80.0;

        let inv_disp = format!("Bank ({}/{})", inventory.items.len(), inventory.limit);

        if draw_button(
            &inv_disp,
            None,
            &Rect::new_top_size(VecTwo::new(10.0, 50.0), 50.0, 50.0),
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::HomePanelTabChange(home_panel::Tab::Inventory));
            ret.push(UpdateSignal::SetPlacingTile(None));
        }

        if draw_button(
            "Shop",
            None,
            &Rect::new_top_size(VecTwo::new(200.0, 50.0), 50.0, 50.0),
            ui_state,
            std::line!(),
            ui_context,
        ) {
            ret.push(UpdateSignal::HomePanelTabChange(home_panel::Tab::Shop));
            ret.push(UpdateSignal::SetPlacingTile(None));
        }

        return ret;
    }
}
