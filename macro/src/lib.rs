// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

extern crate proc_macro;

use std::str::FromStr;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use subxt_codegen::{utils::Uri, CodegenError, DerivesRegistry, TypeSubstitutes};
use syn::{parse_macro_input, punctuated::Punctuated};

#[derive(Debug, FromMeta)]
struct RuntimeMetadataArgs {
    #[darling(default)]
    runtime_metadata_path: Option<String>,
    #[darling(default)]
    runtime_metadata_url: Option<String>,
    #[darling(default)]
    derive_for_all_types: Option<Punctuated<syn::Path, syn::Token![,]>>,
    #[darling(multiple)]
    derive_for_type: Vec<DeriveForType>,
    #[darling(multiple)]
    substitute_type: Vec<SubstituteType>,
    #[darling(default, rename = "crate")]
    crate_path: Option<String>,
    #[darling(default)]
    generate_docs: darling::util::Flag,
    #[darling(default)]
    runtime_types_only: bool,
}

#[derive(Debug, FromMeta)]
struct DeriveForType {
    #[darling(rename = "type")]
    ty: syn::TypePath,
    derive: Punctuated<syn::Path, syn::Token![,]>,
}

#[derive(Debug, FromMeta)]
struct SubstituteType {
    #[darling(rename = "type")]
    ty: syn::Path,
    with: syn::Path,
}

/// Generate a strongly typed API for interacting with a Substrate runtime from its metadata.
///
/// # Metadata
///
/// First, you'll need to get hold of some metadata for the node you'd like to interact with. One
/// way to do this is by using the `subxt` CLI tool:
///
/// ```bash
/// # Install the CLI tool:
/// cargo install subxt-cli
/// # Use it to download metadata (in this case, from a node running locally)
/// subxt metadata > polkadot_metadata.scale
/// ```
///
/// Run `subxt metadata --help` for more options.
///
/// # Basic usage
///
/// Annotate a Rust module with the `subxt` attribute referencing the aforementioned metadata file.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
/// )]
/// mod polkadot {}
/// ```
///
/// The `subxt` macro will populate the annotated module with all of the methods and types required
/// for interacting with the runtime that the metadata came from via Subxt.
///
/// # Configuration
///
/// This macro supports a number of attributes to configure what is generated:
///
/// ## `crate = "..."`
///
/// Use this attribute to specify a custom path to the `subxt` crate:
///
/// ```ignore
/// #[subxt::subxt(crate = "crate::path::to::subxt")]
/// mod polkadot {}
/// ```
///
/// This is useful if you write a library which uses this macro, but don't want to force users to depend on `subxt`
/// at the top level too. By default the path `::subxt` is used.
///
/// ## `substitute_type(type = "...", with = "...")`
///
/// This attribute replaces any reference to the generated type at the path given by `type` with a
/// reference to the path given by `with`.
///
/// ```rust,ignore
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
///     substitute_type(type = "sp_arithmetic::per_things::Perbill", with = "crate::Foo")
/// )]
/// mod polkadot {}
///
/// // For this substitute type to work, it needs to EncodeAsType/DecodeAsType in a
/// // compatible way with Perbill.
/// #[derive(
///     subxt::ext::scale_encode::EncodeAsType,
///     subxt::ext::scale_decode::DecodeAsType,
///     subxt::ext::codec::Encode,
///     subxt::ext::codec::Decode,
///     Clone,
///     Debug,
/// )]
/// #[codec(crate = ::subxt::ext::codec)]
/// #[encode_as_type(crate_path = "::subxt::ext::scale_encode")]
/// #[decode_as_type(crate_path = "::subxt::ext::scale_decode")]
/// struct Foo(u32);
/// ```
///
/// If the type you're substituting contains generic parameters, you can "pattern match" on those, and
/// make use of them in the substituted type, like so:
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
///     substitute_type(
///         type = "sp_runtime::multiaddress::MultiAddress<A, B>",
///         with = "::subxt::utils::Static<::sp_runtime::MultiAddress<A, B>>"
///     )
/// )]
/// mod polkadot {}
/// ```
///
/// The above is also an example of using the `subxt::utils::Static` type to wrap some type which doesn't
/// on it's own implement `EncodeAsType` or `DecodeAsType`, which are required traits for any substitute
/// type to implement by default.
///
/// ## `derive_for_all_types = "..."`
///
/// By default, all generated types derive a small set of traits. This attribute allows you to derive additional
/// traits on all generated types:
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
///     derive_for_all_types = "Eq, PartialEq"
/// )]
/// mod polkadot {}
/// ```
///
/// Any substituted types (including the default substitutes) must also implement these traits in order to avoid errors
/// here.
///
/// ## `derive_for_type(type = "...", derive = "...")`
///
/// Unlike the above, which derives some trait on every generated type, this attribute allows you to derive traits only
/// for specific types. Note that any types which are used inside the specified type may also need to derive the same traits.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
///     derive_for_all_types = "Eq, PartialEq",
///     derive_for_type(type = "frame_support::PalletId", derive = "Ord, PartialOrd"),
///     derive_for_type(type = "sp_runtime::ModuleError", derive = "Hash"),
/// )]
/// mod polkadot {}
/// ```
///
/// ## `runtime_metadata_url = "..."`
///
/// This attribute can be used instead of `runtime_metadata_path` and will tell the macro to download metadata from a node running
/// at the provided URL, rather than a node running locally. This can be useful in CI, but is **not recommended** in production code,
/// since it runs at compile time and will cause compilation to fail if the node at the given address is unavailable or unresponsive.
///
/// ```rust,ignore
/// #[subxt::subxt(
///     runtime_metadata_url = "wss://rpc.polkadot.io:443"
/// )]
/// mod polkadot {}
/// ```
///
/// ## `generate_docs`
///
/// By default, documentation is not generated via the macro, since IDEs do not typically make use of it. This attribute
/// forces documentation to be generated, too.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
///     generate_docs
/// )]
/// mod polkadot {}
/// ```
///
/// ## `runtime_types_only`
///
/// By default, the macro will generate various interfaces to make using Subxt simpler in addition with any types that need
/// generating to make this possible. This attribute makes the codegen only generate the types and not the Subxt interface.
///
/// ```rust,no_run
/// #[subxt::subxt(
///     runtime_metadata_path = "../artifacts/polkadot_metadata.scale",
///     runtime_types_only
/// )]
/// mod polkadot {}
/// ```
///
#[proc_macro_attribute]
#[proc_macro_error]
pub fn subxt(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as syn::AttributeArgs);
    let item_mod = parse_macro_input!(input as syn::ItemMod);
    let args = match RuntimeMetadataArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let crate_path = match args.crate_path {
        Some(crate_path) => crate_path.into(),
        None => subxt_codegen::CratePath::default(),
    };
    let mut derives_registry = DerivesRegistry::new(&crate_path);

    if let Some(derive_for_all) = args.derive_for_all_types {
        derives_registry.extend_for_all(derive_for_all.iter().cloned());
    }
    for derives in &args.derive_for_type {
        derives_registry.extend_for_type(
            derives.ty.clone(),
            derives.derive.iter().cloned(),
            &crate_path,
        )
    }

    let mut type_substitutes = TypeSubstitutes::new(&crate_path);
    let substitute_args_res: Result<(), _> = args.substitute_type.into_iter().try_for_each(|sub| {
        sub.with
            .try_into()
            .and_then(|with| type_substitutes.insert(sub.ty, with))
    });

    if let Err(err) = substitute_args_res {
        return CodegenError::from(err).into_compile_error().into();
    }

    let should_gen_docs = args.generate_docs.is_present();
    match (args.runtime_metadata_path, args.runtime_metadata_url) {
        (Some(rest_of_path), None) => {
            let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
            let root_path = std::path::Path::new(&root);
            let path = root_path.join(rest_of_path);
            subxt_codegen::generate_runtime_api_from_path(
                item_mod,
                path,
                derives_registry,
                type_substitutes,
                crate_path,
                should_gen_docs,
                args.runtime_types_only,
            )
            .map_or_else(|err| err.into_compile_error().into(), Into::into)
        }
        (None, Some(url_string)) => {
            let url = Uri::from_str(&url_string).unwrap_or_else(|_| {
                abort_call_site!("Cannot download metadata; invalid url: {}", url_string)
            });
            subxt_codegen::generate_runtime_api_from_url(
                item_mod,
                &url,
                derives_registry,
                type_substitutes,
                crate_path,
                should_gen_docs,
                args.runtime_types_only,
            )
            .map_or_else(|err| err.into_compile_error().into(), Into::into)
        }
        (None, None) => {
            abort_call_site!(
                "One of 'runtime_metadata_path' or 'runtime_metadata_url' must be provided"
            )
        }
        (Some(_), Some(_)) => {
            abort_call_site!(
                "Only one of 'runtime_metadata_path' or 'runtime_metadata_url' can be provided"
            )
        }
    }
}
