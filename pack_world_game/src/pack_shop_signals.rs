use crate::{game_mode::*, pack::*, pack_shop_display::*, state::*, update_signal::*};
use elara_engine::{
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
    mut game_mode_shop: &mut GameModeShop,
    mut signals: Vec<PackShopSignals>,
    es: &mut EngineState,
    inventory: &mut Inventory,
    platform_api: &PlatformApi,
) -> Vec<UpdateSignal> {
    let mut ret: Vec<UpdateSignal> = vec![];

    for sig in signals {
        println!("{:?}", sig);

        match sig {
            PackShopSignals::Idle { pack_id } => {
                game_mode_shop
                    .pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::Idle);
            }
            PackShopSignals::Open { pack_id } => {
                let pack_info = pack_id.get_pack_info();
                pack_info.spend(inventory);

                (platform_api.send_event)(AnalyticsEvent::PackOpen(format!("{:?}", pack_id)));

                game_mode_shop.opening_pack = true;

                game_mode_shop
                    .pack_display_state
                    .get_mut(&pack_id)
                    .unwrap()
                    .set_state(PackShopDisplayState::Opening);
            }
            PackShopSignals::OpenFinished => {
                game_mode_shop.opening_pack = false;

                game_mode_shop
                    .pack_display_state
                    .iter_mut()
                    .for_each(|p| p.1.set_state(PackShopDisplayState::Idle));
            }
            PackShopSignals::Select { pack_id } => {
                let pack_info = pack_id.get_pack_info();

                game_mode_shop.pack_selected = Some(pack_id);

                game_mode_shop
                    .pack_display_state
                    .iter_mut()
                    .for_each(|p| p.1.set_state(PackShopDisplayState::Hidden));

                game_mode_shop
                    .pack_display_state
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
                if !game_mode_shop.opening_pack {
                    game_mode_shop.pack_selected = None;

                    game_mode_shop
                        .pack_display_state
                        .iter_mut()
                        .for_each(|p| p.1.set_state(PackShopDisplayState::Idle));
                }
            }
            PackShopSignals::StandardUpateSignal { sigs } => {
                let mut s: Vec<UpdateSignal> = sigs;
                ret.append(&mut s);
            }
        }
    }

    ret
}
