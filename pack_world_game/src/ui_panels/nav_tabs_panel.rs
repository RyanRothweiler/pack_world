use crate::{state::*, ui_panels::*, UpdateSignal};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct NavTabsPanel {}

impl NavTabsPanel {
    pub fn update(
        &mut self,
        common: &UIPanelCommon,
        mut state: &mut UIFrameState,
        inventory: &Inventory,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 0.0), 400.0, 100.0),
            BG_COLOR,
            &mut state,
        );

        let y_offset: f64 = 80.0;

        if draw_button(
            "Inventory",
            None,
            &Rect::new_top_size(VecTwo::new(10.0, 50.0), 50.0, 50.0),
            &common.button_font_style,
            state,
            std::line!(),
        ) {
            ret.push(UpdateSignal::SetActivePage(PanelID::TileLibrary));
        }

        if draw_button(
            "Shop",
            None,
            &Rect::new_top_size(VecTwo::new(150.0, 50.0), 50.0, 50.0),
            &common.button_font_style,
            state,
            std::line!(),
        ) {
            ret.push(UpdateSignal::SetActivePage(PanelID::Shop));
        }

        end_panel(&mut state);

        return ret;
    }
}
