use crate::{
    pack::*,
    state::{assets, *},
    ui_panels::{nav_tabs_panel::*, *},
    UpdateSignal,
};
use gengar_engine::{font::*, rect::*, render::material::*, ui::*, vectors::*};

pub struct OpenPackPanel {
    pub pack_id: PackID,
}

impl OpenPackPanel {
    pub fn new(pack_id: PackID) -> Self {
        Self { pack_id }
    }
}

impl UIPanelLifecycle for OpenPackPanel {
    fn update(
        &mut self,
        common: &UIPanelCommon,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &Assets,
    ) -> Vec<UpdateSignal> {
        let mut update_signals: Vec<UpdateSignal> = vec![];

        begin_panel(
            Rect::new_center(ui_state.resolution * 0.5, VecTwo::new(1400.0, 800.0)),
            BG_COLOR,
            &mut ui_state,
        );

        let pack_info: &Pack = get_pack_info(self.pack_id);

        let pack_image_size = VecTwo::new(448.0, 604.0) * 0.35;

        let button_rect = Rect::new_top_size(
            VecTwo::new(10.0, 50.0),
            pack_image_size.x,
            pack_image_size.y,
        );

        if draw_button(
            &pack_info.display_name,
            assets.image_pack_starter.gl_id,
            &button_rect,
            &common.button_font_style,
            ui_state,
            std::line!(),
        ) {
            println!("open!");
            // return vec![Update5Signal::OpenPack(PackID::Starter)];
        }

        /*
        for i in 0..4 {
            let pull_item = pack_info.pull(&gs.inventory).unwrap();
            println!("Gave item {:?}", pull_item);

            gs.inventory.add_item(pull_item, 1).unwrap();
        }
        */

        end_panel(&mut ui_state);

        update_signals
    }
}
