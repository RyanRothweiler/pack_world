use crate::{
    drop_table::*,
    pack::*,
    state::{assets, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{rect::*, render::material::*, typeface::*, ui::*, vectors::*};

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
            Rect::new_top_size(VecTwo::new(0.0, 150.0), 900.0, ui_state.resolution.y),
            BG_COLOR,
            &mut ui_state,
            ui_context,
        );

        let packs: Vec<PackID> = vec![PackID::Starter, PackID::Stick];

        let y_offset: f64 = 80.0;
        let mut cursor_y: f64 = 50.0;

        for (i, pack_id) in packs.iter().enumerate() {
            let pack_info: &Pack = get_pack_info(*pack_id);

            let origin = VecTwo::new(10.0, cursor_y);

            let desc_origin = origin + VecTwo::new(175.0, 40.0);

            // pack title
            draw_text(
                &pack_info.display_name,
                desc_origin + VecTwo::new(0.0, 0.0),
                COLOR_WHITE,
                &ui_context.font_header.clone(),
                ui_state,
                ui_context,
            );

            // cost
            {
                draw_text(
                    "Cost",
                    desc_origin + VecTwo::new(0.0, 30.0),
                    COLOR_WHITE,
                    &ui_context.font_body.clone(),
                    ui_state,
                    ui_context,
                );
                for (j, cost) in pack_info.cost.iter().enumerate() {
                    let cost_origin = desc_origin + VecTwo::new(80.0 * j as f64, 35.0);
                    let icon_size = 40.0;

                    let icon = assets.get_item_icon(&cost.0);
                    let r = Rect::new_top_size(cost_origin, icon_size, icon_size);

                    draw_image(r, icon, COLOR_WHITE, ui_state, ui_context);

                    draw_text(
                        &format!("{}", cost.1),
                        cost_origin + VecTwo::new(40.0, 30.0),
                        COLOR_WHITE,
                        &ui_context.font_body.clone(),
                        ui_state,
                        ui_context,
                    );
                }
            }

            /*
            // drops
            {
                let y: f64 = 90.0;

                draw_text(
                    "Drops",
                    desc_origin + VecTwo::new(0.0, y + 30.0),
                    COLOR_WHITE,
                    &ui_context.font_body.clone(),
                    ui_state,
                    ui_context,
                );
                let list = get_fixed_table(pack_info.table_id).list_drops();

                for (j, drop) in list.iter().enumerate() {
                    let cost_origin = desc_origin + VecTwo::new(80.0 * j as f64, y + 35.0);
                    let icon_size = 40.0;

                    let icon = assets.get_drop_icon(&drop.drop_type);
                    let r = Rect::new_top_size(cost_origin, icon_size, icon_size);

                    draw_image(r, icon, COLOR_WHITE, ui_state, ui_context);

                    draw_text(
                        &format!("{}", drop.amount),
                        cost_origin + VecTwo::new(40.0, 30.0),
                        COLOR_WHITE,
                        &ui_context.font_body.clone(),
                        ui_state,
                        ui_context,
                    );
                }
            }
            */

            // pack button
            {
                let pack_image_size = VecTwo::new(448.0, 604.0) * 0.35;

                let button_rect = Rect::new_top_size(
                    VecTwo::new(10.0, cursor_y),
                    pack_image_size.x,
                    pack_image_size.y,
                );

                if draw_button(
                    "",
                    Some(assets.get_pack_icon(&pack_id)),
                    &button_rect,
                    ui_state,
                    std::line!(),
                    ui_context,
                ) {
                    return vec![UpdateSignal::OpenPack(*pack_id)];
                }
            }

            cursor_y += 250.0;
        }

        end_panel(&mut ui_state, ui_context);

        vec![]
    }
}
