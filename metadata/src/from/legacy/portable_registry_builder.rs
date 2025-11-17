use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::string::ToString;
use alloc::vec::Vec;
use scale_info::PortableRegistry;
use scale_info::{PortableType, form::PortableForm};
use scale_info_legacy::type_registry::TypeRegistryResolveError;
use scale_info_legacy::{LookupName, TypeRegistrySet};
use scale_type_resolver::{
    BitsOrderFormat, BitsStoreFormat, FieldIter, PathIter, Primitive, ResolvedTypeVisitor,
    UnhandledKind, VariantIter,
};

#[derive(thiserror::Error, Debug)]
pub enum PortableRegistryAddTypeError {
    #[error("Error resolving type: {0}")]
    ResolveError(#[from] TypeRegistryResolveError),
    #[error("Cannot find type '{0}'")]
    TypeNotFound(LookupName),
}

/// the purpose of this is to convert a (subset of) [`scale_info_legacy::TypeRegistrySet`]
/// into a [`scale_info::PortableRegistry`]. Type IDs from the former are passed in, and
/// type IDs from the latter are handed back. Calling [`PortableRegistryBuilder::finish()`]
/// then hands back a [`scale_info::PortableRegistry`] which these Ids can be used with.
pub struct PortableRegistryBuilder<'info> {
    legacy_types: &'info TypeRegistrySet<'info>,
    scale_info_types: PortableRegistry,
    old_to_new: BTreeMap<LookupName, u32>,
    ignore_not_found: bool,
}

impl<'info> PortableRegistryBuilder<'info> {
    /// Instantiate a new [`PortableRegistryBuilder`], providing the set of
    /// legacy types you wish to use to construct modern types from.
    pub fn new(legacy_types: &'info TypeRegistrySet<'info>) -> Self {
        PortableRegistryBuilder {
            legacy_types,
            scale_info_types: PortableRegistry {
                types: Default::default(),
            },
            old_to_new: Default::default(),
            ignore_not_found: false,
        }
    }

    /// If this is enabled, any type that isn't found will be replaced by a "special::Unknown" type
    /// instead of a "type not found" error being emitted.
    pub fn ignore_not_found(&mut self, ignore: bool) {
        self.ignore_not_found = ignore;
    }

    /// Try adding a type, given its string name and optionally the pallet it's scoped to.
    pub fn try_add_type_str(
        &mut self,
        id: &str,
        pallet: Option<&str>,
    ) -> Option<Result<u32, TypeRegistryResolveError>> {
        let mut id = match LookupName::parse(id) {
            Ok(id) => id,
            Err(e) => {
                return Some(Err(TypeRegistryResolveError::LookupNameInvalid(
                    id.to_owned(),
                    e,
                )));
            }
        };

        if let Some(pallet) = pallet {
            id = id.in_pallet(pallet);
        }

        self.try_add_type(id)
    }

    /// Try adding a type, returning `None` if the type doesn't exist.
    pub fn try_add_type(
        &mut self,
        id: LookupName,
    ) -> Option<Result<u32, TypeRegistryResolveError>> {
        match self.add_type(id) {
            Ok(id) => Some(Ok(id)),
            Err(PortableRegistryAddTypeError::TypeNotFound(_)) => None,
            Err(PortableRegistryAddTypeError::ResolveError(e)) => Some(Err(e)),
        }
    }

    /// Add a new legacy type, giving its string ID/name and, if applicable, the pallet that it's seen in,
    /// returning the corresponding "modern" type ID to use in its place, or an error if something does wrong.
    pub fn add_type_str(
        &mut self,
        id: &str,
        pallet: Option<&str>,
    ) -> Result<u32, PortableRegistryAddTypeError> {
        let mut id = LookupName::parse(id)
            .map_err(|e| TypeRegistryResolveError::LookupNameInvalid(id.to_owned(), e))?;

        if let Some(pallet) = pallet {
            id = id.in_pallet(pallet);
        }

        self.add_type(id)
    }

    /// Add a new legacy type, returning the corresponding "modern" type ID to use in
    /// its place, or an error if something does wrong.
    pub fn add_type(&mut self, id: LookupName) -> Result<u32, PortableRegistryAddTypeError> {
        if let Some(new_id) = self.old_to_new.get(&id) {
            return Ok(*new_id);
        }

        // Assign a new ID immediately to prevent any recursion. If we don't do this, then
        // recursive types (ie types that contain themselves) will lead to a stack overflow.
        // with this, we assign IDs up front, so the ID is returned immediately on recursing.
        let new_id = self.scale_info_types.types.len() as u32;

        // Add a placeholder type to "reserve" this ID.
        self.scale_info_types.types.push(PortableType {
            id: new_id,
            ty: scale_info::Type::new(
                scale_info::Path { segments: vec![] },
                core::iter::empty(),
                scale_info::TypeDef::Variant(scale_info::TypeDefVariant { variants: vec![] }),
                Default::default(),
            ),
        });

        // Cache the ID so that recursing calls bail early.
        self.old_to_new.insert(id.clone(), new_id);

        let visitor = PortableRegistryVisitor {
            builder: &mut *self,
            current_type: &id,
        };

        match visitor
            .builder
            .legacy_types
            .resolve_type(id.clone(), visitor)
        {
            Ok(Ok(ty)) => {
                self.scale_info_types.types[new_id as usize].ty = ty;
                Ok(new_id)
            }
            Ok(Err(e)) => {
                self.old_to_new.remove(&id);
                Err(e)
            }
            Err(e) => {
                self.old_to_new.remove(&id);
                Err(e.into())
            }
        }
    }

    /// Return the current [`scale_info::PortableRegistry`].
    pub fn types(&self) -> &PortableRegistry {
        &self.scale_info_types
    }

    /// Finish adding types and return the modern type registry.
    pub fn finish(self) -> PortableRegistry {
        self.scale_info_types
    }
}

struct PortableRegistryVisitor<'a, 'info> {
    builder: &'a mut PortableRegistryBuilder<'info>,
    current_type: &'a LookupName,
}

impl<'a, 'info> ResolvedTypeVisitor<'info> for PortableRegistryVisitor<'a, 'info> {
    type TypeId = LookupName;
    type Value = Result<scale_info::Type<PortableForm>, PortableRegistryAddTypeError>;

    fn visit_unhandled(self, kind: UnhandledKind) -> Self::Value {
        panic!("A handler exists for every type, but visit_unhandled({kind:?}) was called");
    }

    fn visit_not_found(self) -> Self::Value {
        if self.builder.ignore_not_found {
            // Return the "unknown" type if we're ignoring not found types:
            Ok(unknown_type())
        } else {
            // Otherwise just return an error at this point:
            Err(PortableRegistryAddTypeError::TypeNotFound(
                self.current_type.clone(),
            ))
        }
    }

    fn visit_primitive(self, primitive: Primitive) -> Self::Value {
        let p = match primitive {
            Primitive::Bool => scale_info::TypeDefPrimitive::Bool,
            Primitive::Char => scale_info::TypeDefPrimitive::Char,
            Primitive::Str => scale_info::TypeDefPrimitive::Str,
            Primitive::U8 => scale_info::TypeDefPrimitive::U8,
            Primitive::U16 => scale_info::TypeDefPrimitive::U16,
            Primitive::U32 => scale_info::TypeDefPrimitive::U32,
            Primitive::U64 => scale_info::TypeDefPrimitive::U64,
            Primitive::U128 => scale_info::TypeDefPrimitive::U128,
            Primitive::U256 => scale_info::TypeDefPrimitive::U256,
            Primitive::I8 => scale_info::TypeDefPrimitive::I8,
            Primitive::I16 => scale_info::TypeDefPrimitive::I16,
            Primitive::I32 => scale_info::TypeDefPrimitive::I32,
            Primitive::I64 => scale_info::TypeDefPrimitive::I64,
            Primitive::I128 => scale_info::TypeDefPrimitive::I128,
            Primitive::I256 => scale_info::TypeDefPrimitive::I256,
        };

        Ok(scale_info::Type::new(
            Default::default(),
            core::iter::empty(),
            scale_info::TypeDef::Primitive(p),
            Default::default(),
        ))
    }

    fn visit_sequence<Path: PathIter<'info>>(
        self,
        path: Path,
        inner_type_id: Self::TypeId,
    ) -> Self::Value {
        let inner_id = self.builder.add_type(inner_type_id)?;
        let path = scale_info::Path {
            segments: path.map(Into::into).collect(),
        };

        Ok(scale_info::Type::new(
            path,
            core::iter::empty(),
            scale_info::TypeDef::Sequence(scale_info::TypeDefSequence {
                type_param: inner_id.into(),
            }),
            Default::default(),
        ))
    }

    fn visit_composite<Path, Fields>(self, path: Path, fields: Fields) -> Self::Value
    where
        Path: PathIter<'info>,
        Fields: FieldIter<'info, Self::TypeId>,
    {
        let path = scale_info::Path {
            segments: path.map(Into::into).collect(),
        };

        let mut scale_info_fields = Vec::<scale_info::Field<_>>::new();
        for field in fields {
            let type_name = field.id.to_string();
            let id = self.builder.add_type(field.id)?;
            scale_info_fields.push(scale_info::Field {
                name: field.name.map(Into::into),
                ty: id.into(),
                type_name: Some(type_name),
                docs: Default::default(),
            });
        }

        Ok(scale_info::Type::new(
            path,
            core::iter::empty(),
            scale_info::TypeDef::Composite(scale_info::TypeDefComposite {
                fields: scale_info_fields,
            }),
            Default::default(),
        ))
    }

    fn visit_array(self, inner_type_id: LookupName, len: usize) -> Self::Value {
        let inner_id = self.builder.add_type(inner_type_id)?;

        Ok(scale_info::Type::new(
            Default::default(),
            core::iter::empty(),
            scale_info::TypeDef::Array(scale_info::TypeDefArray {
                len: len as u32,
                type_param: inner_id.into(),
            }),
            Default::default(),
        ))
    }

    fn visit_tuple<TypeIds>(self, type_ids: TypeIds) -> Self::Value
    where
        TypeIds: ExactSizeIterator<Item = Self::TypeId>,
    {
        let mut scale_info_fields = Vec::new();
        for old_id in type_ids {
            let new_id = self.builder.add_type(old_id)?;
            scale_info_fields.push(new_id.into());
        }

        Ok(scale_info::Type::new(
            Default::default(),
            core::iter::empty(),
            scale_info::TypeDef::Tuple(scale_info::TypeDefTuple {
                fields: scale_info_fields,
            }),
            Default::default(),
        ))
    }

    fn visit_variant<Path, Fields, Var>(self, path: Path, variants: Var) -> Self::Value
    where
        Path: PathIter<'info>,
        Fields: FieldIter<'info, Self::TypeId>,
        Var: VariantIter<'info, Fields>,
    {
        let path = scale_info::Path {
            segments: path.map(Into::into).collect(),
        };

        let mut scale_info_variants = Vec::new();
        for variant in variants {
            let mut scale_info_variant_fields = Vec::<scale_info::Field<_>>::new();
            for field in variant.fields {
                let type_name = field.id.to_string();
                let id = self.builder.add_type(field.id)?;
                scale_info_variant_fields.push(scale_info::Field {
                    name: field.name.map(Into::into),
                    ty: id.into(),
                    type_name: Some(type_name),
                    docs: Default::default(),
                });
            }

            scale_info_variants.push(scale_info::Variant {
                name: variant.name.to_owned(),
                index: variant.index,
                fields: scale_info_variant_fields,
                docs: Default::default(),
            })
        }

        Ok(scale_info::Type::new(
            path,
            core::iter::empty(),
            scale_info::TypeDef::Variant(scale_info::TypeDefVariant {
                variants: scale_info_variants,
            }),
            Default::default(),
        ))
    }

    fn visit_compact(self, inner_type_id: Self::TypeId) -> Self::Value {
        let inner_id = self.builder.add_type(inner_type_id)?;

        // Configure the path and type params to maximise compat.
        let path = ["parity_scale_codec", "Compact"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        let type_params = [scale_info::TypeParameter {
            name: "T".to_owned(),
            ty: Some(inner_id.into()),
        }];

        Ok(scale_info::Type::new(
            scale_info::Path { segments: path },
            type_params,
            scale_info::TypeDef::Compact(scale_info::TypeDefCompact {
                type_param: inner_id.into(),
            }),
            Default::default(),
        ))
    }

    fn visit_bit_sequence(
        self,
        store_format: BitsStoreFormat,
        order_format: BitsOrderFormat,
    ) -> Self::Value {
        // These order types are added by default into a `TypeRegistry`, so we
        // expect them to exist. Parsing should always succeed.
        let order_ty_str = match order_format {
            BitsOrderFormat::Lsb0 => "bitvec::order::Lsb0",
            BitsOrderFormat::Msb0 => "bitvec::order::Msb0",
        };
        let order_ty = LookupName::parse(order_ty_str).unwrap();
        let new_order_ty = self.builder.add_type(order_ty)?;

        // The store types also exist by default. Parsing should always succeed.
        let store_ty_str = match store_format {
            BitsStoreFormat::U8 => "u8",
            BitsStoreFormat::U16 => "u16",
            BitsStoreFormat::U32 => "u32",
            BitsStoreFormat::U64 => "u64",
        };
        let store_ty = LookupName::parse(store_ty_str).unwrap();
        let new_store_ty = self.builder.add_type(store_ty)?;

        // Configure the path and type params to look like BitVec's to try
        // and maximise compatibility.
        let path = ["bitvec", "vec", "BitVec"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        let type_params = [
            scale_info::TypeParameter {
                name: "Store".to_owned(),
                ty: Some(new_store_ty.into()),
            },
            scale_info::TypeParameter {
                name: "Order".to_owned(),
                ty: Some(new_order_ty.into()),
            },
        ];

        Ok(scale_info::Type::new(
            scale_info::Path { segments: path },
            type_params,
            scale_info::TypeDef::BitSequence(scale_info::TypeDefBitSequence {
                bit_order_type: new_order_ty.into(),
                bit_store_type: new_store_ty.into(),
            }),
            Default::default(),
        ))
    }
}

fn unknown_type() -> scale_info::Type<PortableForm> {
    scale_info::Type::new(
        scale_info::Path {
            segments: Vec::from_iter(["special".to_owned(), "Unknown".to_owned()]),
        },
        core::iter::empty(),
        scale_info::TypeDef::Variant(scale_info::TypeDefVariant {
            variants: Vec::new(),
        }),
        Default::default(),
    )
}
