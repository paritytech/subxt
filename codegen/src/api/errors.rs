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
use scale_info::{
    form::PortableForm,
    Field,
    TypeDef,
    TypeDefPrimitive,
};

/// Different substrate versions will have a different `DispatchError::Module`.
/// The following cases are ordered by versions.
enum ModuleErrorType {
    /// Case 1: `DispatchError::Module { index: u8, error: u8 }`
    ///
    /// This is the first supported `DispatchError::Module` format.
    NamedField,
    /// Case 2: `DispatchError::Module ( sp_runtime::ModuleError { index: u8, error: u8 } )`
    ///
    /// Substrate introduced `sp_runtime::ModuleError`, while keeping the error `u8`.
    LegacyError,
    /// Case 3: `DispatchError::Module ( sp_runtime::ModuleError { index: u8, error: [u8; 4] } )`
    ///
    /// The substrate error evolved into `[u8; 4]`.
    ArrayError,
}

impl quote::ToTokens for ModuleErrorType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let trait_fn_body = match self {
            ModuleErrorType::NamedField => {
                quote! {
                    if let &Self::Module { index, error } = self {
                        Some(::subxt::ModuleErrorData { pallet_index: index, error: [error, 0, 0, 0] })
                    } else {
                        None
                    }
                }
            }
            ModuleErrorType::LegacyError => {
                quote! {
                    if let Self::Module (module_error) = self {
                        Some(::subxt::ModuleErrorData { pallet_index: module_error.index, error: [module_error.error, 0, 0, 0] })
                    } else {
                        None
                    }
                }
            }
            ModuleErrorType::ArrayError => {
                quote! {
                    if let Self::Module (module_error) = self {
                        Some(::subxt::ModuleErrorData { pallet_index: module_error.index, error: module_error.error })
                    } else {
                        None
                    }
                }
            }
        };

        tokens.extend(trait_fn_body);
    }
}

/// Determine the `ModuleError` type for the `ModuleErrorType::LegacyError` and
/// `ModuleErrorType::ErrorArray` cases.
fn module_error_type(
    module_field: &Field<PortableForm>,
    metadata: &RuntimeMetadataV14,
) -> ModuleErrorType {
    // Fields are named.
    if module_field.name().is_some() {
        return ModuleErrorType::NamedField
    }

    // Get the `sp_runtime::ModuleError` structure.
    let module_err = metadata
        .types
        .resolve(module_field.ty().id())
        .unwrap_or_else(|| {
            abort_call_site!("sp_runtime::ModuleError type expected in metadata")
        });

    let error_type_def = match module_err.type_def() {
        TypeDef::Composite(composite) => composite,
        _ => abort_call_site!("sp_runtime::ModuleError type should be a composite type"),
    };

    // Get the error field from the `sp_runtime::ModuleError` structure.
    let error_field = error_type_def
        .fields()
        .iter()
        .find(|field| field.name() == Some(&"error".to_string()))
        .unwrap_or_else(|| {
            abort_call_site!("sp_runtime::ModuleError expected to contain error field")
        });

    // Resolve the error type from the metadata.
    let error_field_ty = metadata
        .types
        .resolve(error_field.ty().id())
        .unwrap_or_else(|| {
            abort_call_site!("sp_runtime::ModuleError::error type expected in metadata")
        });

    match error_field_ty.type_def() {
        // Check for legacy error type.
        TypeDef::Primitive(TypeDefPrimitive::U8) => ModuleErrorType::LegacyError,
        TypeDef::Array(array) => {
            // Check new error type of len 4 and type u8.
            if array.len() != 4 {
                abort_call_site!("sp_runtime::ModuleError::error array length is not 4");
            }

            let array_ty = metadata
                .types
                .resolve(array.type_param().id())
                .unwrap_or_else(|| {
                    abort_call_site!(
                        "sp_runtime::ModuleError::error array type expected in metadata"
                    )
                });

            if let TypeDef::Primitive(TypeDefPrimitive::U8) = array_ty.type_def() {
                ModuleErrorType::ArrayError
            } else {
                abort_call_site!(
                    "sp_runtime::ModuleError::error array type expected to be u8"
                )
            }
        }
        _ => {
            abort_call_site!(
                "sp_runtime::ModuleError::error array type or primitive expected"
            )
        }
    }
}

/// The aim of this is to implement the `::subxt::HasModuleError` trait for
/// the generated `DispatchError`, so that we can obtain the module error details,
/// if applicable, from it.
pub fn generate_has_module_error_impl(
    metadata: &RuntimeMetadataV14,
    types_mod_ident: &syn::Ident,
) -> TokenStream2 {
    let dispatch_error = metadata
        .types
        .types()
        .iter()
        .find(|&ty| ty.ty().path().segments() == ["sp_runtime", "DispatchError"])
        .unwrap_or_else(|| {
            abort_call_site!("sp_runtime::DispatchError type expected in metadata")
        })
        .ty()
        .type_def();

    // Get the `DispatchError::Module` variant (either struct or named fields).
    let module_variant = match dispatch_error {
        TypeDef::Variant(variant) => {
            variant
                .variants()
                .iter()
                .find(|variant| variant.name() == "Module")
                .unwrap_or_else(|| {
                    abort_call_site!("DispatchError::Module variant expected in metadata")
                })
        }
        _ => abort_call_site!("DispatchError expected to contain variant in metadata"),
    };

    let module_field = module_variant.fields().get(0).unwrap_or_else(|| {
        abort_call_site!("DispatchError::Module expected to contain 1 or more fields")
    });

    let error_type = module_error_type(module_field, metadata);

    quote! {
        impl ::subxt::HasModuleError for #types_mod_ident::sp_runtime::DispatchError {
            fn module_error_data(&self) -> Option<::subxt::ModuleErrorData> {
                #error_type
            }
        }
    }
}
