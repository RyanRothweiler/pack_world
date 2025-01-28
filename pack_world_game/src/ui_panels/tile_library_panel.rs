use crate::{item::*, state::*, ui_panels::*, UpdateSignal};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct TileLibraryPanel {}

impl TileLibraryPanel {
    pub fn update(
        &mut self,
        common: &UIPanelCommon,
        mut state: &mut UIFrameState,
        inventory: &Inventory,
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
            let disp = format!("{} x {count}", item_type.user_dislay());

            match item_type {
                ItemType::Tile(tile_type) => {
                    if draw_button(
                        &disp,
                        std::line!(),
                        &Rect::new_top_size(
                            VecTwo::new(10.0, 50.0 + (y_offset * i as f64)),
                            50.0,
                            50.0,
                        ),
                        &common.button_font_style,
                        state,
                    ) {
                        ret.push(UpdateSignal::SetPlacingTile(Some(*tile_type)));
                    }
                }
                _ => panic!("invalid item type"),
            };

            i += 1;
        }

        // inventory
        {
            let c: i32 = *inventory.items.get(&ItemType::DirtClod).unwrap_or(&0);

            draw_text(
                &format!("Clods {:?}", c),
                &common.button_font_style,
                VecTwo::new(10.0, 500.0),
                state,
            );
        }

        end_panel(&mut state);

        return ret;
    }
}
