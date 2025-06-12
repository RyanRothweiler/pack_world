use crate::{pack::*, pack_shop_display::*, state::*};
use gengar_engine::{platform_api::*, state::State as EngineState};

#[derive(Debug)]
pub enum PackShopSignals {
    Hover { pack_id: PackID },
    Idle { pack_id: PackID },
    MouseDown { pack_id: PackID },
}

pub fn handle_pack_shop_signals(
    mut signals: Vec<PackShopSignals>,
    gs: &mut State,
    es: &EngineState,
    platform_api: &PlatformApi,
) {
    for sig in signals {
        println!("{:?}", sig);

        match sig {
            PackShopSignals::Hover { pack_id } => {
                gs.pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::Hover);
            }
            PackShopSignals::Idle { pack_id } => {
                gs.pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::Idle);
            }
            PackShopSignals::MouseDown { pack_id } => {
                gs.pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::MouseDown);
            }
        }
    }
}
