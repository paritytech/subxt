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
