use crate::{
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct TileLibraryPanel {}

impl TileLibraryPanel {
    pub fn update(
        &mut self,
        common: &UIPanelCommon,
        mut state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        // begin panel
        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 150.0), 400.0, state.resolution.y),
            BG_COLOR,
            &mut state,
        );

        let y_offset: f64 = 80.0;
        let mut i: i32 = 0;
        for (item_type, count) in &inventory.items {
            if *count == 0 {
                continue;
            }

            let disp = format!("{} x {count}", item_type.user_dislay());
            let y: f64 = 50.0 + (y_offset * i as f64);

            let icon = assets.get_item_icon(item_type);

            match item_type {
                ItemType::Tile(tile_type) => {
                    if draw_button(
                        &disp,
                        Some(icon),
                        &Rect::new_top_size(VecTwo::new(10.0, y), 50.0, 50.0),
                        &common.button_font_style,
                        state,
                        std::line!(),
                    ) {
                        ret.push(UpdateSignal::SetPlacingTile(Some(*tile_type)));
                    }
                }
                ItemType::DirtClod => {
                    draw_image(
                        Rect::new_top_size(VecTwo::new(10.0, y), 50.0, 50.0),
                        icon,
                        COLOR_WHITE,
                        state,
                    );

                    draw_text(
                        &disp,
                        &common.button_font_style,
                        VecTwo::new(10.0, y),
                        state,
                    );
                }
            };

            i += 1;
        }

        end_panel(&mut state);

        return ret;
    }
}
