use crate::user_account::*;

pub struct AccountSystem {
    pub user_account: Option<UserAccount>,
}

impl AccountSystem {
    pub fn new() -> Self {
        Self { user_account: None }
    }

    pub fn login_supabase(&mut self, user_json: String) {
        let user_account = UserAccount::from_supabase_json(user_json).unwrap();
        self.user_account = Some(user_account);
    }
}
