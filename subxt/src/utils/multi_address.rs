// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! The "default" Substrate/Polkadot Address type. This is used in codegen, as well as signing related bits.
//! This doesn't contain much functionality itself, but is easy to convert to/from an `sp_runtime::MultiAddress`
//! for instance, to gain functionality without forcing a dependency on Substrate crates here.

use codec::{Decode, Encode};

/// A multi-format address wrapper for on-chain accounts. This is a simplified version ob Substrate's
/// `sp_runtime::MultiAddress`. To obtain more functionality, convert this into that type.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug)]
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