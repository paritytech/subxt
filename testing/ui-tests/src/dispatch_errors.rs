// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

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
