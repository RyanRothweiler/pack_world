use crate::{account_call::*, util::incrementing_map::*};

/// State of the network call.
/// This does not say if the call was successful or not, only that it was indeed sent.
#[derive(Clone, Debug)]
pub enum NetworkCallStatus {
    /// Waiting for engine to send call
    Waiting,

    /// Platform is sending call
    Sending,

    /// Call has been sent successfully
    Success,

    /// Call was sent but resulted in an error
    Error { error: AccountError },
}

/*
/// Status of the network call.
/// Error or successful?
#[derive(Clone)]
pub enum NetworkCallStatus {
    Success,

    // todo generalize this past account errors
    Error { error: AccountError },
}
    */

pub struct NetworkCall {
    pub status: NetworkCallStatus,

    // todo generalize this. This is specific to supa base account calls currently
    pub call: AccountCall,
}

impl NetworkCall {
    pub fn new(call: AccountCall) -> Self {
        Self {
            call,
            status: NetworkCallStatus::Waiting,
        }
    }
}

pub struct NetworkingSystem {
    // NOTE there is no mechanism for disposing of these yet.
    pub network_calls: IncrementingMap<NetworkCall>,
}

impl NetworkingSystem {
    pub fn new() -> Self {
        Self {
            network_calls: IncrementingMap::new(),
        }
    }

    pub fn start_call(&mut self, call_info: AccountCall) -> usize {
        self.network_calls.push(NetworkCall::new(call_info))
    }

    pub fn get_status(&self, id: usize) -> NetworkCallStatus {
        return self
            .network_calls
            .data
            .get(&id)
            .expect("That network id is missing")
            .status
            .clone();
    }
}
