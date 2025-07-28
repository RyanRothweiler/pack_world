use crate::{account_call::*, util::incrementing_map::*};

pub mod network_call_status;

pub use network_call_status::*;

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
