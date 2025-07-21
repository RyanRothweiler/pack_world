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
        assets: &mut Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut sigs: Vec<UpdateSignal> = vec![];

        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 150.0), 900.0, ui_state.resolution.y),
            *THEME_PANEL_BG,
            &mut ui_state,
            ui_context,
        );

        let packs: Vec<PackID> = vec![PackID::Starter, PackID::Mud, PackID::Stick, PackID::Water];

        let mut cursor_y: f64 = 50.0;

        for (i, pack_id) in packs.iter().enumerate() {
            let pack_info: &Pack = pack_id.get_pack_info();

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

                    let mut color = COLOR_WHITE;
                    if !inventory.has_atleast(cost.0, cost.1) {
                        color = COLOR_RED;
                    }

                    draw_image(r, icon, color, ui_state, ui_context);

                    draw_text(
                        &format!("{}", cost.1),
                        cost_origin + VecTwo::new(40.0, 30.0),
                        color,
                        &ui_context.font_body.clone(),
                        ui_state,
                        ui_context,
                    );
                }
            }

            if draw_text_button_id(
                i as i32,
                "Show Drop List",
                desc_origin + VecTwo::new(10.0, 110.0),
                &ui_context.font_body.clone(),
                false,
                Some(crate::BUTTON_BG),
                ui_state,
                std::line!(),
                ui_context,
            ) {
                sigs.push(UpdateSignal::PushPanel(CreatePanelData::PackDetails {
                    pack_id: *pack_id,
                }));
            }

            // pack button
            {
                let pack_image_size = VecTwo::new(448.0, 604.0) * 0.35;

                let button_rect = Rect::new_top_size(
                    VecTwo::new(10.0, cursor_y),
                    pack_image_size.x,
                    pack_image_size.y,
                );

                if draw_button_id(
                    i as i32,
                    "",
                    ButtonStyleData::new_shrink(Some(assets.get_pack_icon(&pack_id)), None, 1.0),
                    &button_rect,
                    ui_state,
                    std::line!(),
                    ui_context,
                ) {
                    sigs.push(UpdateSignal::OpenPack(*pack_id));
                }
            }

            cursor_y += 250.0;
        }

        end_panel(&mut ui_state, ui_context);

        sigs
    }
}
