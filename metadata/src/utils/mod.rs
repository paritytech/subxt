// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

pub mod ordered_map;
pub mod retain;
pub mod validation;
pub mod variant_index;

use alloc::{vec, borrow::ToOwned};
use scale_info::PortableRegistry;

/// Push a dummy type to the type registry, returning the ID for it.
pub fn push_dummy_type_to_registry(types: &mut PortableRegistry) -> u32 {
    let dummy_runtime_type = scale_info::Type {
        path: scale_info::Path { 
            segments: vec![
                "DummyType".to_owned()
            ] 
        },
        type_params: vec![],
        type_def: scale_info::TypeDef::Composite(scale_info::TypeDefComposite { fields: vec![] }),
        docs: vec![]
    };
    let dummy_type_id = types.types.len() as u32;

    types.types.push(scale_info::PortableType { 
        id: dummy_type_id, 
        ty: dummy_runtime_type 
    });

    dummy_type_id

}