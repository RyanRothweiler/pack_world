#![allow(dead_code)]

use crate::{
    item::*,
    pack::*,
    state::{assets, inventory::*, *},
    ui_panels::{nav_tabs_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{color::*, font::*, rect::*, render::material::*, ui::*, vectors::*};

struct PullDisplay {
    pub item_id: ItemType,
    pub pos: VecTwo,
}

pub struct OpenPackPanel {
    pub pack_id: PackID,
    pub items_remaining: i32,

    pulls: Vec<PullDisplay>,
}

impl OpenPackPanel {
    pub fn new(pack_id: PackID) -> Self {
        let pack_info: &Pack = get_pack_info(pack_id);

        Self {
            pack_id,
            items_remaining: pack_info.content_count,
            pulls: vec![],
        }
    }
}

impl UIPanelLifecycle for OpenPackPanel {
    fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        let panel_r = Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(1400.0, 800.0));

        begin_panel(panel_r, BG_COLOR, &mut ui_state, ui_context);

        let pack_info: &Pack = get_pack_info(self.pack_id);

        let pack_image_size = VecTwo::new(448.0, 604.0) * 0.35;

        let button_rect = Rect::new_center(
            panel_r.get_center() + VecTwo::new(-250.0, 100.0),
            pack_image_size,
        );
        let close_rect = Rect::new_center(
            panel_r.get_center() + VecTwo::new(-250.0, 250.0),
            VecTwo::new(100.0, 30.0),
        );

        if self.items_remaining > 0 {
            if draw_button(
                &pack_info.display_name,
                assets.image_pack_starter.gl_id,
                &button_rect,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                // pull item from pack and give
                let pull_item = pack_info.pull().unwrap();
                update_signals.push(UpdateSignal::GiveItem {
                    item_type: pull_item,
                    count: 1,
                });

                self.items_remaining -= 1;

                self.pulls.push(PullDisplay {
                    item_id: pull_item,
                    pos: button_rect.get_center(),
                });
            }
        } else {
            if draw_button(
                "Close",
                None,
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
            let x_offset: f64 = 120.0;
            let mut i: i32 = 0;

            for p in &mut self.pulls {
                let x: f64 = 50.0 + (x_offset * i as f64);
                p.pos = VecTwo::lerp(p.pos, VecTwo::new(x, 200.0), 0.25);

                let icon = assets.get_item_icon(&p.item_id);

                draw_image(
                    Rect::new_center(p.pos, VecTwo::new(50.0, 50.0)),
                    icon,
                    COLOR_WHITE,
                    ui_state,
                    ui_context,
                );

                i += 1;
            }
        }

        end_panel(&mut ui_state, ui_context);

        update_signals
    }
}
