// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{types::TypeGenerator, CratePath};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use subxt_metadata::PalletMetadata;

use super::CodegenError;

/// Generate events from the provided pallet metadata.
///
/// The function creates a new module named `events` under the pallet's module.
///
/// ```ignore
/// pub mod PalletName {
///     pub mod events {
///     ...
///     }
/// }
/// ```
///
/// The function generates the events as rust structs that implement the `subxt::event::StaticEvent` trait
/// to uniquely identify the event's identity when creating the extrinsic.
///
/// ```ignore
/// pub struct EventName {
///      pub event_param: type,
/// }
/// impl ::subxt::events::StaticEvent for EventName {
/// ...
/// }
/// ```
///
/// # Arguments
///
/// - `type_gen` - The type generator containing all types defined by metadata.
/// - `pallet` - Pallet metadata from which the events are generated.
/// - `types_mod_ident` - The ident of the base module that we can use to access the generated types from.
pub fn generate_events(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    types_mod_ident: &syn::Ident,
    crate_path: &CratePath,
    should_gen_docs: bool,
) -> Result<TokenStream2, CodegenError> {
    // Early return if the pallet has no events.
    let Some(event_ty) = pallet.event_ty_id() else {
        return Ok(quote!())
    };

    let struct_defs = super::generate_structs_from_variants(
        type_gen,
        event_ty,
        |name| name.into(),
        "Event",
        crate_path,
        should_gen_docs,
    )?;

    let event_structs = struct_defs.iter().map(|(variant_name, struct_def)| {
        let pallet_name = pallet.name();
        let event_struct = &struct_def.name;
        let event_name = variant_name;

        quote! {
            #struct_def

            impl #crate_path::events::StaticEvent for #event_struct {
                const PALLET: &'static str = #pallet_name;
                const EVENT: &'static str = #event_name;
            }
        }
    });
    let event_type = type_gen.resolve_type_path(event_ty);
    let event_ty = type_gen.resolve_type(event_ty);
    let docs = &event_ty.docs;
    let docs = should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();

    Ok(quote! {
        #docs
        pub type Event = #event_type;
        pub mod events {
            use super::#types_mod_ident;
            #( #event_structs )*
        }
    })
}
