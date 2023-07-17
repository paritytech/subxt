// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::{node_runtime, test_context, TestContext};
use frame_metadata::v15::{
    CustomMetadata, ExtrinsicMetadata, OuterEnums, PalletCallMetadata, PalletMetadata,
    PalletStorageMetadata, RuntimeMetadataV15, StorageEntryMetadata, StorageEntryModifier,
    StorageEntryType,
};
use scale_info::{
    build::{Fields, Variants},
    meta_type, Path, Type, TypeInfo,
};
use subxt::{Metadata, OfflineClient, SubstrateConfig};

async fn metadata_to_api(metadata: Metadata, ctx: &TestContext) -> OfflineClient<SubstrateConfig> {
    OfflineClient::new(
        ctx.client().genesis_hash(),
        ctx.client().runtime_version(),
        metadata,
    )
}

fn v15_to_metadata(v15: RuntimeMetadataV15) -> Metadata {
    let subxt_md: subxt_metadata::Metadata = v15.try_into().unwrap();
    subxt_md.into()
}

fn modified_metadata<F>(metadata: Metadata, f: F) -> Metadata
where
    F: FnOnce(&mut RuntimeMetadataV15),
{
    let mut metadata = RuntimeMetadataV15::from((*metadata).clone());
    f(&mut metadata);
    v15_to_metadata(metadata)
}

fn default_pallet() -> PalletMetadata {
    PalletMetadata {
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

fn pallets_to_metadata(pallets: Vec<PalletMetadata>) -> Metadata {
    // Extrinsic needs to contain at least the generic type parameter "Call"
    // for the metadata to be valid.
    // The "Call" type from the metadata is used to decode extrinsics.
    // In reality, the extrinsic type has "Call", "Address", "Extra", "Signature" generic types.
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct ExtrinsicType<Call> {
        call: Call,
    }
    // Because this type is used to decode extrinsics, we expect this to be a TypeDefVariant.
    // Each pallet must contain one single variant.
    #[allow(unused)]
    #[derive(TypeInfo)]
    enum RuntimeCall {
        PalletName(Pallet),
    }
    // The calls of the pallet.
    #[allow(unused)]
    #[derive(TypeInfo)]
    enum Pallet {
        #[allow(unused)]
        SomeCall,
    }

    v15_to_metadata(RuntimeMetadataV15::new(
        pallets,
        ExtrinsicMetadata {
            version: 0,
            signed_extensions: vec![],
            address_ty: meta_type::<()>(),
            call_ty: meta_type::<RuntimeCall>(),
            signature_ty: meta_type::<()>(),
            extra_ty: meta_type::<()>(),
        },
        meta_type::<()>(),
        vec![],
        OuterEnums {
            call_enum_ty: meta_type::<()>(),
            event_enum_ty: meta_type::<()>(),
            error_enum_ty: meta_type::<()>(),
        },
        CustomMetadata {
            map: Default::default(),
        },
    ))
}

#[tokio::test]
async fn full_metadata_check() {
    let ctx = test_context().await;
    let api = ctx.client();

    // Runtime metadata is identical to the metadata used during API generation.
    assert!(node_runtime::is_codegen_valid_for(&api.metadata()));

    // Modify the metadata.
    let metadata = modified_metadata(api.metadata(), |md| {
        md.pallets[0].name = "NewPallet".to_string();
    });

    // It should now be invalid:
    assert!(!node_runtime::is_codegen_valid_for(&metadata));
}

#[tokio::test]
async fn constant_values_are_not_validated() {
    let ctx = test_context().await;
    let api = ctx.client();

    let deposit_addr = node_runtime::constants().balances().existential_deposit();

    // Retrieve existential deposit to validate it and confirm that it's OK.
    assert!(api.constants().at(&deposit_addr).is_ok());

    // Modify the metadata.
    let metadata = modified_metadata(api.metadata(), |md| {
        let existential = md
            .pallets
            .iter_mut()
            .find(|pallet| pallet.name == "Balances")
            .expect("Metadata must contain Balances pallet")
            .constants
            .iter_mut()
            .find(|constant| constant.name == "ExistentialDeposit")
            .expect("ExistentialDeposit constant must be present");

        // Modifying a constant value should not lead to an error:
        existential.value = vec![0u8; 32];
    });

    let api = metadata_to_api(metadata, &ctx).await;

    assert!(node_runtime::is_codegen_valid_for(&api.metadata()));
    assert!(api.constants().at(&deposit_addr).is_ok());
}

#[tokio::test]
async fn calls_check() {
    let ctx = test_context().await;
    let api = ctx.client();

    let unbond_tx = node_runtime::tx().staking().unbond(123_456_789_012_345);
    let withdraw_unbonded_addr = node_runtime::tx().staking().withdraw_unbonded(10);

    // Ensure that `Unbond` and `WinthdrawUnbonded` calls are compatible before altering the metadata.
    assert!(api.tx().validate(&unbond_tx).is_ok());
    assert!(api.tx().validate(&withdraw_unbonded_addr).is_ok());

    // Reconstruct the `Staking` call as is.
    struct CallRec;
    impl TypeInfo for CallRec {
        type Identity = Self;
        fn type_info() -> Type {
            Type::builder()
                .path(Path::new("Call", "pallet_staking::pallet::pallet"))
                .variant(
                    Variants::new()
                        .variant("unbond", |v| {
                            v.index(0).fields(Fields::named().field(|f| {
                                f.compact::<u128>().name("value").type_name("BalanceOf<T>")
                            }))
                        })
                        .variant("withdraw_unbonded", |v| {
                            v.index(1).fields(Fields::named().field(|f| {
                                f.ty::<u32>().name("num_slashing_spans").type_name("u32")
                            }))
                        }),
                )
        }
    }
    let pallet = PalletMetadata {
        name: "Staking",
        calls: Some(PalletCallMetadata {
            ty: meta_type::<CallRec>(),
        }),
        ..default_pallet()
    };
    let metadata = pallets_to_metadata(vec![pallet]);
    let api = metadata_to_api(metadata, &ctx).await;

    // The calls should still be valid with this new type info:
    assert!(api.tx().validate(&unbond_tx).is_ok());
    assert!(api.tx().validate(&withdraw_unbonded_addr).is_ok());

    // Change `Unbond` call but leave the rest as is.
    struct CallRecSecond;
    impl TypeInfo for CallRecSecond {
        type Identity = Self;
        fn type_info() -> Type {
            Type::builder()
                .path(Path::new("Call", "pallet_staking::pallet::pallet"))
                .variant(
                    Variants::new()
                        .variant("unbond", |v| {
                            v.index(0).fields(Fields::named().field(|f| {
                                // Is of type u32 instead of u128.
                                f.compact::<u32>().name("value").type_name("BalanceOf<T>")
                            }))
                        })
                        .variant("withdraw_unbonded", |v| {
                            v.index(1).fields(Fields::named().field(|f| {
                                f.ty::<u32>().name("num_slashing_spans").type_name("u32")
                            }))
                        }),
                )
        }
    }
    let pallet = PalletMetadata {
        name: "Staking",
        calls: Some(PalletCallMetadata {
            ty: meta_type::<CallRecSecond>(),
        }),
        ..default_pallet()
    };
    let metadata = pallets_to_metadata(vec![pallet]);
    let api = metadata_to_api(metadata, &ctx).await;

    // Unbond call should fail, while withdraw_unbonded remains compatible.
    assert!(api.tx().validate(&unbond_tx).is_err());
    assert!(api.tx().validate(&withdraw_unbonded_addr).is_ok());
}

#[tokio::test]
async fn storage_check() {
    let ctx = test_context().await;
    let api = ctx.client();

    let tx_count_addr = node_runtime::storage().system().extrinsic_count();
    let tx_len_addr = node_runtime::storage().system().all_extrinsics_len();

    // Ensure that `ExtrinsicCount` and `EventCount` storages are compatible before altering the metadata.
    assert!(api.storage().validate(&tx_count_addr).is_ok());
    assert!(api.storage().validate(&tx_len_addr).is_ok());

    // Reconstruct the storage.
    let storage = PalletStorageMetadata {
        prefix: "System",
        entries: vec![
            StorageEntryMetadata {
                name: "ExtrinsicCount",
                modifier: StorageEntryModifier::Optional,
                ty: StorageEntryType::Plain(meta_type::<u32>()),
                default: vec![0],
                docs: vec![],
            },
            StorageEntryMetadata {
                name: "AllExtrinsicsLen",
                modifier: StorageEntryModifier::Optional,
                ty: StorageEntryType::Plain(meta_type::<u32>()),
                default: vec![0],
                docs: vec![],
            },
        ],
    };
    let pallet = PalletMetadata {
        name: "System",
        storage: Some(storage),
        ..default_pallet()
    };
    let metadata = pallets_to_metadata(vec![pallet]);
    let api = metadata_to_api(metadata, &ctx).await;

    // The addresses should still validate:
    assert!(api.storage().validate(&tx_count_addr).is_ok());
    assert!(api.storage().validate(&tx_len_addr).is_ok());

    // Reconstruct the storage while modifying ExtrinsicCount.
    let storage = PalletStorageMetadata {
        prefix: "System",
        entries: vec![
            StorageEntryMetadata {
                name: "ExtrinsicCount",
                modifier: StorageEntryModifier::Optional,
                // Previously was u32.
                ty: StorageEntryType::Plain(meta_type::<u8>()),
                default: vec![0],
                docs: vec![],
            },
            StorageEntryMetadata {
                name: "AllExtrinsicsLen",
                modifier: StorageEntryModifier::Optional,
                ty: StorageEntryType::Plain(meta_type::<u32>()),
                default: vec![0],
                docs: vec![],
            },
        ],
    };
    let pallet = PalletMetadata {
        name: "System",
        storage: Some(storage),
        ..default_pallet()
    };
    let metadata = pallets_to_metadata(vec![pallet]);
    let api = metadata_to_api(metadata, &ctx).await;

    // The count route should fail now; the other will be ok still.
    assert!(api.storage().validate(&tx_count_addr).is_err());
    assert!(api.storage().validate(&tx_len_addr).is_ok());
}
