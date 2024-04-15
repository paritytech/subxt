// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A couple of client types that we use elsewhere.

use crate::{config::Config, metadata::Metadata};
use derive_where::derive_where;

/// This provides access to some relevant client state in signed extensions,
/// and is just a combination of some of the available properties.
#[derive_where(Clone, Debug)]
pub struct ClientState<C: Config> {
    /// Genesis hash.
    pub genesis_hash: C::Hash,
    /// Runtime version.
    pub runtime_version: RuntimeVersion,
    /// Metadata.
    pub metadata: Metadata,
}

/// Runtime version information needed to submit transactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeVersion {
    /// Version of the runtime specification. A full-node will not attempt to use its native
    /// runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    /// `spec_version` and `authoring_version` are the same between Wasm and native.
    pub spec_version: u32,
    /// All existing dispatches are fully compatible when this number doesn't change. If this
    /// number changes, then `spec_version` must change, also.
    ///
    /// This number must change when an existing dispatchable (module ID, dispatch ID) is changed,
    /// either through an alteration in its user-level semantics, a parameter
    /// added/removed/changed, a dispatchable being removed, a module being removed, or a
    /// dispatchable/module changing its index.
    ///
    /// It need *not* change when a new module is added or when a dispatchable is added.
    pub transaction_version: u32,
}
