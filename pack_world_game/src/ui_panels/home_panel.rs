use crate::{
    pack::*,
    state::{assets, *},
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

    pub ui_nav_tabs: UIPanel,
    pub ui_shop: UIPanel,
    pub ui_inventory: UIPanel,
}

impl UIPanelLifecycle for HomePanel {
    fn update(
        &mut self,
        common: &UIPanelCommon,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
    ) -> Vec<UpdateSignal> {
        begin_panel(
            Rect::new_top_size(VecTwo::new(0.0, 0.0), 400.0, 100.0),
            BG_COLOR,
            &mut ui_state,
        );

        let mut update_signals: Vec<UpdateSignal> = vec![];

        update_signals.append(
            &mut self
                .ui_nav_tabs
                .lifecycle
                .update(common, ui_state, inventory, assets),
        );

        match self.tab {
            Tab::Shop => update_signals.append(
                &mut self
                    .ui_shop
                    .lifecycle
                    .update(common, ui_state, inventory, assets),
            ),
            Tab::Inventory => update_signals.append(
                &mut self
                    .ui_inventory
                    .lifecycle
                    .update(common, ui_state, inventory, assets),
            ),
        };

        // Consume home panel tab switch
        update_signals.retain(|sig| match sig {
            UpdateSignal::HomePanelTabChange(tab) => {
                self.tab = *tab;
                false
            }
            _ => true,
        });

        end_panel(&mut ui_state);

        update_signals
    }
}
