use proc_macro2::{Span, TokenStream};
use quote::quote;
use synstructure::{BindingInfo, Structure};

pub fn use_crate(name: &str) -> syn::Ident {
    let krate = proc_macro_crate::crate_name(name).unwrap();
    syn::Ident::new(&krate, Span::call_site())
}

pub fn bindings<'a>(s: &'a Structure) -> Vec<&'a BindingInfo<'a>> {
    let mut bindings = vec![];
    for variant in s.variants() {
        for binding in variant.bindings() {
            bindings.push(binding);
        }
    }
    bindings
}

pub fn module_name(generics: &syn::Generics) -> &syn::Path {
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

pub fn path_to_ident(path: &syn::Path) -> &syn::Ident {
    &path.segments.iter().last().unwrap().ident
}

pub fn type_params(generics: &syn::Generics) -> Vec<TokenStream> {
    generics
        .params
        .iter()
        .filter_map(|g| {
            match g {
                syn::GenericParam::Type(p) => {
                    let ident = &p.ident;
                    Some(quote!(#ident))
                }
                syn::GenericParam::Lifetime(p) => {
                    let lifetime = &p.lifetime;
                    Some(quote!(#lifetime))
                }
                syn::GenericParam::Const(_) => None,
            }
        })
        .collect()
}

#[cfg(test)]
pub(crate) fn assert_proc_macro(
    result: proc_macro2::TokenStream,
    expected: proc_macro2::TokenStream,
) {
    let result = result.to_string();
    let expected = expected.to_string();
    pretty_assertions::assert_eq!(result, expected);
}
