use crate::{ui_panels::*, update_signal::*, user_account::*};
use gengar_engine::{account_call::*, networking::*, platform_api::*};

mod purchase_flow;

pub use purchase_flow::*;

pub const REFRESH_KEY: &str = "last_refresh";

pub struct AccountSystem {
    pub user_account: Option<UserAccount>,

    purchase_flow: Option<PurchaseFlow>,

    user_login_call: Option<usize>,
    user_fetch_call: Option<usize>,

    user_fetches_finished: bool,
}

impl AccountSystem {
    pub fn new() -> Self {
        Self {
            user_account: None,
            user_login_call: None,
            user_fetch_call: None,
            purchase_flow: None,

            user_fetches_finished: false,
        }
    }

    pub fn logged_in(&self) -> bool {
        self.user_account.is_some()
    }

    pub fn get_user_auth_token(&self) -> Option<String> {
        if let Some(user_account) = &self.user_account {
            return Some(user_account.access_token.clone());
        }
        return None;
    }

    pub fn user_purchased_base(&self) -> bool {
        self.user_account
            .as_ref()
            .and_then(|uc| Some(uc.did_purchase_base()))
            .unwrap_or(false)
    }

    pub fn user_fetches_finished(&self) -> bool {
        self.user_fetches_finished
    }

    pub fn login_supabase(
        &mut self,
        user_json: &str,
        networking_system: &mut NetworkingSystem,
        platform_api: &PlatformApi,
    ) {
        let user_account = UserAccount::from_supabase_json(user_json, platform_api).unwrap();
        self.user_account = Some(user_account);

        self.start_account_fetch(networking_system);
    }

    pub fn start_purchase(&mut self) {
        if !self.user_purchased_base() {
            self.purchase_flow = Some(PurchaseFlow::Initiate);
        }
    }

    pub fn purchase_in_progress(&self) -> bool {
        self.purchase_flow.is_some()
    }

    pub fn start_account_fetch(&mut self, networking_system: &mut NetworkingSystem) {
        // Can't fetch accout if we're not logged in
        if let Some(user_account) = &self.user_account {
            self.user_fetch_call =
                Some(networking_system.start_call(AccountCall::FetchUserAccount {
                    user_auth_token: user_account.access_token.clone(),
                }))
        }
    }

    /// Checking for an existing refresh token and login using that
    pub fn start_try_login_existing(
        &mut self,
        platform_api: &PlatformApi,
        networking_system: &mut NetworkingSystem,
    ) {
        self.user_fetches_finished = false;

        if let Some(refresh_token) = (platform_api.local_persist_get)(REFRESH_KEY) {
            if refresh_token.len() > 0 {
                let call_id = networking_system.start_call(AccountCall::ExchangeRefreshToken {
                    refresh_token: refresh_token,
                });
                self.user_login_call = Some(call_id);
            } else {
                self.user_fetches_finished = true;
            }
        } else {
            // there is no saved token so there is no user to fetch
            self.user_fetches_finished = true;
        }
    }

    pub fn logout(&mut self, platform_api: &PlatformApi) {
        self.user_account = None;
        self.user_login_call = None;
        (platform_api.local_persist_set)(REFRESH_KEY, "");
    }

    /// Handle networking checks and updating refresh tokens and such
    #[must_use]
    pub fn update(
        &mut self,
        platform_api: &PlatformApi,
        networking_system: &mut NetworkingSystem,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        // check for login
        if let Some(call_id) = self.user_login_call {
            let status = networking_system.get_status(call_id);
            match &status {
                NetworkCallStatus::Success { response } => {
                    self.login_supabase(&response, networking_system, platform_api);
                    self.user_login_call = None;
                }

                NetworkCallStatus::Error { error } => {
                    self.user_fetches_finished = true;
                    self.user_login_call = None;
                }

                // log error somewhere?
                _ => {}
            }
        }

        // check for account fetch
        if let Some(user_account) = &mut self.user_account {
            if let Some(call_id) = self.user_fetch_call {
                let status = networking_system.get_status(call_id);
                match &status {
                    NetworkCallStatus::Success { response } => {
                        user_account.user_info =
                            Some(UserInfo::from_supabase_json(response).unwrap());
                        self.user_fetch_call = None;
                        self.user_fetches_finished = true;
                    }

                    NetworkCallStatus::Error { error } => {
                        self.user_fetches_finished = true;
                        self.user_fetch_call = None;
                    }

                    // log error somewhere?
                    _ => {}
                }
            }
        }

        let mut purchse_update_sigs = self.update_purchase_flow(networking_system, platform_api);
        ret.append(&mut purchse_update_sigs);

        return ret;
    }

    fn update_purchase_flow(
        &mut self,
        networking_system: &mut NetworkingSystem,
        platform_api: &PlatformApi,
    ) -> Vec<UpdateSignal> {
        if let Some(purchase_flow) = &self.purchase_flow {
            match purchase_flow {
                PurchaseFlow::Initiate => {
                    if !self.logged_in() {
                        self.purchase_flow = Some(PurchaseFlow::Register);
                        return vec![UpdateSignal::PushPanel(CreatePanelData::CreateAccount)];
                    } else {
                        self.purchase_flow =
                            Some(PurchaseFlow::start_checkout(networking_system, self));
                    }
                }
                PurchaseFlow::Register => {
                    if self.logged_in() {
                        self.purchase_flow =
                            Some(PurchaseFlow::start_checkout(networking_system, self));
                    }
                }

                PurchaseFlow::StartingCheckout { network_call } => {
                    if self.user_fetches_finished() {
                        if self.user_purchased_base() {
                            self.purchase_flow = None;
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

                                self.purchase_flow = Some(PurchaseFlow::RunningCheckout);
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
}
