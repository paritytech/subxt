#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 6usize] = [
        "System",
        "Timestamp",
        "Balances",
        "Staking",
        "Multisig",
        "ParaInherent",
    ];
    pub static RUNTIME_APIS: [&str; 17usize] = [
        "Core",
        "Metadata",
        "BlockBuilder",
        "NominationPoolsApi",
        "StakingApi",
        "TaggedTransactionQueue",
        "OffchainWorkerApi",
        "ParachainHost",
        "BeefyApi",
        "MmrApi",
        "GrandpaApi",
        "BabeApi",
        "AuthorityDiscoveryApi",
        "SessionKeys",
        "AccountNonceApi",
        "TransactionPaymentApi",
        "TransactionPaymentCallApi",
    ];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::polkadot_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::polkadot_runtime::RuntimeCall;
    #[doc = r" The outer error enum representing the DispatchError's Module variant."]
    pub type Error = runtime_types::polkadot_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use subxt::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {
            pub fn core(&self) -> core::Core {
                core::Core
            }
            pub fn metadata(&self) -> metadata::Metadata {
                metadata::Metadata
            }
            pub fn block_builder(&self) -> block_builder::BlockBuilder {
                block_builder::BlockBuilder
            }
            pub fn nomination_pools_api(&self) -> nomination_pools_api::NominationPoolsApi {
                nomination_pools_api::NominationPoolsApi
            }
            pub fn staking_api(&self) -> staking_api::StakingApi {
                staking_api::StakingApi
            }
            pub fn tagged_transaction_queue(
                &self,
            ) -> tagged_transaction_queue::TaggedTransactionQueue {
                tagged_transaction_queue::TaggedTransactionQueue
            }
            pub fn offchain_worker_api(&self) -> offchain_worker_api::OffchainWorkerApi {
                offchain_worker_api::OffchainWorkerApi
            }
            pub fn parachain_host(&self) -> parachain_host::ParachainHost {
                parachain_host::ParachainHost
            }
            pub fn beefy_api(&self) -> beefy_api::BeefyApi {
                beefy_api::BeefyApi
            }
            pub fn mmr_api(&self) -> mmr_api::MmrApi {
                mmr_api::MmrApi
            }
            pub fn grandpa_api(&self) -> grandpa_api::GrandpaApi {
                grandpa_api::GrandpaApi
            }
            pub fn babe_api(&self) -> babe_api::BabeApi {
                babe_api::BabeApi
            }
            pub fn authority_discovery_api(
                &self,
            ) -> authority_discovery_api::AuthorityDiscoveryApi {
                authority_discovery_api::AuthorityDiscoveryApi
            }
            pub fn session_keys(&self) -> session_keys::SessionKeys {
                session_keys::SessionKeys
            }
            pub fn account_nonce_api(&self) -> account_nonce_api::AccountNonceApi {
                account_nonce_api::AccountNonceApi
            }
            pub fn transaction_payment_api(
                &self,
            ) -> transaction_payment_api::TransactionPaymentApi {
                transaction_payment_api::TransactionPaymentApi
            }
            pub fn transaction_payment_call_api(
                &self,
            ) -> transaction_payment_call_api::TransactionPaymentCallApi {
                transaction_payment_call_api::TransactionPaymentCallApi
            }
        }
        pub mod core {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Core` runtime api that every Substrate runtime needs to implement."]
            pub struct Core;
            impl Core {
                #[doc = " Returns the version of the runtime."]
                pub fn version(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Version,
                    runtime_types::sp_version::RuntimeVersion,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "version",
                        types::Version {},
                        [
                            76u8, 202u8, 17u8, 117u8, 189u8, 237u8, 239u8, 237u8, 151u8, 17u8,
                            125u8, 159u8, 218u8, 92u8, 57u8, 238u8, 64u8, 147u8, 40u8, 72u8, 157u8,
                            116u8, 37u8, 195u8, 156u8, 27u8, 123u8, 173u8, 178u8, 102u8, 136u8,
                            6u8,
                        ],
                    )
                }
                #[doc = " Execute the given block."]
                pub fn execute_block(
                    &self,
                    block : runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > >,
                ) -> ::subxt::runtime_api::Payload<types::ExecuteBlock, ()> {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "execute_block",
                        types::ExecuteBlock { block },
                        [
                            133u8, 135u8, 228u8, 65u8, 106u8, 27u8, 85u8, 158u8, 112u8, 254u8,
                            93u8, 26u8, 102u8, 201u8, 118u8, 216u8, 249u8, 247u8, 91u8, 74u8, 56u8,
                            208u8, 231u8, 115u8, 131u8, 29u8, 209u8, 6u8, 65u8, 57u8, 214u8, 125u8,
                        ],
                    )
                }
                #[doc = " Initialize a block with the given header."]
                pub fn initialize_block(
                    &self,
                    header: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                ) -> ::subxt::runtime_api::Payload<types::InitializeBlock, ()> {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "initialize_block",
                        types::InitializeBlock { header },
                        [
                            146u8, 138u8, 72u8, 240u8, 63u8, 96u8, 110u8, 189u8, 77u8, 92u8, 96u8,
                            232u8, 41u8, 217u8, 105u8, 148u8, 83u8, 190u8, 152u8, 219u8, 19u8,
                            87u8, 163u8, 1u8, 232u8, 25u8, 221u8, 74u8, 224u8, 67u8, 223u8, 34u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Version {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExecuteBlock { pub block : runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > > , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InitializeBlock {
                    pub header: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                }
            }
        }
        pub mod metadata {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Metadata` api trait that returns metadata for the runtime."]
            pub struct Metadata;
            impl Metadata {
                #[doc = " Returns the metadata of a runtime."]
                pub fn metadata(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Metadata,
                    runtime_types::sp_core::OpaqueMetadata,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata",
                        types::Metadata {},
                        [
                            231u8, 24u8, 67u8, 152u8, 23u8, 26u8, 188u8, 82u8, 229u8, 6u8, 185u8,
                            27u8, 175u8, 68u8, 83u8, 122u8, 69u8, 89u8, 185u8, 74u8, 248u8, 87u8,
                            217u8, 124u8, 193u8, 252u8, 199u8, 186u8, 196u8, 179u8, 179u8, 96u8,
                        ],
                    )
                }
                #[doc = " Returns the metadata at a given version."]
                #[doc = ""]
                #[doc = " If the given `version` isn't supported, this will return `None`."]
                #[doc = " Use [`Self::metadata_versions`] to find out about supported metadata version of the runtime."]
                pub fn metadata_at_version(
                    &self,
                    version: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::MetadataAtVersion,
                    ::core::option::Option<runtime_types::sp_core::OpaqueMetadata>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata_at_version",
                        types::MetadataAtVersion { version },
                        [
                            131u8, 53u8, 212u8, 234u8, 16u8, 25u8, 120u8, 252u8, 153u8, 153u8,
                            216u8, 28u8, 54u8, 113u8, 52u8, 236u8, 146u8, 68u8, 142u8, 8u8, 10u8,
                            169u8, 131u8, 142u8, 204u8, 38u8, 48u8, 108u8, 134u8, 86u8, 226u8,
                            61u8,
                        ],
                    )
                }
                #[doc = " Returns the supported metadata versions."]
                #[doc = ""]
                #[doc = " This can be used to call `metadata_at_version`."]
                pub fn metadata_versions(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::MetadataVersions,
                    ::std::vec::Vec<::core::primitive::u32>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata_versions",
                        types::MetadataVersions {},
                        [
                            23u8, 144u8, 137u8, 91u8, 188u8, 39u8, 231u8, 208u8, 252u8, 218u8,
                            224u8, 176u8, 77u8, 32u8, 130u8, 212u8, 223u8, 76u8, 100u8, 190u8,
                            82u8, 94u8, 190u8, 8u8, 82u8, 244u8, 225u8, 179u8, 85u8, 176u8, 56u8,
                            16u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Metadata {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MetadataAtVersion {
                    pub version: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MetadataVersions {}
            }
        }
        pub mod block_builder {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `BlockBuilder` api trait that provides the required functionality for building a block."]
            pub struct BlockBuilder;
            impl BlockBuilder {
                #[doc = " Apply the given extrinsic."]
                #[doc = ""]
                #[doc = " Returns an inclusion outcome which specifies if this extrinsic is included in"]
                #[doc = " this block or not."]
                pub fn apply_extrinsic(
                    &self,
                    extrinsic : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) >,
                ) -> ::subxt::runtime_api::Payload<
                    types::ApplyExtrinsic,
                    ::core::result::Result<
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                        runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "apply_extrinsic",
                        types::ApplyExtrinsic { extrinsic },
                        [
                            72u8, 54u8, 139u8, 3u8, 118u8, 136u8, 65u8, 47u8, 6u8, 105u8, 125u8,
                            223u8, 160u8, 29u8, 103u8, 74u8, 79u8, 149u8, 48u8, 90u8, 237u8, 2u8,
                            97u8, 201u8, 123u8, 34u8, 167u8, 37u8, 187u8, 35u8, 176u8, 97u8,
                        ],
                    )
                }
                #[doc = " Finish the current block."]
                pub fn finalize_block(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::FinalizeBlock,
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "finalize_block",
                        types::FinalizeBlock {},
                        [
                            244u8, 207u8, 24u8, 33u8, 13u8, 69u8, 9u8, 249u8, 145u8, 143u8, 122u8,
                            96u8, 197u8, 55u8, 64u8, 111u8, 238u8, 224u8, 34u8, 201u8, 27u8, 146u8,
                            232u8, 99u8, 191u8, 30u8, 114u8, 16u8, 32u8, 220u8, 58u8, 62u8,
                        ],
                    )
                }
                #[doc = " Generate inherent extrinsics. The inherent data will vary from chain to chain."]                pub fn inherent_extrinsics (& self , inherent : runtime_types :: sp_inherents :: InherentData ,) -> :: subxt :: runtime_api :: Payload < types :: InherentExtrinsics , :: std :: vec :: Vec < runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > > >{
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "inherent_extrinsics",
                        types::InherentExtrinsics { inherent },
                        [
                            254u8, 110u8, 245u8, 201u8, 250u8, 192u8, 27u8, 228u8, 151u8, 213u8,
                            166u8, 89u8, 94u8, 81u8, 189u8, 234u8, 64u8, 18u8, 245u8, 80u8, 29u8,
                            18u8, 140u8, 129u8, 113u8, 236u8, 135u8, 55u8, 79u8, 159u8, 175u8,
                            183u8,
                        ],
                    )
                }
                #[doc = " Check that the inherents are valid. The inherent data will vary from chain to chain."]
                pub fn check_inherents(
                    &self,
                    block : runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > >,
                    data: runtime_types::sp_inherents::InherentData,
                ) -> ::subxt::runtime_api::Payload<
                    types::CheckInherents,
                    runtime_types::sp_inherents::CheckInherentsResult,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "check_inherents",
                        types::CheckInherents { block, data },
                        [
                            153u8, 134u8, 1u8, 215u8, 139u8, 11u8, 53u8, 51u8, 210u8, 175u8, 197u8,
                            28u8, 38u8, 209u8, 175u8, 247u8, 142u8, 157u8, 50u8, 151u8, 164u8,
                            191u8, 181u8, 118u8, 80u8, 97u8, 160u8, 248u8, 110u8, 217u8, 181u8,
                            234u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ApplyExtrinsic { pub extrinsic : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FinalizeBlock {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InherentExtrinsics {
                    pub inherent: runtime_types::sp_inherents::InherentData,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckInherents { pub block : runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > > , pub data : runtime_types :: sp_inherents :: InherentData , }
            }
        }
        pub mod nomination_pools_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " Runtime api for accessing information about nomination pools."]
            pub struct NominationPoolsApi;
            impl NominationPoolsApi {
                #[doc = " Returns the pending rewards for the member that the AccountId was given for."]
                pub fn pending_rewards(
                    &self,
                    who: ::subxt::utils::AccountId32,
                ) -> ::subxt::runtime_api::Payload<types::PendingRewards, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "NominationPoolsApi",
                        "pending_rewards",
                        types::PendingRewards { who },
                        [
                            78u8, 79u8, 88u8, 196u8, 232u8, 243u8, 82u8, 234u8, 115u8, 130u8,
                            124u8, 165u8, 217u8, 64u8, 17u8, 48u8, 245u8, 181u8, 130u8, 120u8,
                            217u8, 158u8, 146u8, 242u8, 41u8, 206u8, 90u8, 201u8, 244u8, 10u8,
                            137u8, 19u8,
                        ],
                    )
                }
                #[doc = " Returns the equivalent balance of `points` for a given pool."]
                pub fn points_to_balance(
                    &self,
                    pool_id: ::core::primitive::u32,
                    points: ::core::primitive::u128,
                ) -> ::subxt::runtime_api::Payload<types::PointsToBalance, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "NominationPoolsApi",
                        "points_to_balance",
                        types::PointsToBalance { pool_id, points },
                        [
                            106u8, 191u8, 150u8, 40u8, 231u8, 8u8, 82u8, 104u8, 109u8, 105u8, 94u8,
                            109u8, 38u8, 165u8, 199u8, 81u8, 37u8, 181u8, 115u8, 106u8, 52u8,
                            192u8, 56u8, 255u8, 145u8, 204u8, 12u8, 241u8, 120u8, 20u8, 188u8,
                            12u8,
                        ],
                    )
                }
                #[doc = " Returns the equivalent points of `new_funds` for a given pool."]
                pub fn balance_to_points(
                    &self,
                    pool_id: ::core::primitive::u32,
                    new_funds: ::core::primitive::u128,
                ) -> ::subxt::runtime_api::Payload<types::BalanceToPoints, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "NominationPoolsApi",
                        "balance_to_points",
                        types::BalanceToPoints { pool_id, new_funds },
                        [
                            5u8, 213u8, 46u8, 194u8, 117u8, 119u8, 10u8, 139u8, 191u8, 76u8, 59u8,
                            81u8, 159u8, 38u8, 144u8, 176u8, 63u8, 138u8, 233u8, 138u8, 236u8,
                            208u8, 113u8, 230u8, 131u8, 75u8, 67u8, 204u8, 160u8, 100u8, 198u8,
                            174u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PendingRewards {
                    pub who: ::subxt::utils::AccountId32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PointsToBalance {
                    pub pool_id: ::core::primitive::u32,
                    pub points: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BalanceToPoints {
                    pub pool_id: ::core::primitive::u32,
                    pub new_funds: ::core::primitive::u128,
                }
            }
        }
        pub mod staking_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct StakingApi;
            impl StakingApi {
                #[doc = " Returns the nominations quota for a nominator with a given balance."]
                pub fn nominations_quota(
                    &self,
                    balance: ::core::primitive::u128,
                ) -> ::subxt::runtime_api::Payload<types::NominationsQuota, ::core::primitive::u32>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "StakingApi",
                        "nominations_quota",
                        types::NominationsQuota { balance },
                        [
                            221u8, 113u8, 50u8, 150u8, 51u8, 181u8, 158u8, 235u8, 25u8, 160u8,
                            135u8, 47u8, 196u8, 129u8, 90u8, 137u8, 157u8, 167u8, 212u8, 104u8,
                            33u8, 48u8, 83u8, 106u8, 84u8, 220u8, 62u8, 85u8, 25u8, 151u8, 189u8,
                            114u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct NominationsQuota {
                    pub balance: ::core::primitive::u128,
                }
            }
        }
        pub mod tagged_transaction_queue {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `TaggedTransactionQueue` api trait for interfering with the transaction queue."]
            pub struct TaggedTransactionQueue;
            impl TaggedTransactionQueue {
                #[doc = " Validate the transaction."]
                #[doc = ""]
                #[doc = " This method is invoked by the transaction pool to learn details about given transaction."]
                #[doc = " The implementation should make sure to verify the correctness of the transaction"]
                #[doc = " against current state. The given `block_hash` corresponds to the hash of the block"]
                #[doc = " that is used as current state."]
                #[doc = ""]
                #[doc = " Note that this call may be performed by the pool multiple times and transactions"]
                #[doc = " might be verified in any possible order."]
                pub fn validate_transaction(
                    &self,
                    source: runtime_types::sp_runtime::transaction_validity::TransactionSource,
                    tx : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) >,
                    block_hash: ::subxt::utils::H256,
                ) -> ::subxt::runtime_api::Payload<
                    types::ValidateTransaction,
                    ::core::result::Result<
                        runtime_types::sp_runtime::transaction_validity::ValidTransaction,
                        runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TaggedTransactionQueue",
                        "validate_transaction",
                        types::ValidateTransaction {
                            source,
                            tx,
                            block_hash,
                        },
                        [
                            196u8, 50u8, 90u8, 49u8, 109u8, 251u8, 200u8, 35u8, 23u8, 150u8, 140u8,
                            143u8, 232u8, 164u8, 133u8, 89u8, 32u8, 240u8, 115u8, 39u8, 95u8, 70u8,
                            162u8, 76u8, 122u8, 73u8, 151u8, 144u8, 234u8, 120u8, 100u8, 29u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidateTransaction { pub source : runtime_types :: sp_runtime :: transaction_validity :: TransactionSource , pub tx : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > , pub block_hash : :: subxt :: utils :: H256 , }
            }
        }
        pub mod offchain_worker_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The offchain worker api."]
            pub struct OffchainWorkerApi;
            impl OffchainWorkerApi {
                #[doc = " Starts the off-chain task for given block header."]
                pub fn offchain_worker(
                    &self,
                    header: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                ) -> ::subxt::runtime_api::Payload<types::OffchainWorker, ()> {
                    ::subxt::runtime_api::Payload::new_static(
                        "OffchainWorkerApi",
                        "offchain_worker",
                        types::OffchainWorker { header },
                        [
                            10u8, 135u8, 19u8, 153u8, 33u8, 216u8, 18u8, 242u8, 33u8, 140u8, 4u8,
                            223u8, 200u8, 130u8, 103u8, 118u8, 137u8, 24u8, 19u8, 127u8, 161u8,
                            29u8, 184u8, 111u8, 222u8, 111u8, 253u8, 73u8, 45u8, 31u8, 79u8, 60u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct OffchainWorker {
                    pub header: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                }
            }
        }
        pub mod parachain_host {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The API for querying the state of parachains on-chain."]
            pub struct ParachainHost;
            impl ParachainHost {
                #[doc = " Get the current validators."]
                pub fn validators(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Validators,
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::validator_app::Public>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "validators",
                        types::Validators {},
                        [
                            56u8, 64u8, 189u8, 234u8, 85u8, 75u8, 2u8, 212u8, 192u8, 95u8, 230u8,
                            201u8, 98u8, 220u8, 78u8, 20u8, 101u8, 16u8, 153u8, 192u8, 133u8,
                            179u8, 217u8, 98u8, 247u8, 143u8, 104u8, 147u8, 47u8, 255u8, 111u8,
                            72u8,
                        ],
                    )
                }
                #[doc = " Returns the validator groups and rotation info localized based on the hypothetical child"]
                #[doc = "  of a block whose state  this is invoked on. Note that `now` in the `GroupRotationInfo`"]
                #[doc = " should be the successor of the number of the block."]
                pub fn validator_groups(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::ValidatorGroups,
                    (
                        ::std::vec::Vec<
                            ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
                        >,
                        runtime_types::polkadot_primitives::v5::GroupRotationInfo<
                            ::core::primitive::u32,
                        >,
                    ),
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "validator_groups",
                        types::ValidatorGroups {},
                        [
                            89u8, 221u8, 163u8, 73u8, 194u8, 196u8, 136u8, 242u8, 249u8, 182u8,
                            239u8, 251u8, 157u8, 211u8, 41u8, 58u8, 242u8, 242u8, 177u8, 145u8,
                            107u8, 167u8, 193u8, 204u8, 226u8, 228u8, 82u8, 249u8, 187u8, 211u8,
                            37u8, 124u8,
                        ],
                    )
                }
                #[doc = " Yields information on all availability cores as relevant to the child block."]
                #[doc = " Cores are either free or occupied. Free cores can have paras assigned to them."]
                pub fn availability_cores(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::AvailabilityCores,
                    ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::CoreState<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "availability_cores",
                        types::AvailabilityCores {},
                        [
                            238u8, 20u8, 188u8, 206u8, 26u8, 17u8, 72u8, 123u8, 33u8, 54u8, 66u8,
                            13u8, 244u8, 246u8, 228u8, 177u8, 176u8, 251u8, 82u8, 12u8, 170u8,
                            29u8, 39u8, 158u8, 16u8, 23u8, 253u8, 169u8, 117u8, 12u8, 0u8, 65u8,
                        ],
                    )
                }
                #[doc = " Yields the persisted validation data for the given `ParaId` along with an assumption that"]
                #[doc = " should be used if the para currently occupies a core."]
                #[doc = ""]
                #[doc = " Returns `None` if either the para is not registered or the assumption is `Freed`"]
                #[doc = " and the para already occupies a core."]
                pub fn persisted_validation_data(
                    &self,
                    para_id: runtime_types::polkadot_parachain::primitives::Id,
                    assumption: runtime_types::polkadot_primitives::v5::OccupiedCoreAssumption,
                ) -> ::subxt::runtime_api::Payload<
                    types::PersistedValidationData,
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v5::PersistedValidationData<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "persisted_validation_data",
                        types::PersistedValidationData {
                            para_id,
                            assumption,
                        },
                        [
                            119u8, 217u8, 57u8, 241u8, 70u8, 56u8, 102u8, 20u8, 98u8, 60u8, 47u8,
                            78u8, 124u8, 81u8, 158u8, 254u8, 30u8, 14u8, 223u8, 195u8, 95u8, 179u8,
                            228u8, 53u8, 149u8, 224u8, 62u8, 8u8, 27u8, 3u8, 100u8, 37u8,
                        ],
                    )
                }
                #[doc = " Returns the persisted validation data for the given `ParaId` along with the corresponding"]
                #[doc = " validation code hash. Instead of accepting assumption about the para, matches the validation"]
                #[doc = " data hash against an expected one and yields `None` if they're not equal."]
                pub fn assumed_validation_data(
                    &self,
                    para_id: runtime_types::polkadot_parachain::primitives::Id,
                    expected_persisted_validation_data_hash: ::subxt::utils::H256,
                ) -> ::subxt::runtime_api::Payload<
                    types::AssumedValidationData,
                    ::core::option::Option<(
                        runtime_types::polkadot_primitives::v5::PersistedValidationData<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    )>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "assumed_validation_data",
                        types::AssumedValidationData {
                            para_id,
                            expected_persisted_validation_data_hash,
                        },
                        [
                            37u8, 162u8, 100u8, 72u8, 19u8, 135u8, 13u8, 211u8, 51u8, 153u8, 201u8,
                            97u8, 61u8, 193u8, 167u8, 118u8, 60u8, 242u8, 228u8, 81u8, 165u8, 62u8,
                            191u8, 206u8, 157u8, 232u8, 62u8, 55u8, 240u8, 236u8, 76u8, 204u8,
                        ],
                    )
                }
                #[doc = " Checks if the given validation outputs pass the acceptance criteria."]
                pub fn check_validation_outputs(
                    &self,
                    para_id: runtime_types::polkadot_parachain::primitives::Id,
                    outputs: runtime_types::polkadot_primitives::v5::CandidateCommitments<
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::runtime_api::Payload<
                    types::CheckValidationOutputs,
                    ::core::primitive::bool,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "check_validation_outputs",
                        types::CheckValidationOutputs { para_id, outputs },
                        [
                            128u8, 33u8, 213u8, 120u8, 39u8, 18u8, 135u8, 248u8, 196u8, 43u8, 0u8,
                            143u8, 198u8, 64u8, 93u8, 133u8, 248u8, 206u8, 103u8, 137u8, 168u8,
                            255u8, 144u8, 29u8, 121u8, 246u8, 179u8, 187u8, 83u8, 53u8, 142u8,
                            82u8,
                        ],
                    )
                }
                #[doc = " Returns the session index expected at a child of the block."]
                #[doc = ""]
                #[doc = " This can be used to instantiate a `SigningContext`."]
                pub fn session_index_for_child(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::SessionIndexForChild,
                    ::core::primitive::u32,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "session_index_for_child",
                        types::SessionIndexForChild {},
                        [
                            135u8, 9u8, 1u8, 244u8, 174u8, 151u8, 247u8, 75u8, 226u8, 216u8, 53u8,
                            78u8, 26u8, 109u8, 44u8, 77u8, 208u8, 151u8, 94u8, 212u8, 115u8, 43u8,
                            118u8, 22u8, 140u8, 117u8, 15u8, 224u8, 163u8, 252u8, 90u8, 255u8,
                        ],
                    )
                }
                #[doc = " Fetch the validation code used by a para, making the given `OccupiedCoreAssumption`."]
                #[doc = ""]
                #[doc = " Returns `None` if either the para is not registered or the assumption is `Freed`"]
                #[doc = " and the para already occupies a core."]
                pub fn validation_code(
                    &self,
                    para_id: runtime_types::polkadot_parachain::primitives::Id,
                    assumption: runtime_types::polkadot_primitives::v5::OccupiedCoreAssumption,
                ) -> ::subxt::runtime_api::Payload<
                    types::ValidationCode,
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "validation_code",
                        types::ValidationCode {
                            para_id,
                            assumption,
                        },
                        [
                            231u8, 15u8, 35u8, 159u8, 96u8, 23u8, 246u8, 125u8, 78u8, 79u8, 158u8,
                            116u8, 36u8, 199u8, 53u8, 61u8, 242u8, 136u8, 227u8, 174u8, 136u8,
                            71u8, 143u8, 47u8, 216u8, 21u8, 225u8, 117u8, 50u8, 104u8, 161u8,
                            232u8,
                        ],
                    )
                }
                #[doc = " Get the receipt of a candidate pending availability. This returns `Some` for any paras"]
                #[doc = " assigned to occupied cores in `availability_cores` and `None` otherwise."]
                pub fn candidate_pending_availability(
                    &self,
                    para_id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::runtime_api::Payload<
                    types::CandidatePendingAvailability,
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v5::CommittedCandidateReceipt<
                            ::subxt::utils::H256,
                        >,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "candidate_pending_availability",
                        types::CandidatePendingAvailability { para_id },
                        [
                            139u8, 185u8, 205u8, 255u8, 131u8, 180u8, 248u8, 168u8, 25u8, 124u8,
                            105u8, 141u8, 59u8, 118u8, 109u8, 136u8, 103u8, 200u8, 5u8, 218u8,
                            72u8, 55u8, 114u8, 89u8, 207u8, 140u8, 51u8, 86u8, 167u8, 41u8, 221u8,
                            86u8,
                        ],
                    )
                }
                #[doc = " Get a vector of events concerning candidates that occurred within a block."]
                pub fn candidate_events(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::CandidateEvents,
                    ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::CandidateEvent<
                            ::subxt::utils::H256,
                        >,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "candidate_events",
                        types::CandidateEvents {},
                        [
                            101u8, 145u8, 200u8, 182u8, 213u8, 111u8, 180u8, 73u8, 14u8, 107u8,
                            110u8, 145u8, 122u8, 35u8, 223u8, 219u8, 66u8, 101u8, 130u8, 255u8,
                            44u8, 46u8, 50u8, 61u8, 104u8, 237u8, 34u8, 16u8, 179u8, 214u8, 115u8,
                            7u8,
                        ],
                    )
                }
                #[doc = " Get all the pending inbound messages in the downward message queue for a para."]
                pub fn dmq_contents(
                    &self,
                    recipient: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::runtime_api::Payload<
                    types::DmqContents,
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "dmq_contents",
                        types::DmqContents { recipient },
                        [
                            189u8, 11u8, 38u8, 223u8, 11u8, 108u8, 201u8, 122u8, 207u8, 7u8, 74u8,
                            14u8, 247u8, 226u8, 108u8, 21u8, 213u8, 55u8, 8u8, 137u8, 211u8, 98u8,
                            19u8, 11u8, 212u8, 218u8, 209u8, 63u8, 51u8, 252u8, 86u8, 53u8,
                        ],
                    )
                }
                #[doc = " Get the contents of all channels addressed to the given recipient. Channels that have no"]
                #[doc = " messages in them are also included."]
                pub fn inbound_hrmp_channels_contents(
                    &self,
                    recipient: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::runtime_api::Payload<
                    types::InboundHrmpChannelsContents,
                    ::subxt::utils::KeyedVec<
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::std::vec::Vec<
                            runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                                ::core::primitive::u32,
                            >,
                        >,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "inbound_hrmp_channels_contents",
                        types::InboundHrmpChannelsContents { recipient },
                        [
                            132u8, 29u8, 42u8, 39u8, 72u8, 243u8, 110u8, 43u8, 110u8, 9u8, 21u8,
                            18u8, 91u8, 40u8, 231u8, 223u8, 239u8, 16u8, 110u8, 54u8, 108u8, 234u8,
                            140u8, 205u8, 80u8, 221u8, 115u8, 48u8, 197u8, 248u8, 6u8, 25u8,
                        ],
                    )
                }
                #[doc = " Get the validation code from its hash."]
                pub fn validation_code_by_hash(
                    &self,
                    hash: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                ) -> ::subxt::runtime_api::Payload<
                    types::ValidationCodeByHash,
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "validation_code_by_hash",
                        types::ValidationCodeByHash { hash },
                        [
                            219u8, 250u8, 130u8, 89u8, 178u8, 234u8, 255u8, 33u8, 90u8, 78u8, 58u8,
                            124u8, 141u8, 145u8, 156u8, 81u8, 184u8, 52u8, 65u8, 112u8, 35u8,
                            153u8, 222u8, 23u8, 226u8, 53u8, 164u8, 22u8, 236u8, 103u8, 197u8,
                            236u8,
                        ],
                    )
                }
                #[doc = " Scrape dispute relevant from on-chain, backing votes and resolved disputes."]
                pub fn on_chain_votes(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::OnChainVotes,
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v5::ScrapedOnChainVotes<
                            ::subxt::utils::H256,
                        >,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "on_chain_votes",
                        types::OnChainVotes {},
                        [
                            8u8, 253u8, 248u8, 13u8, 221u8, 83u8, 199u8, 65u8, 180u8, 193u8, 232u8,
                            179u8, 56u8, 186u8, 72u8, 128u8, 27u8, 168u8, 177u8, 82u8, 194u8,
                            139u8, 78u8, 32u8, 147u8, 67u8, 27u8, 252u8, 118u8, 60u8, 74u8, 31u8,
                        ],
                    )
                }
                #[doc = " Get the session info for the given session, if stored."]
                #[doc = ""]
                #[doc = " NOTE: This function is only available since parachain host version 2."]
                pub fn session_info(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::SessionInfo,
                    ::core::option::Option<runtime_types::polkadot_primitives::v5::SessionInfo>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "session_info",
                        types::SessionInfo { index },
                        [
                            77u8, 115u8, 39u8, 190u8, 116u8, 250u8, 66u8, 128u8, 168u8, 24u8,
                            120u8, 153u8, 111u8, 125u8, 249u8, 115u8, 112u8, 169u8, 208u8, 31u8,
                            95u8, 234u8, 14u8, 242u8, 14u8, 190u8, 120u8, 171u8, 202u8, 67u8, 81u8,
                            237u8,
                        ],
                    )
                }
                #[doc = " Submits a PVF pre-checking statement into the transaction pool."]
                #[doc = ""]
                #[doc = " NOTE: This function is only available since parachain host version 2."]
                pub fn submit_pvf_check_statement(
                    &self,
                    stmt: runtime_types::polkadot_primitives::v5::PvfCheckStatement,
                    signature: runtime_types::polkadot_primitives::v5::validator_app::Signature,
                ) -> ::subxt::runtime_api::Payload<types::SubmitPvfCheckStatement, ()>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "submit_pvf_check_statement",
                        types::SubmitPvfCheckStatement { stmt, signature },
                        [
                            91u8, 138u8, 75u8, 79u8, 171u8, 224u8, 206u8, 152u8, 202u8, 131u8,
                            251u8, 200u8, 75u8, 99u8, 49u8, 192u8, 175u8, 212u8, 139u8, 236u8,
                            188u8, 243u8, 82u8, 62u8, 190u8, 79u8, 113u8, 23u8, 222u8, 29u8, 255u8,
                            196u8,
                        ],
                    )
                }
                #[doc = " Returns code hashes of PVFs that require pre-checking by validators in the active set."]
                #[doc = ""]
                #[doc = " NOTE: This function is only available since parachain host version 2."]
                pub fn pvfs_require_precheck(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::PvfsRequirePrecheck,
                    ::std::vec::Vec<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "pvfs_require_precheck",
                        types::PvfsRequirePrecheck {},
                        [
                            251u8, 162u8, 214u8, 223u8, 70u8, 67u8, 170u8, 19u8, 191u8, 37u8,
                            233u8, 249u8, 89u8, 28u8, 76u8, 213u8, 194u8, 28u8, 15u8, 199u8, 167u8,
                            23u8, 139u8, 220u8, 218u8, 223u8, 115u8, 4u8, 95u8, 24u8, 32u8, 29u8,
                        ],
                    )
                }
                #[doc = " Fetch the hash of the validation code used by a para, making the given `OccupiedCoreAssumption`."]
                #[doc = ""]
                #[doc = " NOTE: This function is only available since parachain host version 2."]
                pub fn validation_code_hash(
                    &self,
                    para_id: runtime_types::polkadot_parachain::primitives::Id,
                    assumption: runtime_types::polkadot_primitives::v5::OccupiedCoreAssumption,
                ) -> ::subxt::runtime_api::Payload<
                    types::ValidationCodeHash,
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "validation_code_hash",
                        types::ValidationCodeHash {
                            para_id,
                            assumption,
                        },
                        [
                            226u8, 142u8, 121u8, 182u8, 206u8, 180u8, 8u8, 19u8, 237u8, 84u8,
                            121u8, 1u8, 126u8, 211u8, 241u8, 133u8, 195u8, 182u8, 116u8, 128u8,
                            58u8, 81u8, 12u8, 68u8, 79u8, 212u8, 108u8, 178u8, 237u8, 25u8, 203u8,
                            135u8,
                        ],
                    )
                }
                #[doc = " Returns all onchain disputes."]
                pub fn disputes(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Disputes,
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        runtime_types::polkadot_core_primitives::CandidateHash,
                        runtime_types::polkadot_primitives::v5::DisputeState<
                            ::core::primitive::u32,
                        >,
                    )>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "disputes",
                        types::Disputes {},
                        [
                            183u8, 88u8, 143u8, 44u8, 138u8, 79u8, 65u8, 198u8, 42u8, 109u8, 235u8,
                            152u8, 3u8, 13u8, 106u8, 189u8, 197u8, 126u8, 44u8, 161u8, 67u8, 49u8,
                            163u8, 193u8, 248u8, 207u8, 1u8, 108u8, 188u8, 152u8, 87u8, 125u8,
                        ],
                    )
                }
                #[doc = " Returns execution parameters for the session."]
                pub fn session_executor_params(
                    &self,
                    session_index: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::SessionExecutorParams,
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v5::executor_params::ExecutorParams,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "session_executor_params",
                        types::SessionExecutorParams { session_index },
                        [
                            207u8, 66u8, 10u8, 104u8, 146u8, 219u8, 75u8, 157u8, 93u8, 224u8,
                            215u8, 13u8, 255u8, 62u8, 134u8, 168u8, 185u8, 101u8, 39u8, 78u8, 98u8,
                            44u8, 129u8, 38u8, 48u8, 244u8, 103u8, 205u8, 66u8, 121u8, 18u8, 247u8,
                        ],
                    )
                }
                #[doc = " Returns a list of validators that lost a past session dispute and need to be slashed."]
                #[doc = " NOTE: This function is only available since parachain host version 5."]
                pub fn unapplied_slashes(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::UnappliedSlashes,
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        runtime_types::polkadot_core_primitives::CandidateHash,
                        runtime_types::polkadot_primitives::v5::slashing::PendingSlashes,
                    )>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "unapplied_slashes",
                        types::UnappliedSlashes {},
                        [
                            205u8, 16u8, 246u8, 48u8, 72u8, 160u8, 7u8, 136u8, 225u8, 2u8, 209u8,
                            254u8, 255u8, 115u8, 49u8, 214u8, 131u8, 22u8, 210u8, 9u8, 111u8,
                            170u8, 109u8, 247u8, 110u8, 42u8, 55u8, 68u8, 85u8, 37u8, 250u8, 4u8,
                        ],
                    )
                }
                #[doc = " Returns a merkle proof of a validator session key."]
                #[doc = " NOTE: This function is only available since parachain host version 5."]
                pub fn key_ownership_proof(
                    &self,
                    validator_id: runtime_types::polkadot_primitives::v5::validator_app::Public,
                ) -> ::subxt::runtime_api::Payload<
                    types::KeyOwnershipProof,
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v5::slashing::OpaqueKeyOwnershipProof,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "key_ownership_proof",
                        types::KeyOwnershipProof { validator_id },
                        [
                            194u8, 237u8, 59u8, 4u8, 194u8, 235u8, 38u8, 58u8, 58u8, 221u8, 189u8,
                            69u8, 254u8, 2u8, 242u8, 200u8, 86u8, 4u8, 138u8, 184u8, 198u8, 58u8,
                            200u8, 34u8, 243u8, 91u8, 122u8, 35u8, 18u8, 83u8, 152u8, 191u8,
                        ],
                    )
                }
                #[doc = " Submit an unsigned extrinsic to slash validators who lost a dispute about"]
                #[doc = " a candidate of a past session."]
                #[doc = " NOTE: This function is only available since parachain host version 5."]
                pub fn submit_report_dispute_lost(
                    &self,
                    dispute_proof: runtime_types::polkadot_primitives::v5::slashing::DisputeProof,
                    key_ownership_proof : runtime_types :: polkadot_primitives :: v5 :: slashing :: OpaqueKeyOwnershipProof,
                ) -> ::subxt::runtime_api::Payload<
                    types::SubmitReportDisputeLost,
                    ::core::option::Option<()>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ParachainHost",
                        "submit_report_dispute_lost",
                        types::SubmitReportDisputeLost {
                            dispute_proof,
                            key_ownership_proof,
                        },
                        [
                            98u8, 63u8, 249u8, 13u8, 163u8, 161u8, 43u8, 96u8, 75u8, 65u8, 3u8,
                            116u8, 8u8, 149u8, 122u8, 190u8, 179u8, 108u8, 17u8, 22u8, 59u8, 134u8,
                            43u8, 31u8, 13u8, 254u8, 21u8, 112u8, 129u8, 16u8, 5u8, 180u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Validators {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidatorGroups {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AvailabilityCores {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PersistedValidationData {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub assumption: runtime_types::polkadot_primitives::v5::OccupiedCoreAssumption,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AssumedValidationData {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub expected_persisted_validation_data_hash: ::subxt::utils::H256,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckValidationOutputs {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub outputs: runtime_types::polkadot_primitives::v5::CandidateCommitments<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SessionIndexForChild {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidationCode {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub assumption: runtime_types::polkadot_primitives::v5::OccupiedCoreAssumption,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CandidatePendingAvailability {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CandidateEvents {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DmqContents {
                    pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InboundHrmpChannelsContents {
                    pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidationCodeByHash {
                    pub hash: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct OnChainVotes {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SessionInfo {
                    pub index: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitPvfCheckStatement {
                    pub stmt: runtime_types::polkadot_primitives::v5::PvfCheckStatement,
                    pub signature: runtime_types::polkadot_primitives::v5::validator_app::Signature,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PvfsRequirePrecheck {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidationCodeHash {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub assumption: runtime_types::polkadot_primitives::v5::OccupiedCoreAssumption,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Disputes {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SessionExecutorParams {
                    pub session_index: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UnappliedSlashes {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KeyOwnershipProof {
                    pub validator_id: runtime_types::polkadot_primitives::v5::validator_app::Public,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitReportDisputeLost {
                    pub dispute_proof:
                        runtime_types::polkadot_primitives::v5::slashing::DisputeProof,
                    pub key_ownership_proof:
                        runtime_types::polkadot_primitives::v5::slashing::OpaqueKeyOwnershipProof,
                }
            }
        }
        pub mod beefy_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API necessary for BEEFY voters."]
            pub struct BeefyApi;
            impl BeefyApi {
                #[doc = " Return the block number where BEEFY consensus is enabled/started"]
                pub fn beefy_genesis(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::BeefyGenesis,
                    ::core::option::Option<::core::primitive::u32>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BeefyApi",
                        "beefy_genesis",
                        types::BeefyGenesis {},
                        [
                            246u8, 129u8, 31u8, 77u8, 24u8, 47u8, 5u8, 156u8, 64u8, 222u8, 180u8,
                            78u8, 110u8, 77u8, 218u8, 149u8, 210u8, 151u8, 164u8, 220u8, 165u8,
                            119u8, 116u8, 220u8, 20u8, 122u8, 37u8, 176u8, 75u8, 218u8, 194u8,
                            244u8,
                        ],
                    )
                }
                #[doc = " Return the current active BEEFY validator set"]
                pub fn validator_set(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::ValidatorSet,
                    ::core::option::Option<
                        runtime_types::sp_consensus_beefy::ValidatorSet<
                            runtime_types::sp_consensus_beefy::crypto::Public,
                        >,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BeefyApi",
                        "validator_set",
                        types::ValidatorSet {},
                        [
                            26u8, 174u8, 151u8, 215u8, 199u8, 11u8, 123u8, 18u8, 209u8, 187u8,
                            70u8, 245u8, 59u8, 23u8, 11u8, 26u8, 167u8, 202u8, 83u8, 213u8, 99u8,
                            74u8, 143u8, 140u8, 34u8, 9u8, 225u8, 217u8, 244u8, 169u8, 30u8, 217u8,
                        ],
                    )
                }
                #[doc = " Submits an unsigned extrinsic to report an equivocation. The caller"]
                #[doc = " must provide the equivocation proof and a key ownership proof"]
                #[doc = " (should be obtained using `generate_key_ownership_proof`). The"]
                #[doc = " extrinsic will be unsigned and should only be accepted for local"]
                #[doc = " authorship (not to be broadcast to the network). This method returns"]
                #[doc = " `None` when creation of the extrinsic fails, e.g. if equivocation"]
                #[doc = " reporting is disabled for the given runtime (i.e. this method is"]
                #[doc = " hardcoded to return `None`). Only useful in an offchain context."]
                pub fn submit_report_equivocation_unsigned_extrinsic(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_beefy::EquivocationProof<
                        ::core::primitive::u32,
                        runtime_types::sp_consensus_beefy::crypto::Public,
                        runtime_types::sp_consensus_beefy::crypto::Signature,
                    >,
                    key_owner_proof: runtime_types::sp_consensus_beefy::OpaqueKeyOwnershipProof,
                ) -> ::subxt::runtime_api::Payload<
                    types::SubmitReportEquivocationUnsignedExtrinsic,
                    ::core::option::Option<()>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BeefyApi",
                        "submit_report_equivocation_unsigned_extrinsic",
                        types::SubmitReportEquivocationUnsignedExtrinsic {
                            equivocation_proof,
                            key_owner_proof,
                        },
                        [
                            20u8, 162u8, 43u8, 173u8, 248u8, 140u8, 57u8, 151u8, 189u8, 96u8, 68u8,
                            130u8, 14u8, 162u8, 230u8, 61u8, 169u8, 189u8, 239u8, 71u8, 121u8,
                            137u8, 141u8, 206u8, 91u8, 164u8, 175u8, 93u8, 33u8, 161u8, 166u8,
                            192u8,
                        ],
                    )
                }
                #[doc = " Generates a proof of key ownership for the given authority in the"]
                #[doc = " given set. An example usage of this module is coupled with the"]
                #[doc = " session historical module to prove that a given authority key is"]
                #[doc = " tied to a given staking identity during a specific session. Proofs"]
                #[doc = " of key ownership are necessary for submitting equivocation reports."]
                #[doc = " NOTE: even though the API takes a `set_id` as parameter the current"]
                #[doc = " implementations ignores this parameter and instead relies on this"]
                #[doc = " method being called at the correct block height, i.e. any point at"]
                #[doc = " which the given set id is live on-chain. Future implementations will"]
                #[doc = " instead use indexed data through an offchain worker, not requiring"]
                #[doc = " older states to be available."]
                pub fn generate_key_ownership_proof(
                    &self,
                    set_id: ::core::primitive::u64,
                    authority_id: runtime_types::sp_consensus_beefy::crypto::Public,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateKeyOwnershipProof,
                    ::core::option::Option<
                        runtime_types::sp_consensus_beefy::OpaqueKeyOwnershipProof,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BeefyApi",
                        "generate_key_ownership_proof",
                        types::GenerateKeyOwnershipProof {
                            set_id,
                            authority_id,
                        },
                        [
                            244u8, 175u8, 3u8, 235u8, 173u8, 34u8, 210u8, 81u8, 41u8, 5u8, 85u8,
                            179u8, 53u8, 153u8, 16u8, 62u8, 103u8, 71u8, 180u8, 11u8, 165u8, 90u8,
                            186u8, 156u8, 118u8, 114u8, 22u8, 108u8, 149u8, 9u8, 232u8, 174u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BeefyGenesis {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidatorSet {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitReportEquivocationUnsignedExtrinsic {
                    pub equivocation_proof: runtime_types::sp_consensus_beefy::EquivocationProof<
                        ::core::primitive::u32,
                        runtime_types::sp_consensus_beefy::crypto::Public,
                        runtime_types::sp_consensus_beefy::crypto::Signature,
                    >,
                    pub key_owner_proof: runtime_types::sp_consensus_beefy::OpaqueKeyOwnershipProof,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateKeyOwnershipProof {
                    pub set_id: ::core::primitive::u64,
                    pub authority_id: runtime_types::sp_consensus_beefy::crypto::Public,
                }
            }
        }
        pub mod mmr_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API to interact with MMR pallet."]
            pub struct MmrApi;
            impl MmrApi {
                #[doc = " Return the on-chain MMR root hash."]
                pub fn mmr_root(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::MmrRoot,
                    ::core::result::Result<
                        ::subxt::utils::H256,
                        runtime_types::sp_mmr_primitives::Error,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "MmrApi",
                        "mmr_root",
                        types::MmrRoot {},
                        [
                            148u8, 252u8, 77u8, 233u8, 236u8, 8u8, 119u8, 105u8, 207u8, 161u8,
                            109u8, 158u8, 211u8, 64u8, 67u8, 216u8, 242u8, 52u8, 122u8, 4u8, 83u8,
                            113u8, 54u8, 77u8, 165u8, 89u8, 61u8, 159u8, 98u8, 51u8, 45u8, 90u8,
                        ],
                    )
                }
                #[doc = " Return the number of MMR blocks in the chain."]
                pub fn mmr_leaf_count(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::MmrLeafCount,
                    ::core::result::Result<
                        ::core::primitive::u64,
                        runtime_types::sp_mmr_primitives::Error,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "MmrApi",
                        "mmr_leaf_count",
                        types::MmrLeafCount {},
                        [
                            165u8, 141u8, 127u8, 184u8, 27u8, 185u8, 251u8, 25u8, 44u8, 93u8,
                            239u8, 158u8, 104u8, 91u8, 22u8, 87u8, 101u8, 166u8, 90u8, 90u8, 45u8,
                            105u8, 254u8, 136u8, 233u8, 121u8, 9u8, 216u8, 179u8, 55u8, 126u8,
                            158u8,
                        ],
                    )
                }
                #[doc = " Generate MMR proof for a series of block numbers. If `best_known_block_number = Some(n)`,"]
                #[doc = " use historical MMR state at given block height `n`. Else, use current MMR state."]
                pub fn generate_proof(
                    &self,
                    block_numbers: ::std::vec::Vec<::core::primitive::u32>,
                    best_known_block_number: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateProof,
                    ::core::result::Result<
                        (
                            ::std::vec::Vec<runtime_types::sp_mmr_primitives::EncodableOpaqueLeaf>,
                            runtime_types::sp_mmr_primitives::Proof<::subxt::utils::H256>,
                        ),
                        runtime_types::sp_mmr_primitives::Error,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "MmrApi",
                        "generate_proof",
                        types::GenerateProof {
                            block_numbers,
                            best_known_block_number,
                        },
                        [
                            187u8, 175u8, 153u8, 82u8, 245u8, 180u8, 126u8, 156u8, 67u8, 89u8,
                            253u8, 29u8, 54u8, 168u8, 196u8, 144u8, 24u8, 123u8, 154u8, 69u8,
                            245u8, 90u8, 110u8, 239u8, 15u8, 125u8, 204u8, 148u8, 71u8, 209u8,
                            58u8, 32u8,
                        ],
                    )
                }
                #[doc = " Verify MMR proof against on-chain MMR for a batch of leaves."]
                #[doc = ""]
                #[doc = " Note this function will use on-chain MMR root hash and check if the proof matches the hash."]
                #[doc = " Note, the leaves should be sorted such that corresponding leaves and leaf indices have the"]
                #[doc = " same position in both the `leaves` vector and the `leaf_indices` vector contained in the [Proof]"]
                pub fn verify_proof(
                    &self,
                    leaves: ::std::vec::Vec<runtime_types::sp_mmr_primitives::EncodableOpaqueLeaf>,
                    proof: runtime_types::sp_mmr_primitives::Proof<::subxt::utils::H256>,
                ) -> ::subxt::runtime_api::Payload<
                    types::VerifyProof,
                    ::core::result::Result<(), runtime_types::sp_mmr_primitives::Error>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "MmrApi",
                        "verify_proof",
                        types::VerifyProof { leaves, proof },
                        [
                            236u8, 54u8, 135u8, 196u8, 161u8, 247u8, 183u8, 78u8, 153u8, 69u8,
                            59u8, 78u8, 62u8, 20u8, 187u8, 47u8, 77u8, 209u8, 209u8, 224u8, 127u8,
                            85u8, 122u8, 33u8, 123u8, 128u8, 92u8, 251u8, 110u8, 233u8, 50u8,
                            160u8,
                        ],
                    )
                }
                #[doc = " Verify MMR proof against given root hash for a batch of leaves."]
                #[doc = ""]
                #[doc = " Note this function does not require any on-chain storage - the"]
                #[doc = " proof is verified against given MMR root hash."]
                #[doc = ""]
                #[doc = " Note, the leaves should be sorted such that corresponding leaves and leaf indices have the"]
                #[doc = " same position in both the `leaves` vector and the `leaf_indices` vector contained in the [Proof]"]
                pub fn verify_proof_stateless(
                    &self,
                    root: ::subxt::utils::H256,
                    leaves: ::std::vec::Vec<runtime_types::sp_mmr_primitives::EncodableOpaqueLeaf>,
                    proof: runtime_types::sp_mmr_primitives::Proof<::subxt::utils::H256>,
                ) -> ::subxt::runtime_api::Payload<
                    types::VerifyProofStateless,
                    ::core::result::Result<(), runtime_types::sp_mmr_primitives::Error>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "MmrApi",
                        "verify_proof_stateless",
                        types::VerifyProofStateless {
                            root,
                            leaves,
                            proof,
                        },
                        [
                            163u8, 232u8, 190u8, 65u8, 135u8, 136u8, 50u8, 60u8, 137u8, 37u8,
                            192u8, 24u8, 137u8, 144u8, 165u8, 131u8, 49u8, 88u8, 15u8, 139u8, 83u8,
                            152u8, 162u8, 148u8, 22u8, 74u8, 82u8, 25u8, 183u8, 83u8, 212u8, 56u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MmrRoot {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MmrLeafCount {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateProof {
                    pub block_numbers: ::std::vec::Vec<::core::primitive::u32>,
                    pub best_known_block_number: ::core::option::Option<::core::primitive::u32>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct VerifyProof {
                    pub leaves:
                        ::std::vec::Vec<runtime_types::sp_mmr_primitives::EncodableOpaqueLeaf>,
                    pub proof: runtime_types::sp_mmr_primitives::Proof<::subxt::utils::H256>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct VerifyProofStateless {
                    pub root: ::subxt::utils::H256,
                    pub leaves:
                        ::std::vec::Vec<runtime_types::sp_mmr_primitives::EncodableOpaqueLeaf>,
                    pub proof: runtime_types::sp_mmr_primitives::Proof<::subxt::utils::H256>,
                }
            }
        }
        pub mod grandpa_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " APIs for integrating the GRANDPA finality gadget into runtimes."]
            #[doc = " This should be implemented on the runtime side."]
            #[doc = ""]
            #[doc = " This is primarily used for negotiating authority-set changes for the"]
            #[doc = " gadget. GRANDPA uses a signaling model of changing authority sets:"]
            #[doc = " changes should be signaled with a delay of N blocks, and then automatically"]
            #[doc = " applied in the runtime after those N blocks have passed."]
            #[doc = ""]
            #[doc = " The consensus protocol will coordinate the handoff externally."]
            pub struct GrandpaApi;
            impl GrandpaApi {
                #[doc = " Get the current GRANDPA authorities and weights. This should not change except"]
                #[doc = " for when changes are scheduled and the corresponding delay has passed."]
                #[doc = ""]
                #[doc = " When called at block B, it will return the set of authorities that should be"]
                #[doc = " used to finalize descendants of this block (B+1, B+2, ...). The block B itself"]
                #[doc = " is finalized by the authorities from block B-1."]
                pub fn grandpa_authorities(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::GrandpaAuthorities,
                    ::std::vec::Vec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "grandpa_authorities",
                        types::GrandpaAuthorities {},
                        [
                            166u8, 76u8, 160u8, 101u8, 242u8, 145u8, 213u8, 10u8, 16u8, 130u8,
                            230u8, 196u8, 125u8, 152u8, 92u8, 143u8, 119u8, 223u8, 140u8, 189u8,
                            203u8, 95u8, 52u8, 105u8, 147u8, 107u8, 135u8, 228u8, 62u8, 178u8,
                            128u8, 33u8,
                        ],
                    )
                }
                #[doc = " Submits an unsigned extrinsic to report an equivocation. The caller"]
                #[doc = " must provide the equivocation proof and a key ownership proof"]
                #[doc = " (should be obtained using `generate_key_ownership_proof`). The"]
                #[doc = " extrinsic will be unsigned and should only be accepted for local"]
                #[doc = " authorship (not to be broadcast to the network). This method returns"]
                #[doc = " `None` when creation of the extrinsic fails, e.g. if equivocation"]
                #[doc = " reporting is disabled for the given runtime (i.e. this method is"]
                #[doc = " hardcoded to return `None`). Only useful in an offchain context."]
                pub fn submit_report_equivocation_unsigned_extrinsic(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                ) -> ::subxt::runtime_api::Payload<
                    types::SubmitReportEquivocationUnsignedExtrinsic,
                    ::core::option::Option<()>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "submit_report_equivocation_unsigned_extrinsic",
                        types::SubmitReportEquivocationUnsignedExtrinsic {
                            equivocation_proof,
                            key_owner_proof,
                        },
                        [
                            112u8, 94u8, 150u8, 250u8, 132u8, 127u8, 185u8, 24u8, 113u8, 62u8,
                            28u8, 171u8, 83u8, 9u8, 41u8, 228u8, 92u8, 137u8, 29u8, 190u8, 214u8,
                            232u8, 100u8, 66u8, 100u8, 168u8, 149u8, 122u8, 93u8, 17u8, 236u8,
                            104u8,
                        ],
                    )
                }
                #[doc = " Generates a proof of key ownership for the given authority in the"]
                #[doc = " given set. An example usage of this module is coupled with the"]
                #[doc = " session historical module to prove that a given authority key is"]
                #[doc = " tied to a given staking identity during a specific session. Proofs"]
                #[doc = " of key ownership are necessary for submitting equivocation reports."]
                #[doc = " NOTE: even though the API takes a `set_id` as parameter the current"]
                #[doc = " implementations ignore this parameter and instead rely on this"]
                #[doc = " method being called at the correct block height, i.e. any point at"]
                #[doc = " which the given set id is live on-chain. Future implementations will"]
                #[doc = " instead use indexed data through an offchain worker, not requiring"]
                #[doc = " older states to be available."]
                pub fn generate_key_ownership_proof(
                    &self,
                    set_id: ::core::primitive::u64,
                    authority_id: runtime_types::sp_consensus_grandpa::app::Public,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateKeyOwnershipProof,
                    ::core::option::Option<
                        runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "generate_key_ownership_proof",
                        types::GenerateKeyOwnershipProof {
                            set_id,
                            authority_id,
                        },
                        [
                            40u8, 126u8, 113u8, 27u8, 245u8, 45u8, 123u8, 138u8, 12u8, 3u8, 125u8,
                            186u8, 151u8, 53u8, 186u8, 93u8, 13u8, 150u8, 163u8, 176u8, 206u8,
                            89u8, 244u8, 127u8, 182u8, 85u8, 203u8, 41u8, 101u8, 183u8, 209u8,
                            179u8,
                        ],
                    )
                }
                #[doc = " Get current GRANDPA authority set id."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::runtime_api::Payload<types::CurrentSetId, ::core::primitive::u64>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "current_set_id",
                        types::CurrentSetId {},
                        [
                            42u8, 230u8, 120u8, 211u8, 156u8, 245u8, 109u8, 86u8, 100u8, 146u8,
                            234u8, 205u8, 41u8, 183u8, 109u8, 42u8, 17u8, 33u8, 156u8, 25u8, 139u8,
                            84u8, 101u8, 75u8, 232u8, 198u8, 87u8, 136u8, 218u8, 233u8, 103u8,
                            156u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GrandpaAuthorities {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitReportEquivocationUnsignedExtrinsic {
                    pub equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    pub key_owner_proof:
                        runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateKeyOwnershipProof {
                    pub set_id: ::core::primitive::u64,
                    pub authority_id: runtime_types::sp_consensus_grandpa::app::Public,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CurrentSetId {}
            }
        }
        pub mod babe_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API necessary for block authorship with BABE."]
            pub struct BabeApi;
            impl BabeApi {
                #[doc = " Return the configuration for BABE."]
                pub fn configuration(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Configuration,
                    runtime_types::sp_consensus_babe::BabeConfiguration,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BabeApi",
                        "configuration",
                        types::Configuration {},
                        [
                            8u8, 81u8, 234u8, 29u8, 30u8, 198u8, 76u8, 19u8, 188u8, 198u8, 127u8,
                            33u8, 141u8, 95u8, 132u8, 106u8, 31u8, 41u8, 215u8, 54u8, 240u8, 65u8,
                            59u8, 160u8, 188u8, 237u8, 10u8, 143u8, 250u8, 79u8, 45u8, 161u8,
                        ],
                    )
                }
                #[doc = " Returns the slot that started the current epoch."]
                pub fn current_epoch_start(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::CurrentEpochStart,
                    runtime_types::sp_consensus_slots::Slot,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BabeApi",
                        "current_epoch_start",
                        types::CurrentEpochStart {},
                        [
                            122u8, 125u8, 246u8, 170u8, 27u8, 50u8, 128u8, 137u8, 228u8, 62u8,
                            145u8, 64u8, 65u8, 119u8, 166u8, 237u8, 115u8, 92u8, 125u8, 124u8,
                            11u8, 33u8, 96u8, 88u8, 88u8, 122u8, 141u8, 137u8, 58u8, 182u8, 148u8,
                            170u8,
                        ],
                    )
                }
                #[doc = " Returns information regarding the current epoch."]
                pub fn current_epoch(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::CurrentEpoch,
                    runtime_types::sp_consensus_babe::Epoch,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BabeApi",
                        "current_epoch",
                        types::CurrentEpoch {},
                        [
                            73u8, 171u8, 149u8, 138u8, 230u8, 95u8, 241u8, 189u8, 207u8, 145u8,
                            103u8, 76u8, 79u8, 44u8, 250u8, 68u8, 238u8, 4u8, 149u8, 234u8, 165u8,
                            91u8, 89u8, 228u8, 132u8, 201u8, 203u8, 98u8, 209u8, 137u8, 8u8, 63u8,
                        ],
                    )
                }
                #[doc = " Returns information regarding the next epoch (which was already"]
                #[doc = " previously announced)."]
                pub fn next_epoch(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::NextEpoch,
                    runtime_types::sp_consensus_babe::Epoch,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BabeApi",
                        "next_epoch",
                        types::NextEpoch {},
                        [
                            191u8, 124u8, 183u8, 209u8, 73u8, 171u8, 164u8, 244u8, 68u8, 239u8,
                            196u8, 54u8, 188u8, 85u8, 229u8, 175u8, 29u8, 89u8, 148u8, 108u8,
                            208u8, 156u8, 62u8, 193u8, 167u8, 184u8, 251u8, 245u8, 123u8, 87u8,
                            19u8, 225u8,
                        ],
                    )
                }
                #[doc = " Generates a proof of key ownership for the given authority in the"]
                #[doc = " current epoch. An example usage of this module is coupled with the"]
                #[doc = " session historical module to prove that a given authority key is"]
                #[doc = " tied to a given staking identity during a specific session. Proofs"]
                #[doc = " of key ownership are necessary for submitting equivocation reports."]
                #[doc = " NOTE: even though the API takes a `slot` as parameter the current"]
                #[doc = " implementations ignores this parameter and instead relies on this"]
                #[doc = " method being called at the correct block height, i.e. any point at"]
                #[doc = " which the epoch for the given slot is live on-chain. Future"]
                #[doc = " implementations will instead use indexed data through an offchain"]
                #[doc = " worker, not requiring older states to be available."]
                pub fn generate_key_ownership_proof(
                    &self,
                    slot: runtime_types::sp_consensus_slots::Slot,
                    authority_id: runtime_types::sp_consensus_babe::app::Public,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateKeyOwnershipProof,
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::OpaqueKeyOwnershipProof,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BabeApi",
                        "generate_key_ownership_proof",
                        types::GenerateKeyOwnershipProof { slot, authority_id },
                        [
                            235u8, 220u8, 75u8, 20u8, 175u8, 246u8, 127u8, 176u8, 225u8, 25u8,
                            240u8, 252u8, 58u8, 254u8, 153u8, 133u8, 197u8, 168u8, 19u8, 231u8,
                            234u8, 173u8, 58u8, 152u8, 212u8, 123u8, 13u8, 131u8, 84u8, 221u8,
                            98u8, 46u8,
                        ],
                    )
                }
                #[doc = " Submits an unsigned extrinsic to report an equivocation. The caller"]
                #[doc = " must provide the equivocation proof and a key ownership proof"]
                #[doc = " (should be obtained using `generate_key_ownership_proof`). The"]
                #[doc = " extrinsic will be unsigned and should only be accepted for local"]
                #[doc = " authorship (not to be broadcast to the network). This method returns"]
                #[doc = " `None` when creation of the extrinsic fails, e.g. if equivocation"]
                #[doc = " reporting is disabled for the given runtime (i.e. this method is"]
                #[doc = " hardcoded to return `None`). Only useful in an offchain context."]
                pub fn submit_report_equivocation_unsigned_extrinsic(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_consensus_babe::OpaqueKeyOwnershipProof,
                ) -> ::subxt::runtime_api::Payload<
                    types::SubmitReportEquivocationUnsignedExtrinsic,
                    ::core::option::Option<()>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BabeApi",
                        "submit_report_equivocation_unsigned_extrinsic",
                        types::SubmitReportEquivocationUnsignedExtrinsic {
                            equivocation_proof,
                            key_owner_proof,
                        },
                        [
                            9u8, 163u8, 149u8, 31u8, 89u8, 32u8, 224u8, 116u8, 102u8, 46u8, 10u8,
                            189u8, 35u8, 166u8, 111u8, 156u8, 204u8, 80u8, 35u8, 64u8, 223u8, 3u8,
                            4u8, 0u8, 97u8, 118u8, 124u8, 142u8, 224u8, 160u8, 2u8, 50u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Configuration {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CurrentEpochStart {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CurrentEpoch {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct NextEpoch {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateKeyOwnershipProof {
                    pub slot: runtime_types::sp_consensus_slots::Slot,
                    pub authority_id: runtime_types::sp_consensus_babe::app::Public,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitReportEquivocationUnsignedExtrinsic {
                    pub equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    pub key_owner_proof: runtime_types::sp_consensus_babe::OpaqueKeyOwnershipProof,
                }
            }
        }
        pub mod authority_discovery_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The authority discovery api."]
            #[doc = ""]
            #[doc = " This api is used by the `client/authority-discovery` module to retrieve identifiers"]
            #[doc = " of the current and next authority set."]
            pub struct AuthorityDiscoveryApi;
            impl AuthorityDiscoveryApi {
                #[doc = " Retrieve authority identifiers of the current and next authority set."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Authorities,
                    ::std::vec::Vec<runtime_types::sp_authority_discovery::app::Public>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "AuthorityDiscoveryApi",
                        "authorities",
                        types::Authorities {},
                        [
                            231u8, 109u8, 175u8, 33u8, 103u8, 6u8, 157u8, 241u8, 62u8, 92u8, 246u8,
                            9u8, 109u8, 137u8, 233u8, 96u8, 103u8, 59u8, 201u8, 132u8, 102u8, 32u8,
                            19u8, 183u8, 106u8, 146u8, 41u8, 172u8, 147u8, 55u8, 156u8, 77u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Authorities {}
            }
        }
        pub mod session_keys {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " Session keys runtime api."]
            pub struct SessionKeys;
            impl SessionKeys {
                #[doc = " Generate a set of session keys with optionally using the given seed."]
                #[doc = " The keys should be stored within the keystore exposed via runtime"]
                #[doc = " externalities."]
                #[doc = ""]
                #[doc = " The seed needs to be a valid `utf8` string."]
                #[doc = ""]
                #[doc = " Returns the concatenated SCALE encoded public keys."]
                pub fn generate_session_keys(
                    &self,
                    seed: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateSessionKeys,
                    ::std::vec::Vec<::core::primitive::u8>,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "SessionKeys",
                        "generate_session_keys",
                        types::GenerateSessionKeys { seed },
                        [
                            96u8, 171u8, 164u8, 166u8, 175u8, 102u8, 101u8, 47u8, 133u8, 95u8,
                            102u8, 202u8, 83u8, 26u8, 238u8, 47u8, 126u8, 132u8, 22u8, 11u8, 33u8,
                            190u8, 175u8, 94u8, 58u8, 245u8, 46u8, 80u8, 195u8, 184u8, 107u8, 65u8,
                        ],
                    )
                }
                #[doc = " Decode the given public session keys."]
                #[doc = ""]
                #[doc = " Returns the list of public raw public keys + key type."]
                pub fn decode_session_keys(
                    &self,
                    encoded: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::runtime_api::Payload<
                    types::DecodeSessionKeys,
                    ::core::option::Option<
                        ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            runtime_types::sp_core::crypto::KeyTypeId,
                        )>,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "SessionKeys",
                        "decode_session_keys",
                        types::DecodeSessionKeys { encoded },
                        [
                            57u8, 242u8, 18u8, 51u8, 132u8, 110u8, 238u8, 255u8, 39u8, 194u8, 8u8,
                            54u8, 198u8, 178u8, 75u8, 151u8, 148u8, 176u8, 144u8, 197u8, 87u8,
                            29u8, 179u8, 235u8, 176u8, 78u8, 252u8, 103u8, 72u8, 203u8, 151u8,
                            248u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateSessionKeys {
                    pub seed: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DecodeSessionKeys {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
        }
        pub mod account_nonce_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The API to query account nonce (aka transaction index)."]
            pub struct AccountNonceApi;
            impl AccountNonceApi {
                #[doc = " Get current account nonce of given `AccountId`."]
                pub fn account_nonce(
                    &self,
                    account: ::subxt::utils::AccountId32,
                ) -> ::subxt::runtime_api::Payload<types::AccountNonce, ::core::primitive::u32>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "AccountNonceApi",
                        "account_nonce",
                        types::AccountNonce { account },
                        [
                            231u8, 82u8, 7u8, 227u8, 131u8, 2u8, 215u8, 252u8, 173u8, 82u8, 11u8,
                            103u8, 200u8, 25u8, 114u8, 116u8, 79u8, 229u8, 152u8, 150u8, 236u8,
                            37u8, 101u8, 26u8, 220u8, 146u8, 182u8, 101u8, 73u8, 55u8, 191u8,
                            171u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountNonce {
                    pub account: ::subxt::utils::AccountId32,
                }
            }
        }
        pub mod transaction_payment_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentApi;
            impl TransactionPaymentApi {
                pub fn query_info(
                    &self,
                    uxt : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) >,
                    len: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryInfo,
                    runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                        ::core::primitive::u128,
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_info",
                        types::QueryInfo { uxt, len },
                        [
                            56u8, 30u8, 174u8, 34u8, 202u8, 24u8, 177u8, 189u8, 145u8, 36u8, 1u8,
                            156u8, 98u8, 209u8, 178u8, 49u8, 198u8, 23u8, 150u8, 173u8, 35u8,
                            205u8, 147u8, 129u8, 42u8, 22u8, 69u8, 3u8, 129u8, 8u8, 196u8, 139u8,
                        ],
                    )
                }
                pub fn query_fee_details(
                    &self,
                    uxt : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) >,
                    len: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryFeeDetails,
                    runtime_types::pallet_transaction_payment::types::FeeDetails<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_fee_details",
                        types::QueryFeeDetails { uxt, len },
                        [
                            117u8, 60u8, 137u8, 159u8, 237u8, 252u8, 216u8, 238u8, 232u8, 1u8,
                            100u8, 152u8, 26u8, 185u8, 145u8, 125u8, 68u8, 189u8, 4u8, 30u8, 125u8,
                            7u8, 196u8, 153u8, 235u8, 51u8, 219u8, 108u8, 185u8, 254u8, 100u8,
                            201u8,
                        ],
                    )
                }
                pub fn query_weight_to_fee(
                    &self,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::runtime_api::Payload<types::QueryWeightToFee, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee { weight },
                        [
                            206u8, 243u8, 189u8, 83u8, 231u8, 244u8, 247u8, 52u8, 126u8, 208u8,
                            224u8, 5u8, 163u8, 108u8, 254u8, 114u8, 214u8, 156u8, 227u8, 217u8,
                            211u8, 198u8, 121u8, 164u8, 110u8, 54u8, 181u8, 146u8, 50u8, 146u8,
                            146u8, 23u8,
                        ],
                    )
                }
                pub fn query_length_to_fee(
                    &self,
                    length: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<types::QueryLengthToFee, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee { length },
                        [
                            92u8, 132u8, 29u8, 119u8, 66u8, 11u8, 196u8, 224u8, 129u8, 23u8, 249u8,
                            12u8, 32u8, 28u8, 92u8, 50u8, 188u8, 101u8, 203u8, 229u8, 248u8, 216u8,
                            130u8, 150u8, 212u8, 161u8, 81u8, 254u8, 116u8, 89u8, 162u8, 48u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryInfo { pub uxt : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > , pub len : :: core :: primitive :: u32 , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryFeeDetails { pub uxt : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: polkadot_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment , runtime_types :: polkadot_runtime_common :: claims :: PrevalidateAttests ,) > , pub len : :: core :: primitive :: u32 , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryWeightToFee {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryLengthToFee {
                    pub length: ::core::primitive::u32,
                }
            }
        }
        pub mod transaction_payment_call_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentCallApi;
            impl TransactionPaymentCallApi {
                #[doc = " Query information of a dispatch class, weight, and fee of a given encoded `Call`."]
                pub fn query_call_info(
                    &self,
                    call: runtime_types::polkadot_runtime::RuntimeCall,
                    len: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryCallInfo,
                    runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                        ::core::primitive::u128,
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_info",
                        types::QueryCallInfo { call, len },
                        [
                            67u8, 240u8, 155u8, 34u8, 151u8, 220u8, 47u8, 169u8, 108u8, 149u8,
                            168u8, 148u8, 213u8, 44u8, 150u8, 128u8, 181u8, 92u8, 47u8, 102u8,
                            254u8, 240u8, 202u8, 124u8, 108u8, 243u8, 57u8, 5u8, 70u8, 144u8,
                            155u8, 0u8,
                        ],
                    )
                }
                #[doc = " Query fee details of a given encoded `Call`."]
                pub fn query_call_fee_details(
                    &self,
                    call: runtime_types::polkadot_runtime::RuntimeCall,
                    len: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryCallFeeDetails,
                    runtime_types::pallet_transaction_payment::types::FeeDetails<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_fee_details",
                        types::QueryCallFeeDetails { call, len },
                        [
                            155u8, 140u8, 34u8, 188u8, 60u8, 79u8, 166u8, 144u8, 138u8, 44u8, 0u8,
                            159u8, 184u8, 8u8, 209u8, 149u8, 191u8, 71u8, 133u8, 181u8, 150u8,
                            240u8, 231u8, 255u8, 226u8, 183u8, 75u8, 230u8, 152u8, 53u8, 218u8,
                            227u8,
                        ],
                    )
                }
                #[doc = " Query the output of the current `WeightToFee` given some input."]
                pub fn query_weight_to_fee(
                    &self,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::runtime_api::Payload<types::QueryWeightToFee, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee { weight },
                        [
                            117u8, 91u8, 94u8, 22u8, 248u8, 212u8, 15u8, 23u8, 97u8, 116u8, 64u8,
                            228u8, 83u8, 123u8, 87u8, 77u8, 97u8, 7u8, 98u8, 181u8, 6u8, 165u8,
                            114u8, 141u8, 164u8, 113u8, 126u8, 88u8, 174u8, 171u8, 224u8, 35u8,
                        ],
                    )
                }
                #[doc = " Query the output of the current `LengthToFee` given some input."]
                pub fn query_length_to_fee(
                    &self,
                    length: ::core::primitive::u32,
                ) -> ::subxt::runtime_api::Payload<types::QueryLengthToFee, ::core::primitive::u128>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee { length },
                        [
                            246u8, 40u8, 4u8, 160u8, 152u8, 94u8, 170u8, 53u8, 205u8, 122u8, 5u8,
                            69u8, 70u8, 25u8, 128u8, 156u8, 119u8, 134u8, 116u8, 147u8, 14u8,
                            164u8, 65u8, 140u8, 86u8, 13u8, 250u8, 218u8, 89u8, 95u8, 234u8, 228u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryCallInfo {
                    pub call: runtime_types::polkadot_runtime::RuntimeCall,
                    pub len: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryCallFeeDetails {
                    pub call: runtime_types::polkadot_runtime::RuntimeCall,
                    pub len: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryWeightToFee {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryLengthToFee {
                    pub length: ::core::primitive::u32,
                }
            }
        }
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn staking(&self) -> staking::constants::ConstantsApi {
            staking::constants::ConstantsApi
        }
        pub fn multisig(&self) -> multisig::constants::ConstantsApi {
            multisig::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn staking(&self) -> staking::storage::StorageApi {
            staking::storage::StorageApi
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi {
            multisig::storage::StorageApi
        }
        pub fn para_inherent(&self) -> para_inherent::storage::StorageApi {
            para_inherent::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn staking(&self) -> staking::calls::TransactionApi {
            staking::calls::TransactionApi
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi {
            multisig::calls::TransactionApi
        }
        pub fn para_inherent(&self) -> para_inherent::calls::TransactionApi {
            para_inherent::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                28u8, 32u8, 141u8, 184u8, 38u8, 159u8, 12u8, 46u8, 103u8, 49u8, 119u8, 222u8,
                158u8, 229u8, 60u8, 243u8, 248u8, 77u8, 46u8, 17u8, 132u8, 118u8, 162u8, 107u8,
                224u8, 128u8, 82u8, 206u8, 22u8, 60u8, 251u8, 96u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the System pallet"]
        pub type Error = runtime_types::frame_system::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Remark {
                    pub remark: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetHeapPages {
                    pub pages: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCode {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCodeWithoutChecks {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetStorage {
                    pub items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillStorage {
                    pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillPrefix {
                    pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                    pub subkeys: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemarkWithEvent {
                    pub remark: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::remark`]."]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::Remark> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_heap_pages`]."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::SetHeapPages> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_code`]."]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetCode> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_code_without_checks`]."]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_storage`]."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::tx::Payload<types::SetStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::kill_storage`]."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::tx::Payload<types::KillStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::kill_prefix`]."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::KillPrefix> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::remark_with_event`]."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::utils::AccountId32,
                pub hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![],
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Number",
                        vec![],
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Digest",
                        vec![],
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::polkadot_runtime::RuntimeEvent,
                            ::subxt::utils::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Events",
                        vec![],
                        [
                            165u8, 186u8, 210u8, 144u8, 130u8, 143u8, 192u8, 146u8, 236u8, 25u8,
                            152u8, 0u8, 65u8, 175u8, 231u8, 214u8, 81u8, 99u8, 12u8, 83u8, 9u8,
                            178u8, 250u8, 217u8, 25u8, 120u8, 23u8, 192u8, 76u8, 140u8, 27u8, 31u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![],
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::Phase,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Set {
                    #[codec(compact)]
                    pub now: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::set`]."]
                pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<types::Set> {
                    ::subxt::tx::Payload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAllowDeath {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetBalanceDeprecated {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub new_free: ::core::primitive::u128,
                    #[codec(compact)]
                    pub old_reserved: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBalanceDeprecated {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "set_balance_deprecated";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceTransfer {
                    pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferKeepAlive {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAll {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub keep_alive: ::core::primitive::bool,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceUnreserve {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub amount: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpgradeAccounts {
                    pub who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Transfer {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for Transfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceSetBalance {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub new_free: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::transfer_allow_death`]."]
                pub fn transfer_allow_death(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
                            140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
                            219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
                            130u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_balance_deprecated`]."]
                pub fn set_balance_deprecated(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                    old_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::SetBalanceDeprecated> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "set_balance_deprecated",
                        types::SetBalanceDeprecated {
                            who,
                            new_free,
                            old_reserved,
                        },
                        [
                            125u8, 171u8, 21u8, 186u8, 108u8, 185u8, 241u8, 145u8, 125u8, 8u8,
                            12u8, 42u8, 96u8, 114u8, 80u8, 80u8, 227u8, 76u8, 20u8, 208u8, 93u8,
                            219u8, 36u8, 50u8, 209u8, 155u8, 70u8, 45u8, 6u8, 57u8, 156u8, 77u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_transfer`]."]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceTransfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
                            153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
                            180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::transfer_keep_alive`]."]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
                            55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
                            208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::transfer_all`]."]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::Payload<types::TransferAll> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_unreserve`]."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceUnreserve> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
                            140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
                            199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
                            171u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::upgrade_accounts`]."]
                pub fn upgrade_accounts(
                    &self,
                    who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                ) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::transfer`]."]
                pub fn transfer(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::Transfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer",
                        types::Transfer { dest, value },
                        [
                            154u8, 145u8, 140u8, 54u8, 50u8, 123u8, 225u8, 249u8, 200u8, 217u8,
                            172u8, 110u8, 233u8, 198u8, 77u8, 198u8, 211u8, 89u8, 8u8, 13u8, 240u8,
                            94u8, 28u8, 13u8, 242u8, 217u8, 168u8, 23u8, 106u8, 254u8, 249u8,
                            120u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_set_balance`]."]
                pub fn force_set_balance(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceSetBalance> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                            39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
                            116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::utils::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::utils::AccountId32,
                pub free: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was minted into an account."]
            pub struct Minted {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was burned from an account."]
            pub struct Burned {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was suspended from an account (it can be restored later)."]
            pub struct Suspended {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was restored into an account."]
            pub struct Restored {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was upgraded."]
            pub struct Upgraded {
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
            pub struct Issued {
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
            pub struct Rescinded {
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was locked."]
            pub struct Locked {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unlocked."]
            pub struct Unlocked {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was frozen."]
            pub struct Frozen {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was thawed."]
            pub struct Thawed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![],
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![],
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![],
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            runtime_types::polkadot_runtime::RuntimeHoldReason,
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![],
                        [
                            37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8,
                            117u8, 119u8, 114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8,
                            37u8, 134u8, 22u8, 106u8, 221u8, 215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            runtime_types::polkadot_runtime::RuntimeHoldReason,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8,
                            117u8, 119u8, 114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8,
                            37u8, 134u8, 22u8, 106u8, 221u8, 215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![],
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
                #[doc = ""]
                #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
                #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
                #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
                #[doc = " behaviour if you set this to zero."]
                #[doc = ""]
                #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of holds that can exist on an account at any time."]
                pub fn max_holds(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxHolds",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
                pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod staking {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_staking::pallet::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_staking::pallet::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Bond {
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                    pub payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::utils::AccountId32,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for Bond {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "bond";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BondExtra {
                    #[codec(compact)]
                    pub max_additional: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for BondExtra {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "bond_extra";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Unbond {
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for Unbond {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "unbond";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WithdrawUnbonded {
                    pub num_slashing_spans: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for WithdrawUnbonded {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "withdraw_unbonded";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Validate {
                    pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
                }
                impl ::subxt::blocks::StaticExtrinsic for Validate {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "validate";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Nominate {
                    pub targets: ::std::vec::Vec<
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for Nominate {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "nominate";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Chill;
                impl ::subxt::blocks::StaticExtrinsic for Chill {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "chill";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetPayee {
                    pub payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::utils::AccountId32,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetPayee {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "set_payee";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetController;
                impl ::subxt::blocks::StaticExtrinsic for SetController {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "set_controller";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetValidatorCount {
                    #[codec(compact)]
                    pub new: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetValidatorCount {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "set_validator_count";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IncreaseValidatorCount {
                    #[codec(compact)]
                    pub additional: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for IncreaseValidatorCount {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "increase_validator_count";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ScaleValidatorCount {
                    pub factor: runtime_types::sp_arithmetic::per_things::Percent,
                }
                impl ::subxt::blocks::StaticExtrinsic for ScaleValidatorCount {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "scale_validator_count";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceNoEras;
                impl ::subxt::blocks::StaticExtrinsic for ForceNoEras {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "force_no_eras";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceNewEra;
                impl ::subxt::blocks::StaticExtrinsic for ForceNewEra {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "force_new_era";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetInvulnerables {
                    pub invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetInvulnerables {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "set_invulnerables";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceUnstake {
                    pub stash: ::subxt::utils::AccountId32,
                    pub num_slashing_spans: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceUnstake {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "force_unstake";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceNewEraAlways;
                impl ::subxt::blocks::StaticExtrinsic for ForceNewEraAlways {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "force_new_era_always";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CancelDeferredSlash {
                    pub era: ::core::primitive::u32,
                    pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                }
                impl ::subxt::blocks::StaticExtrinsic for CancelDeferredSlash {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "cancel_deferred_slash";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PayoutStakers {
                    pub validator_stash: ::subxt::utils::AccountId32,
                    pub era: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for PayoutStakers {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "payout_stakers";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Rebond {
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for Rebond {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "rebond";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReapStash {
                    pub stash: ::subxt::utils::AccountId32,
                    pub num_slashing_spans: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReapStash {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "reap_stash";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Kick {
                    pub who: ::std::vec::Vec<
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for Kick {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "kick";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetStakingConfigs {
                    pub min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u128,
                    >,
                    pub min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u128,
                    >,
                    pub max_nominator_count:
                        runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                            ::core::primitive::u32,
                        >,
                    pub max_validator_count:
                        runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                            ::core::primitive::u32,
                        >,
                    pub chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                    pub min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetStakingConfigs {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "set_staking_configs";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ChillOther {
                    pub controller: ::subxt::utils::AccountId32,
                }
                impl ::subxt::blocks::StaticExtrinsic for ChillOther {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "chill_other";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceApplyMinCommission {
                    pub validator_stash: ::subxt::utils::AccountId32,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceApplyMinCommission {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "force_apply_min_commission";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetMinCommission {
                    pub new: runtime_types::sp_arithmetic::per_things::Perbill,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetMinCommission {
                    const PALLET: &'static str = "Staking";
                    const CALL: &'static str = "set_min_commission";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::bond`]."]
                pub fn bond(
                    &self,
                    value: ::core::primitive::u128,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::utils::AccountId32,
                    >,
                ) -> ::subxt::tx::Payload<types::Bond> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "bond",
                        types::Bond { value, payee },
                        [
                            45u8, 207u8, 34u8, 221u8, 252u8, 224u8, 162u8, 185u8, 67u8, 224u8,
                            88u8, 91u8, 232u8, 114u8, 183u8, 44u8, 39u8, 5u8, 12u8, 163u8, 57u8,
                            31u8, 251u8, 58u8, 37u8, 232u8, 206u8, 75u8, 164u8, 26u8, 170u8, 101u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::bond_extra`]."]
                pub fn bond_extra(
                    &self,
                    max_additional: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::BondExtra> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "bond_extra",
                        types::BondExtra { max_additional },
                        [
                            9u8, 143u8, 179u8, 99u8, 91u8, 254u8, 114u8, 189u8, 202u8, 245u8, 48u8,
                            130u8, 103u8, 17u8, 183u8, 177u8, 172u8, 156u8, 227u8, 145u8, 191u8,
                            134u8, 81u8, 3u8, 170u8, 85u8, 40u8, 56u8, 216u8, 95u8, 232u8, 52u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::unbond`]."]
                pub fn unbond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::Unbond> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "unbond",
                        types::Unbond { value },
                        [
                            70u8, 201u8, 146u8, 56u8, 51u8, 237u8, 90u8, 193u8, 69u8, 42u8, 168u8,
                            96u8, 215u8, 128u8, 253u8, 22u8, 239u8, 14u8, 214u8, 103u8, 170u8,
                            140u8, 2u8, 182u8, 3u8, 190u8, 184u8, 191u8, 231u8, 137u8, 50u8, 16u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::withdraw_unbonded`]."]
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::WithdrawUnbonded> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "withdraw_unbonded",
                        types::WithdrawUnbonded { num_slashing_spans },
                        [
                            229u8, 128u8, 177u8, 224u8, 197u8, 118u8, 239u8, 142u8, 179u8, 164u8,
                            10u8, 205u8, 124u8, 254u8, 209u8, 157u8, 172u8, 87u8, 58u8, 120u8,
                            74u8, 12u8, 150u8, 117u8, 234u8, 32u8, 191u8, 182u8, 92u8, 97u8, 77u8,
                            59u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::validate`]."]
                pub fn validate(
                    &self,
                    prefs: runtime_types::pallet_staking::ValidatorPrefs,
                ) -> ::subxt::tx::Payload<types::Validate> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "validate",
                        types::Validate { prefs },
                        [
                            63u8, 83u8, 12u8, 16u8, 56u8, 84u8, 41u8, 141u8, 202u8, 0u8, 37u8,
                            30u8, 115u8, 2u8, 145u8, 101u8, 168u8, 89u8, 94u8, 98u8, 8u8, 45u8,
                            140u8, 237u8, 101u8, 136u8, 179u8, 162u8, 205u8, 41u8, 88u8, 248u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::nominate`]."]
                pub fn nominate(
                    &self,
                    targets: ::std::vec::Vec<
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    >,
                ) -> ::subxt::tx::Payload<types::Nominate> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "nominate",
                        types::Nominate { targets },
                        [
                            14u8, 209u8, 112u8, 222u8, 40u8, 211u8, 118u8, 188u8, 26u8, 88u8,
                            135u8, 233u8, 36u8, 99u8, 68u8, 189u8, 184u8, 169u8, 146u8, 217u8,
                            87u8, 198u8, 89u8, 32u8, 193u8, 135u8, 251u8, 88u8, 241u8, 151u8,
                            205u8, 138u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::chill`]."]
                pub fn chill(&self) -> ::subxt::tx::Payload<types::Chill> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "chill",
                        types::Chill {},
                        [
                            157u8, 75u8, 243u8, 69u8, 110u8, 192u8, 22u8, 27u8, 107u8, 68u8, 236u8,
                            58u8, 179u8, 34u8, 118u8, 98u8, 131u8, 62u8, 242u8, 84u8, 149u8, 24u8,
                            83u8, 223u8, 78u8, 12u8, 192u8, 22u8, 111u8, 11u8, 171u8, 149u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_payee`]."]
                pub fn set_payee(
                    &self,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::utils::AccountId32,
                    >,
                ) -> ::subxt::tx::Payload<types::SetPayee> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "set_payee",
                        types::SetPayee { payee },
                        [
                            86u8, 172u8, 187u8, 98u8, 106u8, 240u8, 184u8, 60u8, 163u8, 244u8, 7u8,
                            64u8, 147u8, 168u8, 192u8, 177u8, 211u8, 138u8, 73u8, 188u8, 159u8,
                            154u8, 175u8, 219u8, 231u8, 235u8, 93u8, 195u8, 204u8, 100u8, 196u8,
                            241u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_controller`]."]
                pub fn set_controller(&self) -> ::subxt::tx::Payload<types::SetController> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "set_controller",
                        types::SetController {},
                        [
                            172u8, 27u8, 195u8, 188u8, 145u8, 203u8, 190u8, 174u8, 145u8, 43u8,
                            253u8, 87u8, 11u8, 229u8, 112u8, 18u8, 57u8, 101u8, 84u8, 235u8, 109u8,
                            228u8, 58u8, 129u8, 179u8, 174u8, 245u8, 169u8, 89u8, 240u8, 39u8,
                            67u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_validator_count`]."]
                pub fn set_validator_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::SetValidatorCount> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "set_validator_count",
                        types::SetValidatorCount { new },
                        [
                            172u8, 225u8, 157u8, 48u8, 242u8, 217u8, 126u8, 206u8, 26u8, 156u8,
                            203u8, 100u8, 116u8, 189u8, 98u8, 89u8, 151u8, 101u8, 77u8, 236u8,
                            101u8, 8u8, 148u8, 236u8, 180u8, 175u8, 232u8, 146u8, 141u8, 141u8,
                            78u8, 165u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::increase_validator_count`]."]
                pub fn increase_validator_count(
                    &self,
                    additional: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::IncreaseValidatorCount> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "increase_validator_count",
                        types::IncreaseValidatorCount { additional },
                        [
                            108u8, 67u8, 131u8, 248u8, 139u8, 227u8, 224u8, 221u8, 248u8, 94u8,
                            141u8, 104u8, 131u8, 250u8, 127u8, 164u8, 137u8, 211u8, 5u8, 27u8,
                            185u8, 251u8, 120u8, 243u8, 165u8, 50u8, 197u8, 161u8, 125u8, 195u8,
                            16u8, 29u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::scale_validator_count`]."]
                pub fn scale_validator_count(
                    &self,
                    factor: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::tx::Payload<types::ScaleValidatorCount> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "scale_validator_count",
                        types::ScaleValidatorCount { factor },
                        [
                            93u8, 200u8, 119u8, 240u8, 148u8, 144u8, 175u8, 135u8, 102u8, 130u8,
                            183u8, 216u8, 28u8, 215u8, 155u8, 233u8, 152u8, 65u8, 49u8, 125u8,
                            196u8, 79u8, 31u8, 195u8, 233u8, 79u8, 150u8, 138u8, 103u8, 161u8,
                            78u8, 154u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_no_eras`]."]
                pub fn force_no_eras(&self) -> ::subxt::tx::Payload<types::ForceNoEras> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "force_no_eras",
                        types::ForceNoEras {},
                        [
                            77u8, 5u8, 105u8, 167u8, 251u8, 78u8, 52u8, 80u8, 177u8, 226u8, 28u8,
                            130u8, 106u8, 62u8, 40u8, 210u8, 110u8, 62u8, 21u8, 113u8, 234u8,
                            227u8, 171u8, 205u8, 240u8, 46u8, 32u8, 84u8, 184u8, 208u8, 61u8,
                            207u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_new_era`]."]
                pub fn force_new_era(&self) -> ::subxt::tx::Payload<types::ForceNewEra> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "force_new_era",
                        types::ForceNewEra {},
                        [
                            119u8, 45u8, 11u8, 87u8, 236u8, 189u8, 41u8, 142u8, 130u8, 10u8, 132u8,
                            140u8, 210u8, 134u8, 66u8, 152u8, 149u8, 55u8, 60u8, 31u8, 190u8, 41u8,
                            177u8, 103u8, 245u8, 193u8, 95u8, 255u8, 29u8, 79u8, 112u8, 188u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_invulnerables`]."]
                pub fn set_invulnerables(
                    &self,
                    invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
                ) -> ::subxt::tx::Payload<types::SetInvulnerables> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "set_invulnerables",
                        types::SetInvulnerables { invulnerables },
                        [
                            31u8, 115u8, 221u8, 229u8, 187u8, 61u8, 33u8, 22u8, 126u8, 142u8,
                            248u8, 190u8, 213u8, 35u8, 49u8, 208u8, 193u8, 0u8, 58u8, 18u8, 136u8,
                            220u8, 32u8, 8u8, 121u8, 36u8, 184u8, 57u8, 6u8, 125u8, 199u8, 245u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_unstake`]."]
                pub fn force_unstake(
                    &self,
                    stash: ::subxt::utils::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::ForceUnstake> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "force_unstake",
                        types::ForceUnstake {
                            stash,
                            num_slashing_spans,
                        },
                        [
                            205u8, 115u8, 222u8, 58u8, 168u8, 3u8, 59u8, 58u8, 220u8, 98u8, 204u8,
                            90u8, 36u8, 250u8, 178u8, 45u8, 213u8, 158u8, 92u8, 107u8, 3u8, 94u8,
                            118u8, 194u8, 187u8, 196u8, 101u8, 250u8, 36u8, 119u8, 21u8, 19u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_new_era_always`]."]
                pub fn force_new_era_always(
                    &self,
                ) -> ::subxt::tx::Payload<types::ForceNewEraAlways> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "force_new_era_always",
                        types::ForceNewEraAlways {},
                        [
                            102u8, 153u8, 116u8, 85u8, 80u8, 52u8, 89u8, 215u8, 173u8, 159u8, 96u8,
                            99u8, 180u8, 5u8, 62u8, 142u8, 181u8, 101u8, 160u8, 57u8, 177u8, 182u8,
                            6u8, 252u8, 107u8, 252u8, 225u8, 104u8, 147u8, 123u8, 244u8, 134u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::cancel_deferred_slash`]."]
                pub fn cancel_deferred_slash(
                    &self,
                    era: ::core::primitive::u32,
                    slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::tx::Payload<types::CancelDeferredSlash> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "cancel_deferred_slash",
                        types::CancelDeferredSlash { era, slash_indices },
                        [
                            49u8, 208u8, 248u8, 109u8, 25u8, 132u8, 73u8, 172u8, 232u8, 194u8,
                            114u8, 23u8, 114u8, 4u8, 64u8, 156u8, 70u8, 41u8, 207u8, 208u8, 78u8,
                            199u8, 81u8, 125u8, 101u8, 31u8, 17u8, 140u8, 190u8, 254u8, 64u8,
                            101u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::payout_stakers`]."]
                pub fn payout_stakers(
                    &self,
                    validator_stash: ::subxt::utils::AccountId32,
                    era: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::PayoutStakers> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "payout_stakers",
                        types::PayoutStakers {
                            validator_stash,
                            era,
                        },
                        [
                            69u8, 67u8, 140u8, 197u8, 89u8, 20u8, 59u8, 55u8, 142u8, 197u8, 62u8,
                            107u8, 239u8, 50u8, 237u8, 52u8, 4u8, 65u8, 119u8, 73u8, 138u8, 57u8,
                            46u8, 78u8, 252u8, 157u8, 187u8, 14u8, 232u8, 244u8, 217u8, 171u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::rebond`]."]
                pub fn rebond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::Rebond> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "rebond",
                        types::Rebond { value },
                        [
                            204u8, 209u8, 27u8, 219u8, 45u8, 129u8, 15u8, 39u8, 105u8, 165u8,
                            255u8, 55u8, 0u8, 59u8, 115u8, 79u8, 139u8, 82u8, 163u8, 197u8, 44u8,
                            89u8, 41u8, 234u8, 116u8, 214u8, 248u8, 123u8, 250u8, 49u8, 15u8, 77u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::reap_stash`]."]
                pub fn reap_stash(
                    &self,
                    stash: ::subxt::utils::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::ReapStash> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "reap_stash",
                        types::ReapStash {
                            stash,
                            num_slashing_spans,
                        },
                        [
                            231u8, 240u8, 152u8, 33u8, 10u8, 60u8, 18u8, 233u8, 0u8, 229u8, 90u8,
                            45u8, 118u8, 29u8, 98u8, 109u8, 89u8, 7u8, 228u8, 254u8, 119u8, 125u8,
                            172u8, 209u8, 217u8, 107u8, 50u8, 226u8, 31u8, 5u8, 153u8, 93u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::kick`]."]
                pub fn kick(
                    &self,
                    who: ::std::vec::Vec<
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    >,
                ) -> ::subxt::tx::Payload<types::Kick> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "kick",
                        types::Kick { who },
                        [
                            27u8, 64u8, 10u8, 21u8, 174u8, 6u8, 40u8, 249u8, 144u8, 247u8, 5u8,
                            123u8, 225u8, 172u8, 143u8, 50u8, 192u8, 248u8, 160u8, 179u8, 119u8,
                            122u8, 147u8, 92u8, 248u8, 123u8, 3u8, 154u8, 205u8, 199u8, 6u8, 126u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_staking_configs`]."]
                pub fn set_staking_configs(
                    &self,
                    min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u128,
                    >,
                    min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u128,
                    >,
                    max_nominator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u32,
                    >,
                    max_validator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u32,
                    >,
                    chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                    min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
                ) -> ::subxt::tx::Payload<types::SetStakingConfigs> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "set_staking_configs",
                        types::SetStakingConfigs {
                            min_nominator_bond,
                            min_validator_bond,
                            max_nominator_count,
                            max_validator_count,
                            chill_threshold,
                            min_commission,
                        },
                        [
                            99u8, 61u8, 196u8, 68u8, 226u8, 64u8, 104u8, 70u8, 173u8, 108u8, 29u8,
                            39u8, 61u8, 202u8, 72u8, 227u8, 190u8, 6u8, 138u8, 137u8, 207u8, 11u8,
                            190u8, 79u8, 73u8, 7u8, 108u8, 131u8, 19u8, 7u8, 173u8, 60u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::chill_other`]."]
                pub fn chill_other(
                    &self,
                    controller: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::Payload<types::ChillOther> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "chill_other",
                        types::ChillOther { controller },
                        [
                            143u8, 82u8, 167u8, 43u8, 102u8, 136u8, 78u8, 139u8, 110u8, 159u8,
                            235u8, 226u8, 237u8, 140u8, 142u8, 47u8, 77u8, 57u8, 209u8, 208u8, 9u8,
                            193u8, 3u8, 77u8, 147u8, 41u8, 182u8, 122u8, 178u8, 185u8, 32u8, 182u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::force_apply_min_commission`]."]
                pub fn force_apply_min_commission(
                    &self,
                    validator_stash: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::Payload<types::ForceApplyMinCommission> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "force_apply_min_commission",
                        types::ForceApplyMinCommission { validator_stash },
                        [
                            158u8, 27u8, 152u8, 23u8, 97u8, 53u8, 54u8, 49u8, 179u8, 236u8, 69u8,
                            65u8, 253u8, 136u8, 232u8, 44u8, 207u8, 66u8, 5u8, 186u8, 49u8, 91u8,
                            173u8, 5u8, 84u8, 45u8, 154u8, 91u8, 239u8, 97u8, 62u8, 42u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::set_min_commission`]."]
                pub fn set_min_commission(
                    &self,
                    new: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::tx::Payload<types::SetMinCommission> {
                    ::subxt::tx::Payload::new_static(
                        "Staking",
                        "set_min_commission",
                        types::SetMinCommission { new },
                        [
                            96u8, 168u8, 55u8, 79u8, 79u8, 49u8, 8u8, 127u8, 98u8, 158u8, 106u8,
                            187u8, 177u8, 201u8, 68u8, 181u8, 219u8, 172u8, 63u8, 120u8, 172u8,
                            173u8, 251u8, 167u8, 84u8, 165u8, 238u8, 115u8, 110u8, 97u8, 144u8,
                            50u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
            #[doc = "the remainder from the maximum amount of reward."]
            pub struct EraPaid {
                pub era_index: ::core::primitive::u32,
                pub validator_payout: ::core::primitive::u128,
                pub remainder: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for EraPaid {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The nominator has been rewarded by this amount."]
            pub struct Rewarded {
                pub stash: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rewarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A staker (validator or nominator) has been slashed by the given amount."]
            pub struct Slashed {
                pub staker: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A slash for the given validator, for the given percentage of their stake, at the given"]
            #[doc = "era as been reported."]
            pub struct SlashReported {
                pub validator: ::subxt::utils::AccountId32,
                pub fraction: runtime_types::sp_arithmetic::per_things::Perbill,
                pub slash_era: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for SlashReported {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "SlashReported";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An old slashing report from a prior era was discarded because it could"]
            #[doc = "not be processed."]
            pub struct OldSlashingReportDiscarded {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for OldSlashingReportDiscarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new set of stakers was elected."]
            pub struct StakersElected;
            impl ::subxt::events::StaticEvent for StakersElected {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakersElected";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account has bonded this amount. \\[stash, amount\\]"]
            #[doc = ""]
            #[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
            #[doc = "it will not be emitted for staking rewards when they are added to stake."]
            pub struct Bonded {
                pub stash: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Bonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account has unbonded this amount."]
            pub struct Unbonded {
                pub stash: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
            #[doc = "from the unlocking queue."]
            pub struct Withdrawn {
                pub stash: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdrawn {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A nominator has been kicked from a validator."]
            pub struct Kicked {
                pub nominator: ::subxt::utils::AccountId32,
                pub stash: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Kicked {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The election failed. No new era is planned."]
            pub struct StakingElectionFailed;
            impl ::subxt::events::StaticEvent for StakingElectionFailed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakingElectionFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account has stopped participating as either a validator or nominator."]
            pub struct Chilled {
                pub stash: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Chilled {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Chilled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The stakers' rewards are getting paid."]
            pub struct PayoutStarted {
                pub era_index: ::core::primitive::u32,
                pub validator_stash: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for PayoutStarted {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "PayoutStarted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A validator has set their preferences."]
            pub struct ValidatorPrefsSet {
                pub stash: ::subxt::utils::AccountId32,
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            impl ::subxt::events::StaticEvent for ValidatorPrefsSet {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "ValidatorPrefsSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new force era mode was set."]
            pub struct ForceEra {
                pub mode: runtime_types::pallet_staking::Forcing,
            }
            impl ::subxt::events::StaticEvent for ForceEra {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "ForceEra";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The ideal number of active validators."]
                pub fn validator_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ValidatorCount",
                        vec![],
                        [
                            105u8, 251u8, 193u8, 198u8, 232u8, 118u8, 73u8, 115u8, 205u8, 78u8,
                            49u8, 253u8, 140u8, 193u8, 161u8, 205u8, 13u8, 147u8, 125u8, 102u8,
                            142u8, 244u8, 210u8, 227u8, 225u8, 46u8, 144u8, 122u8, 254u8, 48u8,
                            44u8, 169u8,
                        ],
                    )
                }
                #[doc = " Minimum number of staking participants before emergency conditions are imposed."]
                pub fn minimum_validator_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "MinimumValidatorCount",
                        vec![],
                        [
                            103u8, 178u8, 29u8, 91u8, 90u8, 31u8, 49u8, 9u8, 11u8, 58u8, 178u8,
                            30u8, 219u8, 55u8, 58u8, 181u8, 80u8, 155u8, 9u8, 11u8, 38u8, 46u8,
                            125u8, 179u8, 220u8, 20u8, 212u8, 181u8, 136u8, 103u8, 58u8, 48u8,
                        ],
                    )
                }
                #[doc = " Any validators that may never be slashed or forcibly kicked. It's a Vec since they're"]
                #[doc = " easy to initialize and the performance hit is minimal (we expect no more than four"]
                #[doc = " invulnerables) and restricted to testnets."]
                pub fn invulnerables(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Invulnerables",
                        vec![],
                        [
                            199u8, 35u8, 0u8, 229u8, 160u8, 128u8, 139u8, 245u8, 27u8, 133u8, 47u8,
                            240u8, 86u8, 195u8, 90u8, 169u8, 158u8, 231u8, 128u8, 58u8, 24u8,
                            173u8, 138u8, 122u8, 226u8, 104u8, 239u8, 114u8, 91u8, 165u8, 207u8,
                            150u8,
                        ],
                    )
                }
                #[doc = " Map from all locked \"stash\" accounts to the controller account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn bonded_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Bonded",
                        vec![],
                        [
                            99u8, 128u8, 108u8, 100u8, 235u8, 102u8, 243u8, 95u8, 61u8, 206u8,
                            220u8, 49u8, 155u8, 85u8, 236u8, 110u8, 99u8, 21u8, 117u8, 127u8,
                            157u8, 226u8, 108u8, 80u8, 126u8, 93u8, 203u8, 0u8, 160u8, 253u8, 56u8,
                            101u8,
                        ],
                    )
                }
                #[doc = " Map from all locked \"stash\" accounts to the controller account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn bonded(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Bonded",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            99u8, 128u8, 108u8, 100u8, 235u8, 102u8, 243u8, 95u8, 61u8, 206u8,
                            220u8, 49u8, 155u8, 85u8, 236u8, 110u8, 99u8, 21u8, 117u8, 127u8,
                            157u8, 226u8, 108u8, 80u8, 126u8, 93u8, 203u8, 0u8, 160u8, 253u8, 56u8,
                            101u8,
                        ],
                    )
                }
                #[doc = " The minimum active bond to become and maintain the role of a nominator."]
                pub fn min_nominator_bond(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "MinNominatorBond",
                        vec![],
                        [
                            102u8, 115u8, 254u8, 15u8, 191u8, 228u8, 85u8, 249u8, 112u8, 190u8,
                            129u8, 243u8, 236u8, 39u8, 195u8, 232u8, 10u8, 230u8, 11u8, 144u8,
                            115u8, 1u8, 45u8, 70u8, 181u8, 161u8, 17u8, 92u8, 19u8, 70u8, 100u8,
                            94u8,
                        ],
                    )
                }
                #[doc = " The minimum active bond to become and maintain the role of a validator."]
                pub fn min_validator_bond(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "MinValidatorBond",
                        vec![],
                        [
                            146u8, 249u8, 26u8, 52u8, 224u8, 81u8, 85u8, 153u8, 118u8, 169u8,
                            140u8, 37u8, 208u8, 242u8, 8u8, 29u8, 156u8, 73u8, 154u8, 162u8, 186u8,
                            159u8, 119u8, 100u8, 109u8, 227u8, 6u8, 139u8, 155u8, 203u8, 167u8,
                            244u8,
                        ],
                    )
                }
                #[doc = " The minimum active nominator stake of the last successful election."]
                pub fn minimum_active_stake(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "MinimumActiveStake",
                        vec![],
                        [
                            166u8, 211u8, 59u8, 23u8, 2u8, 160u8, 244u8, 52u8, 153u8, 12u8, 103u8,
                            113u8, 51u8, 232u8, 145u8, 188u8, 54u8, 67u8, 227u8, 221u8, 186u8, 6u8,
                            28u8, 63u8, 146u8, 212u8, 233u8, 173u8, 134u8, 41u8, 169u8, 153u8,
                        ],
                    )
                }
                #[doc = " The minimum amount of commission that validators can set."]
                #[doc = ""]
                #[doc = " If set to `0`, no limit exists."]
                pub fn min_commission(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "MinCommission",
                        vec![],
                        [
                            220u8, 197u8, 232u8, 212u8, 205u8, 242u8, 121u8, 165u8, 255u8, 199u8,
                            122u8, 20u8, 145u8, 245u8, 175u8, 26u8, 45u8, 70u8, 207u8, 26u8, 112u8,
                            234u8, 181u8, 167u8, 140u8, 75u8, 15u8, 1u8, 221u8, 168u8, 17u8, 211u8,
                        ],
                    )
                }
                #[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
                pub fn ledger_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::StakingLedger,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Ledger",
                        vec![],
                        [
                            210u8, 236u8, 6u8, 49u8, 200u8, 118u8, 116u8, 25u8, 66u8, 60u8, 18u8,
                            75u8, 240u8, 156u8, 58u8, 48u8, 176u8, 10u8, 175u8, 0u8, 86u8, 7u8,
                            16u8, 134u8, 64u8, 41u8, 46u8, 128u8, 33u8, 40u8, 10u8, 129u8,
                        ],
                    )
                }
                #[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
                pub fn ledger(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::StakingLedger,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Ledger",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            210u8, 236u8, 6u8, 49u8, 200u8, 118u8, 116u8, 25u8, 66u8, 60u8, 18u8,
                            75u8, 240u8, 156u8, 58u8, 48u8, 176u8, 10u8, 175u8, 0u8, 86u8, 7u8,
                            16u8, 134u8, 64u8, 41u8, 46u8, 128u8, 33u8, 40u8, 10u8, 129u8,
                        ],
                    )
                }
                #[doc = " Where the reward payment should be made. Keyed by stash."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn payee_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::RewardDestination<::subxt::utils::AccountId32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Payee",
                        vec![],
                        [
                            141u8, 225u8, 44u8, 134u8, 50u8, 229u8, 64u8, 186u8, 166u8, 88u8,
                            213u8, 118u8, 32u8, 154u8, 151u8, 204u8, 104u8, 216u8, 198u8, 66u8,
                            123u8, 143u8, 206u8, 245u8, 53u8, 67u8, 78u8, 82u8, 115u8, 31u8, 39u8,
                            76u8,
                        ],
                    )
                }
                #[doc = " Where the reward payment should be made. Keyed by stash."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn payee(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::RewardDestination<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Payee",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            141u8, 225u8, 44u8, 134u8, 50u8, 229u8, 64u8, 186u8, 166u8, 88u8,
                            213u8, 118u8, 32u8, 154u8, 151u8, 204u8, 104u8, 216u8, 198u8, 66u8,
                            123u8, 143u8, 206u8, 245u8, 53u8, 67u8, 78u8, 82u8, 115u8, 31u8, 39u8,
                            76u8,
                        ],
                    )
                }
                #[doc = " The map from (wannabe) validator stash key to the preferences of that validator."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn validators_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::ValidatorPrefs,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Validators",
                        vec![],
                        [
                            149u8, 207u8, 68u8, 38u8, 24u8, 220u8, 207u8, 84u8, 236u8, 33u8, 210u8,
                            124u8, 200u8, 99u8, 98u8, 29u8, 235u8, 46u8, 124u8, 4u8, 203u8, 6u8,
                            209u8, 21u8, 124u8, 236u8, 112u8, 118u8, 180u8, 85u8, 78u8, 13u8,
                        ],
                    )
                }
                #[doc = " The map from (wannabe) validator stash key to the preferences of that validator."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn validators(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Validators",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            149u8, 207u8, 68u8, 38u8, 24u8, 220u8, 207u8, 84u8, 236u8, 33u8, 210u8,
                            124u8, 200u8, 99u8, 98u8, 29u8, 235u8, 46u8, 124u8, 4u8, 203u8, 6u8,
                            209u8, 21u8, 124u8, 236u8, 112u8, 118u8, 180u8, 85u8, 78u8, 13u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]
                pub fn counter_for_validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "CounterForValidators",
                        vec![],
                        [
                            169u8, 146u8, 194u8, 114u8, 57u8, 232u8, 137u8, 93u8, 214u8, 98u8,
                            176u8, 151u8, 237u8, 165u8, 176u8, 252u8, 73u8, 124u8, 22u8, 166u8,
                            225u8, 217u8, 65u8, 56u8, 174u8, 12u8, 32u8, 2u8, 7u8, 173u8, 125u8,
                            235u8,
                        ],
                    )
                }
                #[doc = " The maximum validator count before we stop allowing new validators to join."]
                #[doc = ""]
                #[doc = " When this value is not set, no limits are enforced."]
                pub fn max_validators_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "MaxValidatorsCount",
                        vec![],
                        [
                            139u8, 116u8, 236u8, 217u8, 110u8, 47u8, 140u8, 197u8, 184u8, 246u8,
                            180u8, 188u8, 233u8, 99u8, 102u8, 21u8, 114u8, 23u8, 143u8, 163u8,
                            224u8, 250u8, 248u8, 185u8, 235u8, 94u8, 110u8, 83u8, 170u8, 123u8,
                            113u8, 168u8,
                        ],
                    )
                }
                #[doc = " The map from nominator stash key to their nomination preferences, namely the validators that"]
                #[doc = " they wish to support."]
                #[doc = ""]
                #[doc = " Note that the keys of this storage map might become non-decodable in case the"]
                #[doc = " [`Config::MaxNominations`] configuration is decreased. In this rare case, these nominators"]
                #[doc = " are still existent in storage, their key is correct and retrievable (i.e. `contains_key`"]
                #[doc = " indicates that they exist), but their value cannot be decoded. Therefore, the non-decodable"]
                #[doc = " nominators will effectively not-exist, until they re-submit their preferences such that it"]
                #[doc = " is within the bounds of the newly set `Config::MaxNominations`."]
                #[doc = ""]
                #[doc = " This implies that `::iter_keys().count()` and `::iter().count()` might return different"]
                #[doc = " values for this map. Moreover, the main `::count()` is aligned with the former, namely the"]
                #[doc = " number of keys that exist."]
                #[doc = ""]
                #[doc = " Lastly, if any of the nominators become non-decodable, they can be chilled immediately via"]
                #[doc = " [`Call::chill_other`] dispatchable by anyone."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn nominators_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Nominations,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Nominators",
                        vec![],
                        [
                            244u8, 174u8, 214u8, 105u8, 215u8, 218u8, 241u8, 145u8, 155u8, 54u8,
                            219u8, 34u8, 158u8, 224u8, 251u8, 17u8, 245u8, 9u8, 150u8, 36u8, 2u8,
                            233u8, 222u8, 218u8, 136u8, 86u8, 37u8, 244u8, 18u8, 50u8, 91u8, 120u8,
                        ],
                    )
                }
                #[doc = " The map from nominator stash key to their nomination preferences, namely the validators that"]
                #[doc = " they wish to support."]
                #[doc = ""]
                #[doc = " Note that the keys of this storage map might become non-decodable in case the"]
                #[doc = " [`Config::MaxNominations`] configuration is decreased. In this rare case, these nominators"]
                #[doc = " are still existent in storage, their key is correct and retrievable (i.e. `contains_key`"]
                #[doc = " indicates that they exist), but their value cannot be decoded. Therefore, the non-decodable"]
                #[doc = " nominators will effectively not-exist, until they re-submit their preferences such that it"]
                #[doc = " is within the bounds of the newly set `Config::MaxNominations`."]
                #[doc = ""]
                #[doc = " This implies that `::iter_keys().count()` and `::iter().count()` might return different"]
                #[doc = " values for this map. Moreover, the main `::count()` is aligned with the former, namely the"]
                #[doc = " number of keys that exist."]
                #[doc = ""]
                #[doc = " Lastly, if any of the nominators become non-decodable, they can be chilled immediately via"]
                #[doc = " [`Call::chill_other`] dispatchable by anyone."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn nominators(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Nominations,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "Nominators",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            244u8, 174u8, 214u8, 105u8, 215u8, 218u8, 241u8, 145u8, 155u8, 54u8,
                            219u8, 34u8, 158u8, 224u8, 251u8, 17u8, 245u8, 9u8, 150u8, 36u8, 2u8,
                            233u8, 222u8, 218u8, 136u8, 86u8, 37u8, 244u8, 18u8, 50u8, 91u8, 120u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]
                pub fn counter_for_nominators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "CounterForNominators",
                        vec![],
                        [
                            150u8, 236u8, 184u8, 12u8, 224u8, 26u8, 13u8, 204u8, 208u8, 178u8,
                            68u8, 148u8, 232u8, 85u8, 74u8, 248u8, 167u8, 61u8, 88u8, 126u8, 40u8,
                            20u8, 73u8, 47u8, 94u8, 57u8, 144u8, 77u8, 156u8, 179u8, 55u8, 49u8,
                        ],
                    )
                }
                #[doc = " The maximum nominator count before we stop allowing new validators to join."]
                #[doc = ""]
                #[doc = " When this value is not set, no limits are enforced."]
                pub fn max_nominators_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "MaxNominatorsCount",
                        vec![],
                        [
                            11u8, 234u8, 179u8, 254u8, 95u8, 119u8, 35u8, 255u8, 141u8, 95u8,
                            148u8, 209u8, 43u8, 202u8, 19u8, 57u8, 185u8, 50u8, 152u8, 192u8, 95u8,
                            13u8, 158u8, 245u8, 113u8, 199u8, 255u8, 187u8, 37u8, 44u8, 8u8, 119u8,
                        ],
                    )
                }
                #[doc = " The current era index."]
                #[doc = ""]
                #[doc = " This is the latest planned era, depending on how the Session pallet queues the validator"]
                #[doc = " set, it might be active or not."]
                pub fn current_era(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "CurrentEra",
                        vec![],
                        [
                            247u8, 239u8, 171u8, 18u8, 137u8, 240u8, 213u8, 3u8, 173u8, 173u8,
                            236u8, 141u8, 202u8, 191u8, 228u8, 120u8, 196u8, 188u8, 13u8, 66u8,
                            253u8, 117u8, 90u8, 8u8, 158u8, 11u8, 236u8, 141u8, 178u8, 44u8, 119u8,
                            25u8,
                        ],
                    )
                }
                #[doc = " The active era information, it holds index and start."]
                #[doc = ""]
                #[doc = " The active era is the era being currently rewarded. Validator set of this era must be"]
                #[doc = " equal to [`SessionInterface::validators`]."]
                pub fn active_era(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::ActiveEraInfo,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ActiveEra",
                        vec![],
                        [
                            24u8, 229u8, 66u8, 56u8, 111u8, 234u8, 139u8, 93u8, 245u8, 137u8,
                            110u8, 110u8, 121u8, 15u8, 216u8, 207u8, 97u8, 120u8, 125u8, 45u8,
                            61u8, 2u8, 50u8, 100u8, 3u8, 106u8, 12u8, 233u8, 123u8, 156u8, 145u8,
                            38u8,
                        ],
                    )
                }
                #[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
                #[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
                pub fn eras_start_session_index_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasStartSessionIndex",
                        vec![],
                        [
                            104u8, 76u8, 102u8, 20u8, 9u8, 146u8, 55u8, 204u8, 12u8, 15u8, 117u8,
                            22u8, 54u8, 230u8, 98u8, 105u8, 191u8, 136u8, 140u8, 65u8, 48u8, 29u8,
                            19u8, 144u8, 159u8, 241u8, 158u8, 77u8, 4u8, 230u8, 216u8, 52u8,
                        ],
                    )
                }
                #[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
                #[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
                pub fn eras_start_session_index(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasStartSessionIndex",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            104u8, 76u8, 102u8, 20u8, 9u8, 146u8, 55u8, 204u8, 12u8, 15u8, 117u8,
                            22u8, 54u8, 230u8, 98u8, 105u8, 191u8, 136u8, 140u8, 65u8, 48u8, 29u8,
                            19u8, 144u8, 159u8, 241u8, 158u8, 77u8, 4u8, 230u8, 216u8, 52u8,
                        ],
                    )
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub fn eras_stakers_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasStakers",
                        vec![],
                        [
                            120u8, 64u8, 232u8, 134u8, 109u8, 212u8, 242u8, 64u8, 68u8, 196u8,
                            108u8, 91u8, 255u8, 123u8, 245u8, 27u8, 55u8, 254u8, 60u8, 74u8, 183u8,
                            183u8, 226u8, 159u8, 244u8, 56u8, 139u8, 34u8, 228u8, 176u8, 241u8,
                            76u8,
                        ],
                    )
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub fn eras_stakers_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasStakers",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            120u8, 64u8, 232u8, 134u8, 109u8, 212u8, 242u8, 64u8, 68u8, 196u8,
                            108u8, 91u8, 255u8, 123u8, 245u8, 27u8, 55u8, 254u8, 60u8, 74u8, 183u8,
                            183u8, 226u8, 159u8, 244u8, 56u8, 139u8, 34u8, 228u8, 176u8, 241u8,
                            76u8,
                        ],
                    )
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub fn eras_stakers(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasStakers",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            120u8, 64u8, 232u8, 134u8, 109u8, 212u8, 242u8, 64u8, 68u8, 196u8,
                            108u8, 91u8, 255u8, 123u8, 245u8, 27u8, 55u8, 254u8, 60u8, 74u8, 183u8,
                            183u8, 226u8, 159u8, 244u8, 56u8, 139u8, 34u8, 228u8, 176u8, 241u8,
                            76u8,
                        ],
                    )
                }
                #[doc = " Clipped Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
                #[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
                #[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
                #[doc = " This is used to limit the i/o cost for the nominator payout."]
                #[doc = ""]
                #[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub fn eras_stakers_clipped_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasStakersClipped",
                        vec![],
                        [
                            85u8, 192u8, 164u8, 53u8, 181u8, 61u8, 132u8, 255u8, 144u8, 41u8, 44u8,
                            199u8, 34u8, 11u8, 248u8, 81u8, 203u8, 204u8, 152u8, 138u8, 112u8,
                            229u8, 145u8, 253u8, 111u8, 111u8, 38u8, 74u8, 199u8, 164u8, 16u8,
                            45u8,
                        ],
                    )
                }
                #[doc = " Clipped Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
                #[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
                #[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
                #[doc = " This is used to limit the i/o cost for the nominator payout."]
                #[doc = ""]
                #[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub fn eras_stakers_clipped_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasStakersClipped",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            85u8, 192u8, 164u8, 53u8, 181u8, 61u8, 132u8, 255u8, 144u8, 41u8, 44u8,
                            199u8, 34u8, 11u8, 248u8, 81u8, 203u8, 204u8, 152u8, 138u8, 112u8,
                            229u8, 145u8, 253u8, 111u8, 111u8, 38u8, 74u8, 199u8, 164u8, 16u8,
                            45u8,
                        ],
                    )
                }
                #[doc = " Clipped Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
                #[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
                #[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
                #[doc = " This is used to limit the i/o cost for the nominator payout."]
                #[doc = ""]
                #[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
                pub fn eras_stakers_clipped(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasStakersClipped",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            85u8, 192u8, 164u8, 53u8, 181u8, 61u8, 132u8, 255u8, 144u8, 41u8, 44u8,
                            199u8, 34u8, 11u8, 248u8, 81u8, 203u8, 204u8, 152u8, 138u8, 112u8,
                            229u8, 145u8, 253u8, 111u8, 111u8, 38u8, 74u8, 199u8, 164u8, 16u8,
                            45u8,
                        ],
                    )
                }
                #[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                pub fn eras_validator_prefs_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::ValidatorPrefs,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasValidatorPrefs",
                        vec![],
                        [
                            134u8, 250u8, 229u8, 21u8, 44u8, 119u8, 43u8, 99u8, 69u8, 94u8, 177u8,
                            180u8, 174u8, 134u8, 54u8, 25u8, 56u8, 144u8, 194u8, 149u8, 56u8,
                            234u8, 78u8, 238u8, 78u8, 247u8, 205u8, 43u8, 16u8, 159u8, 92u8, 169u8,
                        ],
                    )
                }
                #[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                pub fn eras_validator_prefs_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::ValidatorPrefs,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasValidatorPrefs",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            134u8, 250u8, 229u8, 21u8, 44u8, 119u8, 43u8, 99u8, 69u8, 94u8, 177u8,
                            180u8, 174u8, 134u8, 54u8, 25u8, 56u8, 144u8, 194u8, 149u8, 56u8,
                            234u8, 78u8, 238u8, 78u8, 247u8, 205u8, 43u8, 16u8, 159u8, 92u8, 169u8,
                        ],
                    )
                }
                #[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                pub fn eras_validator_prefs(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasValidatorPrefs",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            134u8, 250u8, 229u8, 21u8, 44u8, 119u8, 43u8, 99u8, 69u8, 94u8, 177u8,
                            180u8, 174u8, 134u8, 54u8, 25u8, 56u8, 144u8, 194u8, 149u8, 56u8,
                            234u8, 78u8, 238u8, 78u8, 247u8, 205u8, 43u8, 16u8, 159u8, 92u8, 169u8,
                        ],
                    )
                }
                #[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
                pub fn eras_validator_reward_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasValidatorReward",
                        vec![],
                        [
                            185u8, 85u8, 179u8, 163u8, 178u8, 168u8, 141u8, 200u8, 59u8, 77u8, 2u8,
                            197u8, 36u8, 188u8, 133u8, 117u8, 2u8, 25u8, 105u8, 132u8, 44u8, 75u8,
                            15u8, 82u8, 57u8, 89u8, 242u8, 234u8, 70u8, 244u8, 198u8, 126u8,
                        ],
                    )
                }
                #[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
                pub fn eras_validator_reward(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasValidatorReward",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            185u8, 85u8, 179u8, 163u8, 178u8, 168u8, 141u8, 200u8, 59u8, 77u8, 2u8,
                            197u8, 36u8, 188u8, 133u8, 117u8, 2u8, 25u8, 105u8, 132u8, 44u8, 75u8,
                            15u8, 82u8, 57u8, 89u8, 242u8, 234u8, 70u8, 244u8, 198u8, 126u8,
                        ],
                    )
                }
                #[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
                #[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
                pub fn eras_reward_points_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::EraRewardPoints<::subxt::utils::AccountId32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasRewardPoints",
                        vec![],
                        [
                            135u8, 0u8, 85u8, 241u8, 213u8, 133u8, 30u8, 192u8, 251u8, 191u8, 41u8,
                            38u8, 233u8, 236u8, 218u8, 246u8, 166u8, 93u8, 46u8, 37u8, 48u8, 187u8,
                            172u8, 48u8, 251u8, 178u8, 75u8, 203u8, 60u8, 188u8, 204u8, 207u8,
                        ],
                    )
                }
                #[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
                #[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
                pub fn eras_reward_points(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::EraRewardPoints<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasRewardPoints",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            135u8, 0u8, 85u8, 241u8, 213u8, 133u8, 30u8, 192u8, 251u8, 191u8, 41u8,
                            38u8, 233u8, 236u8, 218u8, 246u8, 166u8, 93u8, 46u8, 37u8, 48u8, 187u8,
                            172u8, 48u8, 251u8, 178u8, 75u8, 203u8, 60u8, 188u8, 204u8, 207u8,
                        ],
                    )
                }
                #[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
                #[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
                pub fn eras_total_stake_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasTotalStake",
                        vec![],
                        [
                            8u8, 78u8, 101u8, 62u8, 124u8, 126u8, 66u8, 26u8, 47u8, 126u8, 239u8,
                            204u8, 222u8, 104u8, 19u8, 108u8, 238u8, 160u8, 112u8, 242u8, 56u8,
                            2u8, 250u8, 164u8, 250u8, 213u8, 201u8, 84u8, 193u8, 117u8, 108u8,
                            146u8,
                        ],
                    )
                }
                #[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
                #[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
                pub fn eras_total_stake(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ErasTotalStake",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            8u8, 78u8, 101u8, 62u8, 124u8, 126u8, 66u8, 26u8, 47u8, 126u8, 239u8,
                            204u8, 222u8, 104u8, 19u8, 108u8, 238u8, 160u8, 112u8, 242u8, 56u8,
                            2u8, 250u8, 164u8, 250u8, 213u8, 201u8, 84u8, 193u8, 117u8, 108u8,
                            146u8,
                        ],
                    )
                }
                #[doc = " Mode of era forcing."]
                pub fn force_era(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::Forcing,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ForceEra",
                        vec![],
                        [
                            177u8, 148u8, 73u8, 108u8, 136u8, 126u8, 89u8, 18u8, 124u8, 66u8, 30u8,
                            102u8, 133u8, 164u8, 78u8, 214u8, 184u8, 163u8, 75u8, 164u8, 117u8,
                            233u8, 209u8, 158u8, 99u8, 208u8, 21u8, 194u8, 152u8, 82u8, 16u8,
                            222u8,
                        ],
                    )
                }
                #[doc = " The percentage of the slash that is distributed to reporters."]
                #[doc = ""]
                #[doc = " The rest of the slashed value is handled by the `Slash`."]
                pub fn slash_reward_fraction(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "SlashRewardFraction",
                        vec![],
                        [
                            53u8, 88u8, 253u8, 237u8, 84u8, 228u8, 187u8, 130u8, 108u8, 195u8,
                            135u8, 25u8, 75u8, 52u8, 238u8, 62u8, 133u8, 38u8, 139u8, 129u8, 216u8,
                            193u8, 197u8, 216u8, 245u8, 171u8, 128u8, 207u8, 125u8, 246u8, 248u8,
                            7u8,
                        ],
                    )
                }
                #[doc = " The amount of currency given to reporters of a slash event which was"]
                #[doc = " canceled by extraordinary circumstances (e.g. governance)."]
                pub fn canceled_slash_payout(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "CanceledSlashPayout",
                        vec![],
                        [
                            221u8, 88u8, 134u8, 81u8, 22u8, 229u8, 100u8, 27u8, 86u8, 244u8, 229u8,
                            107u8, 251u8, 119u8, 58u8, 153u8, 19u8, 20u8, 254u8, 169u8, 248u8,
                            220u8, 98u8, 118u8, 48u8, 213u8, 22u8, 79u8, 242u8, 250u8, 147u8,
                            173u8,
                        ],
                    )
                }
                #[doc = " All unapplied slashes that are queued for later."]
                pub fn unapplied_slashes_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::pallet_staking::UnappliedSlash<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "UnappliedSlashes",
                        vec![],
                        [
                            158u8, 134u8, 7u8, 21u8, 200u8, 222u8, 197u8, 166u8, 199u8, 39u8, 1u8,
                            167u8, 164u8, 154u8, 165u8, 118u8, 92u8, 223u8, 219u8, 136u8, 196u8,
                            155u8, 243u8, 20u8, 198u8, 92u8, 198u8, 61u8, 252u8, 176u8, 175u8,
                            172u8,
                        ],
                    )
                }
                #[doc = " All unapplied slashes that are queued for later."]
                pub fn unapplied_slashes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::pallet_staking::UnappliedSlash<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "UnappliedSlashes",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            158u8, 134u8, 7u8, 21u8, 200u8, 222u8, 197u8, 166u8, 199u8, 39u8, 1u8,
                            167u8, 164u8, 154u8, 165u8, 118u8, 92u8, 223u8, 219u8, 136u8, 196u8,
                            155u8, 243u8, 20u8, 198u8, 92u8, 198u8, 61u8, 252u8, 176u8, 175u8,
                            172u8,
                        ],
                    )
                }
                #[doc = " A mapping from still-bonded eras to the first session index of that era."]
                #[doc = ""]
                #[doc = " Must contains information for eras for the range:"]
                #[doc = " `[active_era - bounding_duration; active_era]`"]
                pub fn bonded_eras(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "BondedEras",
                        vec![],
                        [
                            20u8, 0u8, 164u8, 169u8, 183u8, 130u8, 242u8, 167u8, 92u8, 254u8,
                            191u8, 206u8, 177u8, 182u8, 219u8, 162u8, 7u8, 116u8, 223u8, 166u8,
                            239u8, 216u8, 140u8, 42u8, 174u8, 237u8, 134u8, 186u8, 180u8, 62u8,
                            175u8, 239u8,
                        ],
                    )
                }
                #[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
                #[doc = " and slash value of the era."]
                pub fn validator_slash_in_era_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        ::core::primitive::u128,
                    ),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ValidatorSlashInEra",
                        vec![],
                        [
                            245u8, 72u8, 52u8, 22u8, 10u8, 177u8, 127u8, 83u8, 180u8, 246u8, 17u8,
                            82u8, 6u8, 231u8, 131u8, 68u8, 73u8, 92u8, 241u8, 251u8, 32u8, 97u8,
                            121u8, 137u8, 190u8, 227u8, 162u8, 16u8, 224u8, 207u8, 63u8, 184u8,
                        ],
                    )
                }
                #[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
                #[doc = " and slash value of the era."]
                pub fn validator_slash_in_era_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        ::core::primitive::u128,
                    ),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ValidatorSlashInEra",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            245u8, 72u8, 52u8, 22u8, 10u8, 177u8, 127u8, 83u8, 180u8, 246u8, 17u8,
                            82u8, 6u8, 231u8, 131u8, 68u8, 73u8, 92u8, 241u8, 251u8, 32u8, 97u8,
                            121u8, 137u8, 190u8, 227u8, 162u8, 16u8, 224u8, 207u8, 63u8, 184u8,
                        ],
                    )
                }
                #[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
                #[doc = " and slash value of the era."]
                pub fn validator_slash_in_era(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        ::core::primitive::u128,
                    ),
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ValidatorSlashInEra",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            245u8, 72u8, 52u8, 22u8, 10u8, 177u8, 127u8, 83u8, 180u8, 246u8, 17u8,
                            82u8, 6u8, 231u8, 131u8, 68u8, 73u8, 92u8, 241u8, 251u8, 32u8, 97u8,
                            121u8, 137u8, 190u8, 227u8, 162u8, 16u8, 224u8, 207u8, 63u8, 184u8,
                        ],
                    )
                }
                #[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]
                pub fn nominator_slash_in_era_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "NominatorSlashInEra",
                        vec![],
                        [
                            8u8, 89u8, 171u8, 183u8, 64u8, 29u8, 44u8, 185u8, 11u8, 204u8, 67u8,
                            60u8, 208u8, 132u8, 9u8, 214u8, 13u8, 148u8, 205u8, 26u8, 5u8, 7u8,
                            250u8, 191u8, 83u8, 118u8, 95u8, 17u8, 40u8, 126u8, 16u8, 135u8,
                        ],
                    )
                }
                #[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]
                pub fn nominator_slash_in_era_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "NominatorSlashInEra",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            8u8, 89u8, 171u8, 183u8, 64u8, 29u8, 44u8, 185u8, 11u8, 204u8, 67u8,
                            60u8, 208u8, 132u8, 9u8, 214u8, 13u8, 148u8, 205u8, 26u8, 5u8, 7u8,
                            250u8, 191u8, 83u8, 118u8, 95u8, 17u8, 40u8, 126u8, 16u8, 135u8,
                        ],
                    )
                }
                #[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]
                pub fn nominator_slash_in_era(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "NominatorSlashInEra",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            8u8, 89u8, 171u8, 183u8, 64u8, 29u8, 44u8, 185u8, 11u8, 204u8, 67u8,
                            60u8, 208u8, 132u8, 9u8, 214u8, 13u8, 148u8, 205u8, 26u8, 5u8, 7u8,
                            250u8, 191u8, 83u8, 118u8, 95u8, 17u8, 40u8, 126u8, 16u8, 135u8,
                        ],
                    )
                }
                #[doc = " Slashing spans for stash accounts."]
                pub fn slashing_spans_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::slashing::SlashingSpans,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "SlashingSpans",
                        vec![],
                        [
                            74u8, 169u8, 189u8, 252u8, 193u8, 191u8, 114u8, 107u8, 158u8, 125u8,
                            252u8, 35u8, 177u8, 129u8, 99u8, 24u8, 77u8, 223u8, 238u8, 24u8, 237u8,
                            225u8, 5u8, 117u8, 163u8, 180u8, 139u8, 22u8, 169u8, 185u8, 60u8,
                            217u8,
                        ],
                    )
                }
                #[doc = " Slashing spans for stash accounts."]
                pub fn slashing_spans(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::slashing::SlashingSpans,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "SlashingSpans",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            74u8, 169u8, 189u8, 252u8, 193u8, 191u8, 114u8, 107u8, 158u8, 125u8,
                            252u8, 35u8, 177u8, 129u8, 99u8, 24u8, 77u8, 223u8, 238u8, 24u8, 237u8,
                            225u8, 5u8, 117u8, 163u8, 180u8, 139u8, 22u8, 169u8, 185u8, 60u8,
                            217u8,
                        ],
                    )
                }
                #[doc = " Records information about the maximum slash of a stash within a slashing span,"]
                #[doc = " as well as how much reward has been paid out."]
                pub fn span_slash_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "SpanSlash",
                        vec![],
                        [
                            158u8, 168u8, 151u8, 108u8, 4u8, 168u8, 253u8, 28u8, 69u8, 111u8, 99u8,
                            235u8, 175u8, 72u8, 48u8, 238u8, 239u8, 142u8, 40u8, 142u8, 97u8, 77u8,
                            72u8, 123u8, 210u8, 157u8, 119u8, 180u8, 205u8, 98u8, 110u8, 215u8,
                        ],
                    )
                }
                #[doc = " Records information about the maximum slash of a stash within a slashing span,"]
                #[doc = " as well as how much reward has been paid out."]
                pub fn span_slash_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "SpanSlash",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            158u8, 168u8, 151u8, 108u8, 4u8, 168u8, 253u8, 28u8, 69u8, 111u8, 99u8,
                            235u8, 175u8, 72u8, 48u8, 238u8, 239u8, 142u8, 40u8, 142u8, 97u8, 77u8,
                            72u8, 123u8, 210u8, 157u8, 119u8, 180u8, 205u8, 98u8, 110u8, 215u8,
                        ],
                    )
                }
                #[doc = " Records information about the maximum slash of a stash within a slashing span,"]
                #[doc = " as well as how much reward has been paid out."]
                pub fn span_slash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "SpanSlash",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            158u8, 168u8, 151u8, 108u8, 4u8, 168u8, 253u8, 28u8, 69u8, 111u8, 99u8,
                            235u8, 175u8, 72u8, 48u8, 238u8, 239u8, 142u8, 40u8, 142u8, 97u8, 77u8,
                            72u8, 123u8, 210u8, 157u8, 119u8, 180u8, 205u8, 98u8, 110u8, 215u8,
                        ],
                    )
                }
                #[doc = " The last planned session scheduled by the session pallet."]
                #[doc = ""]
                #[doc = " This is basically in sync with the call to [`pallet_session::SessionManager::new_session`]."]
                pub fn current_planned_session(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "CurrentPlannedSession",
                        vec![],
                        [
                            12u8, 47u8, 20u8, 104u8, 155u8, 181u8, 35u8, 91u8, 172u8, 97u8, 206u8,
                            135u8, 185u8, 142u8, 46u8, 72u8, 32u8, 118u8, 225u8, 191u8, 28u8,
                            130u8, 7u8, 38u8, 181u8, 233u8, 201u8, 8u8, 160u8, 161u8, 86u8, 204u8,
                        ],
                    )
                }
                #[doc = " Indices of validators that have offended in the active era and whether they are currently"]
                #[doc = " disabled."]
                #[doc = ""]
                #[doc = " This value should be a superset of disabled validators since not all offences lead to the"]
                #[doc = " validator being disabled (if there was no slash). This is needed to track the percentage of"]
                #[doc = " validators that have offended in the current era, ensuring a new era is forced if"]
                #[doc = " `OffendingValidatorsThreshold` is reached. The vec is always kept sorted so that we can find"]
                #[doc = " whether a given validator has previously offended using binary search. It gets cleared when"]
                #[doc = " the era ends."]
                pub fn offending_validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "OffendingValidators",
                        vec![],
                        [
                            201u8, 31u8, 141u8, 182u8, 160u8, 180u8, 37u8, 226u8, 50u8, 65u8,
                            103u8, 11u8, 38u8, 120u8, 200u8, 219u8, 219u8, 98u8, 185u8, 137u8,
                            154u8, 20u8, 130u8, 163u8, 126u8, 185u8, 33u8, 194u8, 76u8, 172u8,
                            70u8, 220u8,
                        ],
                    )
                }
                #[doc = " The threshold for when users can start calling `chill_other` for other validators /"]
                #[doc = " nominators. The threshold is compared to the actual number of validators / nominators"]
                #[doc = " (`CountFor*`) in the system compared to the configured max (`Max*Count`)."]
                pub fn chill_threshold(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::per_things::Percent,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Staking",
                        "ChillThreshold",
                        vec![],
                        [
                            133u8, 222u8, 1u8, 208u8, 212u8, 216u8, 247u8, 66u8, 178u8, 96u8, 35u8,
                            112u8, 33u8, 245u8, 11u8, 249u8, 255u8, 212u8, 204u8, 161u8, 44u8,
                            38u8, 126u8, 151u8, 140u8, 42u8, 253u8, 101u8, 1u8, 23u8, 239u8, 39u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Maximum number of nominations per nominator."]
                pub fn max_nominations(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Staking",
                        "MaxNominations",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of eras to keep in history."]
                #[doc = ""]
                #[doc = " Following information is kept for eras in `[current_era -"]
                #[doc = " HistoryDepth, current_era]`: `ErasStakers`, `ErasStakersClipped`,"]
                #[doc = " `ErasValidatorPrefs`, `ErasValidatorReward`, `ErasRewardPoints`,"]
                #[doc = " `ErasTotalStake`, `ErasStartSessionIndex`,"]
                #[doc = " `StakingLedger.claimed_rewards`."]
                #[doc = ""]
                #[doc = " Must be more than the number of eras delayed by session."]
                #[doc = " I.e. active era must always be in history. I.e. `active_era >"]
                #[doc = " current_era - history_depth` must be guaranteed."]
                #[doc = ""]
                #[doc = " If migrating an existing pallet from storage value to config value,"]
                #[doc = " this should be set to same value or greater as in storage."]
                #[doc = ""]
                #[doc = " Note: `HistoryDepth` is used as the upper bound for the `BoundedVec`"]
                #[doc = " item `StakingLedger.claimed_rewards`. Setting this value lower than"]
                #[doc = " the existing value can lead to inconsistencies in the"]
                #[doc = " `StakingLedger` and will need to be handled properly in a migration."]
                #[doc = " The test `reducing_history_depth_abrupt` shows this effect."]
                pub fn history_depth(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Staking",
                        "HistoryDepth",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of sessions per era."]
                pub fn sessions_per_era(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Staking",
                        "SessionsPerEra",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of eras that staked funds must remain bonded for."]
                pub fn bonding_duration(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Staking",
                        "BondingDuration",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of eras that slashes are deferred by, after computation."]
                #[doc = ""]
                #[doc = " This should be less than the bonding duration. Set to 0 if slashes"]
                #[doc = " should be applied immediately, without opportunity for intervention."]
                pub fn slash_defer_duration(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Staking",
                        "SlashDeferDuration",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of nominators rewarded for each validator."]
                #[doc = ""]
                #[doc = " For each validator only the `$MaxNominatorRewardedPerValidator` biggest stakers can"]
                #[doc = " claim their reward. This used to limit the i/o cost for the nominator payout."]
                pub fn max_nominator_rewarded_per_validator(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Staking",
                        "MaxNominatorRewardedPerValidator",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of `unlocking` chunks a [`StakingLedger`] can"]
                #[doc = " have. Effectively determines how many unique eras a staker may be"]
                #[doc = " unbonding in."]
                #[doc = ""]
                #[doc = " Note: `MaxUnlockingChunks` is used as the upper bound for the"]
                #[doc = " `BoundedVec` item `StakingLedger.unlocking`. Setting this value"]
                #[doc = " lower than the existing value can lead to inconsistencies in the"]
                #[doc = " `StakingLedger` and will need to be handled properly in a runtime"]
                #[doc = " migration. The test `reducing_max_unlocking_chunks_abrupt` shows"]
                #[doc = " this effect."]
                pub fn max_unlocking_chunks(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Staking",
                        "MaxUnlockingChunks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod multisig {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_multisig::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_multisig::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AsMultiThreshold1 {
                    pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    pub call: ::std::boxed::Box<::core::primitive::bool>,
                }
                impl ::subxt::blocks::StaticExtrinsic for AsMultiThreshold1 {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "as_multi_threshold_1";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AsMulti {
                    pub threshold: ::core::primitive::u16,
                    pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    pub maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    pub call: ::std::boxed::Box<::core::primitive::bool>,
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for AsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "as_multi";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ApproveAsMulti {
                    pub threshold: ::core::primitive::u16,
                    pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    pub maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    pub call_hash: [::core::primitive::u8; 32usize],
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for ApproveAsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "approve_as_multi";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CancelAsMulti {
                    pub threshold: ::core::primitive::u16,
                    pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    pub timepoint:
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    pub call_hash: [::core::primitive::u8; 32usize],
                }
                impl ::subxt::blocks::StaticExtrinsic for CancelAsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "cancel_as_multi";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::as_multi_threshold_1`]."]
                pub fn as_multi_threshold_1(
                    &self,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    call: ::core::primitive::bool,
                ) -> ::subxt::tx::Payload<types::AsMultiThreshold1> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "as_multi_threshold_1",
                        types::AsMultiThreshold1 {
                            other_signatories,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            154u8, 28u8, 173u8, 102u8, 105u8, 102u8, 48u8, 6u8, 129u8, 79u8, 33u8,
                            108u8, 173u8, 74u8, 126u8, 246u8, 201u8, 84u8, 162u8, 43u8, 74u8,
                            170u8, 140u8, 54u8, 177u8, 254u8, 40u8, 177u8, 146u8, 248u8, 56u8,
                            136u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::as_multi`]."]
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::core::primitive::bool,
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::AsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "as_multi",
                        types::AsMulti {
                            threshold,
                            other_signatories,
                            maybe_timepoint,
                            call: ::std::boxed::Box::new(call),
                            max_weight,
                        },
                        [
                            91u8, 63u8, 101u8, 197u8, 83u8, 209u8, 167u8, 53u8, 148u8, 34u8, 204u8,
                            254u8, 246u8, 141u8, 203u8, 179u8, 232u8, 165u8, 25u8, 5u8, 48u8, 88u8,
                            151u8, 33u8, 146u8, 182u8, 248u8, 191u8, 134u8, 188u8, 252u8, 200u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::approve_as_multi`]."]
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::ApproveAsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "approve_as_multi",
                        types::ApproveAsMulti {
                            threshold,
                            other_signatories,
                            maybe_timepoint,
                            call_hash,
                            max_weight,
                        },
                        [
                            248u8, 46u8, 131u8, 35u8, 204u8, 12u8, 218u8, 150u8, 88u8, 131u8, 89u8,
                            13u8, 95u8, 122u8, 87u8, 107u8, 136u8, 154u8, 92u8, 199u8, 108u8, 92u8,
                            207u8, 171u8, 113u8, 8u8, 47u8, 248u8, 65u8, 26u8, 203u8, 135u8,
                        ],
                    )
                }
                #[doc = "See [`Pallet::cancel_as_multi`]."]
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::tx::Payload<types::CancelAsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "cancel_as_multi",
                        types::CancelAsMulti {
                            threshold,
                            other_signatories,
                            timepoint,
                            call_hash,
                        },
                        [
                            212u8, 179u8, 123u8, 40u8, 209u8, 228u8, 181u8, 0u8, 109u8, 28u8, 27u8,
                            48u8, 15u8, 47u8, 203u8, 54u8, 106u8, 114u8, 28u8, 118u8, 101u8, 201u8,
                            95u8, 187u8, 46u8, 182u8, 4u8, 30u8, 227u8, 105u8, 14u8, 81u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new multisig operation has begun."]
            pub struct NewMultisig {
                pub approving: ::subxt::utils::AccountId32,
                pub multisig: ::subxt::utils::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A multisig operation has been approved by someone."]
            pub struct MultisigApproval {
                pub approving: ::subxt::utils::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::utils::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A multisig operation has been executed."]
            pub struct MultisigExecuted {
                pub approving: ::subxt::utils::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::utils::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A multisig operation has been cancelled."]
            pub struct MultisigCancelled {
                pub cancelling: ::subxt::utils::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::utils::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The set of open multisig operations."]
                pub fn multisigs_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::utils::AccountId32,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Multisig",
                        "Multisigs",
                        vec![],
                        [
                            154u8, 109u8, 45u8, 18u8, 155u8, 151u8, 81u8, 28u8, 86u8, 127u8, 189u8,
                            151u8, 49u8, 61u8, 12u8, 149u8, 84u8, 61u8, 110u8, 197u8, 200u8, 140u8,
                            37u8, 100u8, 14u8, 162u8, 158u8, 161u8, 48u8, 117u8, 102u8, 61u8,
                        ],
                    )
                }
                #[doc = " The set of open multisig operations."]
                pub fn multisigs_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::utils::AccountId32,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Multisig",
                        "Multisigs",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            154u8, 109u8, 45u8, 18u8, 155u8, 151u8, 81u8, 28u8, 86u8, 127u8, 189u8,
                            151u8, 49u8, 61u8, 12u8, 149u8, 84u8, 61u8, 110u8, 197u8, 200u8, 140u8,
                            37u8, 100u8, 14u8, 162u8, 158u8, 161u8, 48u8, 117u8, 102u8, 61u8,
                        ],
                    )
                }
                #[doc = " The set of open multisig operations."]
                pub fn multisigs(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::utils::AccountId32,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Multisig",
                        "Multisigs",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            154u8, 109u8, 45u8, 18u8, 155u8, 151u8, 81u8, 28u8, 86u8, 127u8, 189u8,
                            151u8, 49u8, 61u8, 12u8, 149u8, 84u8, 61u8, 110u8, 197u8, 200u8, 140u8,
                            37u8, 100u8, 14u8, 162u8, 158u8, 161u8, 48u8, 117u8, 102u8, 61u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The base amount of currency needed to reserve for creating a multisig execution or to"]
                #[doc = " store a dispatch call for later."]
                #[doc = ""]
                #[doc = " This is held for an additional storage item whose value size is"]
                #[doc = " `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is"]
                #[doc = " `32 + sizeof(AccountId)` bytes."]
                pub fn deposit_base(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "DepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount of currency needed per unit threshold when creating a multisig execution."]
                #[doc = ""]
                #[doc = " This is held for adding 32 bytes more into a pre-existing storage value."]
                pub fn deposit_factor(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "DepositFactor",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum amount of signatories allowed in the multisig."]
                pub fn max_signatories(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "MaxSignatories",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod para_inherent {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Enter {
                    pub data: runtime_types::polkadot_primitives::v5::InherentData<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for Enter {
                    const PALLET: &'static str = "ParaInherent";
                    const CALL: &'static str = "enter";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "See [`Pallet::enter`]."]
                pub fn enter(
                    &self,
                    data: runtime_types::polkadot_primitives::v5::InherentData<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::tx::Payload<types::Enter> {
                    ::subxt::tx::Payload::new_static(
                        "ParaInherent",
                        "enter",
                        types::Enter { data },
                        [
                            145u8, 120u8, 158u8, 39u8, 139u8, 223u8, 236u8, 209u8, 253u8, 108u8,
                            188u8, 21u8, 23u8, 61u8, 25u8, 171u8, 30u8, 203u8, 161u8, 117u8, 90u8,
                            55u8, 50u8, 107u8, 26u8, 52u8, 26u8, 158u8, 56u8, 218u8, 186u8, 142u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Whether the paras inherent was included within this block."]
                #[doc = ""]
                #[doc = " The `Option<()>` is effectively a `bool`, but it never hits storage in the `None` variant"]
                #[doc = " due to the guarantees of FRAME's storage APIs."]
                #[doc = ""]
                #[doc = " If this is `None` at the end of the block, we panic and render the block invalid."]
                pub fn included(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (),
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParaInherent",
                        "Included",
                        vec![],
                        [
                            108u8, 164u8, 163u8, 34u8, 27u8, 124u8, 202u8, 167u8, 48u8, 130u8,
                            155u8, 211u8, 148u8, 130u8, 76u8, 16u8, 5u8, 250u8, 211u8, 174u8, 90u8,
                            77u8, 198u8, 153u8, 175u8, 168u8, 131u8, 244u8, 27u8, 93u8, 60u8, 46u8,
                        ],
                    )
                }
                #[doc = " Scraped on chain data for extracting resolved disputes as well as backing votes."]
                pub fn on_chain_votes(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::polkadot_primitives::v5::ScrapedOnChainVotes<
                        ::subxt::utils::H256,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ParaInherent",
                        "OnChainVotes",
                        vec![],
                        [
                            200u8, 210u8, 42u8, 153u8, 85u8, 71u8, 171u8, 108u8, 148u8, 212u8,
                            108u8, 61u8, 178u8, 77u8, 129u8, 90u8, 120u8, 218u8, 228u8, 152u8,
                            120u8, 226u8, 29u8, 82u8, 239u8, 146u8, 41u8, 164u8, 193u8, 207u8,
                            246u8, 115u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            :: subxt :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::remark`]."]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::set_heap_pages`]."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::set_code`]."]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::set_code_without_checks`]."]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::set_storage`]."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::kill_storage`]."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::kill_prefix`]."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::remark_with_event`]."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::utils::AccountId32,
                        hash: ::subxt::utils::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::transfer_allow_death`]."]
                    transfer_allow_death {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::set_balance_deprecated`]."]
                    set_balance_deprecated {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        old_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::force_transfer`]."]
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::transfer_keep_alive`]."]
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::transfer_all`]."]
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::force_unreserve`]."]
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::upgrade_accounts`]."]
                    upgrade_accounts {
                        who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::transfer`]."]
                    transfer {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "See [`Pallet::force_set_balance`]."]
                    force_set_balance {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `MaxHolds`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    #[doc = "An account was upgraded."]
                    Upgraded { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 15)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IdAmount<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::as_multi_threshold_1`]."]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        call: ::std::boxed::Box<::core::primitive::bool>,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::as_multi`]."]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::std::boxed::Box<::core::primitive::bool>,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::approve_as_multi`]."]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::cancel_as_multi`]."]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Threshold must be 2 or greater."]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    #[doc = "Call is already approved by this signatory."]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    #[doc = "Call doesn't need any (more) approvals."]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    #[doc = "There are too few signatories in the list."]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    #[doc = "There are too many signatories in the list."]
                    TooManySignatories,
                    #[codec(index = 5)]
                    #[doc = "The signatories were provided out of order; they should be ordered."]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    #[doc = "The sender was contained in the other signatories; it shouldn't be."]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    #[doc = "Multisig operation not found when attempting to cancel."]
                    NotFound,
                    #[codec(index = 8)]
                    #[doc = "Only the account that originally created the multisig is able to cancel it."]
                    NotOwner,
                    #[codec(index = 9)]
                    #[doc = "No timepoint was given, yet the multisig operation is already underway."]
                    NoTimepoint,
                    #[codec(index = 10)]
                    #[doc = "A different timepoint was given to the multisig operation that is underway."]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    #[doc = "A timepoint was given, yet no multisig operation is underway."]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    #[doc = "The maximum weight information provided was too low."]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    #[doc = "The data to be stored is already stored."]
                    AlreadyStored,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new multisig operation has begun."]
                    NewMultisig {
                        approving: ::subxt::utils::AccountId32,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    #[doc = "A multisig operation has been approved by someone."]
                    MultisigApproval {
                        approving: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "A multisig operation has been executed."]
                    MultisigExecuted {
                        approving: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    #[doc = "A multisig operation has been cancelled."]
                    MultisigCancelled {
                        cancelling: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: ::core::primitive::u32,
            }
        }
        pub mod pallet_staking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                    pub enum Call {
                        #[codec(index = 0)]
                        #[doc = "See [`Pallet::bond`]."]
                        bond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::utils::AccountId32,
                            >,
                        },
                        #[codec(index = 1)]
                        #[doc = "See [`Pallet::bond_extra`]."]
                        bond_extra {
                            #[codec(compact)]
                            max_additional: ::core::primitive::u128,
                        },
                        #[codec(index = 2)]
                        #[doc = "See [`Pallet::unbond`]."]
                        unbond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        #[codec(index = 3)]
                        #[doc = "See [`Pallet::withdraw_unbonded`]."]
                        withdraw_unbonded {
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        #[doc = "See [`Pallet::validate`]."]
                        validate {
                            prefs: runtime_types::pallet_staking::ValidatorPrefs,
                        },
                        #[codec(index = 5)]
                        #[doc = "See [`Pallet::nominate`]."]
                        nominate {
                            targets: ::std::vec::Vec<
                                ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                            >,
                        },
                        #[codec(index = 6)]
                        #[doc = "See [`Pallet::chill`]."]
                        chill,
                        #[codec(index = 7)]
                        #[doc = "See [`Pallet::set_payee`]."]
                        set_payee {
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::utils::AccountId32,
                            >,
                        },
                        #[codec(index = 8)]
                        #[doc = "See [`Pallet::set_controller`]."]
                        set_controller,
                        #[codec(index = 9)]
                        #[doc = "See [`Pallet::set_validator_count`]."]
                        set_validator_count {
                            #[codec(compact)]
                            new: ::core::primitive::u32,
                        },
                        #[codec(index = 10)]
                        #[doc = "See [`Pallet::increase_validator_count`]."]
                        increase_validator_count {
                            #[codec(compact)]
                            additional: ::core::primitive::u32,
                        },
                        #[codec(index = 11)]
                        #[doc = "See [`Pallet::scale_validator_count`]."]
                        scale_validator_count {
                            factor: runtime_types::sp_arithmetic::per_things::Percent,
                        },
                        #[codec(index = 12)]
                        #[doc = "See [`Pallet::force_no_eras`]."]
                        force_no_eras,
                        #[codec(index = 13)]
                        #[doc = "See [`Pallet::force_new_era`]."]
                        force_new_era,
                        #[codec(index = 14)]
                        #[doc = "See [`Pallet::set_invulnerables`]."]
                        set_invulnerables {
                            invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        },
                        #[codec(index = 15)]
                        #[doc = "See [`Pallet::force_unstake`]."]
                        force_unstake {
                            stash: ::subxt::utils::AccountId32,
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 16)]
                        #[doc = "See [`Pallet::force_new_era_always`]."]
                        force_new_era_always,
                        #[codec(index = 17)]
                        #[doc = "See [`Pallet::cancel_deferred_slash`]."]
                        cancel_deferred_slash {
                            era: ::core::primitive::u32,
                            slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                        },
                        #[codec(index = 18)]
                        #[doc = "See [`Pallet::payout_stakers`]."]
                        payout_stakers {
                            validator_stash: ::subxt::utils::AccountId32,
                            era: ::core::primitive::u32,
                        },
                        #[codec(index = 19)]
                        #[doc = "See [`Pallet::rebond`]."]
                        rebond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        #[codec(index = 20)]
                        #[doc = "See [`Pallet::reap_stash`]."]
                        reap_stash {
                            stash: ::subxt::utils::AccountId32,
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 21)]
                        #[doc = "See [`Pallet::kick`]."]
                        kick {
                            who: ::std::vec::Vec<
                                ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                            >,
                        },
                        #[codec(index = 22)]
                        #[doc = "See [`Pallet::set_staking_configs`]."]
                        set_staking_configs {
                            min_nominator_bond:
                                runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                    ::core::primitive::u128,
                                >,
                            min_validator_bond:
                                runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                    ::core::primitive::u128,
                                >,
                            max_nominator_count:
                                runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                    ::core::primitive::u32,
                                >,
                            max_validator_count:
                                runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                    ::core::primitive::u32,
                                >,
                            chill_threshold:
                                runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                    runtime_types::sp_arithmetic::per_things::Percent,
                                >,
                            min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                runtime_types::sp_arithmetic::per_things::Perbill,
                            >,
                        },
                        #[codec(index = 23)]
                        #[doc = "See [`Pallet::chill_other`]."]
                        chill_other {
                            controller: ::subxt::utils::AccountId32,
                        },
                        #[codec(index = 24)]
                        #[doc = "See [`Pallet::force_apply_min_commission`]."]
                        force_apply_min_commission {
                            validator_stash: ::subxt::utils::AccountId32,
                        },
                        #[codec(index = 25)]
                        #[doc = "See [`Pallet::set_min_commission`]."]
                        set_min_commission {
                            new: runtime_types::sp_arithmetic::per_things::Perbill,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum ConfigOp<_0> {
                        #[codec(index = 0)]
                        Noop,
                        #[codec(index = 1)]
                        Set(_0),
                        #[codec(index = 2)]
                        Remove,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    #[doc = "The `Error` enum of this pallet."]
                    pub enum Error {
                        #[codec(index = 0)]
                        #[doc = "Not a controller account."]
                        NotController,
                        #[codec(index = 1)]
                        #[doc = "Not a stash account."]
                        NotStash,
                        #[codec(index = 2)]
                        #[doc = "Stash is already bonded."]
                        AlreadyBonded,
                        #[codec(index = 3)]
                        #[doc = "Controller is already paired."]
                        AlreadyPaired,
                        #[codec(index = 4)]
                        #[doc = "Targets cannot be empty."]
                        EmptyTargets,
                        #[codec(index = 5)]
                        #[doc = "Duplicate index."]
                        DuplicateIndex,
                        #[codec(index = 6)]
                        #[doc = "Slash record index out of bounds."]
                        InvalidSlashIndex,
                        #[codec(index = 7)]
                        #[doc = "Cannot have a validator or nominator role, with value less than the minimum defined by"]
                        #[doc = "governance (see `MinValidatorBond` and `MinNominatorBond`). If unbonding is the"]
                        #[doc = "intention, `chill` first to remove one's role as validator/nominator."]
                        InsufficientBond,
                        #[codec(index = 8)]
                        #[doc = "Can not schedule more unlock chunks."]
                        NoMoreChunks,
                        #[codec(index = 9)]
                        #[doc = "Can not rebond without unlocking chunks."]
                        NoUnlockChunk,
                        #[codec(index = 10)]
                        #[doc = "Attempting to target a stash that still has funds."]
                        FundedTarget,
                        #[codec(index = 11)]
                        #[doc = "Invalid era to reward."]
                        InvalidEraToReward,
                        #[codec(index = 12)]
                        #[doc = "Invalid number of nominations."]
                        InvalidNumberOfNominations,
                        #[codec(index = 13)]
                        #[doc = "Items are not sorted and unique."]
                        NotSortedAndUnique,
                        #[codec(index = 14)]
                        #[doc = "Rewards for this era have already been claimed for this validator."]
                        AlreadyClaimed,
                        #[codec(index = 15)]
                        #[doc = "Incorrect previous history depth input provided."]
                        IncorrectHistoryDepth,
                        #[codec(index = 16)]
                        #[doc = "Incorrect number of slashing spans provided."]
                        IncorrectSlashingSpans,
                        #[codec(index = 17)]
                        #[doc = "Internal state has become somehow corrupted and the operation cannot continue."]
                        BadState,
                        #[codec(index = 18)]
                        #[doc = "Too many nomination targets supplied."]
                        TooManyTargets,
                        #[codec(index = 19)]
                        #[doc = "A nomination target was supplied that was blocked or otherwise not a validator."]
                        BadTarget,
                        #[codec(index = 20)]
                        #[doc = "The user has enough bond and thus cannot be chilled forcefully by an external person."]
                        CannotChillOther,
                        #[codec(index = 21)]
                        #[doc = "There are too many nominators in the system. Governance needs to adjust the staking"]
                        #[doc = "settings to keep things safe for the runtime."]
                        TooManyNominators,
                        #[codec(index = 22)]
                        #[doc = "There are too many validator candidates in the system. Governance needs to adjust the"]
                        #[doc = "staking settings to keep things safe for the runtime."]
                        TooManyValidators,
                        #[codec(index = 23)]
                        #[doc = "Commission is too low. Must be at least `MinCommission`."]
                        CommissionTooLow,
                        #[codec(index = 24)]
                        #[doc = "Some bound is not met."]
                        BoundNotMet,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    #[doc = "The `Event` enum of this pallet"]
                    pub enum Event {
                        #[codec(index = 0)]
                        #[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
                        #[doc = "the remainder from the maximum amount of reward."]
                        EraPaid {
                            era_index: ::core::primitive::u32,
                            validator_payout: ::core::primitive::u128,
                            remainder: ::core::primitive::u128,
                        },
                        #[codec(index = 1)]
                        #[doc = "The nominator has been rewarded by this amount."]
                        Rewarded {
                            stash: ::subxt::utils::AccountId32,
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 2)]
                        #[doc = "A staker (validator or nominator) has been slashed by the given amount."]
                        Slashed {
                            staker: ::subxt::utils::AccountId32,
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 3)]
                        #[doc = "A slash for the given validator, for the given percentage of their stake, at the given"]
                        #[doc = "era as been reported."]
                        SlashReported {
                            validator: ::subxt::utils::AccountId32,
                            fraction: runtime_types::sp_arithmetic::per_things::Perbill,
                            slash_era: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        #[doc = "An old slashing report from a prior era was discarded because it could"]
                        #[doc = "not be processed."]
                        OldSlashingReportDiscarded {
                            session_index: ::core::primitive::u32,
                        },
                        #[codec(index = 5)]
                        #[doc = "A new set of stakers was elected."]
                        StakersElected,
                        #[codec(index = 6)]
                        #[doc = "An account has bonded this amount. \\[stash, amount\\]"]
                        #[doc = ""]
                        #[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
                        #[doc = "it will not be emitted for staking rewards when they are added to stake."]
                        Bonded {
                            stash: ::subxt::utils::AccountId32,
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 7)]
                        #[doc = "An account has unbonded this amount."]
                        Unbonded {
                            stash: ::subxt::utils::AccountId32,
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 8)]
                        #[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
                        #[doc = "from the unlocking queue."]
                        Withdrawn {
                            stash: ::subxt::utils::AccountId32,
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 9)]
                        #[doc = "A nominator has been kicked from a validator."]
                        Kicked {
                            nominator: ::subxt::utils::AccountId32,
                            stash: ::subxt::utils::AccountId32,
                        },
                        #[codec(index = 10)]
                        #[doc = "The election failed. No new era is planned."]
                        StakingElectionFailed,
                        #[codec(index = 11)]
                        #[doc = "An account has stopped participating as either a validator or nominator."]
                        Chilled { stash: ::subxt::utils::AccountId32 },
                        #[codec(index = 12)]
                        #[doc = "The stakers' rewards are getting paid."]
                        PayoutStarted {
                            era_index: ::core::primitive::u32,
                            validator_stash: ::subxt::utils::AccountId32,
                        },
                        #[codec(index = 13)]
                        #[doc = "A validator has set their preferences."]
                        ValidatorPrefsSet {
                            stash: ::subxt::utils::AccountId32,
                            prefs: runtime_types::pallet_staking::ValidatorPrefs,
                        },
                        #[codec(index = 14)]
                        #[doc = "A new force era mode was set."]
                        ForceEra {
                            mode: runtime_types::pallet_staking::Forcing,
                        },
                    }
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SlashingSpans {
                    pub span_index: ::core::primitive::u32,
                    pub last_start: ::core::primitive::u32,
                    pub last_nonzero_slash: ::core::primitive::u32,
                    pub prior: ::std::vec::Vec<::core::primitive::u32>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SpanRecord<_0> {
                    pub slashed: _0,
                    pub paid_out: _0,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual: ::subxt::utils::KeyedVec<_0, ::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Exposure<_0, _1> {
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub own: _1,
                pub others:
                    ::std::vec::Vec<runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Forcing {
                #[codec(index = 0)]
                NotForcing,
                #[codec(index = 1)]
                ForceNew,
                #[codec(index = 2)]
                ForceNone,
                #[codec(index = 3)]
                ForceAlways,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                #[codec(compact)]
                pub value: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Nominations {
                pub targets: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::subxt::utils::AccountId32,
                >,
                pub submitted_in: ::core::primitive::u32,
                pub suppressed: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RewardDestination<_0> {
                #[codec(index = 0)]
                Staked,
                #[codec(index = 1)]
                Stash,
                #[codec(index = 2)]
                Controller,
                #[codec(index = 3)]
                Account(_0),
                #[codec(index = 4)]
                None,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct StakingLedger {
                pub stash: ::subxt::utils::AccountId32,
                #[codec(compact)]
                pub total: ::core::primitive::u128,
                #[codec(compact)]
                pub active: ::core::primitive::u128,
                pub unlocking: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    runtime_types::pallet_staking::UnlockChunk<::core::primitive::u128>,
                >,
                pub claimed_rewards: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct UnappliedSlash<_0, _1> {
                pub validator: _0,
                pub own: _1,
                pub others: ::std::vec::Vec<(_0, _1)>,
                pub reporters: ::std::vec::Vec<_0>,
                pub payout: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct UnlockChunk<_0> {
                #[codec(compact)]
                pub value: _0,
                #[codec(compact)]
                pub era: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ValidatorPrefs {
                #[codec(compact)]
                pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
                pub blocked: ::core::primitive::bool,
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::set`]."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FeeDetails<_0> {
                    pub inclusion_fee: ::core::option::Option<
                        runtime_types::pallet_transaction_payment::types::InclusionFee<_0>,
                    >,
                    pub tip: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InclusionFee<_0> {
                    pub base_fee: _0,
                    pub len_fee: _0,
                    pub adjusted_weight_fee: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RuntimeDispatchInfo<_0, _1> {
                    pub weight: _1,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub partial_fee: _0,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CandidateHash(pub ::subxt::utils::H256);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Id(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidationCode(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidationCodeHash(pub ::subxt::utils::H256);
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v5 {
                use super::runtime_types;
                pub mod assignment_app {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
                pub mod collator_app {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
                pub mod executor_params {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum ExecutorParam {
                        #[codec(index = 1)]
                        MaxMemoryPages(::core::primitive::u32),
                        #[codec(index = 2)]
                        StackLogicalMax(::core::primitive::u32),
                        #[codec(index = 3)]
                        StackNativeMax(::core::primitive::u32),
                        #[codec(index = 4)]
                        PrecheckingMaxMemory(::core::primitive::u64),
                        #[codec(index = 5)]
                        PvfPrepTimeout(
                            runtime_types::polkadot_primitives::v5::PvfPrepTimeoutKind,
                            ::core::primitive::u64,
                        ),
                        #[codec(index = 6)]
                        PvfExecTimeout(
                            runtime_types::polkadot_primitives::v5::PvfExecTimeoutKind,
                            ::core::primitive::u64,
                        ),
                        #[codec(index = 7)]
                        WasmExtBulkMemory,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct ExecutorParams(
                        pub  ::std::vec::Vec<
                            runtime_types::polkadot_primitives::v5::executor_params::ExecutorParam,
                        >,
                    );
                }
                pub mod signed {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct UncheckedSigned<_0, _1> {
                        pub payload: _0,
                        pub validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex,
                        pub signature:
                            runtime_types::polkadot_primitives::v5::validator_app::Signature,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod slashing {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct DisputeProof {
                        pub time_slot:
                            runtime_types::polkadot_primitives::v5::slashing::DisputesTimeSlot,
                        pub kind:
                            runtime_types::polkadot_primitives::v5::slashing::SlashingOffenceKind,
                        pub validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex,
                        pub validator_id:
                            runtime_types::polkadot_primitives::v5::validator_app::Public,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct DisputesTimeSlot {
                        pub session_index: ::core::primitive::u32,
                        pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct PendingSlashes {
                        pub keys: ::subxt::utils::KeyedVec<
                            runtime_types::polkadot_primitives::v5::ValidatorIndex,
                            runtime_types::polkadot_primitives::v5::validator_app::Public,
                        >,
                        pub kind:
                            runtime_types::polkadot_primitives::v5::slashing::SlashingOffenceKind,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum SlashingOffenceKind {
                        #[codec(index = 0)]
                        ForInvalid,
                        #[codec(index = 1)]
                        AgainstValid,
                    }
                }
                pub mod validator_app {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AvailabilityBitfield(
                    pub  ::subxt::utils::bits::DecodedBits<
                        ::core::primitive::u8,
                        ::subxt::utils::bits::Lsb0,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BackedCandidate<_0> {
                    pub candidate:
                        runtime_types::polkadot_primitives::v5::CommittedCandidateReceipt<_0>,
                    pub validity_votes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::ValidityAttestation,
                    >,
                    pub validator_indices: ::subxt::utils::bits::DecodedBits<
                        ::core::primitive::u8,
                        ::subxt::utils::bits::Lsb0,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CandidateCommitments<_0> {
                    pub upward_messages:
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::std::vec::Vec<::core::primitive::u8>,
                        >,
                    pub horizontal_messages:
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                                runtime_types::polkadot_parachain::primitives::Id,
                            >,
                        >,
                    pub new_validation_code: ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                    pub head_data: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub processed_downward_messages: ::core::primitive::u32,
                    pub hrmp_watermark: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CandidateDescriptor<_0> {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub relay_parent: _0,
                    pub collator: runtime_types::polkadot_primitives::v5::collator_app::Public,
                    pub persisted_validation_data_hash: ::subxt::utils::H256,
                    pub pov_hash: ::subxt::utils::H256,
                    pub erasure_root: ::subxt::utils::H256,
                    pub signature: runtime_types::polkadot_primitives::v5::collator_app::Signature,
                    pub para_head: ::subxt::utils::H256,
                    pub validation_code_hash:
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum CandidateEvent<_0> {
                    #[codec(index = 0)]
                    CandidateBacked(
                        runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                        runtime_types::polkadot_parachain::primitives::HeadData,
                        runtime_types::polkadot_primitives::v5::CoreIndex,
                        runtime_types::polkadot_primitives::v5::GroupIndex,
                    ),
                    #[codec(index = 1)]
                    CandidateIncluded(
                        runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                        runtime_types::polkadot_parachain::primitives::HeadData,
                        runtime_types::polkadot_primitives::v5::CoreIndex,
                        runtime_types::polkadot_primitives::v5::GroupIndex,
                    ),
                    #[codec(index = 2)]
                    CandidateTimedOut(
                        runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                        runtime_types::polkadot_parachain::primitives::HeadData,
                        runtime_types::polkadot_primitives::v5::CoreIndex,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CandidateReceipt<_0> {
                    pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
                    pub commitments_hash: ::subxt::utils::H256,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CommittedCandidateReceipt<_0> {
                    pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
                    pub commitments: runtime_types::polkadot_primitives::v5::CandidateCommitments<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CoreIndex(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum CoreState<_0, _1> {
                    #[codec(index = 0)]
                    Occupied(runtime_types::polkadot_primitives::v5::OccupiedCore<_0, _1>),
                    #[codec(index = 1)]
                    Scheduled(runtime_types::polkadot_primitives::v5::ScheduledCore),
                    #[codec(index = 2)]
                    Free,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DisputeState<_0> {
                    pub validators_for: ::subxt::utils::bits::DecodedBits<
                        ::core::primitive::u8,
                        ::subxt::utils::bits::Lsb0,
                    >,
                    pub validators_against: ::subxt::utils::bits::DecodedBits<
                        ::core::primitive::u8,
                        ::subxt::utils::bits::Lsb0,
                    >,
                    pub start: _0,
                    pub concluded_at: ::core::option::Option<_0>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DisputeStatement {
                    #[codec(index = 0)]
                    Valid(runtime_types::polkadot_primitives::v5::ValidDisputeStatementKind),
                    #[codec(index = 1)]
                    Invalid(runtime_types::polkadot_primitives::v5::InvalidDisputeStatementKind),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DisputeStatementSet {
                    pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
                    pub session: ::core::primitive::u32,
                    pub statements: ::std::vec::Vec<(
                        runtime_types::polkadot_primitives::v5::DisputeStatement,
                        runtime_types::polkadot_primitives::v5::ValidatorIndex,
                        runtime_types::polkadot_primitives::v5::validator_app::Signature,
                    )>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GroupIndex(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GroupRotationInfo<_0> {
                    pub session_start_block: _0,
                    pub group_rotation_frequency: _0,
                    pub now: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IndexedVec<_0, _1>(
                    pub ::std::vec::Vec<_1>,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InherentData<_0> {
                    pub bitfields: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::signed::UncheckedSigned<
                            runtime_types::polkadot_primitives::v5::AvailabilityBitfield,
                            runtime_types::polkadot_primitives::v5::AvailabilityBitfield,
                        >,
                    >,
                    pub backed_candidates: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::BackedCandidate<
                            ::subxt::utils::H256,
                        >,
                    >,
                    pub disputes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::DisputeStatementSet,
                    >,
                    pub parent_header: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum InvalidDisputeStatementKind {
                    #[codec(index = 0)]
                    Explicit,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct OccupiedCore<_0, _1> {
                    pub next_up_on_available: ::core::option::Option<
                        runtime_types::polkadot_primitives::v5::ScheduledCore,
                    >,
                    pub occupied_since: _1,
                    pub time_out_at: _1,
                    pub next_up_on_time_out: ::core::option::Option<
                        runtime_types::polkadot_primitives::v5::ScheduledCore,
                    >,
                    pub availability: ::subxt::utils::bits::DecodedBits<
                        ::core::primitive::u8,
                        ::subxt::utils::bits::Lsb0,
                    >,
                    pub group_responsible: runtime_types::polkadot_primitives::v5::GroupIndex,
                    pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
                    pub candidate_descriptor:
                        runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum OccupiedCoreAssumption {
                    #[codec(index = 0)]
                    Included,
                    #[codec(index = 1)]
                    TimedOut,
                    #[codec(index = 2)]
                    Free,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PersistedValidationData<_0, _1> {
                    pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub relay_parent_number: _1,
                    pub relay_parent_storage_root: _0,
                    pub max_pov_size: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PvfCheckStatement {
                    pub accept: ::core::primitive::bool,
                    pub subject: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    pub session_index: ::core::primitive::u32,
                    pub validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum PvfExecTimeoutKind {
                    #[codec(index = 0)]
                    Backing,
                    #[codec(index = 1)]
                    Approval,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum PvfPrepTimeoutKind {
                    #[codec(index = 0)]
                    Precheck,
                    #[codec(index = 1)]
                    Lenient,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ScheduledCore {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub collator: ::core::option::Option<
                        runtime_types::polkadot_primitives::v5::collator_app::Public,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ScrapedOnChainVotes<_0> {
                    pub session: ::core::primitive::u32,
                    pub backing_validators_per_candidate: ::std::vec::Vec<(
                        runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                        ::std::vec::Vec<(
                            runtime_types::polkadot_primitives::v5::ValidatorIndex,
                            runtime_types::polkadot_primitives::v5::ValidityAttestation,
                        )>,
                    )>,
                    pub disputes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::DisputeStatementSet,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SessionInfo {
                    pub active_validator_indices:
                        ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
                    pub random_seed: [::core::primitive::u8; 32usize],
                    pub dispute_period: ::core::primitive::u32,
                    pub validators: runtime_types::polkadot_primitives::v5::IndexedVec<
                        runtime_types::polkadot_primitives::v5::ValidatorIndex,
                        runtime_types::polkadot_primitives::v5::validator_app::Public,
                    >,
                    pub discovery_keys:
                        ::std::vec::Vec<runtime_types::sp_authority_discovery::app::Public>,
                    pub assignment_keys: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::assignment_app::Public,
                    >,
                    pub validator_groups: runtime_types::polkadot_primitives::v5::IndexedVec<
                        runtime_types::polkadot_primitives::v5::GroupIndex,
                        ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
                    >,
                    pub n_cores: ::core::primitive::u32,
                    pub zeroth_delay_tranche_width: ::core::primitive::u32,
                    pub relay_vrf_modulo_samples: ::core::primitive::u32,
                    pub n_delay_tranches: ::core::primitive::u32,
                    pub no_show_slots: ::core::primitive::u32,
                    pub needed_approvals: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum ValidDisputeStatementKind {
                    #[codec(index = 0)]
                    Explicit,
                    #[codec(index = 1)]
                    BackingSeconded(::subxt::utils::H256),
                    #[codec(index = 2)]
                    BackingValid(::subxt::utils::H256),
                    #[codec(index = 3)]
                    ApprovalChecking,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidatorIndex(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum ValidityAttestation {
                    #[codec(index = 1)]
                    Implicit(runtime_types::polkadot_primitives::v5::validator_app::Signature),
                    #[codec(index = 2)]
                    Explicit(runtime_types::polkadot_primitives::v5::validator_app::Signature),
                }
            }
        }
        pub mod polkadot_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 3)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 7)]
                Staking(runtime_types::pallet_staking::pallet::pallet::Call),
                #[codec(index = 30)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 54)]
                ParaInherent(
                    runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Call,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 7)]
                Staking(runtime_types::pallet_staking::pallet::pallet::Error),
                #[codec(index = 30)]
                Multisig(runtime_types::pallet_multisig::pallet::Error),
                #[codec(index = 54)]
                ParaInherent(
                    runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Error,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 7)]
                Staking(runtime_types::pallet_staking::pallet::pallet::Event),
                #[codec(index = 30)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeHoldReason {}
        }
        pub mod polkadot_runtime_common {
            use super::runtime_types;
            pub mod claims {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PrevalidateAttests;
            }
        }
        pub mod polkadot_runtime_parachains {
            use super::runtime_types;
            pub mod paras_inherent {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                    pub enum Call {
                        #[codec(index = 0)]
                        #[doc = "See [`Pallet::enter`]."]
                        enter {
                            data: runtime_types::polkadot_primitives::v5::InherentData<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                            >,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    #[doc = "The `Error` enum of this pallet."]
                    pub enum Error {
                        #[codec(index = 0)]
                        #[doc = "Inclusion inherent called more than once per block."]
                        TooManyInclusionInherents,
                        #[codec(index = 1)]
                        #[doc = "The hash of the submitted parent header doesn't correspond to the saved block hash of"]
                        #[doc = "the parent."]
                        InvalidParentHeader,
                        #[codec(index = 2)]
                        #[doc = "Disputed candidate that was concluded invalid."]
                        CandidateConcludedInvalid,
                        #[codec(index = 3)]
                        #[doc = "The data given to the inherent will result in an overweight block."]
                        InherentOverweight,
                        #[codec(index = 4)]
                        #[doc = "The ordering of dispute statements was invalid."]
                        DisputeStatementsUnsortedOrDuplicates,
                        #[codec(index = 5)]
                        #[doc = "A dispute statement was invalid."]
                        DisputeInvalid,
                    }
                }
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Percent(pub ::core::primitive::u8);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BabeConfiguration {
                pub slot_duration: ::core::primitive::u64,
                pub epoch_length: ::core::primitive::u64,
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub authorities: ::std::vec::Vec<(
                    runtime_types::sp_consensus_babe::app::Public,
                    ::core::primitive::u64,
                )>,
                pub randomness: [::core::primitive::u8; 32usize],
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Epoch {
                pub epoch_index: ::core::primitive::u64,
                pub start_slot: runtime_types::sp_consensus_slots::Slot,
                pub duration: ::core::primitive::u64,
                pub authorities: ::std::vec::Vec<(
                    runtime_types::sp_consensus_babe::app::Public,
                    ::core::primitive::u64,
                )>,
                pub randomness: [::core::primitive::u8; 32usize],
                pub config: runtime_types::sp_consensus_babe::BabeEpochConfiguration,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_consensus_beefy {
            use super::runtime_types;
            pub mod commitment {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Commitment<_0> {
                    pub payload: runtime_types::sp_consensus_beefy::payload::Payload,
                    pub block_number: _0,
                    pub validator_set_id: ::core::primitive::u64,
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub runtime_types::sp_core::ecdsa::Signature);
            }
            pub mod payload {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Payload(
                    pub  ::std::vec::Vec<(
                        [::core::primitive::u8; 2usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                );
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1, _2> {
                pub first: runtime_types::sp_consensus_beefy::VoteMessage<_0, _1, _2>,
                pub second: runtime_types::sp_consensus_beefy::VoteMessage<_0, _1, _2>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ValidatorSet<_0> {
                pub validators: ::std::vec::Vec<_0>,
                pub id: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct VoteMessage<_0, _1, _2> {
                pub commitment: runtime_types::sp_consensus_beefy::commitment::Commitment<_0>,
                pub id: _1,
                pub signature: _2,
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueMetadata(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_inherents {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CheckInherentsResult {
                pub okay: ::core::primitive::bool,
                pub fatal_error: ::core::primitive::bool,
                pub errors: runtime_types::sp_inherents::InherentData,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InherentData {
                pub data: ::subxt::utils::KeyedVec<
                    [::core::primitive::u8; 8usize],
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
            }
        }
        pub mod sp_mmr_primitives {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EncodableOpaqueLeaf(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Error {
                #[codec(index = 0)]
                InvalidNumericOp,
                #[codec(index = 1)]
                Push,
                #[codec(index = 2)]
                GetRoot,
                #[codec(index = 3)]
                Commit,
                #[codec(index = 4)]
                GenerateProof,
                #[codec(index = 5)]
                Verify,
                #[codec(index = 6)]
                LeafNotFound,
                #[codec(index = 7)]
                PalletNotIncluded,
                #[codec(index = 8)]
                InvalidLeafIndex,
                #[codec(index = 9)]
                InvalidBestKnownBlock,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Proof<_0> {
                pub leaf_indices: ::std::vec::Vec<::core::primitive::u64>,
                pub leaf_count: ::core::primitive::u64,
                pub items: ::std::vec::Vec<_0>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod block {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Block<_0, _1> {
                        pub header: _0,
                        pub extrinsics: ::std::vec::Vec<_1>,
                    }
                }
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::utils::H256,
                        pub extrinsics_root: ::subxt::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlakeTwo256;
            }
            pub mod transaction_validity {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum InvalidTransaction {
                    #[codec(index = 0)]
                    Call,
                    #[codec(index = 1)]
                    Payment,
                    #[codec(index = 2)]
                    Future,
                    #[codec(index = 3)]
                    Stale,
                    #[codec(index = 4)]
                    BadProof,
                    #[codec(index = 5)]
                    AncientBirthBlock,
                    #[codec(index = 6)]
                    ExhaustsResources,
                    #[codec(index = 7)]
                    Custom(::core::primitive::u8),
                    #[codec(index = 8)]
                    BadMandatory,
                    #[codec(index = 9)]
                    MandatoryValidation,
                    #[codec(index = 10)]
                    BadSigner,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TransactionSource {
                    #[codec(index = 0)]
                    InBlock,
                    #[codec(index = 1)]
                    Local,
                    #[codec(index = 2)]
                    External,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TransactionValidityError {
                    #[codec(index = 0)]
                    Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                    #[codec(index = 1)]
                    Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum UnknownTransaction {
                    #[codec(index = 0)]
                    CannotLookup,
                    #[codec(index = 1)]
                    NoUnsignedValidator,
                    #[codec(index = 2)]
                    Custom(::core::primitive::u8),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidTransaction {
                    pub priority: ::core::primitive::u64,
                    pub requires: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    pub provides: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    pub longevity: ::core::primitive::u64,
                    pub propagate: ::core::primitive::bool,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
    }
}
