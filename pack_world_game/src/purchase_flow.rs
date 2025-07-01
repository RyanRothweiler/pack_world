use crate::state::*;
use gengar_engine::{networking::*, platform_api::*};

pub enum PurchaseFlow {
    /// Call sent to backend to trigger checkout
    StartingCheckout { network_call: usize },

    /// Recieved checkout url, waiting for user to complete checkout
    RunningCheckout,
}

pub fn update_purchase_flow(
    gs: &mut State,
    networking_system: &mut NetworkingSystem,
    platform_api: &PlatformApi,
) {
    if let Some(purchase_flow) = &gs.purchase_flow {
        match purchase_flow {
            PurchaseFlow::StartingCheckout { network_call } => {
                let call_status = networking_system.get_status(*network_call);

                match &call_status {
                    NetworkCallStatus::Success { response } => {
                        let rj = gengar_engine::json::load(response).unwrap();
                        (platform_api.open_url)(
                            rj.get(vec!["url".into()]).unwrap().as_string().unwrap(),
                        );

                        gs.purchase_flow = Some(PurchaseFlow::RunningCheckout);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
