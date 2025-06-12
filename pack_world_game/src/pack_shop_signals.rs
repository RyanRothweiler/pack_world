use crate::{pack::*, pack_shop_display::*, state::*};
use gengar_engine::{platform_api::*, state::State as EngineState};

#[derive(Debug)]
pub enum PackShopSignals {
    Hover { pack_id: PackID },
    Idle { pack_id: PackID },
    Select { pack_id: PackID },
    DeselectAll,
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
            PackShopSignals::Select { pack_id } => {
                gs.pack_selected = Some(pack_id);

                gs.pack_display_state
                    .iter_mut()
                    .for_each(|p| p.1.set_state(PackShopDisplayState::Hidden));

                gs.pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::Selected);
            }
            PackShopSignals::DeselectAll => {
                gs.pack_selected = None;

                gs.pack_display_state
                    .iter_mut()
                    .for_each(|p| p.1.set_state(PackShopDisplayState::Idle));
            }
        }
    }
}
