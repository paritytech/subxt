use derivative::Derivative;

use crate::{config::Config, metadata::Metadata};

/// Each client should be able to provide access to the following fields
/// - runtime version
/// - genesis hash
/// - metadata

#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct ClientMetadata<C: Config> {
    pub genesis_hash: C::Hash,
    pub runtime_version: RuntimeVersion,
    pub metadata: Metadata,
}

impl<C: Config> ClientMetadata<C> {
    pub fn new(genesis_hash: C::Hash, runtime_version: RuntimeVersion, metadata: Metadata) -> Self {
        Self {
            genesis_hash,
            runtime_version,
            metadata,
        }
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn runtime_version(&self) -> RuntimeVersion {
        self.runtime_version
    }

    pub fn genesis_hash(&self) -> C::Hash {
        self.genesis_hash
    }
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
