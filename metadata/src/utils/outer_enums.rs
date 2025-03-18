// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Utility functions for working with v14 metadata.

use scale_info::PortableRegistry;

/// Outer enum type IDs, if found
pub struct OuterEnums {
    /// The RuntimeCall type ID.
    pub call_ty: Option<u32>,
    /// The RuntimeEvent type ID.
    pub event_ty: Option<u32>,
    /// The RuntimeError type ID.
    pub error_ty: Option<u32>,
}

impl OuterEnums {
    /// Iterate over the available outer enum type IDs.
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.call_ty
            .iter()
            .chain(self.event_ty.iter())
            .chain(self.error_ty.iter())
            .copied()
    }

    /// Search for the outer enums in some type registry
    pub fn find_in(types: &PortableRegistry) -> OuterEnums {
        let find_type = |name: &str| {
            types.types.iter().find_map(|ty| {
                let ident = ty.ty.path.ident()?;
    
                if ident != name {
                    return None;
                }
    
                let scale_info::TypeDef::Variant(_) = &ty.ty.type_def else {
                    return None;
                };
    
                Some(ty.id)
            })
        };
    
        OuterEnums {
            call_ty: find_type("RuntimeCall"),
            event_ty: find_type("RuntimeEvent"),
            error_ty: find_type("RuntimeError")
        }
    }
}
