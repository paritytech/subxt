use smoldot_light as sl;
use std::sync::{Arc, Mutex};

/// This wraps [`smoldot_light::Client`] so that it can be cloned and shared.
#[derive(Clone)]
pub struct SharedClient<TPlat: sl::platform::PlatformRef, TChain = ()> {
    client: Arc<Mutex<sl::Client<TPlat, TChain>>>,
}

impl<TPlat: sl::platform::PlatformRef, TChain> From<sl::Client<TPlat, TChain>>
    for SharedClient<TPlat, TChain>
{
    fn from(client: sl::Client<TPlat, TChain>) -> Self {
        SharedClient {
            client: Arc::new(Mutex::new(client)),
        }
    }
}

impl<TPlat: sl::platform::PlatformRef, TChain> SharedClient<TPlat, TChain> {
    /// Delegates to [`smoldot_light::Client::json_rpc_request()`].
    pub(crate) fn json_rpc_request(
        &self,
        json_rpc_request: impl Into<String>,
        chain_id: sl::ChainId,
    ) -> Result<(), sl::HandleRpcError> {
        self.client
            .lock()
            .expect("mutex should not be poisoned")
            .json_rpc_request(json_rpc_request, chain_id)
    }

    /// Delegates to [`smoldot_light::Client::add_chain()`].
    pub(crate) fn add_chain(
        &self,
        config: sl::AddChainConfig<'_, TChain, impl Iterator<Item = sl::ChainId>>,
    ) -> Result<sl::AddChainSuccess, sl::AddChainError> {
        self.client
            .lock()
            .expect("mutex should not be poisoned")
            .add_chain(config)
    }
}
