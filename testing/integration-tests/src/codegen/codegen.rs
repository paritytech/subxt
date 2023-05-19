// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use frame_metadata::{
    v15::{ExtrinsicMetadata, RuntimeMetadataV15},
    RuntimeMetadataPrefixed,
};
use scale_info::{meta_type, IntoPortable, TypeInfo};
use subxt_codegen::{CratePath, DerivesRegistry, RuntimeGenerator, TypeSubstitutes};

fn generate_runtime_interface_from_metadata(metadata: RuntimeMetadataPrefixed) -> String {
    // Generate a runtime interface from the provided metadata.
    let generator = RuntimeGenerator::new(metadata);
    let item_mod = syn::parse_quote!(
        pub mod api {}
    );
    let crate_path = CratePath::default();
    let derives = DerivesRegistry::with_default_derives(&crate_path);
    let type_substitutes = TypeSubstitutes::with_default_substitutes(&crate_path);
    generator
        .generate_runtime(item_mod, derives, type_substitutes, crate_path, false)
        .expect("API generation must be valid")
        .to_string()
}

#[test]
fn dupe_types_do_not_overwrite_each_other() {
    fn generate_metadata_with_duplicate_types() -> RuntimeMetadataPrefixed {
        #[derive(TypeInfo)]
        struct Runtime;
        #[derive(TypeInfo)]
        enum RuntimeCall {}
        #[derive(TypeInfo)]
        enum RuntimeEvent {}
        #[derive(TypeInfo)]
        pub enum DispatchError {}

        // We need these types for codegen to work:
        let mut registry = scale_info::Registry::new();
        let ty = registry.register_type(&meta_type::<Runtime>());
        registry.register_type(&meta_type::<RuntimeCall>());
        registry.register_type(&meta_type::<RuntimeEvent>());
        registry.register_type(&meta_type::<DispatchError>());

        // Now we duplicate some types with same type info:
        enum Foo {}
        impl TypeInfo for Foo {
            type Identity = Self;
            fn type_info() -> scale_info::Type {
                scale_info::Type::builder()
                    .path(scale_info::Path::new("DuplicateType", "dupe_mod"))
                    .variant(
                        scale_info::build::Variants::new()
                            .variant("FirstDupeTypeVariant", |builder| builder.index(0)),
                    )
            }
        }
        enum Bar {}
        impl TypeInfo for Bar {
            type Identity = Self;
            fn type_info() -> scale_info::Type {
                scale_info::Type::builder()
                    .path(scale_info::Path::new("DuplicateType", "dupe_mod"))
                    .variant(
                        scale_info::build::Variants::new()
                            .variant("SecondDupeTypeVariant", |builder| builder.index(0)),
                    )
            }
        }

        registry.register_type(&meta_type::<Foo>());
        registry.register_type(&meta_type::<Bar>());

        let extrinsic = ExtrinsicMetadata {
            ty: meta_type::<()>(),
            version: 0,
            signed_extensions: vec![],
        }
        .into_portable(&mut registry);
        let metadata = RuntimeMetadataV15 {
            types: registry.into(),
            pallets: Vec::new(),
            extrinsic,
            ty,
            apis: vec![],
        };

        RuntimeMetadataPrefixed::from(metadata)
    }

    let metadata = generate_metadata_with_duplicate_types();
    let interface = generate_runtime_interface_from_metadata(metadata);

    assert!(interface.contains("FirstDupeTypeVariant"));
    assert!(interface.contains("SecondDupeTypeVariant"));
}
