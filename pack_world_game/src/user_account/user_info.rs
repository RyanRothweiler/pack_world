use elara_engine::{account_call::*, error::*, json::*, networking::*, platform_api::*};

/// Custom user info in the users table
#[derive(Debug)]
pub struct UserInfo {
    pub purchased_game_base: bool,
}

impl UserInfo {
    pub fn from_supabase_json(json_string: &str) -> Result<Self, Error> {
        let jd = elara_engine::json::load(json_string)?;

        let purchased_game_base = jd
            .get_bool(vec!["user".into(), "purchased_game_base".into()])
            .ok_or(Error::ParsingUserInfoJson)?;

        Ok(Self {
            purchased_game_base,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing_infra::*;

    const SUPABASE_USER_RESP: &str = r#"{"user":{"id":"55a15491-7d06-4ee5-b829-d62163ef9326","created_at":"2025-07-01T21:59:34.15264+00:00","purchased_game_base":true}}"#;

    #[test]
    pub fn supa_login() {
        let user_info = UserInfo::from_supabase_json(SUPABASE_USER_RESP.into()).unwrap();
        assert_eq!(user_info.purchased_game_base, true);
    }
}
