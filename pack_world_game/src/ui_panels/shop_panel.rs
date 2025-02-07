use crate::{
    pack::*,
    state::{assets, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct ShopPanel {}

impl ShopPanel {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 150.0), 400.0, ui_state.resolution.y),
            BG_COLOR,
            &mut ui_state,
            ui_context,
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
                ui_state,
                std::line!(),
                ui_context,
            ) {
                return vec![UpdateSignal::OpenPack(PackID::Starter)];
            }

            for c in &pack_info.cost {
                draw_text(
                    &format!("   {:?}x{}", c.0, c.1),
                    button_rect.top_right() + VecTwo::new(0.0, 10.0),
                    ui_state,
                    ui_context,
                );
            }
        }

        end_panel(&mut ui_state, ui_context);

        vec![]
    }
}
