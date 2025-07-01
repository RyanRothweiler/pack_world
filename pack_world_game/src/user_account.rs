use crate::account_system::*;
use gengar_engine::{account_call::*, error::*, json::*, networking::*, platform_api::*};

pub mod user_info;

pub use user_info::*;

/// Supabase standard user account
#[derive(Debug)]
pub struct UserAccount {
    pub email: String,
    pub user_id: String,
    pub access_token: String,

    pub user_info: Option<UserInfo>,

    refresh_token: String,
}

impl UserAccount {
    pub fn from_supabase_json(
        json_string: &str,
        platform_api: &PlatformApi,
    ) -> Result<Self, Error> {
        let jd = gengar_engine::json::load(json_string)?;

        let email = jd
            .get_string(vec!["user".into(), "email".into()])
            .unwrap_or("missing".into());
        let user_id = jd
            .get_string(vec!["user".into(), "id".into()])
            .unwrap_or("missing".into());

        let access_token = jd
            .get_string(vec!["access_token".into()])
            .unwrap_or("missing".into());
        let refresh_token = jd
            .get_string(vec!["refresh_token".into()])
            .unwrap_or("missing".into());

        let mut ret = Self {
            email: email,
            user_id: user_id,

            refresh_token: refresh_token.clone(),
            access_token: access_token,

            user_info: None,
        };

        ret.set_refresh_token(&refresh_token, platform_api);

        Ok(ret)
    }

    fn set_refresh_token(&mut self, new_refresh: &str, platform_api: &PlatformApi) {
        (platform_api.local_persist_set)(REFRESH_KEY, new_refresh);
        self.refresh_token = new_refresh.into();
    }

    pub fn did_purchase_base(&self) -> bool {
        if let Some(user_info) = &self.user_info {
            return user_info.purchased_game_base;
        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    const SUPABASE_USER_RESP: &str = r#"{
        "access_token": "eyJhbGciOiJIUzI1NiIsImtpZCI6Ii9OWk93TkNmTHhKTjFiZmQiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL3FxaWJxamxndmtoenlyamFhYnZnLnN1cGFiYXNlLmNvL2F1dGgvdjEiLCJzdWIiOiI1YzFjODdkYy1hMTc4LTQxZmEtOTg1MS0yYzBhYzA1NmMzMjUiLCJhdWQiOiJhdXRoZW50aWNhdGVkIiwiZXhwIjoxNzUwODI3MTkyLCJpYXQiOjE3NTA4MjM1OTIsImVtYWlsIjoicnlhbnJvdGh3ZWlsZXJAZ21haWwuY29tIiwicGhvbmUiOiIiLCJhcHBfbWV0YWRhdGEiOnsicHJvdmlkZXIiOiJlbWFpbCIsInByb3ZpZGVycyI6WyJlbWFpbCJdfSwidXNlcl9tZXRhZGF0YSI6eyJlbWFpbCI6InJ5YW5yb3Rod2VpbGVyQGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaG9uZV92ZXJpZmllZCI6ZmFsc2UsInN1YiI6IjVjMWM4N2RjLWExNzgtNDFmYS05ODUxLTJjMGFjMDU2YzMyNSJ9LCJyb2xlIjoiYXV0aGVudGljYXRlZCIsImFhbCI6ImFhbDEiLCJhbXIiOlt7Im1ldGhvZCI6Im90cCIsInRpbWVzdGFtcCI6MTc1MDgyMzU5Mn1dLCJzZXNzaW9uX2lkIjoiNWNiNzliYWItNGZiMi00MTljLTgzMmItODMyNWJkZDYwNjEzIiwiaXNfYW5vbnltb3VzIjpmYWxzZX0.v53bFIVlz-5M0CdbObRl2sJZ3YwxSzBrlPObXviJ1IU",
        "token_type": "bearer",
        "expires_in": 3600,
        "expires_at": 1750827192,
        "refresh_token": "623w4hnfx36u",
        "user": {
            "id": "5c1c87dc-a178-41fa-9851-2c0ac056c325",
            "aud": "authenticated",
            "role": "authenticated",
            "email": "ryanrothweiler@gmail.com",
            "email_confirmed_at": "2025-06-22T23:48:21.931292Z",
            "phone": "",
            "confirmation_sent_at": "2025-06-22T23:46:55.57186Z",
            "confirmed_at": "2025-06-22T23:48:21.931292Z",  
            "recovery_sent_at": "2025-06-25T03:52:58.442765Z",
            "last_sign_in_at": "2025-06-25T03:53:12.411520747Z",
            "app_metadata": {
                "provider": "email",
                "providers": [
                    "email"
                ]
            },
            "user_metadata": {
                "email": "ryanrothweiler@gmail.com",
                "email_verified": true,
                "phone_verified": false,
                "sub": "5c1c87dc-a178-41fa-9851-2c0ac056c325"
            },
            "identities": [
                {
                    "identity_id": "d004cc7b-1345-4ac6-92da-fb40c2b8465d",
                    "id": "5c1c87dc-a178-41fa-9851-2c0ac056c325",
                    "user_id": "5c1c87dc-a178-41fa-9851-2c0ac056c325",
                    "identity_data": {
                        "email": "ryanrothweiler@gmail.com",
                        "email_verified": true,
                        "phone_verified": false,
                        "sub": "5c1c87dc-a178-41fa-9851-2c0ac056c325"
                    },
                    "provider": "email",
                    "last_sign_in_at": "2025-06-22T23:46:55.545039Z",
                    "created_at": "2025-06-22T23:46:55.545958Z",
                    "updated_at": "2025-06-22T23:46:55.545958Z",
                    "email": "ryanrothweiler@gmail.com"
                }
            ],
            "created_at": "2025-06-22T23:46:55.495565Z",
            "updated_at": "2025-06-25T03:53:12.416449Z",
            "is_anonymous": false
        }"#;

    #[test]
    pub fn supa_login() {
        let plat_api = windows_plaform_api();

        let user_account =
            UserAccount::from_supabase_json(SUPABASE_USER_RESP.into(), &plat_api).unwrap();
        assert_eq!(user_account.email, "ryanrothweiler@gmail.com");
        assert_eq!(user_account.user_id, "5c1c87dc-a178-41fa-9851-2c0ac056c325");
    }
}
