//! This module exposes the entrypoint to connect and interact with chains.
//!
//! - See [`OnlineClient`] for instantiating a standard client which is connected to
//!   a chain and capable of interacting with it.
//! - See [`OfflineClient`] if you have no network connection but want to perform certain
//!   actions against some chain.
//!
//! After instantiating a client, you'll typically then select a block to work against
//! via something like [`OnlineClient::at_block`] or [`OfflineClient::at_block`].
//! These hand back [`OnlineClientAtBlock`] or [`OfflineClientAtBlock`], which expose
//! various methods available online or offline at the given block.

mod offline_client;
mod online_client;

use crate::backend::BlockRef;
use crate::config::{Config, HashFor};
use crate::constants::ConstantsClient;
use crate::custom_values::CustomValuesClient;
use crate::error::BlockError;
use crate::events::EventsClient;
use crate::extrinsics::ExtrinsicsClient;
use crate::runtime_apis::RuntimeApisClient;
use crate::storage::StorageClient;
use crate::transactions::TransactionsClient;
use crate::view_functions::ViewFunctionsClient;
use core::marker::PhantomData;
use std::borrow::Cow;
use subxt_metadata::{ArcMetadata, Metadata};

pub use offline_client::{OfflineClient, OfflineClientAtBlockImpl, OfflineClientAtBlockT};
pub use online_client::{
    BlockNumberOrRef, OnlineClient, OnlineClientAtBlockImpl, OnlineClientAtBlockT, Blocks, Block,
};

/// This represents a client at a specific block number, and is created by calling either
/// [`OnlineClient::at_block`] or [`OfflineClient::at_block`].
///
/// This wraps a client implementation, which will either be [`OfflineClientAtBlockImpl`]
/// or [`OnlineClientAtBlockImpl`]. Prefer to use the type aliases [`OfflineClientAtBlock`]
/// and [`OnlineClientAtBlock`] if you need to refer to the concrete instances of this.
#[derive(Clone, Debug)]
pub struct ClientAtBlock<T, Client> {
    pub(crate) client: Client,
    marker: PhantomData<T>,
}

impl<T, Client> ClientAtBlock<T, Client> {
    /// Construct a new client at some block.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            marker: PhantomData,
        }
    }
}

impl<T, Client> ClientAtBlock<T, Client>
where
    T: Config,
    Client: OfflineClientAtBlockT<T>,
{
    /// Construct and submit transactions. This is a
    /// shorthand to [`Self::transactions()`].
    pub fn tx(&self) -> TransactionsClient<T, Client> {
        self.transactions()
    }

    /// Construct and submit transactions.
    pub fn transactions(&self) -> TransactionsClient<T, Client> {
        TransactionsClient::new(self.client.clone())
    }

    /// Access storage at this block.
    pub fn storage(&self) -> StorageClient<'_, T, Client> {
        StorageClient::new(&self.client)
    }

    /// Access constants at this block.
    pub fn constants(&self) -> ConstantsClient<'_, T, Client> {
        ConstantsClient::new(&self.client)
    }

    /// Access custom values at this block.
    pub fn custom_values(&self) -> CustomValuesClient<'_, T, Client> {
        CustomValuesClient::new(&self.client)
    }

    /// Work with the extrinsics in this block.
    pub fn extrinsics(&self) -> ExtrinsicsClient<'_, T, Client> {
        ExtrinsicsClient::new(Cow::Borrowed(&self.client))
    }

    /// Work with the events at this block.
    pub fn events(&self) -> EventsClient<'_, T, Client> {
        EventsClient::new(&self.client)
    }

    /// Access runtime APIs at this block.
    pub fn runtime_apis(&self) -> RuntimeApisClient<'_, T, Client> {
        RuntimeApisClient::new(&self.client)
    }

    /// Access Pallet View Functions at this block.
    pub fn view_functions(&self) -> ViewFunctionsClient<'_, T, Client> {
        ViewFunctionsClient::new(&self.client)
    }

    /// Obtain a clone of the metadata. Prefer [`Self::metadata_ref()`] 
    /// unless you need to take ownership of the metadata.
    pub fn metadata(&self) -> ArcMetadata {
        self.client.metadata()
    }

    /// Obtain a reference to the metadata.
    pub fn metadata_ref(&self) -> &Metadata {
        self.client.metadata_ref()
    }

    /// The current block number.
    pub fn block_number(&self) -> u64 {
        self.client.block_number()
    }

    /// The spec version at this block.
    pub fn spec_version(&self) -> u32 {
        self.client.spec_version()
    }

    /// The transaction version at this block.
    /// Note: This is different from the value encoded at the start of extrinsics.
    pub fn transaction_version(&self) -> u32 {
        self.client.transaction_version()
    }

    /// Return the genesis hash, if it is available. if you're using an
    /// [`OnlineClientAtBlock`], this will always be present.
    pub fn genesis_hash(&self) -> Option<HashFor<T>> {
        self.client.genesis_hash()
    }

    /// Return the hasher that's used at this block.
    pub fn hasher(&self) -> &T::Hasher {
        self.client.hasher()
    }
}

impl<T, Client> ClientAtBlock<T, Client>
where
    T: Config,
    Client: OnlineClientAtBlockT<T>,
{
    /// Return the [`OnlineClient`] behind this.
    pub fn online_client(&self) -> OnlineClient<T> {
        self.client.client()
    }

    /// A reference to the current block. 
    /// 
    /// Depending on the backend, holding onto
    /// this encourages the backend to keep the block available until this
    /// is dropped.
    pub fn block_ref(&self) -> &BlockRef<HashFor<T>> {
        self.client.block_ref()
    }

    /// The current block hash.
    pub fn block_hash(&self) -> HashFor<T> {
        self.client.block_ref().hash()
    }

    /// The header for this block.
    pub async fn block_header(&self) -> Result<T::Header, BlockError> {
        let block_hash = self.block_hash();
        let header = self
            .client
            .backend()
            .block_header(block_hash)
            .await
            .map_err(|e| BlockError::CouldNotDownloadBlockHeader {
                block_hash: block_hash.into(),
                reason: e,
            })?
            .ok_or_else(|| BlockError::BlockNotFound {
                block_hash: block_hash.into(),
            })?;
        Ok(header)
    }
}

/// An offline client at a specific block.
pub type OfflineClientAtBlock<T> = ClientAtBlock<T, OfflineClientAtBlockImpl<T>>;

/// An online client at a specific block.
pub type OnlineClientAtBlock<T> = ClientAtBlock<T, OnlineClientAtBlockImpl<T>>;
