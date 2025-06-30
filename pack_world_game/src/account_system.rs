use crate::user_account::*;
use gengar_engine::{account_call::*, networking::*, platform_api::*};

pub const REFRESH_KEY: &str = "last_refresh";

pub struct AccountSystem {
    pub user_account: Option<UserAccount>,
    pub user_fetch_network_id: Option<usize>,
}

impl AccountSystem {
    pub fn new() -> Self {
        Self {
            user_account: None,
            user_fetch_network_id: None,
        }
    }

    pub fn login_supabase(&mut self, user_json: &str, platform_api: &PlatformApi) {
        let user_account = UserAccount::from_supabase_json(user_json, platform_api).unwrap();
        self.user_account = Some(user_account);
    }

    /// Checking for an existing refresh token and login using that
    pub fn start_try_login_existing(
        &mut self,
        platform_api: &PlatformApi,
        networking_system: &mut NetworkingSystem,
    ) {
        if let Some(refresh_token) = (platform_api.local_persist_get)(REFRESH_KEY) {
            let call_id = networking_system.start_call(AccountCall::ExchangeRefreshToken {
                refresh_token: refresh_token,
            });
            self.user_fetch_network_id = Some(call_id);
        }
    }

    /// Handle networking checks and updating refresh tokens and such
    pub fn update(&mut self, platform_api: &PlatformApi, networking_system: &mut NetworkingSystem) {
        // check for account fetch
        if let Some(call_id) = self.user_fetch_network_id {
            let status = networking_system.get_status(call_id);
            match &status {
                NetworkCallStatus::Success { response } => {
                    self.login_supabase(&response, platform_api);
                }

                // log error somewhere?
                _ => {}
            }
        }
    }
}
