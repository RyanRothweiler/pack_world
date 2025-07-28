use crate::{account_system::*, state::*, ui_panels::*, update_signal::*};
use elara_engine::{account_call::*, networking::*, platform_api::*};

// These happen sequentially
#[derive(Debug)]
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

impl PurchaseFlow {
    pub fn start_checkout(
        networking_system: &mut NetworkingSystem,
        account_system: &mut AccountSystem,
    ) -> Self {
        PurchaseFlow::StartingCheckout {
            network_call: networking_system.start_call(AccountCall::CreateCheckout {
                user_auth_token: account_system.get_user_auth_token().unwrap(),
            }),
        }
    }
}
