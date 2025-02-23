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

        let packs: Vec<PackID> = vec![PackID::Starter, PackID::Stick];

        let y_offset: f64 = 80.0;
        let mut cursor_y: f64 = 50.0;

        for (i, pack_id) in packs.iter().enumerate() {
            let pack_info: &Pack = get_pack_info(*pack_id);

            let pack_image_size = VecTwo::new(448.0, 604.0) * 0.35;

            let button_rect = Rect::new_top_size(
                VecTwo::new(10.0, cursor_y),
                pack_image_size.x,
                pack_image_size.y,
            );

            if draw_button(
                &pack_info.display_name,
                Some(assets.get_pack_icon(&pack_id)),
                &button_rect,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                return vec![UpdateSignal::OpenPack(*pack_id)];
            }

            for (j, cost) in pack_info.cost.iter().enumerate() {
                draw_text(
                    &format!("   {:?} x {}", cost.0, cost.1),
                    button_rect.top_right() + VecTwo::new(0.0, 40.0 * j as f64),
                    COLOR_WHITE,
                    ui_state,
                    ui_context,
                );
            }

            cursor_y += 300.0;
        }

        cursor_y -= 300.0;

        // bank slot
        {
            let mut r = Rect::new(
                VecTwo::new(0.0, cursor_y),
                VecTwo::new(200.0, cursor_y + 50.0),
            );
            r.translate(VecTwo::new(10.0, 300.0));

            if draw_button(
                &format!("1 Bank Slot {} gold", inventory.next_slot_cost()),
                None,
                &r,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                return vec![UpdateSignal::PurchaseBankSlot];
            }
        }

        end_panel(&mut ui_state, ui_context);

        vec![]
    }
}
