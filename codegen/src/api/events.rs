// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::types::TypeGenerator;
use frame_metadata::PalletMetadata;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use scale_info::form::PortableForm;

/// Generate events from the provided pallet metadata.
///
/// The function creates a new module named `events` under the pallet's module.
/// ```ignore
/// pub mod PalletName {
///     pub mod events {
///     ...
///     }
/// }
/// ```
///
/// The function generates the events as rust structs that implement the `subxt::Event` trait
/// to uniquely identify the event's identity when creating the extrinsic.
///
/// ```ignore
/// pub struct EventName {
///      pub event_param: type,
/// }
/// impl ::subxt::Event for EventName {
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
    pallet: &PalletMetadata<PortableForm>,
    types_mod_ident: &syn::Ident,
) -> TokenStream2 {
    // Early return if the pallet has no events.
    let event = if let Some(ref event) = pallet.event {
        event
    } else {
        return quote!()
    };

    let struct_defs = super::generate_structs_from_variants(
        type_gen,
        event.ty.id(),
        |name| name.into(),
        "Event",
    );
    let event_structs = struct_defs.iter().map(|(variant_name, struct_def)| {
        let pallet_name = &pallet.name;
        let event_struct = &struct_def.name;
        let event_name = variant_name;

        quote! {
            #struct_def

            impl ::subxt::Event for #event_struct {
                const PALLET: &'static str = #pallet_name;
                const EVENT: &'static str = #event_name;
            }
        }
    });
    let event_type = type_gen.resolve_type_path(event.ty.id(), &[]);
    let event_ty = type_gen.resolve_type(event.ty.id());
    let docs = event_ty.docs();

    quote! {
        #( #[doc = #docs ] )*
        pub type Event = #event_type;
        pub mod events {
            use super::#types_mod_ident;
            #( #event_structs )*
        }
    }
}
