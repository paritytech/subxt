// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{error::TypeSubstitutionError, types::TypePathType, CratePath};
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
    // Pass any generics from source to target type
    PassThrough,
    // Map the input types based on this
    Specified(Vec<TypeParamReplacement>),
}

#[derive(Debug)]
enum TypeParamReplacement {
    // Replace the type the the input generic type at this index
    InputAtIndex(usize),
    // Replace the type with this concrete path
    ConcreteType(syn::TypePath),
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
                        param_mapping: TypeParamMapping::PassThrough,
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
    ) -> Result<(), TypeSubstitutionError> {
        let (key, val) = TypeSubstitutes::parse_path_substitution(source, target.0)?;
        self.substitutes.entry(key).or_insert(val);
        Ok(())
    }

    /// Add a bunch of source to target type substitutions.
    pub fn extend(
        &mut self,
        elems: impl IntoIterator<Item = (syn::Path, AbsolutePath)>,
    ) -> Result<(), TypeSubstitutionError> {
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
    ) -> Result<(PathSegments, Substitute), TypeSubstitutionError> {
        let param_mapping = Self::parse_path_param_mapping(&src_path, &target_path)?;

        // The generic args of the target path are no longer needed; we store the useful
        // details in the param mapping now. So remove them (in part for nicer debug printing).
        if let Some(last) = target_path.segments.last_mut() {
            last.arguments = Default::default();
        }

        Ok((
            PathSegments::from(&src_path),
            Substitute {
                // Note; at this point, target_path might have some generics still. These
                // might be hardcoded types that we want to keep, so leave them here for now.
                path: target_path,
                param_mapping,
            },
        ))
    }

    /// Given a source and target path, parse the type params to work out the mapping from
    /// source to target, and return it.
    fn parse_path_param_mapping(
        src_path: &syn::Path,
        target_path: &syn::Path,
    ) -> Result<TypeParamMapping, TypeSubstitutionError> {
        let Some(syn::PathSegment { arguments: src_path_args, ..}) = src_path.segments.last() else {
            return Err(TypeSubstitutionError::EmptySubstitutePath(src_path.span()))
        };
        let Some(syn::PathSegment { arguments: target_path_args, ..}) = target_path.segments.last() else {
            return Err(TypeSubstitutionError::EmptySubstitutePath(target_path.span()))
        };

        // Get hold of the generic args for the "from" type, erroring if they aren't valid.
        let source_args = match src_path_args {
            syn::PathArguments::None => {
                // No generics defined on the source type:
                Vec::new()
            }
            syn::PathArguments::AngleBracketed(args) => {
                // We have generics like <A,B> defined on the source type (error for any non-ident type):
                args.args
                    .iter()
                    .map(|arg| match get_valid_from_substitution_type(arg) {
                        Some(ident) => Ok(ident),
                        None => Err(TypeSubstitutionError::InvalidFromType(arg.span())),
                    })
                    .collect::<Result<Vec<_>, _>>()?
            }
            syn::PathArguments::Parenthesized(args) => {
                // Generics like (A,B) -> defined; not allowed:
                return Err(TypeSubstitutionError::ExpectedAngleBracketGenerics(
                    args.span(),
                ));
            }
        };

        // Get hold of the generic args for the "to" type, erroring if they aren't valid.
        let target_args = match target_path_args {
            syn::PathArguments::None => {
                // No generics on target.
                Vec::new()
            }
            syn::PathArguments::AngleBracketed(args) => {
                // We have generics like <A,B> defined on the target type.
                args.args
                    .iter()
                    .map(|arg| match get_valid_to_substitution_type(arg) {
                        Some(arg) => Ok(arg),
                        None => Err(TypeSubstitutionError::InvalidToType(arg.span())),
                    })
                    .collect::<Result<Vec<_>, _>>()?
            }
            syn::PathArguments::Parenthesized(args) => {
                // Generics like (A,B) -> defined; not allowed:
                return Err(TypeSubstitutionError::ExpectedAngleBracketGenerics(
                    args.span(),
                ));
            }
        };

        // If no generics defined on source or target, we just apply any concrete generics
        // to the substitute type.
        if source_args.is_empty() && target_args.is_empty() {
            return Ok(TypeParamMapping::PassThrough);
        }

        // For each target param, we either point to the index of an input type we'll use, or
        // we specify an exact type to always swap use for that param.
        let mapping = target_args
            .into_iter()
            .map(|type_path| {
                if let Some(ident) = get_ident_from_type_path(type_path) {
                    // Does this ident map to a source type; if so, return said mapping:
                    if let Some(idx) = source_args.iter().position(|&src| ident == src) {
                        return Ok(TypeParamReplacement::InputAtIndex(idx));
                    };
                }
                // Not an ident that maps to the "from", so just use the concrete type:
                Ok(TypeParamReplacement::ConcreteType(type_path.clone()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TypeParamMapping::Specified(mapping))
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
                        .filter_map(|replacement| match replacement {
                            TypeParamReplacement::ConcreteType(ty) => {
                                let ty = TypePath::Type(TypePathType::Path {
                                    path: ty.clone(),
                                    params: Vec::new(),
                                });
                                Some(ty)
                            }
                            TypeParamReplacement::InputAtIndex(idx) => params.get(*idx).cloned(),
                        })
                        .collect(),
                ),
                TypeParamMapping::PassThrough => Cow::Borrowed(params),
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
            path.segments()
                .iter()
                .map(|x| x.as_ref().to_owned())
                .collect(),
        )
    }
}

/// Given a "to" type in a type substitution, return the TypePath inside or None if
/// it's not a valid "to" type.
fn get_valid_to_substitution_type(arg: &syn::GenericArgument) -> Option<&syn::TypePath> {
    let syn::GenericArgument::Type(syn::Type::Path(type_path)) = arg else {
        // We are looking for a type, not a lifetime or anything else
        return None
    };
    Some(type_path)
}

/// Given a "from" type in a type substitution, return the Ident inside or None if
/// it's not a valid "from" type.
fn get_valid_from_substitution_type(arg: &syn::GenericArgument) -> Option<&syn::Ident> {
    let syn::GenericArgument::Type(syn::Type::Path(type_path)) = arg else {
        // We are looking for a type, not a lifetime or anything else
        return None
    };
    get_ident_from_type_path(type_path)
}

/// Given a type path, return the single ident representing it if that's all it is.
fn get_ident_from_type_path(type_path: &syn::TypePath) -> Option<&syn::Ident> {
    if type_path.qself.is_some() {
        // No "<Foo as Bar>" type thing
        return None;
    }
    if type_path.path.leading_colon.is_some() {
        // No leading "::"
        return None;
    }
    if type_path.path.segments.len() > 1 {
        // The path should just be a single ident, not multiple
        return None;
    }
    let Some(segment) = type_path.path.segments.last() else {
        // Get the single ident (should be infallible)
        return None
    };
    if !segment.arguments.is_empty() {
        // The ident shouldn't have any of it's own generic args like A<B, C>
        return None;
    }
    Some(&segment.ident)
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
