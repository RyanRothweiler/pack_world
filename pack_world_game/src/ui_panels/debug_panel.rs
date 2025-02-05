use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct DebugPanel {}

impl UIPanelLifecycle for DebugPanel {
    fn update(
        &mut self,
        common: &UIPanelCommon,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let panel_r = Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(1400.0, 800.0));
        begin_panel(panel_r, BG_COLOR, &mut ui_state);

        let item_types: Vec<ItemType> = vec![
            ItemType::Tile(TileType::Dirt),
            ItemType::Tile(TileType::Grass),
            ItemType::Tile(TileType::Boulder),
            ItemType::Tile(TileType::OakTree),
            ItemType::OakLog,
        ];

        let y_offset: f64 = 80.0;
        let mut i: i32 = 0;
        for ty in item_types {
            let y: f64 = 50.0 + (y_offset * i as f64);

            let icon = assets.get_item_icon(&ty);

            if draw_button(
                &format!("{:?}", ty),
                Some(icon),
                &Rect::new_top_size(VecTwo::new(10.0, y), 50.0, 50.0),
                &common.button_font_style,
                ui_state,
                std::line!(),
            ) {
                ret.push(UpdateSignal::GiveItem {
                    item_type: ty,
                    count: 1000,
                });
            }

            i += 1;
        }

        end_panel(&mut ui_state);

        return ret;
    }
}
