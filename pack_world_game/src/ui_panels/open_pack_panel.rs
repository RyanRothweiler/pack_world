#![allow(dead_code)]

use crate::{
    drop_table::*,
    item::*,
    pack::*,
    state::{assets, inventory::*, *},
    ui_panels::{nav_tabs_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{
    color::*, platform_api::*, rect::*, render::material::*, typeface::*, ui::*, vectors::*,
};

struct PullDisplay {
    pub drop: Drop,
    pub pos: VecTwo,
}

pub struct OpenPackPanel {
    pub pack_id: PackID,
    pub items_remaining: i32,

    pulls: Vec<PullDisplay>,
}

impl OpenPackPanel {
    pub fn new(pack_id: PackID) -> Self {
        let pack_info: &Pack = pack_id.get_pack_info();

        Self {
            pack_id,
            items_remaining: pack_info.content_count,
            pulls: vec![],
        }
    }
}

impl OpenPackPanel {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        let panel_r = Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(1400.0, 800.0));

        begin_panel(panel_r, BG_COLOR, &mut ui_state, ui_context);

        let pack_info: &Pack = self.pack_id.get_pack_info();

        let pack_image_size = VecTwo::new(448.0, 604.0) * 0.35;

        let button_rect = Rect::new_center(VecTwo::new(100.0, 150.0), pack_image_size);
        let close_rect = Rect::new_center(VecTwo::new(100.0, 150.0), VecTwo::new(100.0, 30.0));

        if self.items_remaining > 0 {
            if draw_button(
                &pack_info.display_name,
                ButtonStyleData::new_outline(Some(assets.get_pack_icon(&self.pack_id))),
                &button_rect,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                // pull item from pack and give
                let pull = pack_info.pull(platform_api);
                update_signals.push(UpdateSignal::GiveDrop(pull));

                self.items_remaining -= 1;

                self.pulls.push(PullDisplay {
                    drop: pull,
                    pos: button_rect.get_center(),
                });
            }
        } else {
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

        // render pull displays
        {
            let icon_size: f64 = 70.0;
            let x_offset: f64 = 120.0;

            let center = panel_r.width() * 0.5;
            let total_width = x_offset * (pack_info.content_count as f64 - 1.0);
            let start = center - (total_width * 0.5);

            let mut i: i32 = 0;

            for p in &mut self.pulls {
                let x: f64 = start + (x_offset * i as f64);
                p.pos = VecTwo::lerp(p.pos, VecTwo::new(x, 200.0), 0.25);

                let icon = assets.get_drop_icon(&p.drop.drop_type);

                draw_image(
                    Rect::new_center(p.pos, VecTwo::new(icon_size, icon_size)),
                    icon,
                    COLOR_WHITE,
                    ui_state,
                    ui_context,
                );

                if p.drop.amount > 1 {
                    draw_text(
                        &format!("{:?}", p.drop.amount),
                        p.pos,
                        COLOR_WHITE,
                        &ui_context.font_body.clone(),
                        ui_state,
                        ui_context,
                    );
                }

                i += 1;
            }
        }

        end_panel(&mut ui_state, ui_context);

        update_signals
    }
}
