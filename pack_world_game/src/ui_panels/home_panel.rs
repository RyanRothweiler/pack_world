use crate::{
    account_system::*,
    pack::*,
    state::{assets, *},
    ui_panels::{nav_tabs_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{
    platform_api::*,
    rect::*,
    render::{material::*, render_command::*},
    typeface::*,
    ui::*,
    vectors::*,
};

pub struct HomePanel {
    pub current_mode: GameMode,

    pub ui_nav_tabs: Box<UIPanel>,
    pub ui_shop: Box<UIPanel>,
    pub ui_inventory: Box<UIPanel>,
}

impl HomePanel {
    pub fn update(
        &mut self,
        account_system: &AccountSystem,
        networking_system: &mut NetworkingSystem,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        /*
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
                COLOR_WHITE,
                &ui_context.font_body.clone(),
                ui_state,
                ui_context,
            );
        }
        */

        let mut nav_update_sigs = match self.ui_nav_tabs.as_mut() {
            UIPanel::NavTabs(state) => state.update(
                ui_state,
                account_system,
                inventory,
                assets,
                ui_context,
                self.current_mode,
            ),
            _ => {
                panic!("Only nav tab panel shoul be here");
            }
        };
        update_signals.append(&mut nav_update_sigs);

        match self.current_mode {
            /*
            Tab::Shop => update_signals.append(&mut self.ui_shop.update(
                ui_state,
                inventory,
                assets,
                ui_context,
                platform_api,
            )),
            */
            GameMode::World => update_signals.append(&mut self.ui_inventory.update(
                account_system,
                networking_system,
                ui_state,
                inventory,
                assets,
                ui_context,
                platform_api,
            )),
            _ => {}
        };

        // Consume home panel tab switch
        update_signals.retain(|sig| match sig {
            UpdateSignal::SetGameMode { new_mode } => {
                self.current_mode = *new_mode;
                true
            }
            _ => true,
        });

        update_signals
    }
}
