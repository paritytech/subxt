extern crate proc_macro;

use heck::{
    CamelCase,
    SnakeCase,
};
use proc_macro::TokenStream;
use proc_macro2::{
    Span,
    TokenStream as TokenStream2,
};
use proc_macro_crate::crate_name;
use quote::{
    format_ident,
    quote,
};
use syn::parse_macro_input;
use synstructure::{
    decl_derive,
    BindingInfo,
    Structure,
};

fn use_crate(name: &str) -> syn::Ident {
    let krate = crate_name(name).unwrap();
    syn::Ident::new(&krate, Span::call_site())
}

#[proc_macro_attribute]
pub fn module(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemTrait);

    let subxt = use_crate("substrate-subxt");
    let module_name = &input.ident;
    let module_name_str = module_name.to_string();
    let module_events_decoder = format_ident!("{}EventsDecoder", module_name);
    let with_module = format_ident!("with_{}", module_name_str.to_snake_case());

    let types = input.items.iter().filter_map(|item| {
        if let syn::TraitItem::Type(ty) = item {
            let ident = &ty.ident;
            let ident_str = ident.to_string();
            Some(quote! {
                decoder.register_type_size::<T::#ident>(#ident_str)?;
            })
        } else {
            None
        }
    });

    let expanded = quote! {
        #input

        const MODULE: &str = #module_name_str;

        pub trait #module_events_decoder {
            fn #with_module(self) -> Self;
        }

        impl<T: #module_name, P, S, E> #module_events_decoder for
            #subxt::EventsSubscriber<T, P, S, E>
        {
            fn #with_module(self) -> Self {
                self.events_decoder(|decoder| {
                    #(#types)*
                    Ok(0)
                })
            }
        }
    };

    TokenStream::from(expanded)
}

fn bindings<'a>(s: &'a Structure) -> Vec<&'a BindingInfo<'a>> {
    let mut bindings = vec![];
    for variant in s.variants() {
        for binding in variant.bindings() {
            bindings.push(binding);
        }
    }
    bindings
}

decl_derive!([Call] => call);

fn module_name<'a>(generics: &'a syn::Generics) -> &'a syn::Path {
    generics
        .params
        .iter()
        .filter_map(|p| {
            if let syn::GenericParam::Type(p) = p {
                p.bounds
                    .iter()
                    .filter_map(|b| {
                        if let syn::TypeParamBound::Trait(t) = b {
                            Some(&t.path)
                        } else {
                            None
                        }
                    })
                    .next()
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn call(s: Structure) -> TokenStream {
    let subxt = use_crate("substrate-subxt");
    let codec = use_crate("parity-scale-codec");
    let sp_core = use_crate("sp-core");
    let sp_runtime = use_crate("sp-runtime");
    let ident = &s.ast().ident;
    let generics = &s.ast().generics;
    let module_name = module_name(generics);
    let call_name = ident.to_string().trim_end_matches("Call").to_snake_case();
    let call = format_ident!("{}", call_name);
    let trait_name = format_ident!("{}CallExt", call_name.to_camel_case());
    let bindings = bindings(&s);
    let fields = bindings.iter().map(|bi| {
        let ident = bi.ast().ident.as_ref().unwrap();
        quote!(#ident,)
    });
    let args = bindings.iter().map(|bi| {
        let ident = bi.ast().ident.as_ref().unwrap();
        let ty = &bi.ast().ty;
        quote!(#ident: #ty,)
    });
    let ret = quote!(#subxt::ExtrinsicSuccess<T>);
    let ret = quote!(core::pin::Pin<Box<dyn core::future::Future<Output = Result<#ret, #subxt::Error>>>>);
    let build_call = quote!(#subxt::Call::<#ident<T>>::new(MODULE, #call_name, #ident { #(#fields)* }));
    let args = quote!(#(#args)*);

    let expanded = quote! {
        pub fn #call#generics(
            #args
        ) -> #subxt::Call<#ident<T>> {
            #build_call
        }

        pub trait #trait_name#generics {
            fn #call(
                self,
                #args
            ) -> #ret;
        }

        impl<T, P, S, E> #trait_name<T> for #subxt::EventsSubscriber<T, P, S, E>
        where
            T: #module_name + #subxt::system::System + Send + Sync,
            P: #sp_core::Pair,
            S: #sp_runtime::traits::Verify + #codec::Codec + From<P::Signature> + 'static,
            S::Signer: From<P::Public> + #sp_runtime::traits::IdentifyAccount<AccountId = T::AccountId>,
            T::Address: From<T::AccountId>,
            E: #subxt::SignedExtra<T> + #sp_runtime::traits::SignedExtension + 'static,
        {
            fn #call(self, #args) -> #ret {
                Box::pin(self.submit(#build_call))
            }
        }
    };

    TokenStream::from(expanded)
}

decl_derive!([Event] => event);

fn event(s: Structure) -> TokenStream {
    let subxt = use_crate("substrate-subxt");
    let codec = use_crate("parity-scale-codec");
    let ident = &s.ast().ident;
    let generics = &s.ast().generics;
    let event_name = ident.to_string().trim_end_matches("Event").to_camel_case();
    let event = format_ident!("{}", event_name.to_snake_case());
    let event_trait_ext = format_ident!("{}EventExt", event_name);

    let expanded = quote! {
        pub trait #event_trait_ext#generics {
            fn #event(&self) -> Option<Result<#ident<T>, #codec::Error>>;
        }

        impl#generics #event_trait_ext<T> for #subxt::ExtrinsicSuccess<T> {
            fn #event(&self) -> Option<Result<#ident<T>, #codec::Error>> {
                self.find_event(MODULE, #event_name)
            }
        }
    };

    TokenStream::from(expanded)
}

struct StorageItem<'a> {
    ident: &'a syn::Ident,
    args: Vec<(&'a syn::Ident, &'a syn::Type)>,
    ret: TokenStream2,
}

impl<'a> From<&'a syn::TraitItemMethod> for StorageItem<'a> {
    fn from(method: &'a syn::TraitItemMethod) -> Self {
        let ret = if let syn::ReturnType::Type(_, ty) = &method.sig.output {
            quote!(#ty)
        } else {
            quote!(())
        };
        let mut args = vec![];
        for arg in &method.sig.inputs {
            if let syn::FnArg::Typed(pat) = arg {
                if let syn::Pat::Ident(ipat) = &*pat.pat {
                    args.push((&ipat.ident, &*pat.ty));
                }
            }
        }
        Self {
            ident: &method.sig.ident,
            args,
            ret,
        }
    }
}

impl<'a> StorageItem<'a> {
    fn return_ty(&self) -> TokenStream2 {
        let subxt = use_crate("substrate-subxt");
        let ret = &self.ret;
        let result = quote!(Result<#ret, #subxt::Error>);
        quote! {
            core::pin::Pin<Box<dyn core::future::Future<Output = #result> + Send + 'a>>
        }
    }

    fn method(&self) -> TokenStream2 {
        let ident = self.ident;
        let ret = self.return_ty();
        let args = self.args.iter().map(|(arg, ty)| quote!(#arg: #ty,));
        quote!(fn #ident<'a>(&'a self, #(#args)*) -> #ret)
    }

    fn trait_item(&self) -> TokenStream2 {
        let method = self.method();
        quote!(#method;)
    }

    fn impl_item(&self) -> TokenStream2 {
        let futures = use_crate("futures");
        let method = self.method();
        let storage = self.ident.to_string().to_camel_case();
        let storage_ty = match self.args.len() {
            0 => format_ident!("plain"),
            1 => format_ident!("map"),
            2 => format_ident!("double_map"),
            _ => unimplemented!(),
        };
        let keys = self.args.iter().map(|(arg, _)| quote!(#arg,));
        quote! {
            #method {
                let store_fn = || {
                    Ok(self.metadata().module(MODULE)?.storage(#storage)?.#storage_ty()?)
                };
                let store = match store_fn() {
                    Ok(v) => v,
                    Err(e) => return Box::pin(#futures::future::err(e)),
                };
                let future = self.fetch(store.key(#(#keys)*), None);
                Box::pin(async move {
                    let v = if let Some(v) = future.await? {
                        Some(v)
                    } else {
                        store.default().cloned()
                    };
                    Ok(v.unwrap_or_default())
                })
            }
        }
    }
}

#[proc_macro_attribute]
pub fn storage(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemTrait);
    let subxt = use_crate("substrate-subxt");
    let ident = &input.ident;
    let bound = format_ident!("{}", ident.to_string().trim_end_matches("Store"));
    let storage_items = input
        .items
        .iter()
        .filter_map(|item| {
            if let syn::TraitItem::Method(method) = item {
                Some(StorageItem::from(method))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let trait_items = storage_items.iter().map(|s| s.trait_item());
    let impl_items = storage_items.iter().map(|s| s.impl_item());

    let expanded = quote! {
        pub trait #ident<T: #bound> {
            #(#trait_items)*
        }

        impl<T, S, E> #ident<T> for #subxt::Client<T, S, E>
        where
            T: #bound + Send + Sync,
            S: 'static,
            E: Send + Sync + 'static,
        {
            #(#impl_items)*
        }
    };

    TokenStream::from(expanded)
}
