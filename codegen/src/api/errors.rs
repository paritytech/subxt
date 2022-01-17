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

use frame_metadata::{
    v14::RuntimeMetadataV14,
};
use proc_macro2::{
    TokenStream as TokenStream2,
    Span as Span2,
};
use quote::{
    quote,
};

pub struct ErrorDetails {
    pub type_def: TokenStream2,
    pub dispatch_error_impl_fn: TokenStream2
}

impl ErrorDetails {
    fn emit_compile_error(err: &str) -> ErrorDetails {
        let err_lit_str = syn::LitStr::new(&err, Span2::call_site());
        ErrorDetails {
            type_def: quote!(),
            dispatch_error_impl_fn: quote!(compile_error!(#err_lit_str))
        }
    }
}

pub fn generate_error_details(metadata: &RuntimeMetadataV14) -> ErrorDetails {
    let errors = match pallet_errors(metadata) {
        Ok(errors) => errors,
        Err(e) => {
            let err_string = format!("Failed to generate error details from metadata: {}", e);
            return ErrorDetails::emit_compile_error(&err_string)
        },
    };

    let match_body_items = errors.into_iter().map(|err| {
        let docs = err.description();
        let pallet_index = err.pallet_index;
        let error_index = err.error_index;
        let pallet_name = err.pallet;
        let error_name = err.error;

        quote!{
            (#pallet_index, #error_index) => Some(ErrorDetails {
                pallet: #pallet_name,
                error: #error_name,
                docs: #docs
            })
        }
    });

    ErrorDetails {
        // A type we'll be returning that needs defining at the top level:
        type_def: quote!{
            pub struct ErrorDetails {
                pub pallet: &'static str,
                pub error: &'static str,
                pub docs: &'static str,
            }
        },
        // A function which will live in an impl block for our DispatchError,
        // to statically return details for known error types:
        dispatch_error_impl_fn: quote!{
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
        }
    }
}

fn pallet_errors(metadata: &RuntimeMetadataV14) -> Result<Vec<ErrorMetadata>, InvalidMetadataError> {
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

    let pallet_errors = metadata
        .pallets
        .iter()
        .filter_map(|pallet| {
            pallet.error.as_ref().map(|error| {
                let type_def_variant = get_type_def_variant(error.ty.id())?;
                Ok((pallet, type_def_variant))
            })
        })
        .collect::<Result<Vec<(_,_)>, _>>()?;

    let errors = pallet_errors
        .iter()
        .flat_map(|(pallet, type_def_variant)| {
            type_def_variant.variants().iter().map(move |var| {
                ErrorMetadata {
                    pallet_index: pallet.index,
                    error_index: var.index(),
                    pallet: pallet.name.clone(),
                    error: var.name().clone(),
                    variant: var.clone(),
                }
            })
        })
        .collect();

    Ok(errors)
}

#[derive(Clone, Debug)]
pub struct ErrorMetadata {
    pub pallet_index: u8,
    pub error_index: u8,
    pub pallet: String,
    pub error: String,
    variant: scale_info::Variant<scale_info::form::PortableForm>,
}

impl ErrorMetadata {
    pub fn description(&self) -> String {
        self.variant.docs().join("\n")
    }
}

#[derive(Debug)]
pub enum InvalidMetadataError {
    MissingType(u32),
    TypeDefNotVariant(u32),
}

impl std::fmt::Display for InvalidMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidMetadataError::MissingType(n) => write!(f, "Type {} missing from type registry", n),
            InvalidMetadataError::TypeDefNotVariant(n) => write!(f, "Type {} was not a variant/enum type", n),
        }
    }
}