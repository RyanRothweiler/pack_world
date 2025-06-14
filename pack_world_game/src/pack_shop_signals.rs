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
                let pack_info = pack_id.get_pack_info();

                gs.pack_selected = Some(pack_id);

                gs.pack_display_state
                    .iter_mut()
                    .for_each(|p| p.1.set_state(PackShopDisplayState::Hidden));

                gs.pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::Selected);

                // Update camera
                {
                    gs.target_camera_pos.x = pack_info.shop_position.x + 5.0;
                    gs.target_camera_pos.y = 20.0;

                    // This 10 is becuse the camera is rotate a bit and not looking straight down
                    gs.target_camera_pos.z = pack_info.shop_position.z + 8.0;
                }
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
