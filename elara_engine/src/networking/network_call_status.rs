use crate::account_call::*;

/// State of the network call.
/// This does not say if the call was successful or not, only that it was indeed sent.
#[derive(Clone, Debug)]
pub enum NetworkCallStatus {
    /// Waiting for engine to send call
    Waiting,

    /// Platform is sending call
    Sending,

    /// Call has been sent successfully
    Success { response: String },

    /// Call was sent but resulted in an error
    Error { error: AccountError },
}

impl NetworkCallStatus {
    pub fn display(&self) -> String {
        match self {
            NetworkCallStatus::Waiting | NetworkCallStatus::Sending => "Sending...".into(),
            NetworkCallStatus::Success { response: _ } => "Success".into(),
            NetworkCallStatus::Error { error } => format!("Error: {}", error.display()),
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            NetworkCallStatus::Waiting
            | NetworkCallStatus::Sending
            | NetworkCallStatus::Success { response: _ } => false,
            NetworkCallStatus::Error { error } => true,
        }
    }

    pub fn is_success(&self) -> bool {
        match self {
            NetworkCallStatus::Waiting
            | NetworkCallStatus::Sending
            | NetworkCallStatus::Error { error: _ } => false,
            NetworkCallStatus::Success { response: _ } => true,
        }
    }
}
