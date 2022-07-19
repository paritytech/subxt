// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Types representing the metadata obtained from a node.

mod decode_with_metadata;
mod encode_with_metadata;
mod hash_cache;
mod metadata_location;
mod metadata_type;

pub use metadata_location::MetadataLocation;

pub use metadata_type::{
    ErrorMetadata,
    EventMetadata,
    InvalidMetadataError,
    Metadata,
    MetadataError,
    PalletMetadata,
};

pub use decode_with_metadata::{
    DecodeStaticType,
    DecodeWithMetadata,
};

pub use encode_with_metadata::{
    EncodeStaticType,
    EncodeWithMetadata,
};
