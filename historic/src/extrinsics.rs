use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
use crate::error::ExtrinsicsError;

mod extrinsic_call;
mod extrinsic_info;
mod extrinsic_transaction_extensions;
mod extrinsics_type;

pub use extrinsic_transaction_extensions::ExtrinsicTransactionParams;
pub use extrinsics_type::{Extrinsic, Extrinsics};

/// Work with extrinsics.
pub struct ExtrinsicsClient<'atblock, Client, T> {
    client: &'atblock Client,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, Client, T> ExtrinsicsClient<'atblock, Client, T> {
    /// Work with extrinsics.
    pub(crate) fn new(client: &'atblock Client) -> Self {
        Self {
            client,
            marker: std::marker::PhantomData,
        }
    }
}

// Things that we can do online with extrinsics.
impl<'atblock, 'client: 'atblock, Client, T> ExtrinsicsClient<'atblock, Client, T>
where
    T: Config + 'client,
    Client: OnlineClientAtBlockT<'client, T>,
{
    /// Fetch the extrinsics for the current block. This is essentially a
    /// combination of [`Self::fetch_bytes`] and [`Self::decode_from`].
    pub async fn fetch(&self) -> Result<Extrinsics<'atblock>, ExtrinsicsError> {
        let bytes: Vec<Vec<u8>> = self.fetch_bytes().await?;

        // Small optimization; no need to decode anything if no bytes.
        if bytes.is_empty() {
            return Ok(Extrinsics::empty());
        }

        self.decode_from(bytes)
    }

    /// Fetch the bytes for the extrinsics in the current block.
    pub async fn fetch_bytes(&self) -> Result<Vec<Vec<u8>>, ExtrinsicsError> {
        let bytes: Vec<Vec<u8>> = self
            .client
            .rpc_methods()
            .archive_v1_body(self.client.block_hash().into())
            .await
            .map_err(|e| ExtrinsicsError::FetchError { reason: e })?
            .map(|body| body.into_iter().map(|b| b.0).collect())
            .unwrap_or_default();

        Ok(bytes)
    }
}

// Things that we can do offline with extrinsics.
impl<'atblock, 'client: 'atblock, Client, T> ExtrinsicsClient<'atblock, Client, T>
where
    T: Config + 'client,
    Client: OfflineClientAtBlockT<'client, T>,
{
    /// Given some bytes representing the extrinsics in this block, decode them into an [`Extrinsics`] type.
    pub fn decode_from(
        &self,
        bytes: Vec<Vec<u8>>,
    ) -> Result<Extrinsics<'atblock>, ExtrinsicsError> {
        Extrinsics::new(bytes, self.client)
    }
}
