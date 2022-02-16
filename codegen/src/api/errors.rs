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

use frame_metadata::v14::RuntimeMetadataV14;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::quote;
use scale_info::TypeDef;

/// The aim of this is to attach a `details` function to our runtime error struct
/// which takes runtime metadata (this is important; we can generate error info from
/// compile time metadata, but using runtime metadata gives a certain amount of flexibility
/// in case the node we're talking to doesn't exactly line up what we've assumed from codegen.
/// This is also why we resolve pallet indexes for calls at runtime; they might differ between
/// static codegen and node we're quering.
pub fn generate_error_details_fn(metadata: &RuntimeMetadataV14) -> TokenStream2 {
    let dispatch_error_def = metadata
        .types
        .types()
        .iter()
        .find(|&ty| ty.ty().path().segments() == ["sp_runtime", "DispatchError"])
        .unwrap_or_else(|| {
            abort_call_site!("sp_runtime::DispatchError type expected in metadata")
        })
        .ty()
        .type_def();

    // Slightly older versions of substrate have a `DispatchError::Module { index, error }`
    // variant. Newer versions have something like a `DispatchError::Module (Details)` variant.
    // We check to see which type of variant we're dealing with based on the metadata, and
    // generate the correct code to handle either older or newer substrate versions.
    let module_variant_is_struct = if let TypeDef::Variant(details) = dispatch_error_def {
        let module_variant = details
            .variants()
            .iter()
            .find(|variant| variant.name() == "Module")
            .unwrap_or_else(|| {
                abort_call_site!("DispatchError::Module variant expected in metadata")
            });
        let are_fields_named = module_variant
            .fields()
            .get(0)
            .unwrap_or_else(|| {
                abort_call_site!(
                    "DispatchError::Module expected to contain 1 or more fields"
                )
            })
            .name()
            .is_some();
        are_fields_named
    } else {
        false
    };

    // Return a function that will obtain error details given some metadata.
    if module_variant_is_struct {
        quote! {
            pub fn details<'a>(&self, metadata: &'a ::subxt::Metadata) -> Result<Option<&'a ::subxt::ErrorMetadata>, ::subxt::MetadataError> {
                if let &Self::Module { index, error } = self {
                    metadata.error(index, error).map(Some)
                } else {
                    Ok(None)
                }
            }
        }
    } else {
        quote! {
            pub fn details<'a>(&self, metadata: &'a ::subxt::Metadata) -> Result<Option<&'a ::subxt::ErrorMetadata>, ::subxt::MetadataError> {
                if let Self::Module (module_error) = self {
                    metadata.error(module_error.index, module_error.error).map(Some)
                } else {
                    Ok(None)
                }
            }
        }
    }
}
