use super::Either;
use scale_info_legacy::LookupName;
use scale_type_resolver::ResolvedTypeVisitor;

/// A type resolver which could either be for modern or historic resolving.
pub type AnyResolver<'a, 'b> =
    Either<&'a scale_info::PortableRegistry, &'a scale_info_legacy::TypeRegistrySet<'b>>;

/// A type ID which is either a modern or historic ID.
pub type AnyTypeId = Either<u32, scale_info_legacy::LookupName>;

impl Default for AnyTypeId {
    fn default() -> Self {
        // Not a sensible default, but we don't need / can't provide a sensible one.
        AnyTypeId::A(u32::MAX)
    }
}
impl From<u32> for AnyTypeId {
    fn from(value: u32) -> Self {
        AnyTypeId::A(value)
    }
}
impl From<LookupName> for AnyTypeId {
    fn from(value: LookupName) -> Self {
        AnyTypeId::B(value)
    }
}
impl TryFrom<AnyTypeId> for u32 {
    type Error = ();
    fn try_from(value: AnyTypeId) -> Result<Self, Self::Error> {
        match value {
            AnyTypeId::A(v) => Ok(v),
            AnyTypeId::B(_) => Err(()),
        }
    }
}
impl TryFrom<AnyTypeId> for LookupName {
    type Error = ();
    fn try_from(value: AnyTypeId) -> Result<Self, Self::Error> {
        match value {
            AnyTypeId::A(_) => Err(()),
            AnyTypeId::B(v) => Ok(v),
        }
    }
}

/// A resolve error that comes from using [`AnyResolver`] to resolve some [`AnyTypeId`] into a type.
#[derive(Debug, thiserror::Error)]
pub enum AnyResolverError {
    #[error("got a {got} type ID but expected a {expected} type ID")]
    TypeIdMismatch {
        got: &'static str,
        expected: &'static str,
    },
    #[error("{0}")]
    ScaleInfo(scale_type_resolver::portable_registry::Error),
    #[error("{0}")]
    ScaleInfoLegacy(scale_info_legacy::type_registry::TypeRegistryResolveError),
}

impl<'a, 'b> scale_type_resolver::TypeResolver for AnyResolver<'a, 'b> {
    type TypeId = AnyTypeId;
    type Error = AnyResolverError;

    fn resolve_type<'this, V: ResolvedTypeVisitor<'this, TypeId = Self::TypeId>>(
        &'this self,
        type_id: Self::TypeId,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match (self, type_id) {
            (Either::A(resolver), Either::A(id)) => resolver
                .resolve_type(id, ModernVisitor(visitor))
                .map_err(AnyResolverError::ScaleInfo),
            (Either::B(resolver), Either::B(id)) => resolver
                .resolve_type(id, LegacyVisitor(visitor))
                .map_err(AnyResolverError::ScaleInfoLegacy),
            (Either::A(_), Either::B(_)) => Err(AnyResolverError::TypeIdMismatch {
                got: "LookupName",
                expected: "u32",
            }),
            (Either::B(_), Either::A(_)) => Err(AnyResolverError::TypeIdMismatch {
                got: "u32",
                expected: "LookupName",
            }),
        }
    }
}

// We need to have a visitor which understands only modern or legacy types, and can wrap the more generic visitor
// that must be provided to AnyResolver::resolve_type. This then allows us to visit historic _or_ modern types
// using the single visitor provided by the user.
struct LegacyVisitor<V>(V);
struct ModernVisitor<V>(V);

mod impls {
    use super::{AnyTypeId, LegacyVisitor, LookupName, ModernVisitor};
    use scale_type_resolver::*;

    // An ugly implementation which maps from modern or legacy types into our AnyTypeId,
    // to make LegacyVisitor and ModernVisitor valid visitors when wrapping a generic "any" visitor.
    macro_rules! impl_visitor_mapper {
        ($struc:ident, $type_id_ty:ident, $variant:ident) => {
            impl<'this, V> ResolvedTypeVisitor<'this> for $struc<V>
            where
                V: ResolvedTypeVisitor<'this, TypeId = AnyTypeId>,
            {
                type TypeId = $type_id_ty;
                type Value = V::Value;

                fn visit_unhandled(self, kind: UnhandledKind) -> Self::Value {
                    self.0.visit_unhandled(kind)
                }
                fn visit_array(self, type_id: Self::TypeId, len: usize) -> Self::Value {
                    self.0.visit_array(AnyTypeId::$variant(type_id), len)
                }
                fn visit_not_found(self) -> Self::Value {
                    self.0.visit_not_found()
                }
                fn visit_composite<Path, Fields>(self, path: Path, fields: Fields) -> Self::Value
                where
                    Path: PathIter<'this>,
                    Fields: FieldIter<'this, Self::TypeId>,
                {
                    self.0.visit_composite(
                        path,
                        fields.map(|field| Field {
                            name: field.name,
                            id: AnyTypeId::$variant(field.id),
                        }),
                    )
                }
                fn visit_variant<Path, Fields, Var>(self, path: Path, variants: Var) -> Self::Value
                where
                    Path: PathIter<'this>,
                    Fields: FieldIter<'this, Self::TypeId>,
                    Var: VariantIter<'this, Fields>,
                {
                    self.0.visit_variant(
                        path,
                        variants.map(|variant| Variant {
                            index: variant.index,
                            name: variant.name,
                            fields: variant.fields.map(|field| Field {
                                name: field.name,
                                id: AnyTypeId::$variant(field.id),
                            }),
                        }),
                    )
                }
                fn visit_sequence<Path>(self, path: Path, type_id: Self::TypeId) -> Self::Value
                where
                    Path: PathIter<'this>,
                {
                    self.0.visit_sequence(path, AnyTypeId::$variant(type_id))
                }

                fn visit_tuple<TypeIds>(self, type_ids: TypeIds) -> Self::Value
                where
                    TypeIds: ExactSizeIterator<Item = Self::TypeId>,
                {
                    self.0
                        .visit_tuple(type_ids.map(|id| AnyTypeId::$variant(id)))
                }

                fn visit_primitive(self, primitive: Primitive) -> Self::Value {
                    self.0.visit_primitive(primitive)
                }

                fn visit_compact(self, type_id: Self::TypeId) -> Self::Value {
                    self.0.visit_compact(AnyTypeId::$variant(type_id))
                }

                fn visit_bit_sequence(
                    self,
                    store_format: BitsStoreFormat,
                    order_format: BitsOrderFormat,
                ) -> Self::Value {
                    self.0.visit_bit_sequence(store_format, order_format)
                }
            }
        };
    }

    impl_visitor_mapper!(ModernVisitor, u32, A);
    impl_visitor_mapper!(LegacyVisitor, LookupName, B);
}
