// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::utils;
use heck::SnakeCase;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{
    format_ident,
    quote,
};
use syn::parse::{
    Parse,
    ParseStream,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(ignore);
}

#[derive(Debug)]
enum ModuleAttr {
    Ignore(kw::ignore),
}

impl Parse for ModuleAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self::Ignore(input.parse()?))
    }
}

type ModuleAttrs = utils::Attrs<ModuleAttr>;

fn ignore(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "module" {
                let attrs: ModuleAttrs = syn::parse2(attr.tokens.clone())
                    .map_err(|err| abort!("{}", err))
                    .unwrap();
                if !attrs.attrs.is_empty() {
                    return true
                }
            }
        }
    }
    false
}

fn events_decoder_trait_name(module: &syn::Ident) -> syn::Ident {
    format_ident!("{}EventsDecoder", module.to_string())
}

fn with_module_ident(module: &syn::Ident) -> syn::Ident {
    format_ident!("with_{}", module.to_string().to_snake_case())
}
/// Attribute macro that registers the type sizes used by the module; also sets the `MODULE` constant.
pub fn module(_args: TokenStream, tokens: TokenStream) -> TokenStream {
    let input: Result<syn::ItemTrait, _> = syn::parse2(tokens.clone());
    let input = if let Ok(input) = input {
        input
    } else {
        // handle #[module(ignore)] by just returning the tokens
        return tokens
    };

    let subxt = utils::use_crate("substrate-subxt");
    let module = &input.ident;
    let module_name = module.to_string();
    let module_events_decoder = events_decoder_trait_name(module);
    let with_module = with_module_ident(module);

    let bounds = input.supertraits.iter().filter_map(|bound| {
        if let syn::TypeParamBound::Trait(syn::TraitBound { path, .. }) = bound {
            let module = utils::path_to_ident(path);
            let with_module = with_module_ident(module);
            Some(quote! {
                self.#with_module();
            })
        } else {
            None
        }
    });
    let types = input.items.iter().filter_map(|item| {
        if let syn::TraitItem::Type(ty) = item {
            if ignore(&ty.attrs) {
                return None
            }
            let ident = &ty.ident;
            let ident_str = ident.to_string();
            Some(quote! {
                self.register_type_size::<T::#ident>(#ident_str);
            })
        } else {
            None
        }
    });

    quote! {
        #input

        const MODULE: &str = #module_name;

        /// `EventsDecoder` extension trait.
        pub trait #module_events_decoder {
            /// Registers this modules types.
            fn #with_module(&mut self);
        }

        impl<T: #module> #module_events_decoder for
            #subxt::EventsDecoder<T>
        {
            fn #with_module(&mut self) {
                #(#bounds)*
                #(#types)*
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_module() {
        let attr = quote!(#[module]);
        let input = quote! {
            pub trait Balances: System {
                type Balance: frame_support::Parameter
                    + sp_runtime::traits::Member
                    + sp_runtime::traits::AtLeast32Bit
                    + codec::Codec
                    + Default
                    + Copy
                    + sp_runtime::traits::MaybeSerialize
                    + std::fmt::Debug
                    + From<<Self as System>::BlockNumber>;
            }
        };
        let expected = quote! {
            pub trait Balances: System {
                type Balance: frame_support::Parameter
                    + sp_runtime::traits::Member
                    + sp_runtime::traits::AtLeast32Bit
                    + codec::Codec
                    + Default
                    + Copy
                    + sp_runtime::traits::MaybeSerialize
                    + std::fmt::Debug
                    + From< <Self as System>::BlockNumber>;
            }

            const MODULE: &str = "Balances";

            /// `EventsDecoder` extension trait.
            pub trait BalancesEventsDecoder {
                /// Registers this modules types.
                fn with_balances(&mut self);
            }

            impl<T: Balances> BalancesEventsDecoder for
                substrate_subxt::EventsDecoder<T>
            {
                fn with_balances(&mut self) {
                    self.with_system();
                    self.register_type_size::<T::Balance>("Balance");
                }
            }
        };

        let result = module(attr, input);
        utils::assert_proc_macro(result, expected);
    }

    #[test]
    fn test_herd() {
        let attr = quote!(#[module]);
        let input = quote! {
            pub trait Herd: Husbandry {
                type Hoves: u8;
                type Wool: bool;
                #[module(ignore)]
                type Digestion: EnergyProducer + fmt::Debug;
            }
        };
        let expected = quote! {
            pub trait Herd: Husbandry {
                type Hoves: u8;
                type Wool: bool;
                #[module(ignore)]
                type Digestion: EnergyProducer + fmt::Debug;
            }

            const MODULE: &str = "Herd";

            /// `EventsDecoder` extension trait.
            pub trait HerdEventsDecoder {
                /// Registers this modules types.
                fn with_herd(&mut self);
            }

            impl<T: Herd> HerdEventsDecoder for
                substrate_subxt::EventsDecoder<T>
            {
                fn with_herd(&mut self) {
                    self.with_husbandry();
                    self.register_type_size::<T::Hoves>("Hoves");
                    self.register_type_size::<T::Wool>("Wool");
                }
            }
        };

        let result = module(attr, input);
        utils::assert_proc_macro(result, expected);
    }
}
