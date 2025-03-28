#![allow(dead_code)]

use crate::{
    drop_table::*,
    item::*,
    pack::*,
    state::{assets, inventory::*, *},
    ui_panels::{nav_tabs_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{color::*, rect::*, render::material::*, typeface::*, ui::*, vectors::*};

pub struct PackDetailsData {
    pub pack_id: PackID,
}

impl PackDetailsData {
    pub fn new(pack_id: PackID) -> Self {
        Self { pack_id }
    }
}

impl PackDetailsData {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        let panel_r = Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(900.0, 800.0));

        begin_panel(panel_r, BG_COLOR, &mut ui_state, ui_context);

        let pack_info: &Pack = self.pack_id.get_pack_info();

        let pack_image_size = VecTwo::new(448.0, 604.0) * 0.35;

        let button_rect = Rect::new_center(
            panel_r.get_center() + VecTwo::new(-250.0, 100.0),
            pack_image_size,
        );
        let close_rect = Rect::new_center(
            panel_r.get_center() + VecTwo::new(-250.0, 250.0),
            VecTwo::new(100.0, 30.0),
        );

        // pack title
        draw_text(
            &pack_info.display_name,
            VecTwo::new(10.0, 40.0),
            COLOR_WHITE,
            &ui_context.font_header.clone(),
            ui_state,
            ui_context,
        );

        // drop list
        {
            let y: f64 = 90.0;
            let desc_origin = VecTwo::new(10.0, 20.0);

            draw_text(
                "Possible Drops",
                desc_origin + VecTwo::new(0.0, y + 30.0),
                COLOR_WHITE,
                &ui_context.font_body.clone(),
                ui_state,
                ui_context,
            );
            let list = get_fixed_table(pack_info.table_id).list_drops();

            for (j, drop) in list.iter().enumerate() {
                let cost_origin = desc_origin + VecTwo::new(80.0 * j as f64, y + 40.0);
                let icon_size = 40.0;

                let mut icon: u32;
                if inventory.drop_seen(drop) {
                    icon = assets.get_drop_icon(&drop.drop_type);
                } else {
                    icon = assets.image_question_mark.gl_id.unwrap();
                }

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

        // close button
        {
            let close_rect = Rect::new_top_size(VecTwo::new(10.0, 60.0), 100.0, 30.0);

            if draw_button(
                "Close",
                ButtonStyleData::new_outline(None),
                &close_rect,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                update_signals.push(UpdateSignal::SetActivePage(CreatePanelData::Home));
            }
        }

        end_panel(&mut ui_state, ui_context);

        update_signals
    }
}
