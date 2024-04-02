// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    Error,
};

/// Environment variable for setting the timeout for the test.
const SUBXT_TEST_TIMEOUT: &str = "SUBXT_TEST_TIMEOUT";

/// Default timeout for the test.
const DEFAULT_TIMEOUT: u64 = 60 * 6;

#[proc_macro_attribute]
pub fn subxt_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let subxt_attr = match syn::parse::<SubxtTestAttr>(attr) {
        Ok(subxt_attr) => subxt_attr,
        Err(err) => return err.into_compile_error().into(),
    };

    // Timeout is determined by:
    // - The timeout attribute if it is set.
    // - The SUBXT_TEST_TIMEOUT environment variable if it is set.
    // - A default of 6 minutes.
    let timeout_duration = subxt_attr.timeout.unwrap_or_else(|| {
        std::env::var(SUBXT_TEST_TIMEOUT)
            .map(|str| str.parse().unwrap_or(DEFAULT_TIMEOUT))
            .unwrap_or(DEFAULT_TIMEOUT)
    });

    let func: syn::ItemFn = match syn::parse(item) {
        Ok(func) => func,
        Err(err) => return err.into_compile_error().into(),
    };

    let func_attrs = &func.attrs;
    let func_vis = &func.vis;
    let func_sig = &func.sig;
    let func_block = &func.block;

    let mut inner_func_sig = func.sig.clone();
    inner_func_sig.ident = format_ident!("{}_inner", inner_func_sig.ident);
    let inner_func_name = &inner_func_sig.ident;

    let result = quote! {
        #[tokio::test]
        #( #func_attrs )*
        #func_vis #func_sig {
            #func_vis #inner_func_sig
            #func_block

            tokio::time::timeout(std::time::Duration::from_secs(#timeout_duration), #inner_func_name())
                .await
                .expect("Test timedout")
        }
    };
    result.into()
}

mod keywords {
    syn::custom_keyword!(timeout);
}

struct SubxtTestAttr {
    timeout: Option<u64>,
}

impl Parse for SubxtTestAttr {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        if input.is_empty() {
            return Ok(Self { timeout: None });
        }

        let _keyword = input.parse::<keywords::timeout>()?;
        input.parse::<syn::token::Eq>()?;
        let timeout = input.parse::<syn::LitInt>()?.base10_parse::<u64>()?;

        if !input.is_empty() {
            return Err(Error::new(
                input.span(),
                "Expected tokens: `timeout = value`",
            ));
        }

        Ok(Self {
            timeout: Some(timeout),
        })
    }
}
