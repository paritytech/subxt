use crate::backend::{BlockRef, StreamOfResults};
use crate::client::{ClientAtBlock, OnlineClient, OnlineClientAtBlockImpl};
use crate::config::{Config, HashFor, Header};
use crate::error::{BlocksError, OnlineClientAtBlockError};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

/// A stream of blocks.
#[derive(Debug)]
pub struct Blocks<T: Config> {
    client: OnlineClient<T>,
    stream: StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>,
}

impl<T: Config> Blocks<T> {
    pub(crate) fn from_headers_stream(
        client: OnlineClient<T>,
        stream: StreamOfResults<(T::Header, BlockRef<HashFor<T>>)>,
    ) -> Self {
        Blocks { client, stream }
    }

    /// Return the next block in the stream when it is produced.
    pub async fn next(&mut self) -> Option<Result<Block<T>, BlocksError>> {
        StreamExt::next(self).await
    }
}

impl<T: Config> Stream for Blocks<T> {
    type Item = Result<Block<T>, BlocksError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let item = match self.stream.poll_next_unpin(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(None),
            Poll::Ready(Some(item)) => item,
        };

        let res = match item {
            Ok((block_header, block_ref)) => Ok(Block {
                block_ref,
                block_header,
                client: self.client.clone(),
            }),
            Err(e) => Err(BlocksError::CannotGetBlockHeader(e)),
        };

        Poll::Ready(Some(res))
    }
}

/// A block from the stream of blocks.
#[derive(Debug, Clone)]
pub struct Block<T: Config> {
    block_ref: BlockRef<HashFor<T>>,
    block_header: T::Header,
    client: OnlineClient<T>,
}

impl<T: Config> Block<T> {
    /// The block hash
    pub fn hash(&self) -> HashFor<T> {
        self.block_ref.hash()
    }

    /// The block number.
    pub fn number(&self) -> u64 {
        self.block_header.number()
    }

    /// The block header.
    pub fn header(&self) -> &T::Header {
        &self.block_header
    }

    /// Instantiate a client at this block.
    pub async fn at(
        &self,
    ) -> Result<ClientAtBlock<T, OnlineClientAtBlockImpl<T>>, OnlineClientAtBlockError> {
        self.client.at_block(self.block_ref.clone()).await
    }
}
