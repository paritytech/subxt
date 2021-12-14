// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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
use frame_metadata::{
    PalletCallMetadata,
    PalletMetadata,
};
use heck::SnakeCase as _;
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{
    format_ident,
    quote,
};
use scale_info::form::PortableForm;

/// Generate the config struct and default implementations if required.
pub fn generate_config(
    config_struct: &syn::ItemStruct,
    generate_default_impls: bool,
) -> TokenStream {
    let config_name = &struct_.ident;
    let default_impls = generate_default_impls.then(|| quote! {
        impl ::subxt::Config for #config_name {
            type Index = u32;
            type BlockNumber = u32;
            type Hash = ::subxt::sp_core::H256;
            type Hashing = ::subxt::sp_runtime::traits::BlakeTwo256;
            type AccountId = ::subxt::sp_runtime::AccountId32;
            type Address = ::subxt::sp_runtime::MultiAddress<Self::AccountId, u32>;
            type Header = ::subxt::sp_runtime::generic::Header<
                Self::BlockNumber, ::subxt::sp_runtime::traits::BlakeTwo256
            >;
            type Signature = ::subxt::sp_runtime::MultiSignature;
            type Extrinsic = ::subxt::sp_runtime::OpaqueExtrinsic;
        }

        impl ::subxt::ExtrinsicExtraData<#config_name> for #config_name {
            type AccountData = AccountData;
            type Extra = ::subxt::DefaultExtra<#config_name>;
        }

        pub type AccountData = self::system::storage::Account;

        impl ::subxt::AccountData<#config_name> for AccountData {
            fn nonce(result: &<Self as ::subxt::StorageEntry>::Value) -> <#config_name as ::subxt::Config>::Index {
                result.nonce
            }
            fn storage_entry(account_id: <#config_name as ::subxt::Config>::AccountId) -> Self {
                Self(account_id)
            }
        }
    });
    quote! {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        #config_struct

        #default_impls
    }
}