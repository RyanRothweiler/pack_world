use crate::{
    item::*,
    state::{assets::*, *},
    ui_panels::*,
    UpdateSignal,
};
use gengar_engine::{
    account_call::*, networking::*, rect::*, render::material::*, typeface::*, ui::*, vectors::*,
};

pub struct InventoryPanel {}

impl InventoryPanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(
        &mut self,
        mut ui_state: &mut UIFrameState,
        inventory: &Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        return ret;
    }
}
