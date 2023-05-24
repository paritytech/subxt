// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::sync::Arc;

/// A cheaply clone-able representation of the runtime metadata received from a node.
#[derive(Clone, Debug)]
pub struct Metadata {
    inner: Arc<subxt_metadata::Metadata>,
}

impl std::ops::Deref for Metadata {
    type Target = subxt_metadata::Metadata;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Metadata {
    pub(crate) fn new(md: subxt_metadata::Metadata) -> Self {
        Metadata {
            inner: Arc::new(md),
        }
    }
}

impl From<subxt_metadata::Metadata> for Metadata {
    fn from(md: subxt_metadata::Metadata) -> Self {
        Metadata::new(md)
    }
}

impl codec::Decode for Metadata {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        subxt_metadata::Metadata::decode(input).map(Metadata::new)
    }
}
