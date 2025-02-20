// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::CodegenError;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use scale_typegen::typegen::ir::ToTokensWithSettings;
use scale_typegen::TypeGenerator;
use subxt_metadata::PalletMetadata;

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
/// - `type_gen` - [`scale_typegen::TypeGenerator`] that contains settings and all types from the runtime metadata.
/// - `pallet` - Pallet metadata from which the events are generated.
/// - `crate_path` - The crate path under which the `subxt-core` crate is located, e.g. `::subxt::ext::subxt_core` when using subxt as a dependency.
pub fn generate_events(
    type_gen: &TypeGenerator,
    pallet: &PalletMetadata,
    crate_path: &syn::Path,
) -> Result<TokenStream2, CodegenError> {
    // Early return if the pallet has no events.
    let Some(event_ty) = pallet.event_ty_id() else {
        return Ok(quote!());
    };

    let variant_names_and_struct_defs =
        super::generate_structs_from_variants(type_gen, event_ty, |name| name.into(), "Event")?;

    let event_structs = variant_names_and_struct_defs.into_iter().map(|var| {
        let pallet_name = pallet.name();
        let event_struct_name = &var.composite.name;
        let event_name = var.variant_name;
        let alias_mod = var.type_alias_mod;
        let struct_def = type_gen
            .upcast_composite(&var.composite)
            .to_token_stream(type_gen.settings());
        quote! {
            #struct_def
            #alias_mod

            impl #crate_path::events::StaticEvent for #event_struct_name {
                const PALLET: &'static str = #pallet_name;
                const EVENT: &'static str = #event_name;
            }
        }
    });

    let event_type = type_gen
        .resolve_type_path(event_ty)?
        .to_token_stream(type_gen.settings());
    let event_ty = type_gen.resolve_type(event_ty)?;
    let docs = &event_ty.docs;
    let docs = type_gen
        .settings()
        .should_gen_docs
        .then_some(quote! { #( #[doc = #docs ] )* })
        .unwrap_or_default();
    let types_mod_ident = type_gen.types_mod_ident();

    Ok(quote! {
        #docs
        pub type Event = #event_type;
        pub mod events {
            use super::#types_mod_ident;
            #( #event_structs )*
        }
    })
}
