// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::TryFromError;
use crate::Metadata;
use frame_metadata::{v14, v15};

impl TryFrom<v14::RuntimeMetadataV14> for Metadata {
    type Error = TryFromError;
    fn try_from(value: v14::RuntimeMetadataV14) -> Result<Self, Self::Error> {
        // Convert to v15 and then convert that into Metadata.
        v14_to_v15(value).try_into()
    }
}

impl From<Metadata> for v14::RuntimeMetadataV14 {
    fn from(val: Metadata) -> Self {
        let v15 = val.into();
        v15_to_v14(v15)
    }
}

fn v15_to_v14(metadata: v15::RuntimeMetadataV15) -> v14::RuntimeMetadataV14 {
    v14::RuntimeMetadataV14 {
        types: metadata.types,
        pallets: metadata
            .pallets
            .into_iter()
            .map(|pallet| frame_metadata::v14::PalletMetadata {
                name: pallet.name,
                storage: pallet
                    .storage
                    .map(|storage| frame_metadata::v14::PalletStorageMetadata {
                        prefix: storage.prefix,
                        entries: storage
                            .entries
                            .into_iter()
                            .map(|entry| {
                                let modifier = match entry.modifier {
                                    frame_metadata::v15::StorageEntryModifier::Optional => {
                                        frame_metadata::v14::StorageEntryModifier::Optional
                                    }
                                    frame_metadata::v15::StorageEntryModifier::Default => {
                                        frame_metadata::v14::StorageEntryModifier::Default
                                    }
                                };

                                let ty = match entry.ty {
                                    frame_metadata::v15::StorageEntryType::Plain(ty) => {
                                        frame_metadata::v14::StorageEntryType::Plain(ty)
                                    },
                                    frame_metadata::v15::StorageEntryType::Map {
                                        hashers,
                                        key,
                                        value,
                                    } => frame_metadata::v14::StorageEntryType::Map {
                                        hashers: hashers.into_iter().map(|hasher| match hasher {
                                            frame_metadata::v15::StorageHasher::Blake2_128 => frame_metadata::v14::StorageHasher::Blake2_128,
                                            frame_metadata::v15::StorageHasher::Blake2_256 => frame_metadata::v14::StorageHasher::Blake2_256,
                                            frame_metadata::v15::StorageHasher::Blake2_128Concat  => frame_metadata::v14::StorageHasher::Blake2_128Concat ,
                                            frame_metadata::v15::StorageHasher::Twox128 => frame_metadata::v14::StorageHasher::Twox128,
                                            frame_metadata::v15::StorageHasher::Twox256 => frame_metadata::v14::StorageHasher::Twox256,
                                            frame_metadata::v15::StorageHasher::Twox64Concat => frame_metadata::v14::StorageHasher::Twox64Concat,
                                            frame_metadata::v15::StorageHasher::Identity=> frame_metadata::v14::StorageHasher::Identity,
                                        }).collect(),
                                        key,
                                        value,
                                    },
                                };

                                frame_metadata::v14::StorageEntryMetadata {
                                    name: entry.name,
                                    modifier,
                                    ty,
                                    default: entry.default,
                                    docs: entry.docs,
                                }
                            })
                            .collect(),
                    }),
                calls: pallet.calls.map(|calls| frame_metadata::v14::PalletCallMetadata { ty: calls.ty } ),
                event: pallet.event.map(|event| frame_metadata::v14::PalletEventMetadata { ty: event.ty } ),
                constants: pallet.constants.into_iter().map(|constant| frame_metadata::v14::PalletConstantMetadata {
                    name: constant.name,
                    ty: constant.ty,
                    value: constant.value,
                    docs: constant.docs,
                } ).collect(),
                error: pallet.error.map(|error| frame_metadata::v14::PalletErrorMetadata { ty: error.ty } ),
                index: pallet.index,
            })
            .collect(),
        extrinsic: frame_metadata::v14::ExtrinsicMetadata {
            ty: metadata.extrinsic.ty,
            version: metadata.extrinsic.version,
            signed_extensions: metadata.extrinsic.signed_extensions.into_iter().map(|ext| {
                frame_metadata::v14::SignedExtensionMetadata {
                    identifier: ext.identifier,
                    ty: ext.ty,
                    additional_signed: ext.additional_signed,
                }
            }).collect()
        },
        ty: metadata.ty,
    }
}

fn v14_to_v15(metadata: v14::RuntimeMetadataV14) -> v15::RuntimeMetadataV15 {
    v15::RuntimeMetadataV15 {
        types: metadata.types,
        pallets: metadata
            .pallets
            .into_iter()
            .map(|pallet| frame_metadata::v15::PalletMetadata {
                name: pallet.name,
                storage: pallet
                    .storage
                    .map(|storage| frame_metadata::v15::PalletStorageMetadata {
                        prefix: storage.prefix,
                        entries: storage
                            .entries
                            .into_iter()
                            .map(|entry| {
                                let modifier = match entry.modifier {
                                    frame_metadata::v14::StorageEntryModifier::Optional => {
                                        frame_metadata::v15::StorageEntryModifier::Optional
                                    }
                                    frame_metadata::v14::StorageEntryModifier::Default => {
                                        frame_metadata::v15::StorageEntryModifier::Default
                                    }
                                };

                                let ty = match entry.ty {
                                    frame_metadata::v14::StorageEntryType::Plain(ty) => {
                                        frame_metadata::v15::StorageEntryType::Plain(ty)
                                    },
                                    frame_metadata::v14::StorageEntryType::Map {
                                        hashers,
                                        key,
                                        value,
                                    } => frame_metadata::v15::StorageEntryType::Map {
                                        hashers: hashers.into_iter().map(|hasher| match hasher {
                                            frame_metadata::v14::StorageHasher::Blake2_128 => frame_metadata::v15::StorageHasher::Blake2_128,
                                            frame_metadata::v14::StorageHasher::Blake2_256 => frame_metadata::v15::StorageHasher::Blake2_256,
                                            frame_metadata::v14::StorageHasher::Blake2_128Concat  => frame_metadata::v15::StorageHasher::Blake2_128Concat ,
                                            frame_metadata::v14::StorageHasher::Twox128 => frame_metadata::v15::StorageHasher::Twox128,
                                            frame_metadata::v14::StorageHasher::Twox256 => frame_metadata::v15::StorageHasher::Twox256,
                                            frame_metadata::v14::StorageHasher::Twox64Concat => frame_metadata::v15::StorageHasher::Twox64Concat,
                                            frame_metadata::v14::StorageHasher::Identity=> frame_metadata::v15::StorageHasher::Identity,
                                        }).collect(),
                                        key,
                                        value,
                                    },
                                };

                                frame_metadata::v15::StorageEntryMetadata {
                                    name: entry.name,
                                    modifier,
                                    ty,
                                    default: entry.default,
                                    docs: entry.docs,
                                }
                            })
                            .collect(),
                    }),
                calls: pallet.calls.map(|calls| frame_metadata::v15::PalletCallMetadata { ty: calls.ty } ),
                event: pallet.event.map(|event| frame_metadata::v15::PalletEventMetadata { ty: event.ty } ),
                constants: pallet.constants.into_iter().map(|constant| frame_metadata::v15::PalletConstantMetadata {
                    name: constant.name,
                    ty: constant.ty,
                    value: constant.value,
                    docs: constant.docs,
                } ).collect(),
                error: pallet.error.map(|error| frame_metadata::v15::PalletErrorMetadata { ty: error.ty } ),
                index: pallet.index,
                docs: Default::default(),
            })
            .collect(),
        extrinsic: frame_metadata::v15::ExtrinsicMetadata {
            ty: metadata.extrinsic.ty,
            version: metadata.extrinsic.version,
            signed_extensions: metadata.extrinsic.signed_extensions.into_iter().map(|ext| {
                frame_metadata::v15::SignedExtensionMetadata {
                    identifier: ext.identifier,
                    ty: ext.ty,
                    additional_signed: ext.additional_signed,
                }
            }).collect()
        },
        ty: metadata.ty,
        apis: Default::default(),
    }
}
