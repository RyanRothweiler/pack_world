#[derive(Debug, Clone)]
pub enum AccountCall {
    // supabase calls
    SendOTP { email: String },
    VerifyPairingCode { pairing_code: String, email: String },
    ExchangeRefreshToken { refresh_token: String },

    // supbase edge function calls
    FetchUserAccount { user_auth_token: String },
    CreateCheckout,
}

#[derive(Debug, Clone)]
pub enum AccountError {
    EmailInvalid,
    UnknownError { response: String },
}

impl AccountError {
    pub fn display(&self) -> String {
        match self {
            AccountError::EmailInvalid => "Email Invalid".into(),
            AccountError::UnknownError { response } => format!("Unknown error: {}", response),
        }
    }
}
