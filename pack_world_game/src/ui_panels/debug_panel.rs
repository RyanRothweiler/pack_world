use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{rect::*, render::material::*, typeface::*, ui::*, vectors::*};

pub struct DebugPanel {}

impl DebugPanel {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        let panel_r = Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(1400.0, 800.0));
        begin_panel(panel_r, BG_COLOR, &mut ui_state, ui_context);

        let item_types: Vec<ItemType> = vec![
            ItemType::DirtClod,
            ItemType::Stick,
            ItemType::Tile(TileType::Dirt),
            ItemType::Tile(TileType::Grass),
            ItemType::Tile(TileType::Water),
            ItemType::Tile(TileType::Newt),
            ItemType::Tile(TileType::OakTree),
            ItemType::Tile(TileType::TallGrass),
            ItemType::Tile(TileType::Shrub),
            ItemType::MudBaby,
            ItemType::OakLog,
            ItemType::Baby,
            ItemType::DragonEgg,
        ];

        // giving old
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

        let y_offset: f64 = 80.0;
        let mut i: i32 = 0;
        for ty in item_types {
            let y: f64 = 50.0 + (y_offset * i as f64);

            let icon = assets.get_item_icon(&ty);

            if draw_button(
                &format!("{:?}", ty),
                ButtonStyleData::new_outline(Some(icon)),
                &Rect::new_top_size(VecTwo::new(10.0, y), 50.0, 50.0),
                ui_state,
                std::line!(),
                ui_context,
            ) {
                ret.push(UpdateSignal::GiveItem {
                    item_type: ty,
                    count: 1000,
                });
            }

            i += 1;
        }

        end_panel(&mut ui_state, ui_context);

        return ret;
    }
}
