//! Hash representations of the `frame_metadata::v15::OuterEnums`.

use hashbrown::HashMap;

use scale_info::{PortableRegistry, TypeDef};

use crate::{
    utils::{
        retain,
        validation::{get_type_def_variant_hash, get_type_hash},
    },
    Metadata,
};

use super::{concat_and_hash3, Hash, HASH_LEN};

/// Hash representations of the `frame_metadata::v15::OuterEnums`.
pub struct OuterEnumHashes {
    call_hash: (u32, Hash),
    error_hash: (u32, Hash),
    event_hash: (u32, Hash),
}

impl OuterEnumHashes {
    /// Constructs new `OuterEnumHashes` from metadata. If `only_these_variants` is set, the enums are stripped down to only these variants, before their hashes are calculated.
    pub fn new(
        metadata: &Metadata,
        specific_pallets: Option<&[&str]>,
        specific_runtimes: Option<&[&str]>,
    ) -> Self {
        let filter = |names: Option<&[&str]>, name: &str| match names {
            Some(names) => names.contains(&name),
            None => true,
        };
        let mut check_enum_type_id = retain::keep_outer_enum(
            metadata,
            &mut |name| filter(specific_pallets, name),
            &mut |name| filter(specific_runtimes, name),
        );

        let variants = |filter: bool| {
            if !filter {
                specific_pallets
            } else {
                None
            }
        };
        fn get_enum_hash(
            registry: &PortableRegistry,
            id: u32,
            only_these_variants: Option<&[&str]>,
        ) -> Hash {
            let ty = registry
                .types
                .get(id as usize)
                .expect("Metadata should contain enum type in registry");

            if let TypeDef::Variant(variant) = &ty.ty.type_def {
                get_type_def_variant_hash(
                    registry,
                    variant,
                    only_these_variants,
                    &mut HashMap::new(),
                    // ignored, because not computed yet...
                    &OuterEnumHashes::empty(),
                )
            } else {
                get_type_hash(registry, id, &OuterEnumHashes::empty())
            }
        }
        let enums = &metadata.outer_enums;

        let call_variants = variants(check_enum_type_id(enums.call_enum_ty));
        let call_hash = get_enum_hash(metadata.types(), enums.call_enum_ty, call_variants);
        let event_variants = variants(check_enum_type_id(enums.event_enum_ty));
        let event_hash = get_enum_hash(metadata.types(), enums.event_enum_ty, event_variants);
        let error_variants = variants(check_enum_type_id(enums.error_enum_ty));
        let error_hash = get_enum_hash(metadata.types(), enums.error_enum_ty, error_variants);

        Self {
            call_hash: (enums.call_enum_ty, call_hash),
            error_hash: (enums.error_enum_ty, error_hash),
            event_hash: (enums.event_enum_ty, event_hash),
        }
    }

    /// Constructs empty `OuterEnumHashes` with type ids that are never a real type id.
    /// Can be used as a placeholder when outer enum hashes are required but should be ignored.
    pub fn empty() -> Self {
        Self {
            call_hash: (u32::MAX, [0; HASH_LEN]),
            error_hash: (u32::MAX, [0; HASH_LEN]),
            event_hash: (u32::MAX, [0; HASH_LEN]),
        }
    }

    /// Returns a combined hash of the top level enums.
    pub fn combined_hash(&self) -> Hash {
        concat_and_hash3(&self.call_hash.1, &self.error_hash.1, &self.event_hash.1)
    }

    /// Checks if a type is one of the 3 top level enum types. If so, returns Some(hash).
    ///
    /// This is useful, because top level enums are sometimes stripped down to only certain pallets.
    /// The hashes of these stripped down types are stored in this struct.
    pub fn resolve(&self, id: u32) -> Option<[u8; HASH_LEN]> {
        match id {
            e if e == self.error_hash.0 => Some(self.error_hash.1),
            e if e == self.event_hash.0 => Some(self.event_hash.1),
            e if e == self.call_hash.0 => Some(self.call_hash.1),
            _ => None,
        }
    }
}
