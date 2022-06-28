// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::utils::{
    dispatch_error::{
        ArrayDispatchError,
        LegacyDispatchError,
        NamedFieldDispatchError,
    },
    generate_metadata_from_pallets_custom_dispatch_error,
};
use frame_metadata::RuntimeMetadataPrefixed;

pub fn metadata_array_dispatch_error() -> RuntimeMetadataPrefixed {
    generate_metadata_from_pallets_custom_dispatch_error::<ArrayDispatchError>(vec![])
}

pub fn metadata_legacy_dispatch_error() -> RuntimeMetadataPrefixed {
    generate_metadata_from_pallets_custom_dispatch_error::<LegacyDispatchError>(vec![])
}

pub fn metadata_named_field_dispatch_error() -> RuntimeMetadataPrefixed {
    generate_metadata_from_pallets_custom_dispatch_error::<NamedFieldDispatchError>(
        vec![],
    )
}
