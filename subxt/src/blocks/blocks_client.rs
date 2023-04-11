// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::Block;
use crate::{
    client::OnlineClientT,
    config::{Config, Header},
    error::{BlockError, Error},
    utils::PhantomDataSendSync,
};
use derivative::Derivative;
use futures::{future::Either, stream, Stream, StreamExt};
use std::{future::Future, pin::Pin};

type BlockStream<T> = Pin<Box<dyn Stream<Item = Result<T, Error>> + Send>>;
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
        block_hash: T::Hash,
    ) -> impl Future<Output = Result<Block<T, Client>, Error>> + Send + 'static {
        self.at_or_latest(Some(block_hash))
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
        block_hash: Option<T::Hash>,
    ) -> impl Future<Output = Result<Block<T, Client>, Error>> + Send + 'static {
        let client = self.client.clone();
        async move {
            // If block hash is not provided, get the hash
            // for the latest block and use that.
            let block_hash = match block_hash {
                Some(hash) => hash,
                None => client
                    .rpc()
                    .block_hash(None)
                    .await?
                    .expect("didn't pass a block number; qed"),
            };

            let block_header = match client.rpc().header(Some(block_hash)).await? {
                Some(header) => header,
                None => return Err(BlockError::not_found(block_hash).into()),
            };

            Ok(Block::new(block_header, client))
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
            let sub = client.rpc().subscribe_all_block_headers().await?;
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
            let sub = client.rpc().subscribe_best_block_headers().await?;
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
            let last_finalized_block_hash = client.rpc().finalized_head().await?;
            let last_finalized_block_num = client
                .rpc()
                .header(Some(last_finalized_block_hash))
                .await?
                .map(|h| h.number().into());

            let sub = client.rpc().subscribe_finalized_block_headers().await?;

            // Adjust the subscription stream to fill in any missing blocks.
            BlockStreamRes::Ok(
                subscribe_to_block_headers_filling_in_gaps(client, last_finalized_block_num, sub)
                    .boxed(),
            )
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
    S: Future<Output = Result<BlockStream<T::Header>, Error>> + Send + 'static,
    Client: OnlineClientT<T> + Send + Sync + 'static,
{
    let sub = sub.await?.then(move |header| {
        let client = blocks_client.client.clone();
        async move {
            let header = match header {
                Ok(header) => header,
                Err(e) => return Err(e),
            };

            Ok(Block::new(header, client))
        }
    });
    BlockStreamRes::Ok(Box::pin(sub))
}

/// Note: This is exposed for testing but is not considered stable and may change
/// without notice in a patch release.
#[doc(hidden)]
pub fn subscribe_to_block_headers_filling_in_gaps<T, Client, S, E>(
    client: Client,
    mut last_block_num: Option<u64>,
    sub: S,
) -> impl Stream<Item = Result<T::Header, Error>> + Send
where
    T: Config,
    Client: OnlineClientT<T>,
    S: Stream<Item = Result<T::Header, E>> + Send,
    E: Into<Error> + Send + 'static,
{
    sub.flat_map(move |s| {
        let client = client.clone();

        // Get the header, or return a stream containing just the error.
        let header = match s {
            Ok(header) => header,
            Err(e) => return Either::Left(stream::once(async { Err(e.into()) })),
        };

        // We want all previous details up to, but not including this current block num.
        let end_block_num = header.number().into();

        // This is one after the last block we returned details for last time.
        let start_block_num = last_block_num.map(|n| n + 1).unwrap_or(end_block_num);

        // Iterate over all of the previous blocks we need headers for, ignoring the current block
        // (which we already have the header info for):
        let previous_headers = stream::iter(start_block_num..end_block_num)
            .then(move |n| {
                let rpc = client.rpc().clone();
                async move {
                    let hash = rpc.block_hash(Some(n.into())).await?;
                    let header = rpc.header(hash).await?;
                    Ok::<_, Error>(header)
                }
            })
            .filter_map(|h| async { h.transpose() });

        // On the next iteration, we'll get details starting just after this end block.
        last_block_num = Some(end_block_num);

        // Return a combination of any previous headers plus the new header.
        Either::Right(previous_headers.chain(stream::once(async { Ok(header) })))
    })
}
