use async_trait::async_trait;
use subxt::backend::{Backend, BlockRef, StorageResponse, StreamOfResults, TransactionStatus};
use subxt::config::{Config, HashFor, PolkadotConfig};
use subxt::error::BackendError;

struct ExternalBackend;

#[async_trait]
impl Backend<PolkadotConfig> for ExternalBackend {
    async fn storage_fetch_values(
        &self,
        _keys: Vec<Vec<u8>>,
        _at: HashFor<PolkadotConfig>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        unimplemented!()
    }

    async fn storage_fetch_descendant_keys(
        &self,
        _key: Vec<u8>,
        _at: HashFor<PolkadotConfig>,
    ) -> Result<StreamOfResults<Vec<u8>>, BackendError> {
        unimplemented!()
    }

    async fn storage_fetch_descendant_values(
        &self,
        _key: Vec<u8>,
        _at: HashFor<PolkadotConfig>,
    ) -> Result<StreamOfResults<StorageResponse>, BackendError> {
        unimplemented!()
    }

    async fn genesis_hash(&self) -> Result<HashFor<PolkadotConfig>, BackendError> {
        unimplemented!()
    }

    async fn block_number_to_hash(
        &self,
        _number: u64,
    ) -> Result<Option<BlockRef<HashFor<PolkadotConfig>>>, BackendError> {
        unimplemented!()
    }

    async fn block_header(
        &self,
        _at: HashFor<PolkadotConfig>,
    ) -> Result<Option<<PolkadotConfig as Config>::Header>, BackendError> {
        unimplemented!()
    }

    async fn block_body(
        &self,
        _at: HashFor<PolkadotConfig>,
    ) -> Result<Option<Vec<Vec<u8>>>, BackendError> {
        unimplemented!()
    }

    async fn latest_finalized_block_ref(
        &self,
    ) -> Result<BlockRef<HashFor<PolkadotConfig>>, BackendError> {
        unimplemented!()
    }

    async fn stream_all_block_headers(
        &self,
        _hasher: <PolkadotConfig as Config>::Hasher,
    ) -> Result<
        StreamOfResults<(
            <PolkadotConfig as Config>::Header,
            BlockRef<HashFor<PolkadotConfig>>,
        )>,
        BackendError,
    > {
        unimplemented!()
    }

    async fn stream_best_block_headers(
        &self,
        _hasher: <PolkadotConfig as Config>::Hasher,
    ) -> Result<
        StreamOfResults<(
            <PolkadotConfig as Config>::Header,
            BlockRef<HashFor<PolkadotConfig>>,
        )>,
        BackendError,
    > {
        unimplemented!()
    }

    async fn stream_finalized_block_headers(
        &self,
        _hasher: <PolkadotConfig as Config>::Hasher,
    ) -> Result<
        StreamOfResults<(
            <PolkadotConfig as Config>::Header,
            BlockRef<HashFor<PolkadotConfig>>,
        )>,
        BackendError,
    > {
        unimplemented!()
    }

    async fn submit_transaction(
        &self,
        _bytes: &[u8],
    ) -> Result<StreamOfResults<TransactionStatus<HashFor<PolkadotConfig>>>, BackendError> {
        unimplemented!()
    }

    async fn call(
        &self,
        _method: &str,
        _call_parameters: Option<&[u8]>,
        _at: HashFor<PolkadotConfig>,
    ) -> Result<Vec<u8>, BackendError> {
        unimplemented!()
    }
}

fn main() {}
