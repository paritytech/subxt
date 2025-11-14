use super::*;
use codec::Decode;
use frame_metadata::RuntimeMetadata;
use scale_info_legacy::LookupName;
use core::str::FromStr;
use scale_type_resolver::TypeResolver;
use frame_decode::constants::ConstantTypeInfo;
use frame_decode::runtime_apis::RuntimeApiEntryInfo;

/// Load some legacy kusama metadata from our artifacts.
fn legacy_kusama_metadata(version: u8) -> (u64, RuntimeMetadata) {
    const VERSIONS: [(u8, u64, &'static str); 5] = [
        (9, 1021, "metadata_v9_1021.scale"),
        (10, 1038, "metadata_v10_1038.scale"),
        (11, 1045, "metadata_v11_1045.scale"),
        (12, 2025, "metadata_v12_2025.scale"),
        (13, 9030, "metadata_v13_9030.scale"),
    ];

    let (spec_version, filename) = VERSIONS
        .iter()
        .find(|(v, _spec_version, _filename)| *v == version)
        .map(|(_, spec_version, name)| (*spec_version, *name))
        .unwrap_or_else(|| panic!("v{version} metadata artifact does not exist"));

    let mut path = std::path::PathBuf::from_str("../artifacts/kusama/").unwrap();
    path.push(filename);
    
    let bytes = std::fs::read(path).expect("Could not read file");
    let metadata = RuntimeMetadata::decode(&mut &*bytes).expect("Could not SCALE decode metadata");

    (spec_version, metadata)
}

/// Load our kusama types.
/// TODO: This is WRONG at the moment; change to point to kusama types when they exist:
fn kusama_types() -> scale_info_legacy::ChainTypeRegistry {
    frame_decode::legacy_types::polkadot::relay_chain()
}

/// Return a pair of original metadata + converted subxt_metadata::Metadata
fn metadata_pair(version: u8) -> (TypeRegistrySet<'static>, RuntimeMetadata, crate::Metadata) {
    let (spec_version, metadata) = legacy_kusama_metadata(version);
    let types = kusama_types();

    // Extend the types with builtins.
    let types_for_spec = {
        let mut types_for_spec = types.for_spec_version(spec_version).to_owned();
        let extended_types = frame_decode::helpers::type_registry_from_metadata_any(&metadata).unwrap();
        types_for_spec.prepend(extended_types);
        types_for_spec
    };

    let subxt_metadata = match &metadata {
        RuntimeMetadata::V9(m) => crate::Metadata::from_v9(m, &types_for_spec),
        RuntimeMetadata::V10(m) => crate::Metadata::from_v10(m, &types_for_spec),
        RuntimeMetadata::V11(m) => crate::Metadata::from_v11(m, &types_for_spec),
        RuntimeMetadata::V12(m) => crate::Metadata::from_v12(m, &types_for_spec),
        RuntimeMetadata::V13(m) => crate::Metadata::from_v13(m, &types_for_spec),
        _ => panic!("Metadata version {} not expected", metadata.version())
    }.expect("Could not convert to subxt_metadata::Metadata");

    (types_for_spec, metadata, subxt_metadata)
}

/// A representation of the shape of some type that we can compare across metadatas.
#[derive(PartialEq, Debug, Clone)]
enum Shape {
    Array(Box<Shape>, usize),
    BitSequence(scale_type_resolver::BitsStoreFormat, scale_type_resolver::BitsOrderFormat),
    Compact(Box<Shape>),
    Composite(Vec<String>, Vec<(Option<String>, Shape)>),
    Primitive(scale_type_resolver::Primitive),
    Sequence(Vec<String>, Box<Shape>),
    Tuple(Vec<Shape>),
    Variant(Vec<String>, Vec<Variant>),
    // To avoid recursion we return this if we spot it:
    Recursive,
}

#[derive(PartialEq, Debug, Clone)]
struct Variant {
    index: u8,
    name: String,
    fields: Vec<(Option<String>, Shape)>
}


/// A dumb immutable stack to help prevent type recursion in our tests below.
mod stack {
    pub struct Stack<T> {
        items: Vec<T>
    }
    
    impl <T: Clone + PartialEq> Stack<T> {
        /// Create a new stack with a single item.
        pub fn new(item: T) -> Self {
            Stack { items: vec![item] }
        }
        /// Fetch the current top item of the stack.
        pub fn current(&self) -> &T {
            self.items.last().unwrap()
        }
        /// Push an item to the stack, returning a new stack.
        pub fn push(&self, t: T) -> Self {
            let mut items = self.items.clone();
            items.push(t);
            Stack { items }
        }
        /// Return true if the item at the top of the stack is equal to any of the others.
        pub fn has_recursed(&self) -> bool {
            if self.items.len() <= 1 {
                return false
            }

            let last_item = self.items.last().unwrap();
            (0..self.items.len() - 1).any(|idx| &self.items[idx] == last_item)
        }
    }
}

impl Shape {
    /// convert some modern type definition into a [`Shape`].
    fn from_modern_type(id: u32, types: &scale_info::PortableRegistry) -> Shape {
        Shape::from_modern_type_inner(stack::Stack::new(id), types)
    }

    fn from_modern_type_inner(ids: stack::Stack<u32>, types: &scale_info::PortableRegistry) -> Shape {
        if ids.has_recursed() {
            return Shape::Recursive
        }

        let id = *ids.current();
        let visitor = scale_type_resolver::visitor::new((ids, types), |_, _| panic!("Unhandled"))
            .visit_array(|(ids, types), type_id, len| {
                let inner = Shape::from_modern_type_inner(ids.push(type_id), types);
                Shape::Array(Box::new(inner), len)
            })
            .visit_bit_sequence(|_, store, order| {
                Shape::BitSequence(store, order)
            })
            .visit_compact(|(ids, types), type_id| {
                let inner = Shape::from_modern_type_inner(ids.push(type_id), types);
                Shape::Compact(Box::new(inner))
            })
            .visit_composite(|(ids, types), path, fields| {
                let path = path.map(|p| p.to_owned()).collect();
                let inners = fields.map(|field| {
                    let name = field.name.map(|n| n.to_owned());
                    let inner = Shape::from_modern_type_inner(ids.push(field.id), types);
                    (name, inner)
                }).collect();
                Shape::Composite(path, inners)
            })
            .visit_primitive(|_types, prim| {
                Shape::Primitive(prim)
            })
            .visit_sequence(|(ids, types), path, type_id| {
                let path = path.map(|p| p.to_owned()).collect();
                let inner = Shape::from_modern_type_inner(ids.push(type_id), types);
                Shape::Sequence(path, Box::new(inner))
            })
            .visit_tuple(|(ids, types), fields| {
                let inners = fields.map(|field| {
                     Shape::from_modern_type_inner(ids.push(field), types)
                }).collect();
                Shape::Tuple(inners)
            })
            .visit_variant(|(ids, types), path, variants| {
                let path = path.map(|p| p.to_owned()).collect();
                let variants = variants.map(|v| {
                    Variant {
                        index: v.index,
                        name: v.name.to_owned(),
                        fields: v.fields.map(|field| {
                            let name = field.name.map(|n| n.to_owned());
                            let inner = Shape::from_modern_type_inner(ids.push(field.id), types);
                            (name, inner)
                        }).collect()
                    }
                }).collect();
                Shape::Variant(path, variants)
            })
            .visit_not_found(|_types| {
                panic!("PortableRegistry should not have a type which can't be found")
            });
    
        types.resolve_type(id, visitor).unwrap()
    }
    
    /// convert some historic type definition into a [`Shape`].
    fn from_legacy_type(name: &scale_info_legacy::LookupName, types: &TypeRegistrySet<'_>) -> Shape {
        Shape::from_legacy_type_inner(stack::Stack::new(name.clone()), types)
    }

    fn from_legacy_type_inner(ids: stack::Stack<scale_info_legacy::LookupName>, types: &TypeRegistrySet<'_>) -> Shape {
        if ids.has_recursed() {
            return Shape::Recursive
        }

        let id = ids.current().clone();
        let visitor = scale_type_resolver::visitor::new(types, |_, _| panic!("Unhandled"))
            .visit_array(|types, type_id, len| {
                let inner = Shape::from_legacy_type_inner(ids.push(type_id), types);
                Shape::Array(Box::new(inner), len)
            })
            .visit_bit_sequence(|_types, store, order| {
                Shape::BitSequence(store, order)
            })
            .visit_compact(|types, type_id| {
                let inner = Shape::from_legacy_type_inner(ids.push(type_id), types);
                Shape::Compact(Box::new(inner))
            })
            .visit_composite(|types, path, fields| {
                let path = path.map(|p| p.to_owned()).collect();
                let inners = fields.map(|field| {
                    let name = field.name.map(|n| n.to_owned());
                    let inner = Shape::from_legacy_type_inner(ids.push(field.id), types);
                    (name, inner)
                }).collect();
                Shape::Composite(path, inners)
            })
            .visit_primitive(|_types, prim| {
                Shape::Primitive(prim)
            })
            .visit_sequence(|types, path, type_id| {
                let path = path.map(|p| p.to_owned()).collect();
                let inner = Shape::from_legacy_type_inner(ids.push(type_id), types);
                Shape::Sequence(path, Box::new(inner))
            })
            .visit_tuple(|types, fields| {
                let inners = fields.map(|field| {
                     Shape::from_legacy_type_inner(ids.push(field), types)
                }).collect();
                Shape::Tuple(inners)
            })
            .visit_variant(|types, path, variants| {
                let path = path.map(|p| p.to_owned()).collect();
                let variants = variants.map(|v| {
                    Variant {
                        index: v.index,
                        name: v.name.to_owned(),
                        fields: v.fields.map(|field| {
                            let name = field.name.map(|n| n.to_owned());
                            let inner = Shape::from_legacy_type_inner(ids.push(field.id), types);
                            (name, inner)
                        }).collect()
                    }
                }).collect();
                Shape::Variant(path, variants)
            })
            .visit_not_found(|types| {
                // When we convert legacy to modern types, any types we don't find
                // are replaced with empty variants (since we can't have dangling types
                // in our new PortableRegistry). Do the same here so they compare equal.
                Shape::from_legacy_type_inner(ids.push(LookupName::parse("special::Unknown").unwrap()), types)
            });
    
        types.resolve_type(id, visitor).unwrap()
    }
}

// Go over all of the constants listed via frame-decode and check that our old
// and new metadatas both have identical output.
macro_rules! constants_eq {
    ($name:ident, $version:literal, $version_path:ident) => {
        #[test]
        fn $name() {
            let (old_types, old_md, new_md) = metadata_pair($version);
            let RuntimeMetadata::$version_path(old_md) = old_md else { panic!("Wrong version") };

            let old: Vec<_> = old_md.constant_tuples()
                .map(|(p,n)| {
                    old_md.constant_info(&p, &n).unwrap()
                })
                .map(|c| {
                    (c.bytes.to_owned(), Shape::from_legacy_type(&c.type_id, &old_types))
                })
                .collect();
            let new: Vec<_> = new_md.constant_tuples()
                .map(|(p,n)| {
                    new_md.constant_info(&p, &n).unwrap()
                })
                .map(|c| {
                    (c.bytes.to_owned(), Shape::from_modern_type(c.type_id, new_md.types()))
                })
                .collect();

            assert_eq!(old, new);
        }
    }
}

constants_eq!(v9_constants_eq, 9, V9);
constants_eq!(v10_constants_eq, 10, V10);
constants_eq!(v11_constants_eq, 11, V11);
constants_eq!(v12_constants_eq, 12, V12);
constants_eq!(v13_constants_eq, 13, V13);

/// Make sure all Runtime APIs are the same once translated.
#[test]
fn runtime_apis() {
    for version in 9..=13 {
        let (old_types, _old_md, new_md) = metadata_pair(version);
    
        let old: Vec<_> = old_types.runtime_api_tuples()
            .map(|(p,n)| {
                old_types.runtime_api_info(&p, &n).unwrap().map_ids(|id| {
                    Ok::<_,()>(Shape::from_legacy_type(&id, &old_types))
                }).unwrap()
            })
            .collect();
        let new: Vec<_> = new_md.runtime_api_tuples()
            .map(|(p,n)| {
                new_md.runtime_api_info(&p, &n).unwrap().map_ids(|id| {
                    Ok::<_,()>(Shape::from_modern_type(id, new_md.types()))
                }).unwrap()
            })
            .collect();
    
        assert_eq!(old, new);
    }
}

macro_rules! storage_eq {
    ($name:ident, $version:literal, $version_path:ident) => {
        #[test]
        fn $name() {
            let (old_types, old_md, new_md) = metadata_pair($version);
            let RuntimeMetadata::$version_path(old_md) = old_md else { panic!("Wrong version") };

            let old: Vec<_> = old_md.storage_tuples()
                .map(|(p,n)| {
                    let info = old_md.storage_info(&p, &n).unwrap().map_ids(|id| {
                        Ok::<_,()>(Shape::from_legacy_type(&id, &old_types))
                    }).unwrap();
                    (p.into_owned(), n.into_owned(), info)
                })
                .collect();
            let new: Vec<_> = new_md.storage_tuples()
                .map(|(p,n)| {
                    let info = new_md.storage_info(&p, &n).unwrap().map_ids(|id| {
                        Ok::<_,()>(Shape::from_modern_type(id, new_md.types()))
                    }).unwrap();
                    (p.into_owned(), n.into_owned(), info)
                })
                .collect();

            if old.len() != new.len() {
                panic!("Storage entries for version 9 metadata differ in length");
            }

            for (old, new) in old.into_iter().zip(new.into_iter()) {
                assert_eq!((&old.0, &old.1), (&new.0, &new.1), "Storage entry mismatch");
                assert_eq!(old.2, new.2, "Storage entry {}.{} does not match!", old.0, old.1);
            }
        }
    }
}

storage_eq!(v9_storage_eq, 9, V9);
storage_eq!(v10_storage_eq, 10, V10);
storage_eq!(v11_storage_eq, 11, V11);
storage_eq!(v12_storage_eq, 12, V12);
storage_eq!(v13_storage_eq, 13, V13);

#[test]
fn builtin_call() {
    for version in 9..=13 {
        let (old_types, _old_md, new_md) = metadata_pair(version);
    
        let old = Shape::from_legacy_type(&LookupName::parse("builtin::Call").unwrap(), &old_types);
        let new = Shape::from_modern_type(new_md.outer_enums.call_enum_ty, new_md.types());
        assert_eq!(old, new, "Call types do not match in metadata V{version}!");
    }
}

#[test]
fn builtin_error() {
    for version in 9..=13 {
        let (old_types, _old_md, new_md) = metadata_pair(version);
    
        let old = Shape::from_legacy_type(&LookupName::parse("builtin::Error").unwrap(), &old_types);
        let new = Shape::from_modern_type(new_md.outer_enums.error_enum_ty, new_md.types());
        assert_eq!(old, new, "Error types do not match in metadata V{version}!");
    }
}

#[test]
fn builtin_event() {
    for version in 9..=13 {
        let (old_types, _old_md, new_md) = metadata_pair(version);
        
        let old = Shape::from_legacy_type(&LookupName::parse("builtin::Event").unwrap(), &old_types);
        let new = Shape::from_modern_type(new_md.outer_enums.event_enum_ty, new_md.types());
        assert_eq!(old, new, "Event types do not match in metadata V{version}!");
    }
}