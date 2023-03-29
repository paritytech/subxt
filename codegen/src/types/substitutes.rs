// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{api::CodegenError, CratePath};
use std::{borrow::Cow, collections::HashMap};
use syn::{parse_quote, spanned::Spanned as _};

use super::TypePath;

#[derive(Debug)]
pub struct TypeSubstitutes {
    substitutes: HashMap<PathSegments, Substitute>,
}

#[derive(Debug)]
struct Substitute {
    path: syn::Path,
    param_mapping: TypeParamMapping,
}

#[derive(Debug)]
enum TypeParamMapping {
    None,
    Specified(Vec<u8>),
}

#[macro_export]
macro_rules! path_segments {
    ($($ident: ident)::*) => {
        PathSegments(
            [$(stringify!($ident)),*].into_iter().map(String::from).collect::<Vec<_>>()
        )
    }
}

impl TypeSubstitutes {
    pub fn new(crate_path: &CratePath) -> Self {
        // Some hardcoded default type substitutes, can be overridden by user
        let defaults = [
            (
                path_segments!(bitvec::order::Lsb0),
                parse_quote!(#crate_path::utils::bits::Lsb0),
            ),
            (
                path_segments!(bitvec::order::Msb0),
                parse_quote!(#crate_path::utils::bits::Msb0),
            ),
            (
                path_segments!(sp_core::crypto::AccountId32),
                parse_quote!(#crate_path::utils::AccountId32),
            ),
            (
                path_segments!(sp_runtime::multiaddress::MultiAddress),
                parse_quote!(#crate_path::utils::MultiAddress),
            ),
            (
                path_segments!(primitive_types::H160),
                parse_quote!(#crate_path::utils::H160),
            ),
            (
                path_segments!(primitive_types::H256),
                parse_quote!(#crate_path::utils::H256),
            ),
            (
                path_segments!(primitive_types::H512),
                parse_quote!(#crate_path::utils::H512),
            ),
            (
                path_segments!(frame_support::traits::misc::WrapperKeepOpaque),
                parse_quote!(#crate_path::utils::WrapperKeepOpaque),
            ),
            // BTreeMap and BTreeSet impose an `Ord` constraint on their key types. This
            // can cause an issue with generated code that doesn't impl `Ord` by default.
            // Decoding them to Vec by default (KeyedVec is just an alias for Vec with
            // suitable type params) avoids these issues.
            (
                path_segments!(BTreeMap),
                parse_quote!(#crate_path::utils::KeyedVec),
            ),
            (path_segments!(BTreeSet), parse_quote!(::std::vec::Vec)),
        ];

        let default_substitutes = defaults
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Substitute {
                        path: v,
                        param_mapping: TypeParamMapping::None,
                    },
                )
            })
            .collect();

        Self {
            substitutes: default_substitutes,
        }
    }

    /// Only insert the given substitution if a substitution at that path doesn't
    /// already exist.
    pub fn insert_if_not_exists(
        &mut self,
        source: syn::Path,
        target: AbsolutePath,
    ) -> Result<(), CodegenError> {
        let (key, val) = TypeSubstitutes::parse_path_substitution(source, target.0)?;
        self.substitutes.entry(key).or_insert(val);
        Ok(())
    }

    /// Add a bunch of source to target type substitutions.
    pub fn extend(
        &mut self,
        elems: impl IntoIterator<Item = (syn::Path, AbsolutePath)>,
    ) -> Result<(), CodegenError> {
        for (source, target) in elems.into_iter() {
            let (key, val) = TypeSubstitutes::parse_path_substitution(source, target.0)?;
            self.substitutes.insert(key, val);
        }
        Ok(())
    }

    /// Given a source and target path, parse the type params to work out the mapping from
    /// source to target, and output the source => substitution mapping that we work out from this.
    fn parse_path_substitution(
        src_path: syn::Path,
        mut target_path: syn::Path,
    ) -> Result<(PathSegments, Substitute), CodegenError> {
        let Some(syn::PathSegment { arguments: src_path_args, ..}) = src_path.segments.last() else {
            return Err(CodegenError::EmptySubstitutePath(src_path.span()))
        };
        let Some(syn::PathSegment { arguments: target_path_args, ..}) = target_path.segments.last_mut() else {
            return Err(CodegenError::EmptySubstitutePath(target_path.span()))
        };

        let source_args: Vec<_> = type_args(src_path_args).collect();

        let param_mapping = if source_args.is_empty() {
            // If the type parameters on the source type are not specified, then this means that
            // the type is either not generic or the user wants to pass through all the parameters
            TypeParamMapping::None
        } else {
            // Describe the mapping in terms of "which source param idx is used for each target param".
            // So, for each target param, find the matching source param index.
            let mapping = type_args(target_path_args)
                .filter_map(|arg| {
                    source_args
                        .iter()
                        .position(|&src| src == arg)
                        .map(|src_idx| {
                            u8::try_from(src_idx).expect("type arguments to be fewer than 256; qed")
                        })
                })
                .collect();
            TypeParamMapping::Specified(mapping)
        };

        // Now that we've parsed the type params from our target path, remove said params from
        // that path, since we're storing them separately.
        *target_path_args = syn::PathArguments::None;

        Ok((
            PathSegments::from(&src_path),
            Substitute {
                path: target_path,
                param_mapping,
            },
        ))
    }

    /// Given a source type path, return a substituted type path if a substitution is defined.
    pub fn for_path(&self, path: impl Into<PathSegments>) -> Option<&syn::Path> {
        self.substitutes.get(&path.into()).map(|s| &s.path)
    }

    /// Given a source type path and the resolved, supplied type parameters,
    /// return a new path and optionally overwritten type parameters.
    pub fn for_path_with_params<'a: 'b, 'b>(
        &'a self,
        path: impl Into<PathSegments>,
        params: &'b [TypePath],
    ) -> Option<(&'a syn::Path, Cow<'b, [TypePath]>)> {
        // For now, we only support:
        // 1. Reordering the generics
        // 2. Omitting certain generics
        fn reorder_params<'a>(
            params: &'a [TypePath],
            mapping: &TypeParamMapping,
        ) -> Cow<'a, [TypePath]> {
            match mapping {
                TypeParamMapping::Specified(mapping) => Cow::Owned(
                    mapping
                        .iter()
                        .filter_map(|&idx| params.get(idx as usize))
                        .cloned()
                        .collect(),
                ),
                _ => Cow::Borrowed(params),
            }
        }

        let path = path.into();

        self.substitutes
            .get(&path)
            .map(|sub| (&sub.path, reorder_params(params, &sub.param_mapping)))
    }
}

/// Identifiers joined by the `::` separator.
///
/// We use this as a common denominator, since we need a consistent keys for both
/// `syn::TypePath` and `scale_info::ty::path::Path` types.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PathSegments(Vec<String>);

impl From<&syn::Path> for PathSegments {
    fn from(path: &syn::Path) -> Self {
        PathSegments(path.segments.iter().map(|x| x.ident.to_string()).collect())
    }
}

impl<T: scale_info::form::Form> From<&scale_info::Path<T>> for PathSegments {
    fn from(path: &scale_info::Path<T>) -> Self {
        PathSegments(
            path.segments
                .iter()
                .map(|x| x.as_ref().to_owned())
                .collect(),
        )
    }
}

/// Returns an iterator over generic type parameters for `syn::PathArguments`.
/// For example:
/// - `<'a, T>` should only return T
/// - `(A, B) -> String` shouldn't return anything
fn type_args(path_args: &syn::PathArguments) -> impl Iterator<Item = &syn::Path> {
    let args_opt = match path_args {
        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            ref args,
            ..
        }) => Some(args),
        _ => None,
    };

    args_opt.into_iter().flatten().filter_map(|arg| match arg {
        syn::GenericArgument::Type(syn::Type::Path(type_path)) => Some(&type_path.path),
        _ => None,
    })
}

/// Whether a path is absolute - starts with `::` or `crate`.
fn is_absolute(path: &syn::Path) -> bool {
    path.leading_colon.is_some()
        || path
            .segments
            .first()
            .map_or(false, |segment| segment.ident == "crate")
}

pub struct AbsolutePath(pub syn::Path);

impl TryFrom<syn::Path> for AbsolutePath {
    type Error = (syn::Path, String);
    fn try_from(value: syn::Path) -> Result<Self, Self::Error> {
        if is_absolute(&value) {
            Ok(AbsolutePath(value))
        } else {
            Err(
                (value, "The substitute path must be a global absolute path; try prefixing with `::` or `crate`".to_owned())
            )
        }
    }
}
