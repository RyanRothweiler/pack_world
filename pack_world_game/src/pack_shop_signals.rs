use crate::{pack::*, pack_shop_display::*, state::*, update_signal::*};
use gengar_engine::{
    analytics::*,
    platform_api::*,
    render::{render_pack::*, *},
    state::State as EngineState,
    vectors::*,
};

#[derive(Debug)]
pub enum PackShopSignals {
    Idle { pack_id: PackID },
    Select { pack_id: PackID },
    Open { pack_id: PackID },
    DeselectAll,
    OpenFinished,

    StandardUpateSignal { sigs: Vec<UpdateSignal> },
}

pub fn handle_pack_shop_signals(
    mut signals: Vec<PackShopSignals>,
    gs: &mut State,
    es: &mut EngineState,
    platform_api: &PlatformApi,
) {
    for sig in signals {
        println!("{:?}", sig);

        match sig {
            PackShopSignals::Idle { pack_id } => {
                gs.pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::Idle);
            }
            PackShopSignals::Open { pack_id } => {
                let pack_info = pack_id.get_pack_info();
                pack_info.spend(&mut gs.inventory);

                (platform_api.send_event)(AnalyticsEvent::PackOpen(format!("{:?}", pack_id)));

                gs.opening_pack = true;

                gs.pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::Opening);
            }
            PackShopSignals::OpenFinished => {
                gs.opening_pack = false;

                gs.pack_display_state
                    .iter_mut()
                    .for_each(|p| p.1.set_state(PackShopDisplayState::Idle));
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
                    // This 10 is becuse the camera is rotate a bit and not looking straight down
                    let new_pos = VecThreeFloat::new(
                        pack_info.shop_position.x,
                        20.0,
                        pack_info.shop_position.z + 8.0,
                    );

                    let cam_pack = es
                        .render_system
                        .render_packs
                        .get_mut(&RenderPackID::Shop)
                        .unwrap()
                        .camera
                        .move_target_position = new_pos;
                }
            }
            PackShopSignals::DeselectAll => {
                if !gs.opening_pack {
                    gs.pack_selected = None;

                    gs.pack_display_state
                        .iter_mut()
                        .for_each(|p| p.1.set_state(PackShopDisplayState::Idle));
                }
            }
            PackShopSignals::StandardUpateSignal { sigs } => {
                handle_signals(sigs, gs, es, platform_api);
            }
        }
    }
}
