// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The "default" Substrate/Polkadot Signature type. This is used in codegen, as well as signing related bits.
//! This doesn't contain much functionality itself, but is easy to convert to/from an `sp_runtime::MultiSignature`
//! for instance, to gain functionality without forcing a dependency on Substrate crates here.

use codec::{Decode, Encode};

/// Signature container that can store known signature types. This is a simplified version of
/// `sp_runtime::MultiSignature`. To obtain more functionality, convert this into that type.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug)]
pub enum MultiSignature {
    /// An Ed25519 signature.
    Ed25519([u8; 64]),
    /// An Sr25519 signature.
    Sr25519([u8; 64]),
    /// An ECDSA/SECP256k1 signature (a 512-bit value, plus 8 bits for recovery ID).
    Ecdsa([u8; 65]),
}

// Improve compat with the substrate version if we're using those crates:
#[cfg(feature = "substrate-compat")]
mod substrate_impls {
    use super::*;

    impl From<sp_runtime::MultiSignature> for MultiSignature {
        fn from(value: sp_runtime::MultiSignature) -> Self {
            match value {
                sp_runtime::MultiSignature::Ed25519(s) => Self::Ed25519(s.0),
                sp_runtime::MultiSignature::Sr25519(s) => Self::Sr25519(s.0),
                sp_runtime::MultiSignature::Ecdsa(s) => Self::Ecdsa(s.0),
            }
        }
    }

    impl From<sp_core::ed25519::Signature> for MultiSignature {
        fn from(value: sp_core::ed25519::Signature) -> Self {
            let sig: sp_runtime::MultiSignature = value.into();
            sig.into()
        }
    }

    impl From<sp_core::sr25519::Signature> for MultiSignature {
        fn from(value: sp_core::sr25519::Signature) -> Self {
            let sig: sp_runtime::MultiSignature = value.into();
            sig.into()
        }
    }

    impl From<sp_core::ecdsa::Signature> for MultiSignature {
        fn from(value: sp_core::ecdsa::Signature) -> Self {
            let sig: sp_runtime::MultiSignature = value.into();
            sig.into()
        }
    }
}
