// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The "default" Substrate/Polkadot Address type. This is used in codegen, as well as signing related bits.
//! This doesn't contain much functionality itself, but is easy to convert to/from an `sp_runtime::MultiAddress`
//! for instance, to gain functionality without forcing a dependency on Substrate crates here.

use codec::{Decode, Encode};

/// A multi-format address wrapper for on-chain accounts. This is a simplified version of Substrate's
/// `sp_runtime::MultiAddress`. To obtain more functionality, convert this into that type (this conversion
/// functionality is provided via `From` impls if the `substrate-compat` feature is enabled).
#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    Debug,
    scale_encode::EncodeAsType,
    scale_decode::DecodeAsType,
)]
pub enum MultiAddress<AccountId, AccountIndex> {
    /// It's an account ID (pubkey).
    Id(AccountId),
    /// It's an account index.
    Index(#[codec(compact)] AccountIndex),
    /// It's some arbitrary raw bytes.
    Raw(Vec<u8>),
    /// It's a 32 byte representation.
    Address32([u8; 32]),
    /// Its a 20 byte representation.
    Address20([u8; 20]),
}

impl<AccountId, AccountIndex> From<AccountId> for MultiAddress<AccountId, AccountIndex> {
    fn from(a: AccountId) -> Self {
        Self::Id(a)
    }
}

// Improve compat with the substrate version if we're using those crates:
#[cfg(feature = "substrate-compat")]
mod substrate_impls {
    use super::{super::AccountId32, *};

    impl<N> From<sp_runtime::AccountId32> for MultiAddress<AccountId32, N> {
        fn from(value: sp_runtime::AccountId32) -> Self {
            let val: AccountId32 = value.into();
            val.into()
        }
    }

    impl<Id, N> From<sp_runtime::MultiAddress<Id, N>> for MultiAddress<AccountId32, N>
    where
        Id: Into<AccountId32>,
    {
        fn from(value: sp_runtime::MultiAddress<Id, N>) -> Self {
            match value {
                sp_runtime::MultiAddress::Id(v) => Self::Id(v.into()),
                sp_runtime::MultiAddress::Index(v) => Self::Index(v),
                sp_runtime::MultiAddress::Raw(v) => Self::Raw(v),
                sp_runtime::MultiAddress::Address32(v) => Self::Address32(v),
                sp_runtime::MultiAddress::Address20(v) => Self::Address20(v),
            }
        }
    }
}
