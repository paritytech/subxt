// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::Block;
use crate::{
    client::OnlineClientT,
    config::{Config, Header},
    error::{BlockError, Error},
    utils::PhantomDataSendSync,
    backend::{StreamOfResults, BlockRef},
};
use derivative::Derivative;
use futures::StreamExt;
use std::future::Future;

type BlockStream<T> = StreamOfResults<T>;
type BlockStreamRes<T> = Result<BlockStream<T>, Error>;

/// A client for working with blocks.
#[derive(Derivative)]
#[derivative(Clone(bound = "Client: Clone"))]
pub struct BlocksClient<T, Client> {
    client: Client,
    _marker: PhantomDataSendSync<T>,
}

impl<T, Client> BlocksClient<T, Client> {
    /// Create a new [`BlocksClient`].
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _marker: PhantomDataSendSync::new(),
        }
    }
}

impl<T, Client> BlocksClient<T, Client>
where
    T: Config,
    Client: OnlineClientT<T>,
{
    /// Obtain block details given the provided block hash.
    ///
    /// # Warning
    ///
    /// This call only supports blocks produced since the most recent
    /// runtime upgrade. You can attempt to retrieve older blocks,
    /// but may run into errors attempting to work with them.
    pub fn at(
        &self,
        block_ref: impl Into<BlockRef<T::Hash>>,
    ) -> impl Future<Output = Result<Block<T, Client>, Error>> + Send + 'static {
        self.at_or_latest(Some(block_ref.into()))
    }

    /// Obtain block details of the latest block hash.
    pub fn at_latest(
        &self,
    ) -> impl Future<Output = Result<Block<T, Client>, Error>> + Send + 'static {
        self.at_or_latest(None)
    }

    /// Obtain block details given the provided block hash, or the latest block if `None` is
    /// provided.
    fn at_or_latest(
        &self,
        block_ref: Option<BlockRef<T::Hash>>,
    ) -> impl Future<Output = Result<Block<T, Client>, Error>> + Send + 'static {
        let client = self.client.clone();
        async move {
            // If a block ref isn't provided, we'll get the latest best block to use.
            let block_ref = match block_ref {
                Some(r) => r,
                None => client
                    .backend()
                    .latest_best_block_hash()
                    .await?
            };

            let block_header = match client.backend().block_header(block_ref.hash()).await? {
                Some(header) => header,
                None => return Err(BlockError::not_found(block_ref.hash()).into()),
            };

            Ok(Block::new(block_header, block_ref, client))
        }
    }

    /// Subscribe to all new blocks imported by the node.
    ///
    /// **Note:** You probably want to use [`Self::subscribe_finalized()`] most of
    /// the time.
    pub fn subscribe_all(
        &self,
    ) -> impl Future<Output = Result<BlockStream<Block<T, Client>>, Error>> + Send + 'static
    where
        Client: Send + Sync + 'static,
    {
        let client = self.client.clone();
        header_sub_fut_to_block_sub(self.clone(), async move {
            let sub = client.backend().stream_all_block_headers().await?;
            BlockStreamRes::Ok(Box::pin(sub))
        })
    }

    /// Subscribe to all new blocks imported by the node onto the current best fork.
    ///
    /// **Note:** You probably want to use [`Self::subscribe_finalized()`] most of
    /// the time.
    pub fn subscribe_best(
        &self,
    ) -> impl Future<Output = Result<BlockStream<Block<T, Client>>, Error>> + Send + 'static
    where
        Client: Send + Sync + 'static,
    {
        let client = self.client.clone();
        header_sub_fut_to_block_sub(self.clone(), async move {
            let sub = client.backend().stream_best_block_headers().await?;
            BlockStreamRes::Ok(Box::pin(sub))
        })
    }

    /// Subscribe to finalized blocks.
    pub fn subscribe_finalized(
        &self,
    ) -> impl Future<Output = Result<BlockStream<Block<T, Client>>, Error>> + Send + 'static
    where
        Client: Send + Sync + 'static,
    {
        let client = self.client.clone();
        header_sub_fut_to_block_sub(self.clone(), async move {
            // Fetch the last finalised block details immediately, so that we'll get
            // all blocks after this one.
            let last_finalized_block_ref = client.backend().latest_finalized_block_hash().await?;
            let last_finalized_block_num = client
                .backend()
                .block_header(last_finalized_block_ref.hash())
                .await?
                .map(|h| h.number().into());

            let sub = client.backend().stream_finalized_block_headers().await?;
            BlockStreamRes::Ok(sub)
        })
    }
}

/// Take a promise that will return a subscription to some block headers,
/// and return a subscription to some blocks based on this.
async fn header_sub_fut_to_block_sub<T, Client, S>(
    blocks_client: BlocksClient<T, Client>,
    sub: S,
) -> Result<BlockStream<Block<T, Client>>, Error>
where
    T: Config,
    S: Future<Output = Result<BlockStream<(T::Header, BlockRef<T::Hash>)>, Error>> + Send + 'static,
    Client: OnlineClientT<T> + Send + Sync + 'static,
{
    let sub = sub.await?.then(move |header_and_ref| {
        let client = blocks_client.client.clone();
        async move {
            let (header, block_ref) = match header_and_ref {
                Ok(header_and_ref) => header_and_ref,
                Err(e) => return Err(e),
            };

            Ok(Block::new(header, block_ref, client))
        }
    });
    BlockStreamRes::Ok(Box::pin(sub))
}
