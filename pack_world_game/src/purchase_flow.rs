use crate::{account_system::*, state::*, ui_panels::*, update_signal::*};
use gengar_engine::{account_call::*, networking::*, platform_api::*};

// These happen sequentially
pub enum PurchaseFlow {
    /// First entry point to start the purchse flow
    Initiate,

    /// Need to register first. Only happens if needed.
    Register,

    /// Call sent to backend to trigger checkout
    StartingCheckout { network_call: usize },

    /// Recieved checkout url, waiting for user to complete checkout
    RunningCheckout,
}

pub fn update_purchase_flow(
    gs: &mut State,
    networking_system: &mut NetworkingSystem,
    platform_api: &PlatformApi,
) -> Vec<UpdateSignal> {
    if let Some(purchase_flow) = &gs.purchase_flow {
        match purchase_flow {
            PurchaseFlow::Initiate => {
                if !gs.account_system.logged_in() {
                    gs.purchase_flow = Some(PurchaseFlow::Register);
                    return vec![UpdateSignal::PushPanel(CreatePanelData::CreateAccount)];
                }
            }
            PurchaseFlow::Register => {
                if gs.account_system.logged_in() {
                    let pc = PurchaseFlow::StartingCheckout {
                        network_call: networking_system.start_call(AccountCall::CreateCheckout {
                            user_auth_token: gs.account_system.get_user_auth_token().unwrap(),
                        }),
                    };
                    gs.purchase_flow = Some(pc);
                }
            }

            PurchaseFlow::StartingCheckout { network_call } => {
                if gs.account_system.user_fetches_finished() {
                    if gs.account_system.user_purchased_base() {
                        gs.purchase_flow = None;
                        return vec![];
                    }

                    let call_status = networking_system.get_status(*network_call);

                    match &call_status {
                        NetworkCallStatus::Success { response } => {
                            let rj = gengar_engine::json::load(response).unwrap();
                            (platform_api.open_url)(
                                rj.get(vec!["url".into()]).unwrap().as_string().unwrap(),
                                false,
                            );

                            gs.purchase_flow = Some(PurchaseFlow::RunningCheckout);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    return vec![];
}
