// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
    test_context,
    TestContext,
};
use frame_metadata::{
    ExtrinsicMetadata,
    PalletCallMetadata,
    PalletMetadata,
    PalletStorageMetadata,
    RuntimeMetadataPrefixed,
    RuntimeMetadataV14,
    StorageEntryMetadata,
    StorageEntryModifier,
    StorageEntryType,
};
use scale_info::{
    build::{
        Fields,
        Variants,
    },
    meta_type,
    Path,
    Type,
    TypeInfo,
};
use subxt::{
    ClientBuilder,
    DefaultConfig,
    Metadata,
    SubstrateExtrinsicParams,
};

use crate::utils::node_runtime;

type RuntimeApi =
    node_runtime::RuntimeApi<DefaultConfig, SubstrateExtrinsicParams<DefaultConfig>>;

async fn metadata_to_api(metadata: RuntimeMetadataV14, cxt: &TestContext) -> RuntimeApi {
    let prefixed = RuntimeMetadataPrefixed::from(metadata);
    let metadata = Metadata::try_from(prefixed).unwrap();

    ClientBuilder::new()
            .set_url(cxt.node_proc.ws_url().to_string())
            .set_metadata(metadata)
            .build()
            .await
            .unwrap()
            .to_runtime_api::<node_runtime::RuntimeApi<
                DefaultConfig,
                SubstrateExtrinsicParams<DefaultConfig>,
            >>()
}

#[tokio::test]
async fn full_metadata_check() {
    let cxt = test_context().await;
    let api = &cxt.api;

    // Runtime metadata is identical to the metadata used during API generation.
    assert!(api.validate_metadata().is_ok());

    // Modify the metadata.
    let mut metadata: RuntimeMetadataV14 = {
        let locked_client_metadata = api.client.metadata();
        let client_metadata = locked_client_metadata.read();
        client_metadata.runtime_metadata().clone()
    };
    metadata.pallets[0].name = "NewPallet".to_string();

    let new_api = metadata_to_api(metadata, &cxt).await;
    assert_eq!(
        new_api
            .validate_metadata()
            .err()
            .expect("Validation should fail for incompatible metadata"),
        ::subxt::MetadataError::IncompatibleMetadata
    );
}

#[tokio::test]
async fn constants_check() {
    let cxt = test_context().await;
    let api = &cxt.api;

    // Ensure that `ExistentialDeposit` is compatible before altering the metadata.
    assert!(cxt.api.constants().balances().existential_deposit().is_ok());

    // Modify the metadata.
    let mut metadata: RuntimeMetadataV14 = {
        let locked_client_metadata = api.client.metadata();
        let client_metadata = locked_client_metadata.read();
        client_metadata.runtime_metadata().clone()
    };

    let mut existential = metadata
        .pallets
        .iter_mut()
        .find(|pallet| pallet.name == "Balances")
        .expect("Metadata must contain Balances pallet")
        .constants
        .iter_mut()
        .find(|constant| constant.name == "ExistentialDeposit")
        .expect("ExistentialDeposit constant must be present");
    existential.value = vec![0u8; 32];

    let new_api = metadata_to_api(metadata, &cxt).await;

    assert!(new_api.validate_metadata().is_err());
    assert!(new_api
        .constants()
        .balances()
        .existential_deposit()
        .is_err());

    // Other constant validation should not be impacted.
    assert!(new_api.constants().balances().max_locks().is_ok());
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
    }
}

fn pallets_to_metadata(pallets: Vec<PalletMetadata>) -> RuntimeMetadataV14 {
    RuntimeMetadataV14::new(
        pallets,
        ExtrinsicMetadata {
            ty: meta_type::<()>(),
            version: 0,
            signed_extensions: vec![],
        },
        meta_type::<()>(),
    )
}

#[tokio::test]
async fn calls_check() {
    let cxt = test_context().await;

    // Ensure that `Unbond` and `WinthdrawUnbonded` calls are compatible before altering the metadata.
    assert!(cxt.api.tx().staking().unbond(123_456_789_012_345).is_ok());
    assert!(cxt.api.tx().staking().withdraw_unbonded(10).is_ok());

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
                                f.compact::<u128>()
                                    .name("value")
                                    .type_name("BalanceOf<T>")
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
    let new_api = metadata_to_api(metadata, &cxt).await;
    assert!(new_api.tx().staking().unbond(123_456_789_012_345).is_ok());
    assert!(new_api.tx().staking().withdraw_unbonded(10).is_ok());

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
    let new_api = metadata_to_api(metadata, &cxt).await;
    // Unbond call should fail, while withdraw_unbonded remains compatible.
    assert!(new_api.tx().staking().unbond(123_456_789_012_345).is_err());
    assert!(new_api.tx().staking().withdraw_unbonded(10).is_ok());
}

#[tokio::test]
async fn storage_check() {
    let cxt = test_context().await;

    // Ensure that `ExtrinsicCount` and `EventCount` storages are compatible before altering the metadata.
    assert!(cxt
        .api
        .storage()
        .system()
        .extrinsic_count(None)
        .await
        .is_ok());
    assert!(cxt
        .api
        .storage()
        .system()
        .all_extrinsics_len(None)
        .await
        .is_ok());

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
    let new_api = metadata_to_api(metadata, &cxt).await;
    assert!(new_api
        .storage()
        .system()
        .extrinsic_count(None)
        .await
        .is_ok());
    assert!(new_api
        .storage()
        .system()
        .all_extrinsics_len(None)
        .await
        .is_ok());

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
    let new_api = metadata_to_api(metadata, &cxt).await;
    assert!(new_api
        .storage()
        .system()
        .extrinsic_count(None)
        .await
        .is_err());
    assert!(new_api
        .storage()
        .system()
        .all_extrinsics_len(None)
        .await
        .is_ok());
}
