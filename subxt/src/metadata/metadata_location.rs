// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/// Locate an item of a known type in the metadata.
/// We should already know that the item we're looking
/// for is a call or event for instance, and then with this,
/// we can dig up details for that item in the metadata.
pub trait MetadataLocation {
    /// The pallet in which the item lives.
    fn pallet(&self) -> &str;
    /// The name of the item.
    fn item(&self) -> &str;
}
