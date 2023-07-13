// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utility functions for metadata validation.

use crate::{
    ExtrinsicMetadata, Metadata, PalletMetadata, RuntimeApiMetadata, RuntimeApiMethodMetadata,
    StorageEntryMetadata, StorageEntryType,
};
use scale_info::{form::PortableForm, Field, PortableRegistry, TypeDef, Variant};
use std::collections::HashSet;

/// Predefined value to be returned when we already visited a type.
const MAGIC_RECURSIVE_TYPE_VALUE: &[u8] = &[123];

// The number of bytes our `hash` function produces.
const HASH_LEN: usize = 32;

/// Internal byte representation for various metadata types utilized for
/// generating deterministic hashes between different rust versions.
#[repr(u8)]
enum TypeBeingHashed {
    Composite,
    Variant,
    Sequence,
    Array,
    Tuple,
    Primitive,
    Compact,
    BitSequence,
}

/// Hashing function utilized internally.
fn hash(data: &[u8]) -> [u8; HASH_LEN] {
    sp_core_hashing::twox_256(data)
}

/// XOR two hashes together. Only use this when you don't care about the order
/// of the things you're hashing together.
fn xor(a: [u8; HASH_LEN], b: [u8; HASH_LEN]) -> [u8; HASH_LEN] {
    let mut out = [0u8; HASH_LEN];
    for (idx, (a, b)) in a.into_iter().zip(b).enumerate() {
        out[idx] = a ^ b;
    }
    out
}

// Combine some number of HASH_LEN byte hashes and output a single HASH_LEN
// byte hash to uniquely represent the inputs.
macro_rules! count_idents {
    () => { 0 };
    ($n:ident $($rest:ident)*) => { 1 + count_idents!($($rest)*) }
}
macro_rules! concat_and_hash_n {
    ($name:ident($($arg:ident)+)) => {
        fn $name($($arg: &[u8; HASH_LEN]),+) -> [u8; HASH_LEN] {
            let mut out = [0u8; HASH_LEN * count_idents!($($arg)+)];
            let mut start = 0;
            $(
                out[start..start+HASH_LEN].copy_from_slice(&$arg[..]);
                #[allow(unused_assignments)]
                { start += HASH_LEN; }
            )+
            hash(&out)
        }
    }
}
concat_and_hash_n!(concat_and_hash2(a b));
concat_and_hash_n!(concat_and_hash3(a b c));
concat_and_hash_n!(concat_and_hash4(a b c d));
concat_and_hash_n!(concat_and_hash5(a b c d e));

/// Obtain the hash representation of a `scale_info::Field`.
fn get_field_hash(
    registry: &PortableRegistry,
    field: &Field<PortableForm>,
    visited_ids: &mut HashSet<u32>,
) -> [u8; HASH_LEN] {
    let field_name_bytes = match &field.name {
        Some(name) => hash(name.as_bytes()),
        None => [0u8; HASH_LEN],
    };

    concat_and_hash2(
        &field_name_bytes,
        &get_type_hash(registry, field.ty.id, visited_ids),
    )
}

/// Obtain the hash representation of a `scale_info::Variant`.
fn get_variant_hash(
    registry: &PortableRegistry,
    var: &Variant<PortableForm>,
    visited_ids: &mut HashSet<u32>,
) -> [u8; HASH_LEN] {
    let variant_name_bytes = hash(var.name.as_bytes());
    let variant_field_bytes = var.fields.iter().fold([0u8; HASH_LEN], |bytes, field| {
        // EncodeAsType and DecodeAsType don't care about variant field ordering,
        // so XOR the fields to ensure that it doesn't matter.
        xor(bytes, get_field_hash(registry, field, visited_ids))
    });

    concat_and_hash2(&variant_name_bytes, &variant_field_bytes)
}

/// Obtain the hash representation of a `scale_info::TypeDef`.
fn get_type_def_hash(
    registry: &PortableRegistry,
    ty_def: &TypeDef<PortableForm>,
    visited_ids: &mut HashSet<u32>,
) -> [u8; HASH_LEN] {
    match ty_def {
        TypeDef::Composite(composite) => {
            let composite_id_bytes = [TypeBeingHashed::Composite as u8; HASH_LEN];
            let composite_field_bytes =
                composite
                    .fields
                    .iter()
                    .fold([0u8; HASH_LEN], |bytes, field| {
                        // With EncodeAsType and DecodeAsType we no longer care which order the fields are in,
                        // as long as all of the names+types are there. XOR to not care about ordering.
                        xor(bytes, get_field_hash(registry, field, visited_ids))
                    });
            concat_and_hash2(&composite_id_bytes, &composite_field_bytes)
        }
        TypeDef::Variant(variant) => {
            let variant_id_bytes = [TypeBeingHashed::Variant as u8; HASH_LEN];
            let variant_field_bytes =
                variant.variants.iter().fold([0u8; HASH_LEN], |bytes, var| {
                    // With EncodeAsType and DecodeAsType we no longer care which order the variants are in,
                    // as long as all of the names+types are there. XOR to not care about ordering.
                    xor(bytes, get_variant_hash(registry, var, visited_ids))
                });
            concat_and_hash2(&variant_id_bytes, &variant_field_bytes)
        }
        TypeDef::Sequence(sequence) => concat_and_hash2(
            &[TypeBeingHashed::Sequence as u8; HASH_LEN],
            &get_type_hash(registry, sequence.type_param.id, visited_ids),
        ),
        TypeDef::Array(array) => {
            // Take length into account too; different length must lead to different hash.
            let array_id_bytes = {
                let mut a = [0u8; HASH_LEN];
                a[0] = TypeBeingHashed::Array as u8;
                a[1..5].copy_from_slice(&array.len.to_be_bytes());
                a
            };
            concat_and_hash2(
                &array_id_bytes,
                &get_type_hash(registry, array.type_param.id, visited_ids),
            )
        }
        TypeDef::Tuple(tuple) => {
            let mut bytes = hash(&[TypeBeingHashed::Tuple as u8]);
            for field in &tuple.fields {
                bytes = concat_and_hash2(&bytes, &get_type_hash(registry, field.id, visited_ids));
            }
            bytes
        }
        TypeDef::Primitive(primitive) => {
            // Cloning the 'primitive' type should essentially be a copy.
            hash(&[TypeBeingHashed::Primitive as u8, primitive.clone() as u8])
        }
        TypeDef::Compact(compact) => concat_and_hash2(
            &[TypeBeingHashed::Compact as u8; HASH_LEN],
            &get_type_hash(registry, compact.type_param.id, visited_ids),
        ),
        TypeDef::BitSequence(bitseq) => concat_and_hash3(
            &[TypeBeingHashed::BitSequence as u8; HASH_LEN],
            &get_type_hash(registry, bitseq.bit_order_type.id, visited_ids),
            &get_type_hash(registry, bitseq.bit_store_type.id, visited_ids),
        ),
    }
}

/// Obtain the hash representation of a `scale_info::Type` identified by id.
pub fn get_type_hash(
    registry: &PortableRegistry,
    id: u32,
    visited_ids: &mut HashSet<u32>,
) -> [u8; HASH_LEN] {
    // Guard against recursive types and return a fixed arbitrary hash
    if !visited_ids.insert(id) {
        return hash(MAGIC_RECURSIVE_TYPE_VALUE);
    }

    let ty = registry
        .resolve(id)
        .expect("Type ID provided by the metadata is registered; qed");
    get_type_def_hash(registry, &ty.type_def, visited_ids)
}

/// Obtain the hash representation of a `frame_metadata::v15::ExtrinsicMetadata`.
fn get_extrinsic_hash(
    registry: &PortableRegistry,
    extrinsic: &ExtrinsicMetadata,
) -> [u8; HASH_LEN] {
    let mut visited_ids = HashSet::<u32>::new();

    let mut bytes = concat_and_hash2(
        &get_type_hash(registry, extrinsic.ty, &mut visited_ids),
        &[extrinsic.version; 32],
    );

    for signed_extension in extrinsic.signed_extensions.iter() {
        bytes = concat_and_hash4(
            &bytes,
            &hash(signed_extension.identifier.as_bytes()),
            &get_type_hash(registry, signed_extension.extra_ty, &mut visited_ids),
            &get_type_hash(registry, signed_extension.additional_ty, &mut visited_ids),
        )
    }

    bytes
}

/// Get the hash corresponding to a single storage entry.
fn get_storage_entry_hash(
    registry: &PortableRegistry,
    entry: &StorageEntryMetadata,
    visited_ids: &mut HashSet<u32>,
) -> [u8; HASH_LEN] {
    let mut bytes = concat_and_hash3(
        &hash(entry.name.as_bytes()),
        // Cloning 'entry.modifier' should essentially be a copy.
        &[entry.modifier as u8; HASH_LEN],
        &hash(&entry.default),
    );

    match &entry.entry_type {
        StorageEntryType::Plain(ty) => {
            concat_and_hash2(&bytes, &get_type_hash(registry, *ty, visited_ids))
        }
        StorageEntryType::Map {
            hashers,
            key_ty,
            value_ty,
        } => {
            for hasher in hashers {
                // Cloning the hasher should essentially be a copy.
                bytes = concat_and_hash2(&bytes, &[*hasher as u8; HASH_LEN]);
            }
            concat_and_hash3(
                &bytes,
                &get_type_hash(registry, *key_ty, visited_ids),
                &get_type_hash(registry, *value_ty, visited_ids),
            )
        }
    }
}

/// Get the hash corresponding to a single runtime API method.
fn get_runtime_method_hash(
    registry: &PortableRegistry,
    trait_name: &str,
    method_metadata: &RuntimeApiMethodMetadata,
    visited_ids: &mut HashSet<u32>,
) -> [u8; HASH_LEN] {
    // The trait name is part of the runtime API call that is being
    // generated for this method. Therefore the trait name is strongly
    // connected to the method in the same way as a parameter is
    // to the method.
    let mut bytes = concat_and_hash2(
        &hash(trait_name.as_bytes()),
        &hash(method_metadata.name.as_bytes()),
    );

    for input in &method_metadata.inputs {
        bytes = concat_and_hash3(
            &bytes,
            &hash(input.name.as_bytes()),
            &get_type_hash(registry, input.ty, visited_ids),
        );
    }

    bytes = concat_and_hash2(
        &bytes,
        &get_type_hash(registry, method_metadata.output_ty, visited_ids),
    );

    bytes
}

/// Obtain the hash of all of a runtime API trait, including all of its methods.
pub fn get_runtime_trait_hash(trait_metadata: RuntimeApiMetadata) -> [u8; HASH_LEN] {
    let mut visited_ids = HashSet::new();
    let trait_name = &*trait_metadata.inner.name;
    let method_bytes = trait_metadata
        .methods()
        .fold([0u8; HASH_LEN], |bytes, method_metadata| {
            // We don't care what order the trait methods exist in, and want the hash to
            // be identical regardless. For this, we can just XOR the hashes for each method
            // together; we'll get the same output whichever order they are XOR'd together in,
            // so long as each individual method is the same.
            xor(
                bytes,
                get_runtime_method_hash(
                    trait_metadata.types,
                    trait_name,
                    method_metadata,
                    &mut visited_ids,
                ),
            )
        });

    concat_and_hash2(&hash(trait_name.as_bytes()), &method_bytes)
}

/// Obtain the hash for a specific storage item, or an error if it's not found.
pub fn get_storage_hash(pallet: &PalletMetadata, entry_name: &str) -> Option<[u8; HASH_LEN]> {
    let storage = pallet.storage()?;
    let entry = storage.entry_by_name(entry_name)?;

    let hash = get_storage_entry_hash(pallet.types, entry, &mut HashSet::new());
    Some(hash)
}

/// Obtain the hash for a specific constant, or an error if it's not found.
pub fn get_constant_hash(pallet: &PalletMetadata, constant_name: &str) -> Option<[u8; HASH_LEN]> {
    let constant = pallet.constant_by_name(constant_name)?;

    // We only need to check that the type of the constant asked for matches.
    let bytes = get_type_hash(pallet.types, constant.ty, &mut HashSet::new());
    Some(bytes)
}

/// Obtain the hash for a specific call, or an error if it's not found.
pub fn get_call_hash(pallet: &PalletMetadata, call_name: &str) -> Option<[u8; HASH_LEN]> {
    let call_variant = pallet.call_variant_by_name(call_name)?;

    // hash the specific variant representing the call we are interested in.
    let hash = get_variant_hash(pallet.types, call_variant, &mut HashSet::new());
    Some(hash)
}

/// Obtain the hash of a specific runtime API function, or an error if it's not found.
pub fn get_runtime_api_hash(
    runtime_apis: &RuntimeApiMetadata,
    method_name: &str,
) -> Option<[u8; HASH_LEN]> {
    let trait_name = &*runtime_apis.inner.name;
    let method_metadata = runtime_apis.method_by_name(method_name)?;

    Some(get_runtime_method_hash(
        runtime_apis.types,
        trait_name,
        method_metadata,
        &mut HashSet::new(),
    ))
}

/// Obtain the hash representation of a `frame_metadata::v15::PalletMetadata`.
pub fn get_pallet_hash(pallet: PalletMetadata) -> [u8; HASH_LEN] {
    let mut visited_ids = HashSet::<u32>::new();
    let registry = pallet.types;

    let call_bytes = match pallet.call_ty_id() {
        Some(calls) => get_type_hash(registry, calls, &mut visited_ids),
        None => [0u8; HASH_LEN],
    };
    let event_bytes = match pallet.event_ty_id() {
        Some(event) => get_type_hash(registry, event, &mut visited_ids),
        None => [0u8; HASH_LEN],
    };
    let error_bytes = match pallet.error_ty_id() {
        Some(error) => get_type_hash(registry, error, &mut visited_ids),
        None => [0u8; HASH_LEN],
    };
    let constant_bytes = pallet.constants().fold([0u8; HASH_LEN], |bytes, constant| {
        // We don't care what order the constants occur in, so XOR together the combinations
        // of (constantName, constantType) to make the order we see them irrelevant.
        let constant_hash = concat_and_hash2(
            &hash(constant.name.as_bytes()),
            &get_type_hash(registry, constant.ty(), &mut visited_ids),
        );
        xor(bytes, constant_hash)
    });
    let storage_bytes = match pallet.storage() {
        Some(storage) => {
            let prefix_hash = hash(storage.prefix().as_bytes());
            let entries_hash = storage
                .entries()
                .iter()
                .fold([0u8; HASH_LEN], |bytes, entry| {
                    // We don't care what order the storage entries occur in, so XOR them together
                    // to make the order irrelevant.
                    xor(
                        bytes,
                        get_storage_entry_hash(registry, entry, &mut visited_ids),
                    )
                });
            concat_and_hash2(&prefix_hash, &entries_hash)
        }
        None => [0u8; HASH_LEN],
    };

    // Hash all of the above together:
    concat_and_hash5(
        &call_bytes,
        &event_bytes,
        &error_bytes,
        &constant_bytes,
        &storage_bytes,
    )
}

/// Obtain a hash representation of our metadata or some part of it.
/// This is obtained by calling [`crate::Metadata::hasher()`].
pub struct MetadataHasher<'a> {
    metadata: &'a Metadata,
    specific_pallets: Option<Vec<&'a str>>,
    specific_runtime_apis: Option<Vec<&'a str>>,
}

impl<'a> MetadataHasher<'a> {
    /// Create a new [`MetadataHasher`]
    pub(crate) fn new(metadata: &'a Metadata) -> Self {
        Self {
            metadata,
            specific_pallets: None,
            specific_runtime_apis: None,
        }
    }

    /// Only hash the provided pallets instead of hashing every pallet.
    pub fn only_these_pallets<S: AsRef<str>>(&mut self, specific_pallets: &'a [S]) -> &mut Self {
        self.specific_pallets = Some(specific_pallets.iter().map(|n| n.as_ref()).collect());
        self
    }

    /// Only hash the provided runtime APIs instead of hashing every runtime API
    pub fn only_these_runtime_apis<S: AsRef<str>>(
        &mut self,
        specific_runtime_apis: &'a [S],
    ) -> &mut Self {
        self.specific_runtime_apis =
            Some(specific_runtime_apis.iter().map(|n| n.as_ref()).collect());
        self
    }

    /// Hash the given metadata.
    pub fn hash(&self) -> [u8; HASH_LEN] {
        let mut visited_ids = HashSet::<u32>::new();

        let metadata = self.metadata;

        let pallet_hash = metadata.pallets().fold([0u8; HASH_LEN], |bytes, pallet| {
            // If specific pallets are given, only include this pallet if it is in the specific pallets.
            let should_hash = self
                .specific_pallets
                .as_ref()
                .map(|specific_pallets| specific_pallets.contains(&pallet.name()))
                .unwrap_or(true);
            // We don't care what order the pallets are seen in, so XOR their
            // hashes together to be order independent.
            if should_hash {
                xor(bytes, get_pallet_hash(pallet))
            } else {
                bytes
            }
        });

        let apis_hash = metadata
            .runtime_api_traits()
            .fold([0u8; HASH_LEN], |bytes, api| {
                // If specific runtime APIs are given, only include this pallet if it is in the specific runtime APIs.
                let should_hash = self
                    .specific_runtime_apis
                    .as_ref()
                    .map(|specific_runtime_apis| specific_runtime_apis.contains(&api.name()))
                    .unwrap_or(true);
                // We don't care what order the runtime APIs are seen in, so XOR their
                // hashes together to be order independent.
                if should_hash {
                    xor(bytes, xor(bytes, get_runtime_trait_hash(api)))
                } else {
                    bytes
                }
            });

        let extrinsic_hash = get_extrinsic_hash(&metadata.types, &metadata.extrinsic);
        let runtime_hash = get_type_hash(&metadata.types, metadata.runtime_ty(), &mut visited_ids);

        concat_and_hash4(&pallet_hash, &apis_hash, &extrinsic_hash, &runtime_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::{order::Lsb0, vec::BitVec};
    use frame_metadata::v15;
    use scale_info::meta_type;

    // Define recursive types.
    #[allow(dead_code)]
    #[derive(scale_info::TypeInfo)]
    struct A {
        pub b: Box<B>,
    }

    #[allow(dead_code)]
    #[derive(scale_info::TypeInfo)]
    struct B {
        pub a: Box<A>,
    }

    // Define TypeDef supported types.
    #[allow(dead_code)]
    #[derive(scale_info::TypeInfo)]
    // TypeDef::Composite with TypeDef::Array with Typedef::Primitive.
    struct AccountId32([u8; HASH_LEN]);

    #[allow(dead_code)]
    #[derive(scale_info::TypeInfo)]
    // TypeDef::Variant.
    enum DigestItem {
        PreRuntime(
            // TypeDef::Array with primitive.
            [::core::primitive::u8; 4usize],
            // TypeDef::Sequence.
            ::std::vec::Vec<::core::primitive::u8>,
        ),
        Other(::std::vec::Vec<::core::primitive::u8>),
        // Nested TypeDef::Tuple.
        RuntimeEnvironmentUpdated(((i8, i16), (u32, u64))),
        // TypeDef::Compact.
        Index(#[codec(compact)] ::core::primitive::u8),
        // TypeDef::BitSequence.
        BitSeq(BitVec<u8, Lsb0>),
    }

    #[allow(dead_code)]
    #[derive(scale_info::TypeInfo)]
    // Ensure recursive types and TypeDef variants are captured.
    struct MetadataTestType {
        recursive: A,
        composite: AccountId32,
        type_def: DigestItem,
    }

    #[allow(dead_code)]
    #[derive(scale_info::TypeInfo)]
    // Simulate a PalletCallMetadata.
    enum Call {
        #[codec(index = 0)]
        FillBlock { ratio: AccountId32 },
        #[codec(index = 1)]
        Remark { remark: DigestItem },
    }

    fn build_default_extrinsic() -> v15::ExtrinsicMetadata {
        v15::ExtrinsicMetadata {
            ty: meta_type::<()>(),
            version: 0,
            signed_extensions: vec![],
        }
    }

    fn default_pallet() -> v15::PalletMetadata {
        v15::PalletMetadata {
            name: "Test",
            storage: None,
            calls: None,
            event: None,
            constants: vec![],
            error: None,
            index: 0,
            docs: vec![],
        }
    }

    fn build_default_pallets() -> Vec<v15::PalletMetadata> {
        vec![
            v15::PalletMetadata {
                name: "First",
                calls: Some(v15::PalletCallMetadata {
                    ty: meta_type::<MetadataTestType>(),
                }),
                ..default_pallet()
            },
            v15::PalletMetadata {
                name: "Second",
                index: 1,
                calls: Some(v15::PalletCallMetadata {
                    ty: meta_type::<(DigestItem, AccountId32, A)>(),
                }),
                ..default_pallet()
            },
        ]
    }

    fn pallets_to_metadata(pallets: Vec<v15::PalletMetadata>) -> Metadata {
        v15::RuntimeMetadataV15::new(
            pallets,
            build_default_extrinsic(),
            meta_type::<()>(),
            vec![],
        )
        .try_into()
        .expect("can build valid metadata")
    }

    #[test]
    fn different_pallet_index() {
        let pallets = build_default_pallets();
        let mut pallets_swap = pallets.clone();

        let metadata = pallets_to_metadata(pallets);

        // Change the order in which pallets are registered.
        pallets_swap.swap(0, 1);
        pallets_swap[0].index = 0;
        pallets_swap[1].index = 1;
        let metadata_swap = pallets_to_metadata(pallets_swap);

        let hash = MetadataHasher::new(&metadata).hash();
        let hash_swap = MetadataHasher::new(&metadata_swap).hash();

        // Changing pallet order must still result in a deterministic unique hash.
        assert_eq!(hash, hash_swap);
    }

    #[test]
    fn recursive_type() {
        let mut pallet = default_pallet();
        pallet.calls = Some(v15::PalletCallMetadata {
            ty: meta_type::<A>(),
        });
        let metadata = pallets_to_metadata(vec![pallet]);

        // Check hashing algorithm finishes on a recursive type.
        MetadataHasher::new(&metadata).hash();
    }

    #[test]
    /// Ensure correctness of hashing when parsing the `metadata.types`.
    ///
    /// Having a recursive structure `A: { B }` and `B: { A }` registered in different order
    /// `types: { { id: 0, A }, { id: 1, B } }` and `types: { { id: 0, B }, { id: 1, A } }`
    /// must produce the same deterministic hashing value.
    fn recursive_types_different_order() {
        let mut pallets = build_default_pallets();
        pallets[0].calls = Some(v15::PalletCallMetadata {
            ty: meta_type::<A>(),
        });
        pallets[1].calls = Some(v15::PalletCallMetadata {
            ty: meta_type::<B>(),
        });
        pallets[1].index = 1;
        let mut pallets_swap = pallets.clone();
        let metadata = pallets_to_metadata(pallets);

        pallets_swap.swap(0, 1);
        pallets_swap[0].index = 0;
        pallets_swap[1].index = 1;
        let metadata_swap = pallets_to_metadata(pallets_swap);

        let hash = MetadataHasher::new(&metadata).hash();
        let hash_swap = MetadataHasher::new(&metadata_swap).hash();

        // Changing pallet order must still result in a deterministic unique hash.
        assert_eq!(hash, hash_swap);
    }

    #[test]
    // Redundant clone clippy warning is a lie; https://github.com/rust-lang/rust-clippy/issues/10870
    #[allow(clippy::redundant_clone)]
    fn pallet_hash_correctness() {
        let compare_pallets_hash = |lhs: &v15::PalletMetadata, rhs: &v15::PalletMetadata| {
            let metadata = pallets_to_metadata(vec![lhs.clone()]);
            let hash = MetadataHasher::new(&metadata).hash();

            let metadata = pallets_to_metadata(vec![rhs.clone()]);
            let new_hash = MetadataHasher::new(&metadata).hash();

            assert_ne!(hash, new_hash);
        };

        // Build metadata progressively from an empty pallet to a fully populated pallet.
        let mut pallet = default_pallet();
        let pallet_lhs = pallet.clone();
        pallet.storage = Some(v15::PalletStorageMetadata {
            prefix: "Storage",
            entries: vec![v15::StorageEntryMetadata {
                name: "BlockWeight",
                modifier: v15::StorageEntryModifier::Default,
                ty: v15::StorageEntryType::Plain(meta_type::<u8>()),
                default: vec![],
                docs: vec![],
            }],
        });
        compare_pallets_hash(&pallet_lhs, &pallet);

        let pallet_lhs = pallet.clone();
        // Calls are similar to:
        //
        // ```
        // pub enum Call {
        //     call_name_01 { arg01: type },
        //     call_name_02 { arg01: type, arg02: type }
        // }
        // ```
        pallet.calls = Some(v15::PalletCallMetadata {
            ty: meta_type::<Call>(),
        });
        compare_pallets_hash(&pallet_lhs, &pallet);

        let pallet_lhs = pallet.clone();
        // Events are similar to Calls.
        pallet.event = Some(v15::PalletEventMetadata {
            ty: meta_type::<Call>(),
        });
        compare_pallets_hash(&pallet_lhs, &pallet);

        let pallet_lhs = pallet.clone();
        pallet.constants = vec![v15::PalletConstantMetadata {
            name: "BlockHashCount",
            ty: meta_type::<u64>(),
            value: vec![96u8, 0, 0, 0],
            docs: vec![],
        }];
        compare_pallets_hash(&pallet_lhs, &pallet);

        let pallet_lhs = pallet.clone();
        pallet.error = Some(v15::PalletErrorMetadata {
            ty: meta_type::<MetadataTestType>(),
        });
        compare_pallets_hash(&pallet_lhs, &pallet);
    }

    #[test]
    fn metadata_per_pallet_hash_correctness() {
        let pallets = build_default_pallets();

        // Build metadata with just the first pallet.
        let metadata_one = pallets_to_metadata(vec![pallets[0].clone()]);
        // Build metadata with both pallets.
        let metadata_both = pallets_to_metadata(pallets);

        // Hashing will ignore any non-existant pallet and return the same result.
        let hash = MetadataHasher::new(&metadata_one)
            .only_these_pallets(&["First", "Second"])
            .hash();
        let hash_rhs = MetadataHasher::new(&metadata_one)
            .only_these_pallets(&["First"])
            .hash();
        assert_eq!(hash, hash_rhs, "hashing should ignore non-existant pallets");

        // Hashing one pallet from metadata with 2 pallets inserted will ignore the second pallet.
        let hash_second = MetadataHasher::new(&metadata_both)
            .only_these_pallets(&["First"])
            .hash();
        assert_eq!(
            hash_second, hash,
            "hashing one pallet should ignore the others"
        );

        // Check hashing with all pallets.
        let hash_second = MetadataHasher::new(&metadata_both)
            .only_these_pallets(&["First", "Second"])
            .hash();
        assert_ne!(
            hash_second, hash,
            "hashing both pallets should produce a different result from hashing just one pallet"
        );
    }

    #[test]
    fn field_semantic_changes() {
        // Get a hash representation of the provided meta type,
        // inserted in the context of pallet metadata call.
        let to_hash = |meta_ty| {
            let pallet = v15::PalletMetadata {
                calls: Some(v15::PalletCallMetadata { ty: meta_ty }),
                ..default_pallet()
            };
            let metadata = pallets_to_metadata(vec![pallet]);
            MetadataHasher::new(&metadata).hash()
        };

        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        enum EnumA1 {
            First { hi: u8, bye: String },
            Second(u32),
            Third,
        }
        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        enum EnumA2 {
            Second(u32),
            Third,
            First { bye: String, hi: u8 },
        }

        // EncodeAsType and DecodeAsType only care about enum variant names
        // and not indexes or field ordering or the enum name itself..
        assert_eq!(
            to_hash(meta_type::<EnumA1>()),
            to_hash(meta_type::<EnumA2>())
        );

        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        struct StructB1 {
            hello: bool,
            another: [u8; 32],
        }
        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        struct StructB2 {
            another: [u8; 32],
            hello: bool,
        }

        // As with enums, struct names and field orders are irrelevant as long as
        // the field names and types are the same.
        assert_eq!(
            to_hash(meta_type::<StructB1>()),
            to_hash(meta_type::<StructB2>())
        );

        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        enum EnumC1 {
            First(u8),
        }
        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        enum EnumC2 {
            Second(u8),
        }

        // The enums are binary compatible, but the variants have different names, so
        // semantically they are different and should not be equal.
        assert_ne!(
            to_hash(meta_type::<EnumC1>()),
            to_hash(meta_type::<EnumC2>())
        );

        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        enum EnumD1 {
            First { a: u8 },
        }
        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        enum EnumD2 {
            First { b: u8 },
        }

        // Named fields contain a different semantic meaning ('a' and 'b')  despite
        // being binary compatible, so hashes should be different.
        assert_ne!(
            to_hash(meta_type::<EnumD1>()),
            to_hash(meta_type::<EnumD2>())
        );

        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        struct StructE1 {
            a: u32,
        }
        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        struct StructE2 {
            b: u32,
        }

        // Similar to enums, struct fields that contain a different semantic meaning
        // ('a' and 'b') despite being binary compatible will have different hashes.
        assert_ne!(
            to_hash(meta_type::<StructE1>()),
            to_hash(meta_type::<StructE2>())
        );
    }
}
