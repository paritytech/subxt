// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The "default" Substrate/Polkadot Signer type. This is used in signature verification and related bits.
//! This doesn't contain much functionality itself, but is easy to convert to/from an `sp_runtime::MultiSigner`
//! for instance, to gain functionality without forcing a dependency on Substrate crates here.

use codec::{Decode, Encode};

/// Signer container that can store known signer types. This is a simplified version of
/// `sp_runtime::MultiSigner`. To obtain more functionality, convert this into that type.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug, scale_info::TypeInfo)]
pub enum MultiSigner {
    /// An Ed25519 public key.
    Ed25519([u8; 32]),
    /// An Sr25519 public key.
    Sr25519([u8; 32]),
    /// An ECDSA/SECP256k1 public key.
    Ecdsa([u8; 33]),
}

// Improve compat with the substrate version if we're using those crates:
#[cfg(feature = "substrate-compat")]
mod substrate_impls {
    use super::*;

    impl From<sp_runtime::MultiSigner> for MultiSigner {
        fn from(value: sp_runtime::MultiSigner) -> Self {
            match value {
                sp_runtime::MultiSigner::Ed25519(s) => Self::Ed25519(s.0),
                sp_runtime::MultiSigner::Sr25519(s) => Self::Sr25519(s.0),
                sp_runtime::MultiSigner::Ecdsa(s) => Self::Ecdsa(s.0),
            }
        }
    }

    impl From<sp_core::ed25519::Public> for MultiSigner {
        fn from(value: sp_core::ed25519::Public) -> Self {
            let signer: sp_runtime::MultiSigner = value.into();
            signer.into()
        }
    }

    impl From<sp_core::sr25519::Public> for MultiSigner {
        fn from(value: sp_core::sr25519::Public) -> Self {
            let signer: sp_runtime::MultiSigner = value.into();
            signer.into()
        }
    }

    impl From<sp_core::ecdsa::Public> for MultiSigner {
        fn from(value: sp_core::ecdsa::Public) -> Self {
            let signer: sp_runtime::MultiSigner = value.into();
            signer.into()
        }
    }
}
