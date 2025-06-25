#[derive(Debug, Clone)]
pub enum AccountCall {
    SendOTP { email: String },
    VerifyPairingCode { pairing_code: String, email: String },
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
