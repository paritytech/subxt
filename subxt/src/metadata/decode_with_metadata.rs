// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{
    Metadata,
};
use crate::{
    error::BasicError,
};

pub trait DecodeWithMetadata: Sized {
    type Target;
    /// Given some metadata and a type ID, attempt to SCALE decode the provided bytes into `Self`.
    fn decode_with_metadata(bytes: &mut &[u8], type_id: u32, metadata: &Metadata) -> Result<Self::Target, BasicError>;
}

// Things can be dynamically decoded to our Value type:
impl DecodeWithMetadata for scale_value::Value<scale_value::scale::TypeId> {
    type Target = Self;
    fn decode_with_metadata(bytes: &mut &[u8], type_id: u32, metadata: &Metadata) -> Result<Self::Target, BasicError> {
        let res = scale_value::scale::decode_as_type(bytes, type_id, metadata.types())?;
        Ok(res)
    }
}

