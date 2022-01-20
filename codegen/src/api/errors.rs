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
use proc_macro2::{
    Span as Span2,
    TokenStream as TokenStream2,
};
use quote::quote;

/// Tokens which allow us to provide static error information in the generated output.
pub struct ErrorDetails {
    /// This type definition will be used in the `dispatch_error_impl_fn` and is
    /// expected to be generated somewhere in scope for that to be possible.
    pub type_def: TokenStream2,
    // A function which will live in an impl block for our `DispatchError`,
    // to statically return details for known error types:
    pub dispatch_error_impl_fn: TokenStream2,
}

impl ErrorDetails {
    fn emit_compile_error(err: &str) -> ErrorDetails {
        let err_lit_str = syn::LitStr::new(err, Span2::call_site());
        ErrorDetails {
            type_def: quote!(),
            dispatch_error_impl_fn: quote!(compile_error!(#err_lit_str)),
        }
    }
}

/// The purpose of this is to enumerate all of the possible `(module_index, error_index)` error
/// variants, so that we can convert `u8` error codes inside a generated `DispatchError` into
/// nicer error strings with documentation. To do this, we emit the type we'll return instances of,
/// and a function that returns such an instance for all of the error codes seen in the metadata.
pub fn generate_error_details(metadata: &RuntimeMetadataV14) -> ErrorDetails {
    let errors = match pallet_errors(metadata) {
        Ok(errors) => errors,
        Err(e) => {
            let err_string =
                format!("Failed to generate error details from metadata: {}", e);
            return ErrorDetails::emit_compile_error(&err_string)
        }
    };

    let match_body_items = errors.into_iter().map(|err| {
        let docs = err.docs;
        let pallet_index = err.pallet_index;
        let error_index = err.error_index;
        let pallet_name = err.pallet;
        let error_name = err.error;

        quote! {
            (#pallet_index, #error_index) => Some(ErrorDetails {
                pallet: #pallet_name,
                error: #error_name,
                docs: #docs
            })
        }
    });

    ErrorDetails {
        type_def: quote! {
            pub struct ErrorDetails {
                pub pallet: &'static str,
                pub error: &'static str,
                pub docs: &'static str,
            }
        },
        dispatch_error_impl_fn: quote! {
            pub fn details(&self) -> Option<ErrorDetails> {
                if let Self::Module { index, error } = self {
                    match (index, error) {
                        #( #match_body_items ),*,
                        _ => None
                    }
                } else {
                    None
                }
            }
        },
    }
}

fn pallet_errors(
    metadata: &RuntimeMetadataV14,
) -> Result<Vec<ErrorMetadata>, InvalidMetadataError> {
    let get_type_def_variant = |type_id: u32| {
        let ty = metadata
            .types
            .resolve(type_id)
            .ok_or(InvalidMetadataError::MissingType(type_id))?;
        if let scale_info::TypeDef::Variant(var) = ty.type_def() {
            Ok(var)
        } else {
            Err(InvalidMetadataError::TypeDefNotVariant(type_id))
        }
    };

    let mut pallet_errors = vec![];
    for pallet in &metadata.pallets {
        let error = match &pallet.error {
            Some(err) => err,
            None => continue,
        };

        let type_def_variant = get_type_def_variant(error.ty.id())?;
        for var in type_def_variant.variants().iter() {
            pallet_errors.push(ErrorMetadata {
                pallet_index: pallet.index,
                error_index: var.index(),
                pallet: pallet.name.clone(),
                error: var.name().clone(),
                docs: var.docs().join("\n"),
            });
        }
    }

    Ok(pallet_errors)
}

/// Information about each error that we find in the metadata;
/// used to generate the static error information.
#[derive(Clone, Debug)]
struct ErrorMetadata {
    pub pallet_index: u8,
    pub error_index: u8,
    pub pallet: String,
    pub error: String,
    pub docs: String,
}

#[derive(Debug)]
enum InvalidMetadataError {
    MissingType(u32),
    TypeDefNotVariant(u32),
}

impl std::fmt::Display for InvalidMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidMetadataError::MissingType(n) => {
                write!(f, "Type {} missing from type registry", n)
            }
            InvalidMetadataError::TypeDefNotVariant(n) => {
                write!(f, "Type {} was not a variant/enum type", n)
            }
        }
    }
}
