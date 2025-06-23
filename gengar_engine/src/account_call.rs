#[derive(Debug, Clone)]
pub enum AccountCall {
    SendOTP { email: String },
}

#[derive(Debug, Clone)]
pub enum AccountError {
    EmailInvalid,
    UnknownError { response: String },
}
