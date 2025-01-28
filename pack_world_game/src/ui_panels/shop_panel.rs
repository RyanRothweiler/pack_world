use crate::{item::*, state::*, ui_panels::*, UpdateSignal};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct ShopPanel {}

impl ShopPanel {
    pub fn update(
        &mut self,
        common: &UIPanelCommon,
        mut state: &mut UIFrameState,
        inventory: &Inventory,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        // begin panel
        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 150.0), 400.0, state.resolution.y),
            BG_COLOR,
            &mut state,
        );

        let y_offset: f64 = 80.0;

        if draw_button(
            "!! pack !!",
            std::line!(),
            &Rect::new_top_size(VecTwo::new(10.0, 50.0), 50.0, 50.0),
            &common.button_font_style,
            state,
        ) {}

        end_panel(&mut state);

        return ret;
    }
}
