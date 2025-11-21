use super::extrinsic_call::ExtrinsicCall;
use super::extrinsic_info::{AnyExtrinsicInfo, with_info};
use super::extrinsic_transaction_extensions::ExtrinsicTransactionParams;
use crate::client::OfflineClientAtBlockT;
use crate::config::Config;
use crate::error::ExtrinsicsError;

/// This represents some extrinsics in a block, and carries everything that we need to decode information out of them.
pub struct Extrinsics<'atblock> {
    bytes: Vec<Vec<u8>>,
    // Each index in this vec should line up with one index in the above vec.
    infos: Vec<AnyExtrinsicInfo<'atblock>>,
}

impl<'atblock> Extrinsics<'atblock> {
    // In here we hide the messy logic needed to decode extrinsics into a consistent output given either current or legacy metadata.
    pub(crate) fn new<'client: 'atblock, T, Client>(
        bytes: Vec<Vec<u8>>,
        client: &'atblock Client,
    ) -> Result<Self, ExtrinsicsError>
    where
        T: Config + 'client,
        Client: OfflineClientAtBlockT<'client, T>,
    {
        let infos = AnyExtrinsicInfo::new(&bytes, client.metadata(), client.legacy_types())?;
        Ok(Extrinsics { bytes, infos })
    }

    pub(crate) fn empty() -> Self {
        Self {
            bytes: vec![],
            infos: vec![],
        }
    }

    /// How many extrinsics are in this block?
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Are there any extrinsics in this block?
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Iterate over the extrinsics.
    pub fn iter(&self) -> impl Iterator<Item = Extrinsic<'_, 'atblock>> {
        self.bytes
            .iter()
            .zip(self.infos.iter())
            .enumerate()
            .map(|(idx, (bytes, info))| Extrinsic { idx, bytes, info })
    }
}

/// This represents an extrinsic, and carries everything that we need to decode information out of it.
pub struct Extrinsic<'extrinsics, 'atblock> {
    idx: usize,
    bytes: &'extrinsics [u8],
    info: &'extrinsics AnyExtrinsicInfo<'atblock>,
}

impl<'extrinsics, 'atblock> Extrinsic<'extrinsics, 'atblock> {
    /// Get the index of this extrinsic in the block.
    pub fn index(&self) -> usize {
        self.idx
    }

    /// Get the raw bytes of this extrinsic.
    pub fn bytes(&self) -> &'extrinsics [u8] {
        self.bytes
    }

    /// Is this extrinsic signed?
    pub fn is_signed(&self) -> bool {
        with_info!(&self.info => info.info.is_signed())
    }

    /// Return information about the call that this extrinsic is making.
    pub fn call(&self) -> ExtrinsicCall<'extrinsics, 'atblock> {
        ExtrinsicCall::new(self.bytes, self.info)
    }

    /// Return only the bytes of the address that signed this extrinsic.
    ///
    /// # Note
    ///
    /// Returns `None` if the extrinsic is not signed.
    pub fn address_bytes(&self) -> Option<&'extrinsics [u8]> {
        with_info!(&self.info => {
            info.info
                .signature_payload()
                .map(|s| &self.bytes[s.address_range()])
        })
    }

    /// Returns Some(signature_bytes) if the extrinsic was signed otherwise None is returned.
    pub fn signature_bytes(&self) -> Option<&'extrinsics [u8]> {
        with_info!(&self.info => {
            info.info
                .signature_payload()
                .map(|s| &self.bytes[s.signature_range()])
        })
    }

    /// Get information about the transaction extensions of this extrinsic.
    pub fn transaction_extensions(
        &self,
    ) -> Option<ExtrinsicTransactionParams<'extrinsics, 'atblock>> {
        ExtrinsicTransactionParams::new(self.bytes, self.info)
    }
}
