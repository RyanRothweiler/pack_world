use crate::{item::*, state::*, ui_panels::*, UpdateSignal};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct TileLibraryPanel {}

impl TileLibraryPanel {
    pub fn update(
        &mut self,
        common: &UIPanelCommon,
        mut state: &mut UIFrameState,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        // begin panel
        begin_panel(
            Rect::new_top_size(VecTwo::new(10.0, 10.0), 100.0, 100.0),
            BG_COLOR,
            &mut state,
        );

        // button
        if draw_button(
            "tile library ",
            std::line!(),
            &Rect::new_top_size(VecTwo::new(600.0, 100.0), 200.0, 200.0),
            &common.button_font_style,
            state,
        ) {}

        end_panel(&mut state);

        return ret;
    }
}
