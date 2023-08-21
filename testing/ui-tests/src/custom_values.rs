// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::Encode;
use frame_metadata::v15::{CustomMetadata, ExtrinsicMetadata, OuterEnums, RuntimeMetadataV15};
use frame_metadata::RuntimeMetadataPrefixed;
use scale_info::form::PortableForm;
use scale_info::TypeInfo;
use scale_info::{meta_type, IntoPortable};
use std::collections::BTreeMap;

/// Generate metadata which contains a `Foo { a: u8, b: &str }` custom value.
pub fn metadata_custom_values_foo() -> RuntimeMetadataPrefixed {
    let mut registry = scale_info::Registry::new();

    // create foo value and type:

    #[derive(TypeInfo, Encode)]
    struct Foo {
        a: u8,
        b: &'static str,
    }

    let foo_value_metadata: frame_metadata::v15::CustomValueMetadata<PortableForm> = {
        let foo = Foo { a: 0, b: "Hello" };
        let foo_ty = scale_info::MetaType::new::<Foo>();
        let foo_ty_id = registry.register_type(&foo_ty);
        frame_metadata::v15::CustomValueMetadata {
            ty: foo_ty_id,
            value: foo.encode(),
        }
    };

    // We don't care about the extrinsic type.
    let extrinsic = ExtrinsicMetadata {
        version: 0,
        signed_extensions: vec![],
        address_ty: meta_type::<()>(),
        call_ty: meta_type::<()>(),
        signature_ty: meta_type::<()>(),
        extra_ty: meta_type::<()>(),
    };

    let pallets = vec![];
    let extrinsic = extrinsic.into_portable(&mut registry);

    let unit_ty = registry.register_type(&meta_type::<()>());

    // Metadata needs to contain this DispatchError, since codegen looks for it.
    registry.register_type(&meta_type::<crate::utils::dispatch_error::ArrayDispatchError>());

    let metadata = RuntimeMetadataV15 {
        types: registry.into(),
        pallets,
        extrinsic,
        ty: unit_ty,
        apis: vec![],
        outer_enums: OuterEnums {
            call_enum_ty: unit_ty,
            event_enum_ty: unit_ty,
            error_enum_ty: unit_ty,
        },
        custom: CustomMetadata {
            map: BTreeMap::from_iter([("Foo".into(), foo_value_metadata)]),
        },
    };

    RuntimeMetadataPrefixed::from(metadata)
}
