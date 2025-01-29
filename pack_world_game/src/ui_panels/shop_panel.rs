use crate::{pack::*, state::*, ui_panels::*, UpdateSignal};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct ShopPanel {}

impl ShopPanel {
    pub fn update(
        &mut self,
        common: &UIPanelCommon,
        mut state: &mut UIFrameState,
        inventory: &Inventory,
    ) -> Vec<UpdateSignal> {
        // begin panel
        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 150.0), 400.0, state.resolution.y),
            BG_COLOR,
            &mut state,
        );

        let y_offset: f64 = 80.0;

        let pack_info: &Pack = get_pack_info(PackID::Starter);

        {
            if draw_button(
                &pack_info.display_name,
                None,
                &Rect::new_top_size(VecTwo::new(10.0, 50.0), 50.0, 50.0),
                &common.button_font_style,
                state,
                std::line!(),
            ) {
                return vec![UpdateSignal::OpenPack(PackID::Starter)];
            }

            for c in &pack_info.cost {
                draw_text(
                    &format!("   {:?}x{}", c.0, c.1),
                    &common.button_font_style,
                    VecTwo::new(10.0, 70.0),
                    state,
                );
            }
        }

        end_panel(&mut state);

        vec![]
    }
}
