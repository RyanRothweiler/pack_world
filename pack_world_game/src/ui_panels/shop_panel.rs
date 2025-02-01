use crate::{
    pack::*,
    state::{assets, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct ShopPanel {}

impl UIPanelLifecycle for ShopPanel {
    fn update(
        &mut self,
        common: &UIPanelCommon,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
    ) -> Vec<UpdateSignal> {
        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 150.0), 400.0, ui_state.resolution.y),
            BG_COLOR,
            &mut ui_state,
        );

        {
            let y_offset: f64 = 80.0;

            let pack_info: &Pack = get_pack_info(PackID::Starter);

            let pack_image_size = VecTwo::new(448.0, 604.0) * 0.35;

            let button_rect = Rect::new_top_size(
                VecTwo::new(10.0, 50.0),
                pack_image_size.x,
                pack_image_size.y,
            );

            if draw_button(
                &pack_info.display_name,
                assets.image_pack_starter.gl_id,
                &button_rect,
                &common.button_font_style,
                ui_state,
                std::line!(),
            ) {
                return vec![UpdateSignal::OpenPack(PackID::Starter)];
            }

            for c in &pack_info.cost {
                draw_text(
                    &format!("   {:?}x{}", c.0, c.1),
                    &common.button_font_style,
                    button_rect.top_right() + VecTwo::new(0.0, 10.0),
                    ui_state,
                );
            }
        }

        end_panel(&mut ui_state);

        vec![]
    }
}
