use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use elara_engine::{rect::*, render::material::*, typeface::*, ui::*, vectors::*};

pub struct DebugPanel {}

impl DebugPanel {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let panel_r = Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(1400.0, 800.0));
        begin_panel(panel_r, *THEME_PANEL_BG, &mut ui_state, ui_context);

        /*
        // giving gold
        {
            if draw_button(
                "Gold 10,000",
                ButtonStyleData::new_outline(Some(assets.image_gold.gl_id.unwrap())),
                &Rect::new_top_size(VecTwo::new(300.0, 10.0), 50.0, 50.0),
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::GiveGold { amount: 10_000 });
            }
        }
        */

        // tiles
        let grid_rects = get_grid_layout(GridLayoutInfo {
            bounds_width: 1000.0,
            col_count: 10,
            cell_height: 100.0,
            gutter: 10.0,
            cells_count: 100,
        });

        for (i, ty) in ALL_TILE_TYPES.iter().enumerate() {
            let icon = assets.get_item_icon(&ItemType::Tile(*ty));

            if draw_button(
                &format!("{:?}", ty),
                ButtonStyleData::new_shrink(None, Some(icon), 4.0),
                &grid_rects[i],
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::GiveItem {
                    item_type: ItemType::Tile(*ty),
                    count: 1000,
                });
            }
        }

        // items
        for (i, ty) in ALL_ITEM_TYPES.iter().enumerate() {
            let icon = assets.get_item_icon(ty);

            let mut r: Rect = grid_rects[i];
            r.translate(VecTwo::new(0.0, 400.0));

            if draw_button(
                &format!("{:?}", ty),
                ButtonStyleData::new_shrink(Some(icon), None, 4.0),
                &r,
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::GiveItem {
                    item_type: *ty,
                    count: 1000,
                });
            }
        }

        end_panel(&mut ui_state, ui_context);

        return ret;
    }
}
