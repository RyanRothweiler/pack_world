use crate::{
    pack::*,
    state::{assets, player_state::*, *},
    ui_panels::{nav_tabs_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

#[derive(Clone, Copy, Debug)]
pub enum Tab {
    Shop,
    Inventory,
}

pub struct HomePanel {
    pub tab: Tab,

    pub ui_nav_tabs: Box<UIPanel>,
    pub ui_shop: Box<UIPanel>,
    pub ui_inventory: Box<UIPanel>,
}

impl HomePanel {
    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
        player_state: &PlayerState,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 0.0), 400.0, 200.0),
            BG_COLOR,
            &mut ui_state,
            ui_context,
        );

        // draw gold
        {
            let mut gold_rect = Rect::new(VecTwo::new(0.0, 0.0), VecTwo::new(50.0, 50.0));
            gold_rect.translate(VecTwo::new(0.0, 100.0));
            draw_image(
                gold_rect,
                assets.image_gold.gl_id.unwrap(),
                COLOR_WHITE,
                ui_state,
                ui_context,
            );

            draw_text(
                &format!("{}", inventory.gold),
                gold_rect.bottom_right + VecTwo::new(10.0, -10.0),
                ui_state,
                ui_context,
            );
        }

        update_signals.append(&mut self.ui_nav_tabs.update(
            ui_state,
            inventory,
            assets,
            player_state,
            ui_context,
        ));

        match self.tab {
            Tab::Shop => update_signals.append(&mut self.ui_shop.update(
                ui_state,
                inventory,
                assets,
                player_state,
                ui_context,
            )),
            Tab::Inventory => update_signals.append(&mut self.ui_inventory.update(
                ui_state,
                inventory,
                assets,
                player_state,
                ui_context,
            )),
        };

        // Consume home panel tab switch
        update_signals.retain(|sig| match sig {
            UpdateSignal::HomePanelTabChange(tab) => {
                self.tab = *tab;
                false
            }
            _ => true,
        });

        end_panel(&mut ui_state, ui_context);

        update_signals
    }
}
