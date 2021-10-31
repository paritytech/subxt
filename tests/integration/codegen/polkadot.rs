#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 1)]
        Utility(utility::Event),
        #[codec(index = 5)]
        Indices(indices::Event),
        #[codec(index = 6)]
        Balances(balances::Event),
        #[codec(index = 8)]
        ElectionProviderMultiPhase(election_provider_multi_phase::Event),
        #[codec(index = 9)]
        Staking(staking::Event),
        #[codec(index = 10)]
        Session(session::Event),
        #[codec(index = 11)]
        Democracy(democracy::Event),
        #[codec(index = 12)]
        Council(council::Event),
        #[codec(index = 13)]
        TechnicalCommittee(technical_committee::Event),
        #[codec(index = 14)]
        Elections(elections::Event),
        #[codec(index = 15)]
        TechnicalMembership(technical_membership::Event),
        #[codec(index = 16)]
        Grandpa(grandpa::Event),
        #[codec(index = 17)]
        Treasury(treasury::Event),
        #[codec(index = 18)]
        Contracts(contracts::Event),
        #[codec(index = 19)]
        Sudo(sudo::Event),
        #[codec(index = 20)]
        ImOnline(im_online::Event),
        #[codec(index = 22)]
        Offences(offences::Event),
        #[codec(index = 25)]
        Identity(identity::Event),
        #[codec(index = 26)]
        Society(society::Event),
        #[codec(index = 27)]
        Recovery(recovery::Event),
        #[codec(index = 28)]
        Vesting(vesting::Event),
        #[codec(index = 29)]
        Scheduler(scheduler::Event),
        #[codec(index = 30)]
        Proxy(proxy::Event),
        #[codec(index = 31)]
        Multisig(multisig::Event),
        #[codec(index = 32)]
        Bounties(bounties::Event),
        #[codec(index = 33)]
        Tips(tips::Event),
        #[codec(index = 34)]
        Assets(assets::Event),
        #[codec(index = 36)]
        Lottery(lottery::Event),
        #[codec(index = 37)]
        Gilt(gilt::Event),
        #[codec(index = 38)]
        Uniques(uniques::Event),
        #[codec(index = 39)]
        TransactionStorage(transaction_storage::Event),
        #[codec(index = 40)]
        BagsList(bags_list::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct FillBlock {
                pub ratio: ::subxt::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetChangesTrieConfig {
                pub changes_trie_config: ::core::option::Option<
                    runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                >,
            }
            impl ::subxt::Call for SetChangesTrieConfig {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_changes_trie_config";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn fill_block(
                    &self,
                    ratio: ::subxt::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<T, FillBlock> {
                    let call = FillBlock { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Remark> {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHeapPages> {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCode> {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCodeWithoutChecks>
                {
                    let call = SetCodeWithoutChecks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_changes_trie_config(
                    &self,
                    changes_trie_config: ::core::option::Option<
                        runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetChangesTrieConfig>
                {
                    let call = SetChangesTrieConfig {
                        changes_trie_config,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetStorage> {
                    let call = SetStorage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<T, KillStorage> {
                    let call = KillStorage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, KillPrefix> {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, RemarkWithEvent> {
                    let call = RemarkWithEvent { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtrinsicSuccess(
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CodeUpdated {}
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value = runtime_types::frame_support::weights::PerDispatchClass<
                    ::core::primitive::u64,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value = runtime_types::sp_runtime::generic::digest::Digest<
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::node_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value =
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u64,
                    >,
                    ::subxt::Error,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BlockHash>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::Error,
                > {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::node_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EventTopics>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    >,
                    ::subxt::Error,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::Error,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod utility {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Batch {
                pub calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
            }
            impl ::subxt::Call for Batch {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsDerivative {
                pub index: ::core::primitive::u16,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for AsDerivative {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "as_derivative";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchAll {
                pub calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
            }
            impl ::subxt::Call for BatchAll {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch_all";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, Batch> {
                    let call = Batch { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsDerivative> {
                    let call = AsDerivative { index, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, BatchAll> {
                    let call = BatchAll { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchInterrupted(
                pub ::core::primitive::u32,
                pub runtime_types::sp_runtime::DispatchError,
            );
            impl ::subxt::Event for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchCompleted {}
            impl ::subxt::Event for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ItemCompleted {}
            impl ::subxt::Event for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
        }
    }
    pub mod babe {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocation {
                pub equivocation_proof:
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof:
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PlanConfigChange {
                pub config:
                    runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
            impl ::subxt::Call for PlanConfigChange {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "plan_config_change";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof : runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocation>
                {
                    let call = ReportEquivocation {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof : runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocationUnsigned>
                {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn plan_config_change(
                    &self,
                    config : runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor,
                ) -> ::subxt::SubmittableExtrinsic<T, PlanConfigChange> {
                    let call = PlanConfigChange { config };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EpochIndex;
            impl ::subxt::StorageEntry for EpochIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochIndex";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , :: core :: primitive :: u64 ,) > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct GenesisSlot;
            impl ::subxt::StorageEntry for GenesisSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "GenesisSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Randomness;
            impl ::subxt::StorageEntry for Randomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Randomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingEpochConfigChange;
            impl ::subxt::StorageEntry for PendingEpochConfigChange {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "PendingEpochConfigChange";
                type Value =
                    runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextRandomness;
            impl ::subxt::StorageEntry for NextRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextRandomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , :: core :: primitive :: u64 ,) > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnderConstruction {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value =
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Initialized;
            impl ::subxt::StorageEntry for Initialized {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Initialized";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochConfig;
            impl ::subxt::StorageEntry for EpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextEpochConfig;
            impl ::subxt::StorageEntry for NextEpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextEpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn epoch_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn authorities (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , :: core :: primitive :: u64 ,) > , :: subxt :: Error >{
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn genesis_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::Error,
                > {
                    let entry = GenesisSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::Error,
                > {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    [::core::primitive::u8; 32usize],
                    ::subxt::Error,
                > {
                    let entry = Randomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_epoch_config_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingEpochConfigChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    [::core::primitive::u8; 32usize],
                    ::subxt::Error,
                > {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn next_authorities (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , :: core :: primitive :: u64 ,) > , :: subxt :: Error >{
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >,
                    ::subxt::Error,
                > {
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::Error,
                > {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::Error,
                > {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Lateness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::Error,
                > {
                    let entry = EpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NextEpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, Set> {
                    let call = Set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod authorship {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetUncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for SetUncles {
                const PALLET: &'static str = "Authorship";
                const FUNCTION: &'static str = "set_uncles";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetUncles> {
                    let call = SetUncles { new_uncles };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Uncles;
            impl ::subxt::StorageEntry for Uncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Uncles";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_authorship::UncleEntryItem<
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Author;
            impl ::subxt::StorageEntry for Author {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Author";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetUncles;
            impl ::subxt::StorageEntry for DidSetUncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "DidSetUncles";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_authorship::UncleEntryItem<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Uncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod indices {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Claim {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Claim {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "claim";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Free {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Free {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "free";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceTransfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
                pub freeze: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Freeze {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "freeze";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn claim(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Claim> {
                    let call = Claim { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { new, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn free(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Free> {
                    let call = Free { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                    freeze: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
                    let call = ForceTransfer { new, index, freeze };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Freeze> {
                    let call = Freeze { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_indices::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IndexAssigned(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for IndexAssigned {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexAssigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IndexFreed(pub ::core::primitive::u32);
            impl ::subxt::Event for IndexFreed {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFreed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IndexFrozen(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for IndexFrozen {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFrozen";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Accounts(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Accounts {
                const PALLET: &'static str = "Indices";
                const STORAGE: &'static str = "Accounts";
                type Value = (
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::bool,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn accounts(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::bool,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Accounts(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn accounts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Accounts>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetBalance {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceTransfer {
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferAll {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceUnreserve {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetBalance> {
                    let call = SetBalance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
                    let call = ForceTransfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferKeepAlive> {
                    let call = TransferKeepAlive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferAll> {
                    let call = TransferAll { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceUnreserve> {
                    let call = ForceUnreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Endowed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DustLost(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveRepatriated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            );
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdraw(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value =
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value =
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn locks (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > , :: subxt :: Error >{
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reserves>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::Error,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod election_provider_multi_phase {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubmitUnsigned { pub raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: node_runtime :: NposSolution16 > , pub witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize }
            impl ::subxt::Call for SubmitUnsigned {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMinimumUntrustedScore {
                pub maybe_next_score:
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
            }
            impl ::subxt::Call for SetMinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_minimum_untrusted_score";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetEmergencyElectionResult {
                pub supports: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_npos_elections::Support<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                )>,
            }
            impl ::subxt::Call for SetEmergencyElectionResult {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_emergency_election_result";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Submit {
                pub raw_solution:
                    runtime_types::pallet_election_provider_multi_phase::RawSolution<
                        runtime_types::node_runtime::NposSolution16,
                    >,
                pub num_signed_submissions: ::core::primitive::u32,
            }
            impl ::subxt::Call for Submit {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn submit_unsigned(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: node_runtime :: NposSolution16 >,
                    witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize,
                ) -> ::subxt::SubmittableExtrinsic<T, SubmitUnsigned> {
                    let call = SubmitUnsigned {
                        raw_solution,
                        witness,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_minimum_untrusted_score(
                    &self,
                    maybe_next_score: ::core::option::Option<
                        [::core::primitive::u128; 3usize],
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMinimumUntrustedScore>
                {
                    let call = SetMinimumUntrustedScore { maybe_next_score };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_emergency_election_result(
                    &self,
                    supports: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::sp_npos_elections::Support<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetEmergencyElectionResult>
                {
                    let call = SetEmergencyElectionResult { supports };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: node_runtime :: NposSolution16 >,
                    num_signed_submissions: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Submit> {
                    let call = Submit {
                        raw_solution,
                        num_signed_submissions,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event =
            runtime_types::pallet_election_provider_multi_phase::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SolutionStored(
                pub runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for SolutionStored {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SolutionStored";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ElectionFinalized(
                pub  ::core::option::Option<
                    runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                >,
            );
            impl ::subxt::Event for ElectionFinalized {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "ElectionFinalized";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SignedPhaseStarted(pub ::core::primitive::u32);
            impl ::subxt::Event for SignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SignedPhaseStarted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnsignedPhaseStarted(pub ::core::primitive::u32);
            impl ::subxt::Event for UnsignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "UnsignedPhaseStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Round;
            impl ::subxt::StorageEntry for Round {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Round";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPhase;
            impl ::subxt::StorageEntry for CurrentPhase {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "CurrentPhase";
                type Value = runtime_types::pallet_election_provider_multi_phase::Phase<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedSolution;
            impl ::subxt::StorageEntry for QueuedSolution {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "QueuedSolution";
                type Value =
                    runtime_types::pallet_election_provider_multi_phase::ReadySolution<
                        ::subxt::sp_core::crypto::AccountId32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Snapshot;
            impl ::subxt::StorageEntry for Snapshot {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Snapshot";
                type Value =
                    runtime_types::pallet_election_provider_multi_phase::RoundSnapshot<
                        ::subxt::sp_core::crypto::AccountId32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DesiredTargets;
            impl ::subxt::StorageEntry for DesiredTargets {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "DesiredTargets";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SnapshotMetadata;
            impl ::subxt::StorageEntry for SnapshotMetadata {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SnapshotMetadata";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionNextIndex;
            impl ::subxt::StorageEntry for SignedSubmissionNextIndex {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionNextIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionIndices;
            impl ::subxt::StorageEntry for SignedSubmissionIndices {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionIndices";
                type Value = runtime_types :: frame_support :: storage :: bounded_btree_map :: BoundedBTreeMap < [:: core :: primitive :: u128 ; 3usize] , :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionsMap(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for SignedSubmissionsMap {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionsMap";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: node_runtime :: NposSolution16 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct MinimumUntrustedScore;
            impl ::subxt::StorageEntry for MinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "MinimumUntrustedScore";
                type Value = [::core::primitive::u128; 3usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn round(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Round;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentPhase;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn queued_solution (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: ReadySolution < :: subxt :: sp_core :: crypto :: AccountId32 > > , :: subxt :: Error >{
                    let entry = QueuedSolution;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn snapshot (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: RoundSnapshot < :: subxt :: sp_core :: crypto :: AccountId32 > > , :: subxt :: Error >{
                    let entry = Snapshot;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn desired_targets(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = DesiredTargets;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn snapshot_metadata (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize > , :: subxt :: Error >{
                    let entry = SnapshotMetadata;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_submission_next_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = SignedSubmissionNextIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submission_indices (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_btree_map :: BoundedBTreeMap < [:: core :: primitive :: u128 ; 3usize] , :: core :: primitive :: u32 > , :: subxt :: Error >{
                    let entry = SignedSubmissionIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submissions_map (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: node_runtime :: NposSolution16 > , :: subxt :: Error >{
                    let entry = SignedSubmissionsMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signed_submissions_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignedSubmissionsMap>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn minimum_untrusted_score(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
                    ::subxt::Error,
                > {
                    let entry = MinimumUntrustedScore;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod staking {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bond {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for Bond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BondExtra {
                #[codec(compact)]
                pub max_additional: ::core::primitive::u128,
            }
            impl ::subxt::Call for BondExtra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond_extra";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unbond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Unbond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "unbond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct WithdrawUnbonded {
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for WithdrawUnbonded {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "withdraw_unbonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Validate {
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            impl ::subxt::Call for Validate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "validate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Nominate {
                pub targets: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for Nominate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "nominate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Chill {}
            impl ::subxt::Call for Chill {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetPayee {
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for SetPayee {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_payee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetController {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetController {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_controller";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetValidatorCount {
                #[codec(compact)]
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IncreaseValidatorCount {
                #[codec(compact)]
                pub additional: ::core::primitive::u32,
            }
            impl ::subxt::Call for IncreaseValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "increase_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScaleValidatorCount {
                pub factor: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for ScaleValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "scale_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceNoEras {}
            impl ::subxt::Call for ForceNoEras {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_no_eras";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceNewEra {}
            impl ::subxt::Call for ForceNewEra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetInvulnerables {
                pub invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for SetInvulnerables {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_invulnerables";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceUnstake {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceUnstake {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_unstake";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceNewEraAlways {}
            impl ::subxt::Call for ForceNewEraAlways {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era_always";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelDeferredSlash {
                pub era: ::core::primitive::u32,
                pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for CancelDeferredSlash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "cancel_deferred_slash";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PayoutStakers {
                pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
                pub era: ::core::primitive::u32,
            }
            impl ::subxt::Call for PayoutStakers {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "payout_stakers";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rebond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Rebond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "rebond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHistoryDepth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHistoryDepth {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_history_depth";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReapStash {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for ReapStash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "reap_stash";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Kick {
                pub who: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for Kick {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "kick";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetStakingLimits {
                pub min_nominator_bond: ::core::primitive::u128,
                pub min_validator_bond: ::core::primitive::u128,
                pub max_nominator_count: ::core::option::Option<::core::primitive::u32>,
                pub max_validator_count: ::core::option::Option<::core::primitive::u32>,
                pub threshold: ::core::option::Option<
                    runtime_types::sp_arithmetic::per_things::Percent,
                >,
            }
            impl ::subxt::Call for SetStakingLimits {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_staking_limits";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChillOther {
                pub controller: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ChillOther {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill_other";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn bond(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Bond> {
                    let call = Bond {
                        controller,
                        value,
                        payee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bond_extra(
                    &self,
                    max_additional: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, BondExtra> {
                    let call = BondExtra { max_additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Unbond> {
                    let call = Unbond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, WithdrawUnbonded> {
                    let call = WithdrawUnbonded { num_slashing_spans };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn validate(
                    &self,
                    prefs: runtime_types::pallet_staking::ValidatorPrefs,
                ) -> ::subxt::SubmittableExtrinsic<T, Validate> {
                    let call = Validate { prefs };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn nominate(
                    &self,
                    targets: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Nominate> {
                    let call = Nominate { targets };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill(&self) -> ::subxt::SubmittableExtrinsic<T, Chill> {
                    let call = Chill {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_payee(
                    &self,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetPayee> {
                    let call = SetPayee { payee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_controller(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetController> {
                    let call = SetController { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validator_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetValidatorCount> {
                    let call = SetValidatorCount { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_validator_count(
                    &self,
                    additional: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, IncreaseValidatorCount>
                {
                    let call = IncreaseValidatorCount { additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn scale_validator_count(
                    &self,
                    factor: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::SubmittableExtrinsic<T, ScaleValidatorCount>
                {
                    let call = ScaleValidatorCount { factor };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_no_eras(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceNoEras> {
                    let call = ForceNoEras {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceNewEra> {
                    let call = ForceNewEra {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_invulnerables(
                    &self,
                    invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetInvulnerables> {
                    let call = SetInvulnerables { invulnerables };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unstake(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceUnstake> {
                    let call = ForceUnstake {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era_always(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceNewEraAlways> {
                    let call = ForceNewEraAlways {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_deferred_slash(
                    &self,
                    era: ::core::primitive::u32,
                    slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelDeferredSlash>
                {
                    let call = CancelDeferredSlash { era, slash_indices };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn payout_stakers(
                    &self,
                    validator_stash: ::subxt::sp_core::crypto::AccountId32,
                    era: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, PayoutStakers> {
                    let call = PayoutStakers {
                        validator_stash,
                        era,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rebond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Rebond> {
                    let call = Rebond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHistoryDepth> {
                    let call = SetHistoryDepth {
                        new_history_depth,
                        era_items_deleted,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reap_stash(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ReapStash> {
                    let call = ReapStash {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kick(
                    &self,
                    who: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Kick> {
                    let call = Kick { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_staking_limits(
                    &self,
                    min_nominator_bond: ::core::primitive::u128,
                    min_validator_bond: ::core::primitive::u128,
                    max_nominator_count: ::core::option::Option<::core::primitive::u32>,
                    max_validator_count: ::core::option::Option<::core::primitive::u32>,
                    threshold: ::core::option::Option<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetStakingLimits> {
                    let call = SetStakingLimits {
                        min_nominator_bond,
                        min_validator_bond,
                        max_nominator_count,
                        max_validator_count,
                        threshold,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill_other(
                    &self,
                    controller: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, ChillOther> {
                    let call = ChillOther { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EraPaid(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for EraPaid {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OldSlashingReportDiscarded(pub ::core::primitive::u32);
            impl ::subxt::Event for OldSlashingReportDiscarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakersElected {}
            impl ::subxt::Event for StakersElected {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakersElected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Bonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unbonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdrawn(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdrawn {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Kicked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Kicked {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakingElectionFailed {}
            impl ::subxt::Event for StakingElectionFailed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakingElectionFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Chilled(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Chilled {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Chilled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PayoutStarted(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for PayoutStarted {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "PayoutStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HistoryDepth;
            impl ::subxt::StorageEntry for HistoryDepth {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "HistoryDepth";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorCount;
            impl ::subxt::StorageEntry for ValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinimumValidatorCount;
            impl ::subxt::StorageEntry for MinimumValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinimumValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Invulnerables;
            impl ::subxt::StorageEntry for Invulnerables {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Invulnerables";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Bonded(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Bonded {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Bonded";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct MinNominatorBond;
            impl ::subxt::StorageEntry for MinNominatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinNominatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinValidatorBond;
            impl ::subxt::StorageEntry for MinValidatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinValidatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Ledger(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Ledger {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Ledger";
                type Value = runtime_types::pallet_staking::StakingLedger<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Payee(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Payee {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Payee";
                type Value = runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Validators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Validators";
                type Value = runtime_types::pallet_staking::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CounterForValidators;
            impl ::subxt::StorageEntry for CounterForValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForValidators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxValidatorsCount;
            impl ::subxt::StorageEntry for MaxValidatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxValidatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nominators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Nominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Nominators";
                type Value = runtime_types::pallet_staking::Nominations<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CounterForNominators;
            impl ::subxt::StorageEntry for CounterForNominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForNominators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxNominatorsCount;
            impl ::subxt::StorageEntry for MaxNominatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxNominatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveEra;
            impl ::subxt::StorageEntry for ActiveEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ActiveEra";
                type Value = runtime_types::pallet_staking::ActiveEraInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ErasStartSessionIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasStakers(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakers";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasStakersClipped(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakersClipped {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakersClipped";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasValidatorPrefs(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasValidatorPrefs {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorPrefs";
                type Value = runtime_types::pallet_staking::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasValidatorReward(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasRewardPoints(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasRewardPoints";
                type Value = runtime_types::pallet_staking::EraRewardPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasTotalStake(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ForceEra;
            impl ::subxt::StorageEntry for ForceEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ForceEra";
                type Value = runtime_types::pallet_staking::Forcing;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SlashRewardFraction;
            impl ::subxt::StorageEntry for SlashRewardFraction {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashRewardFraction";
                type Value = ::subxt::sp_arithmetic::per_things::Perbill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CanceledSlashPayout;
            impl ::subxt::StorageEntry for CanceledSlashPayout {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CanceledSlashPayout";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnappliedSlashes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnappliedSlashes {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "UnappliedSlashes";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_staking::UnappliedSlash<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BondedEras;
            impl ::subxt::StorageEntry for BondedEras {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "BondedEras";
                type Value =
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSlashInEra(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ValidatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorSlashInEra";
                type Value = (
                    ::subxt::sp_arithmetic::per_things::Perbill,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct NominatorSlashInEra(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for NominatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "NominatorSlashInEra";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct SlashingSpans(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SlashingSpans {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashingSpans";
                type Value = runtime_types::pallet_staking::slashing::SlashingSpans;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct SpanSlash(
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for SpanSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SpanSlash";
                type Value = runtime_types::pallet_staking::slashing::SpanRecord<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct EarliestUnappliedSlash;
            impl ::subxt::StorageEntry for EarliestUnappliedSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "EarliestUnappliedSlash";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OffendingValidators;
            impl ::subxt::StorageEntry for OffendingValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "OffendingValidators";
                type Value =
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_staking::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChillThreshold;
            impl ::subxt::StorageEntry for ChillThreshold {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ChillThreshold";
                type Value = runtime_types::sp_arithmetic::per_things::Percent;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn history_depth(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn minimum_validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = MinimumValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn invulnerables(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Invulnerables;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bonded(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Bonded(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bonded_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Bonded>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn min_nominator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = MinNominatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn min_validator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = MinValidatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ledger(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::StakingLedger<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Ledger(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn ledger_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Ledger>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn payee(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Payee(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn payee_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Payee>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn validators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::Error,
                > {
                    let entry = Validators(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Validators>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CounterForValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_validators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = MaxValidatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::Nominations<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Nominators(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Nominators>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_nominators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CounterForNominators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_nominators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = MaxNominatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = CurrentEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_staking::ActiveEraInfo>,
                    ::subxt::Error,
                > {
                    let entry = ActiveEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakers>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers_clipped(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasStakersClipped(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_clipped_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakersClipped>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_prefs(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::Error,
                > {
                    let entry = ErasValidatorPrefs(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_validator_prefs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorPrefs>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_validator_reward_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasRewardPoints(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_reward_points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasRewardPoints>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_total_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasTotalStake>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn force_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Forcing,
                    ::subxt::Error,
                > {
                    let entry = ForceEra;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn slash_reward_fraction(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_arithmetic::per_things::Perbill,
                    ::subxt::Error,
                > {
                    let entry = SlashRewardFraction;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn canceled_slash_payout(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = CanceledSlashPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_staking::UnappliedSlash<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UnappliedSlashes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnappliedSlashes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_arithmetic::per_things::Perbill,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = ValidatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn validator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ValidatorSlashInEra>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nominator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = NominatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NominatorSlashInEra>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn slashing_spans(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::slashing::SlashingSpans,
                    >,
                    ::subxt::Error,
                > {
                    let entry = SlashingSpans(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn slashing_spans_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SlashingSpans>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn span_slash(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::slashing::SpanRecord<
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = SpanSlash(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn span_slash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SpanSlash>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn earliest_unapplied_slash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = EarliestUnappliedSlash;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CurrentPlannedSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn offending_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
                    ::subxt::Error,
                > {
                    let entry = OffendingValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn chill_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ChillThreshold;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetKeys {
                pub keys: runtime_types::node_runtime::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PurgeKeys {}
            impl ::subxt::Call for PurgeKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_keys(
                    &self,
                    keys: runtime_types::node_runtime::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetKeys> {
                    let call = SetKeys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(&self) -> ::subxt::SubmittableExtrinsic<T, PurgeKeys> {
                    let call = PurgeKeys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewSession(pub ::core::primitive::u32);
            impl ::subxt::Event for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Validators;
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "Validators";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::node_runtime::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value = runtime_types::node_runtime::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct KeyOwner(
                runtime_types::sp_core::crypto::KeyTypeId,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for KeyOwner {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::node_runtime::SessionKeys,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::node_runtime::SessionKeys>,
                    ::subxt::Error,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextKeys>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, KeyOwner>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod democracy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Propose {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Propose {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Second {
                #[codec(compact)]
                pub proposal: ::core::primitive::u32,
                #[codec(compact)]
                pub seconds_upper_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Second {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "second";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vote {
                #[codec(compact)]
                pub ref_index: ::core::primitive::u32,
                pub vote: runtime_types::pallet_democracy::vote::AccountVote<
                    ::core::primitive::u128,
                >,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EmergencyCancel {
                pub ref_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for EmergencyCancel {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "emergency_cancel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExternalPropose {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalPropose {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExternalProposeMajority {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalProposeMajority {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose_majority";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExternalProposeDefault {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalProposeDefault {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose_default";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct FastTrack {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub voting_period: ::core::primitive::u32,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for FastTrack {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "fast_track";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VetoExternal {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for VetoExternal {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "veto_external";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelReferendum {
                #[codec(compact)]
                pub ref_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelReferendum {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "cancel_referendum";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelQueued {
                pub which: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelQueued {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "cancel_queued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Delegate {
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub conviction: runtime_types::pallet_democracy::conviction::Conviction,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for Delegate {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "delegate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Undelegate {}
            impl ::subxt::Call for Undelegate {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "undelegate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearPublicProposals {}
            impl ::subxt::Call for ClearPublicProposals {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "clear_public_proposals";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotePreimage {
                pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for NotePreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_preimage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotePreimageOperational {
                pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for NotePreimageOperational {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_preimage_operational";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NoteImminentPreimage {
                pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for NoteImminentPreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_imminent_preimage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NoteImminentPreimageOperational {
                pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for NoteImminentPreimageOperational {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_imminent_preimage_operational";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReapPreimage {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub proposal_len_upper_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for ReapPreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "reap_preimage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unlock {
                pub target: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for Unlock {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "unlock";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveVote {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for RemoveVote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "remove_vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveOtherVote {
                pub target: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for RemoveOtherVote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "remove_other_vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EnactProposal {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for EnactProposal {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "enact_proposal";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Blacklist {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for Blacklist {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "blacklist";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelProposal {
                #[codec(compact)]
                pub prop_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelProposal {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "cancel_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn propose(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Propose> {
                    let call = Propose {
                        proposal_hash,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn second(
                    &self,
                    proposal: ::core::primitive::u32,
                    seconds_upper_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Second> {
                    let call = Second {
                        proposal,
                        seconds_upper_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vote(
                    &self,
                    ref_index: ::core::primitive::u32,
                    vote: runtime_types::pallet_democracy::vote::AccountVote<
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Vote> {
                    let call = Vote { ref_index, vote };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn emergency_cancel(
                    &self,
                    ref_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, EmergencyCancel> {
                    let call = EmergencyCancel { ref_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn external_propose(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, ExternalPropose> {
                    let call = ExternalPropose { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn external_propose_majority(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, ExternalProposeMajority>
                {
                    let call = ExternalProposeMajority { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn external_propose_default(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, ExternalProposeDefault>
                {
                    let call = ExternalProposeDefault { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn fast_track(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    voting_period: ::core::primitive::u32,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, FastTrack> {
                    let call = FastTrack {
                        proposal_hash,
                        voting_period,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn veto_external(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, VetoExternal> {
                    let call = VetoExternal { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_referendum(
                    &self,
                    ref_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelReferendum> {
                    let call = CancelReferendum { ref_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_queued(
                    &self,
                    which: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelQueued> {
                    let call = CancelQueued { which };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn delegate(
                    &self,
                    to: ::subxt::sp_core::crypto::AccountId32,
                    conviction: runtime_types::pallet_democracy::conviction::Conviction,
                    balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Delegate> {
                    let call = Delegate {
                        to,
                        conviction,
                        balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn undelegate(&self) -> ::subxt::SubmittableExtrinsic<T, Undelegate> {
                    let call = Undelegate {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_public_proposals(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearPublicProposals>
                {
                    let call = ClearPublicProposals {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_preimage(
                    &self,
                    encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, NotePreimage> {
                    let call = NotePreimage { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_preimage_operational(
                    &self,
                    encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, NotePreimageOperational>
                {
                    let call = NotePreimageOperational { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_imminent_preimage(
                    &self,
                    encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, NoteImminentPreimage>
                {
                    let call = NoteImminentPreimage { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_imminent_preimage_operational(
                    &self,
                    encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, NoteImminentPreimageOperational>
                {
                    let call = NoteImminentPreimageOperational { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reap_preimage(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    proposal_len_upper_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ReapPreimage> {
                    let call = ReapPreimage {
                        proposal_hash,
                        proposal_len_upper_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unlock(
                    &self,
                    target: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, Unlock> {
                    let call = Unlock { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_vote(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveVote> {
                    let call = RemoveVote { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_other_vote(
                    &self,
                    target: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveOtherVote> {
                    let call = RemoveOtherVote { target, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn enact_proposal(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, EnactProposal> {
                    let call = EnactProposal {
                        proposal_hash,
                        index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn blacklist(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, Blacklist> {
                    let call = Blacklist {
                        proposal_hash,
                        maybe_ref_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_proposal(
                    &self,
                    prop_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelProposal> {
                    let call = CancelProposal { prop_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_democracy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Proposed(pub ::core::primitive::u32, pub ::core::primitive::u128);
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Tabled(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            );
            impl ::subxt::Event for Tabled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Tabled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExternalTabled {}
            impl ::subxt::Event for ExternalTabled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "ExternalTabled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Started(
                pub ::core::primitive::u32,
                pub runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
            );
            impl ::subxt::Event for Started {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Started";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Passed(pub ::core::primitive::u32);
            impl ::subxt::Event for Passed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Passed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotPassed(pub ::core::primitive::u32);
            impl ::subxt::Event for NotPassed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "NotPassed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Cancelled(pub ::core::primitive::u32);
            impl ::subxt::Event for Cancelled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Cancelled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Executed(
                pub ::core::primitive::u32,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Executed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Delegated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Delegated {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Delegated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Undelegated(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Undelegated {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Undelegated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vetoed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Vetoed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Vetoed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PreimageNoted(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for PreimageNoted {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageNoted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PreimageUsed(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for PreimageUsed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageUsed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PreimageInvalid(
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for PreimageInvalid {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageInvalid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PreimageMissing(
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for PreimageMissing {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageMissing";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PreimageReaped(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for PreimageReaped {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageReaped";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Blacklisted(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Blacklisted {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Blacklisted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PublicPropCount;
            impl ::subxt::StorageEntry for PublicPropCount {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "PublicPropCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PublicProps;
            impl ::subxt::StorageEntry for PublicProps {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "PublicProps";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DepositOf(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for DepositOf {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "DepositOf";
                type Value = (
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Preimages(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Preimages {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "Preimages";
                type Value = runtime_types::pallet_democracy::PreimageStatus<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ReferendumCount;
            impl ::subxt::StorageEntry for ReferendumCount {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "ReferendumCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LowestUnbaked;
            impl ::subxt::StorageEntry for LowestUnbaked {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "LowestUnbaked";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReferendumInfoOf(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ReferendumInfoOf {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "ReferendumInfoOf";
                type Value = runtime_types::pallet_democracy::types::ReferendumInfo<
                    ::core::primitive::u32,
                    ::subxt::sp_core::H256,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct VotingOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for VotingOf {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "VotingOf";
                type Value = runtime_types::pallet_democracy::vote::Voting<
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "Locks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct LastTabledWasExternal;
            impl ::subxt::StorageEntry for LastTabledWasExternal {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "LastTabledWasExternal";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextExternal;
            impl ::subxt::StorageEntry for NextExternal {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "NextExternal";
                type Value = (
                    ::subxt::sp_core::H256,
                    runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Blacklist(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Blacklist {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "Blacklist";
                type Value = (
                    ::core::primitive::u32,
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct Cancellations(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Cancellations {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "Cancellations";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_democracy::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn public_prop_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = PublicPropCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn public_props(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = PublicProps;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn deposit_of(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = DepositOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn deposit_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DepositOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn preimages(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_democracy::PreimageStatus<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Preimages(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn preimages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Preimages>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn referendum_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ReferendumCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lowest_unbaked(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = LowestUnbaked;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn referendum_info_of(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_democracy::types::ReferendumInfo<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ReferendumInfoOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn referendum_info_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReferendumInfoOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn voting_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_democracy::vote::Voting<
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = VotingOf(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn voting_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VotingOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_tabled_was_external(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = LastTabledWasExternal;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_external(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::H256,
                        runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = NextExternal;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn blacklist(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Blacklist(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn blacklist_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Blacklist>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn cancellations(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = Cancellations(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn cancellations_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Cancellations>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_democracy::Releases>,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod council {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMembers {
                pub new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                pub old_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMembers {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "set_members";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Execute {
                pub proposal: runtime_types::node_runtime::Call,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Execute {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Propose {
                #[codec(compact)]
                pub threshold: ::core::primitive::u32,
                pub proposal: runtime_types::node_runtime::Call,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Propose {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vote {
                pub proposal: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub approve: ::core::primitive::bool,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Close {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub proposal_weight_bound: ::core::primitive::u64,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Close {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "close";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DisapproveProposal {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for DisapproveProposal {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "disapprove_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_members(
                    &self,
                    new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    old_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMembers> {
                    let call = SetMembers {
                        new_members,
                        prime,
                        old_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute(
                    &self,
                    proposal: runtime_types::node_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Execute> {
                    let call = Execute {
                        proposal,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose(
                    &self,
                    threshold: ::core::primitive::u32,
                    proposal: runtime_types::node_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Propose> {
                    let call = Propose {
                        threshold,
                        proposal,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vote(
                    &self,
                    proposal: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, Vote> {
                    let call = Vote {
                        proposal,
                        index,
                        approve,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                    proposal_weight_bound: ::core::primitive::u64,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Close> {
                    let call = Close {
                        proposal_hash,
                        index,
                        proposal_weight_bound,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn disapprove_proposal(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, DisapproveProposal>
                {
                    let call = DisapproveProposal { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_collective::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Proposed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Voted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::bool,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Voted {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Voted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Approved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Approved {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Approved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Disapproved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Disapproved {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Executed(
                pub ::subxt::sp_core::H256,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Executed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MemberExecuted(
                pub ::subxt::sp_core::H256,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MemberExecuted {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "MemberExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Closed(
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Closed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Closed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proposals;
            impl ::subxt::StorageEntry for Proposals {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Proposals";
                type Value =
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::subxt::sp_core::H256,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ProposalOf(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for ProposalOf {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "ProposalOf";
                type Value = runtime_types::node_runtime::Call;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct Voting(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Voting {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Voting";
                type Value = runtime_types::pallet_collective::Votes<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ProposalCount;
            impl ::subxt::StorageEntry for ProposalCount {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "ProposalCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Members";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Prime;
            impl ::subxt::StorageEntry for Prime {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Prime";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn proposals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Proposals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposal_of(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::node_runtime::Call>,
                    ::subxt::Error,
                > {
                    let entry = ProposalOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposal_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ProposalOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn voting(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn voting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn prime(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Prime;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod technical_committee {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMembers {
                pub new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                pub old_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMembers {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "set_members";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Execute {
                pub proposal: runtime_types::node_runtime::Call,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Execute {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Propose {
                #[codec(compact)]
                pub threshold: ::core::primitive::u32,
                pub proposal: runtime_types::node_runtime::Call,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Propose {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vote {
                pub proposal: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub approve: ::core::primitive::bool,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Close {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub proposal_weight_bound: ::core::primitive::u64,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for Close {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "close";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DisapproveProposal {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for DisapproveProposal {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "disapprove_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_members(
                    &self,
                    new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    old_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMembers> {
                    let call = SetMembers {
                        new_members,
                        prime,
                        old_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute(
                    &self,
                    proposal: runtime_types::node_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Execute> {
                    let call = Execute {
                        proposal,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose(
                    &self,
                    threshold: ::core::primitive::u32,
                    proposal: runtime_types::node_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Propose> {
                    let call = Propose {
                        threshold,
                        proposal,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vote(
                    &self,
                    proposal: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, Vote> {
                    let call = Vote {
                        proposal,
                        index,
                        approve,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                    proposal_weight_bound: ::core::primitive::u64,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Close> {
                    let call = Close {
                        proposal_hash,
                        index,
                        proposal_weight_bound,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn disapprove_proposal(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, DisapproveProposal>
                {
                    let call = DisapproveProposal { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_collective::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Proposed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Voted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::bool,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Voted {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Voted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Approved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Approved {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Approved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Disapproved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Disapproved {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Executed(
                pub ::subxt::sp_core::H256,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Executed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MemberExecuted(
                pub ::subxt::sp_core::H256,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MemberExecuted {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "MemberExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Closed(
                pub ::subxt::sp_core::H256,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for Closed {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Closed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proposals;
            impl ::subxt::StorageEntry for Proposals {
                const PALLET: &'static str = "TechnicalCommittee";
                const STORAGE: &'static str = "Proposals";
                type Value =
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::subxt::sp_core::H256,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ProposalOf(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for ProposalOf {
                const PALLET: &'static str = "TechnicalCommittee";
                const STORAGE: &'static str = "ProposalOf";
                type Value = runtime_types::node_runtime::Call;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct Voting(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Voting {
                const PALLET: &'static str = "TechnicalCommittee";
                const STORAGE: &'static str = "Voting";
                type Value = runtime_types::pallet_collective::Votes<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ProposalCount;
            impl ::subxt::StorageEntry for ProposalCount {
                const PALLET: &'static str = "TechnicalCommittee";
                const STORAGE: &'static str = "ProposalCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "TechnicalCommittee";
                const STORAGE: &'static str = "Members";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Prime;
            impl ::subxt::StorageEntry for Prime {
                const PALLET: &'static str = "TechnicalCommittee";
                const STORAGE: &'static str = "Prime";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn proposals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Proposals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposal_of(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::node_runtime::Call>,
                    ::subxt::Error,
                > {
                    let entry = ProposalOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposal_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ProposalOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn voting(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn voting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn prime(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Prime;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod elections {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vote {
                pub votes: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveVoter {}
            impl ::subxt::Call for RemoveVoter {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "remove_voter";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubmitCandidacy {
                #[codec(compact)]
                pub candidate_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for SubmitCandidacy {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "submit_candidacy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RenounceCandidacy {
                pub renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
            }
            impl ::subxt::Call for RenounceCandidacy {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "renounce_candidacy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveMember {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub has_replacement: ::core::primitive::bool,
            }
            impl ::subxt::Call for RemoveMember {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "remove_member";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CleanDefunctVoters {
                pub num_voters: ::core::primitive::u32,
                pub num_defunct: ::core::primitive::u32,
            }
            impl ::subxt::Call for CleanDefunctVoters {
                const PALLET: &'static str = "Elections";
                const FUNCTION: &'static str = "clean_defunct_voters";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn vote(
                    &self,
                    votes: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Vote> {
                    let call = Vote { votes, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_voter(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveVoter> {
                    let call = RemoveVoter {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_candidacy(
                    &self,
                    candidate_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SubmitCandidacy> {
                    let call = SubmitCandidacy { candidate_count };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn renounce_candidacy(
                    &self,
                    renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
                ) -> ::subxt::SubmittableExtrinsic<T, RenounceCandidacy> {
                    let call = RenounceCandidacy { renouncing };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_member(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    has_replacement: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveMember> {
                    let call = RemoveMember {
                        who,
                        has_replacement,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clean_defunct_voters(
                    &self,
                    num_voters: ::core::primitive::u32,
                    num_defunct: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CleanDefunctVoters>
                {
                    let call = CleanDefunctVoters {
                        num_voters,
                        num_defunct,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_elections_phragmen::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewTerm(
                pub  ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            );
            impl ::subxt::Event for NewTerm {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "NewTerm";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EmptyTerm {}
            impl ::subxt::Event for EmptyTerm {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "EmptyTerm";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ElectionError {}
            impl ::subxt::Event for ElectionError {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "ElectionError";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MemberKicked(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for MemberKicked {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "MemberKicked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Renounced(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Renounced {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "Renounced";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateSlashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for CandidateSlashed {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "CandidateSlashed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SeatHolderSlashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for SeatHolderSlashed {
                const PALLET: &'static str = "Elections";
                const EVENT: &'static str = "SeatHolderSlashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "Members";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_elections_phragmen::SeatHolder<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RunnersUp;
            impl ::subxt::StorageEntry for RunnersUp {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "RunnersUp";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_elections_phragmen::SeatHolder<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Candidates;
            impl ::subxt::StorageEntry for Candidates {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "Candidates";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ElectionRounds;
            impl ::subxt::StorageEntry for ElectionRounds {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "ElectionRounds";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Voting(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Voting {
                const PALLET: &'static str = "Elections";
                const STORAGE: &'static str = "Voting";
                type Value = runtime_types::pallet_elections_phragmen::Voter<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_elections_phragmen::SeatHolder<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn runners_up(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_elections_phragmen::SeatHolder<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = RunnersUp;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn candidates(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Candidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn election_rounds(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ElectionRounds;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn voting(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_elections_phragmen::Voter<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn voting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod technical_membership {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddMember {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for AddMember {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "add_member";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveMember {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for RemoveMember {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "remove_member";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SwapMember {
                pub remove: ::subxt::sp_core::crypto::AccountId32,
                pub add: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SwapMember {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "swap_member";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ResetMembers {
                pub members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for ResetMembers {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "reset_members";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChangeKey {
                pub new: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ChangeKey {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "change_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetPrime {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetPrime {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "set_prime";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearPrime {}
            impl ::subxt::Call for ClearPrime {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "clear_prime";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn add_member(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, AddMember> {
                    let call = AddMember { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_member(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveMember> {
                    let call = RemoveMember { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn swap_member(
                    &self,
                    remove: ::subxt::sp_core::crypto::AccountId32,
                    add: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, SwapMember> {
                    let call = SwapMember { remove, add };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reset_members(
                    &self,
                    members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<T, ResetMembers> {
                    let call = ResetMembers { members };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn change_key(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, ChangeKey> {
                    let call = ChangeKey { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_prime(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetPrime> {
                    let call = SetPrime { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_prime(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearPrime> {
                    let call = ClearPrime {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_membership::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MemberAdded {}
            impl ::subxt::Event for MemberAdded {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "MemberAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MemberRemoved {}
            impl ::subxt::Event for MemberRemoved {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "MemberRemoved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MembersSwapped {}
            impl ::subxt::Event for MembersSwapped {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "MembersSwapped";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MembersReset {}
            impl ::subxt::Event for MembersReset {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "MembersReset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KeyChanged {}
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Dummy {}
            impl ::subxt::Event for Dummy {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "Dummy";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "TechnicalMembership";
                const STORAGE: &'static str = "Members";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Prime;
            impl ::subxt::StorageEntry for Prime {
                const PALLET: &'static str = "TechnicalMembership";
                const STORAGE: &'static str = "Prime";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn prime(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Prime;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocation {
                pub equivocation_proof:
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof:
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocation>
                {
                    let call = ReportEquivocation {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocationUnsigned>
                {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, NoteStalled> {
                    let call = NoteStalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewAuthorities(
                pub  ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            );
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Paused {}
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Resumed {}
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value =
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value = runtime_types::pallet_grandpa::StoredPendingChange<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod treasury {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProposeSpend {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub beneficiary: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ProposeSpend {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "propose_spend";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RejectProposal {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for RejectProposal {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "reject_proposal";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveProposal {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ApproveProposal {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "approve_proposal";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn propose_spend(
                    &self,
                    value: ::core::primitive::u128,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ProposeSpend> {
                    let call = ProposeSpend { value, beneficiary };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_proposal(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RejectProposal> {
                    let call = RejectProposal { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_proposal(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveProposal> {
                    let call = ApproveProposal { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_treasury::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Proposed(pub ::core::primitive::u32);
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Spending(pub ::core::primitive::u128);
            impl ::subxt::Event for Spending {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Spending";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Awarded(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Awarded {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Awarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rejected(pub ::core::primitive::u32, pub ::core::primitive::u128);
            impl ::subxt::Event for Rejected {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rejected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Burnt(pub ::core::primitive::u128);
            impl ::subxt::Event for Burnt {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Burnt";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rollover(pub ::core::primitive::u128);
            impl ::subxt::Event for Rollover {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rollover";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deposit(pub ::core::primitive::u128);
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Deposit";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ProposalCount;
            impl ::subxt::StorageEntry for ProposalCount {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "ProposalCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Proposals(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Proposals {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "Proposals";
                type Value = runtime_types::pallet_treasury::Proposal<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Approvals;
            impl ::subxt::StorageEntry for Approvals {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "Approvals";
                type Value =
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposals(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_treasury::Proposal<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Proposals(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Proposals>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn approvals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Approvals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod contracts {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Call {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Call {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "call";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InstantiateWithCode {
                #[codec(compact)]
                pub endowment: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub code: ::std::vec::Vec<::core::primitive::u8>,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for InstantiateWithCode {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "instantiate_with_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Instantiate {
                #[codec(compact)]
                pub endowment: ::core::primitive::u128,
                #[codec(compact)]
                pub gas_limit: ::core::primitive::u64,
                pub code_hash: ::subxt::sp_core::H256,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Instantiate {
                const PALLET: &'static str = "Contracts";
                const FUNCTION: &'static str = "instantiate";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn call(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Call> {
                    let call = Call {
                        dest,
                        value,
                        gas_limit,
                        data,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn instantiate_with_code(
                    &self,
                    endowment: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, InstantiateWithCode>
                {
                    let call = InstantiateWithCode {
                        endowment,
                        gas_limit,
                        code,
                        data,
                        salt,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn instantiate(
                    &self,
                    endowment: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    code_hash: ::subxt::sp_core::H256,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Instantiate> {
                    let call = Instantiate {
                        endowment,
                        gas_limit,
                        code_hash,
                        data,
                        salt,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_contracts::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Instantiated {
                pub deployer: ::subxt::sp_core::crypto::AccountId32,
                pub contract: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Instantiated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Instantiated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Terminated {
                pub contract: ::subxt::sp_core::crypto::AccountId32,
                pub beneficiary: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Terminated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Terminated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CodeStored {
                pub code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for CodeStored {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeStored";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduleUpdated {
                pub version: ::core::primitive::u32,
            }
            impl ::subxt::Event for ScheduleUpdated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ScheduleUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ContractEmitted {
                pub contract: ::subxt::sp_core::crypto::AccountId32,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for ContractEmitted {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractEmitted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CodeRemoved {
                pub code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for CodeRemoved {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PristineCode(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for PristineCode {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "PristineCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CodeStorage(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for CodeStorage {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "CodeStorage";
                type Value = runtime_types::pallet_contracts::wasm::PrefabWasmModule;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct AccountCounter;
            impl ::subxt::StorageEntry for AccountCounter {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "AccountCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ContractInfoOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ContractInfoOf {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "ContractInfoOf";
                type Value = runtime_types::pallet_contracts::storage::RawContractInfo<
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct DeletionQueue;
            impl ::subxt::StorageEntry for DeletionQueue {
                const PALLET: &'static str = "Contracts";
                const STORAGE: &'static str = "DeletionQueue";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_contracts::storage::DeletedContract,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn pristine_code(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = PristineCode(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pristine_code_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PristineCode>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn code_storage(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_contracts::wasm::PrefabWasmModule,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CodeStorage(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn code_storage_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, CodeStorage>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn account_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = AccountCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn contract_info_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_contracts::storage::RawContractInfo<
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ContractInfoOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn contract_info_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ContractInfoOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn deletion_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_contracts::storage::DeletedContract,
                    >,
                    ::subxt::Error,
                > {
                    let entry = DeletionQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudo {
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoUncheckedWeight {
                pub call: runtime_types::node_runtime::Call,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetKey {
                pub new: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoAs {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, Sudo> {
                    let call = Sudo { call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::node_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoUncheckedWeight>
                {
                    let call = SudoUncheckedWeight { call, weight };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetKey> {
                    let call = SetKey { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoAs> {
                    let call = SudoAs { who, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudid(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KeyChanged(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoAsDone(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::Error,
                > {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod im_online {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Heartbeat {
                pub heartbeat:
                    runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                pub signature:
                    runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
            impl ::subxt::Call for Heartbeat {
                const PALLET: &'static str = "ImOnline";
                const FUNCTION: &'static str = "heartbeat";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<
                        ::core::primitive::u32,
                    >,
                    signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature,
                ) -> ::subxt::SubmittableExtrinsic<T, Heartbeat> {
                    let call = Heartbeat {
                        heartbeat,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct HeartbeatReceived(
                pub runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            );
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AllGood {}
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SomeOffline(
                pub  ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                )>,
            );
            impl ::subxt::Event for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HeartbeatAfter;
            impl ::subxt::StorageEntry for HeartbeatAfter {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "HeartbeatAfter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "Keys";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReceivedHeartbeats(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for ReceivedHeartbeats {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "ReceivedHeartbeats";
                type Value = runtime_types::frame_support::traits::misc::WrapperOpaque<
                    runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct AuthoredBlocks(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AuthoredBlocks {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn heartbeat_after(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn keys (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public > , :: subxt :: Error >{
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn received_heartbeats(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ReceivedHeartbeats(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn received_heartbeats_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authored_blocks(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authored_blocks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AuthoredBlocks>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod authority_discovery {
        use super::runtime_types;
    }
    pub mod offences {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_offences::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Offence(
                pub [::core::primitive::u8; 16usize],
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for Offence {
                const PALLET: &'static str = "Offences";
                const EVENT: &'static str = "Offence";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Reports(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Reports {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "Reports";
                type Value = runtime_types::sp_staking::offence::OffenceDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    (
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_staking::Exposure<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    ),
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ConcurrentReportsIndex(
                [::core::primitive::u8; 16usize],
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for ConcurrentReportsIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ConcurrentReportsIndex";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ReportsByKindIndex(pub [::core::primitive::u8; 16usize]);
            impl ::subxt::StorageEntry for ReportsByKindIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ReportsByKindIndex";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn reports(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_staking::offence::OffenceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            (
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::pallet_staking::Exposure<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    ::core::primitive::u128,
                                >,
                            ),
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reports(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reports_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reports>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn concurrent_reports_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = ConcurrentReportsIndex(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn concurrent_reports_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConcurrentReportsIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn reports_by_kind_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::Error,
                > {
                    let entry = ReportsByKindIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reports_by_kind_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReportsByKindIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
    }
    pub mod randomness_collective_flip {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn random_material(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = RandomMaterial;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod identity {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddRegistrar {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for AddRegistrar {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "add_registrar";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetIdentity {
                pub info: runtime_types::pallet_identity::types::IdentityInfo,
            }
            impl ::subxt::Call for SetIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetSubs {
                pub subs: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_identity::types::Data,
                )>,
            }
            impl ::subxt::Call for SetSubs {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_subs";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearIdentity {}
            impl ::subxt::Call for ClearIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "clear_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RequestJudgement {
                #[codec(compact)]
                pub reg_index: ::core::primitive::u32,
                #[codec(compact)]
                pub max_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for RequestJudgement {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "request_judgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelRequest {
                pub reg_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelRequest {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "cancel_request";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetFee {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetFee {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_fee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetAccountId {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub new: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetAccountId {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_account_id";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetFields {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub fields: runtime_types::pallet_identity::types::BitFlags<
                    runtime_types::pallet_identity::types::IdentityField,
                >,
            }
            impl ::subxt::Call for SetFields {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_fields";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProvideJudgement {
                #[codec(compact)]
                pub reg_index: ::core::primitive::u32,
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub judgement: runtime_types::pallet_identity::types::Judgement<
                    ::core::primitive::u128,
                >,
            }
            impl ::subxt::Call for ProvideJudgement {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "provide_judgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillIdentity {
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for KillIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "kill_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddSub {
                pub sub: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            impl ::subxt::Call for AddSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "add_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RenameSub {
                pub sub: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            impl ::subxt::Call for RenameSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "rename_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveSub {
                pub sub: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for RemoveSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "remove_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct QuitSub {}
            impl ::subxt::Call for QuitSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "quit_sub";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn add_registrar(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, AddRegistrar> {
                    let call = AddRegistrar { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_identity(
                    &self,
                    info: runtime_types::pallet_identity::types::IdentityInfo,
                ) -> ::subxt::SubmittableExtrinsic<T, SetIdentity> {
                    let call = SetIdentity { info };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_subs(
                    &self,
                    subs: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetSubs> {
                    let call = SetSubs { subs };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_identity(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearIdentity> {
                    let call = ClearIdentity {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn request_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    max_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, RequestJudgement> {
                    let call = RequestJudgement { reg_index, max_fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_request(
                    &self,
                    reg_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelRequest> {
                    let call = CancelRequest { reg_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_fee(
                    &self,
                    index: ::core::primitive::u32,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetFee> {
                    let call = SetFee { index, fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_account_id(
                    &self,
                    index: ::core::primitive::u32,
                    new: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetAccountId> {
                    let call = SetAccountId { index, new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_fields(
                    &self,
                    index: ::core::primitive::u32,
                    fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetFields> {
                    let call = SetFields { index, fields };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn provide_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    judgement: runtime_types::pallet_identity::types::Judgement<
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ProvideJudgement> {
                    let call = ProvideJudgement {
                        reg_index,
                        target,
                        judgement,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_identity(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, KillIdentity> {
                    let call = KillIdentity { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::SubmittableExtrinsic<T, AddSub> {
                    let call = AddSub { sub, data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rename_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::SubmittableExtrinsic<T, RenameSub> {
                    let call = RenameSub { sub, data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveSub> {
                    let call = RemoveSub { sub };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn quit_sub(&self) -> ::subxt::SubmittableExtrinsic<T, QuitSub> {
                    let call = QuitSub {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_identity::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentitySet(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for IdentitySet {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentitySet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentityCleared(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for IdentityCleared {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentityKilled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for IdentityKilled {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityKilled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementRequested(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for JudgementRequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementRequested";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementUnrequested(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for JudgementUnrequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementUnrequested";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementGiven(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for JudgementGiven {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementGiven";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RegistrarAdded(pub ::core::primitive::u32);
            impl ::subxt::Event for RegistrarAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "RegistrarAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for SubIdentityAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityRemoved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for SubIdentityRemoved {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRemoved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityRevoked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for SubIdentityRevoked {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRevoked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct IdentityOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for IdentityOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "IdentityOf";
                type Value = runtime_types::pallet_identity::types::Registration<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct SuperOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SuperOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "SuperOf";
                type Value = (
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_identity::types::Data,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct SubsOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SubsOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "SubsOf";
                type Value = (
                    ::core::primitive::u128,
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Registrars;
            impl ::subxt::StorageEntry for Registrars {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "Registrars";
                type Value =
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::option::Option<
                            runtime_types::pallet_identity::types::RegistrarInfo<
                                ::core::primitive::u128,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn identity_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_identity::types::Registration<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = IdentityOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn identity_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, IdentityOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn super_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = SuperOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn super_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SuperOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn subs_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    ),
                    ::subxt::Error,
                > {
                    let entry = SubsOf(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn subs_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SubsOf>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn registrars(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::option::Option<
                            runtime_types::pallet_identity::types::RegistrarInfo<
                                ::core::primitive::u128,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Registrars;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod society {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bid {
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Bid {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "bid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unbid {
                pub pos: ::core::primitive::u32,
            }
            impl ::subxt::Call for Unbid {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "unbid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vouch {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub value: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
            }
            impl ::subxt::Call for Vouch {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "vouch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unvouch {
                pub pos: ::core::primitive::u32,
            }
            impl ::subxt::Call for Unvouch {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "unvouch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vote {
                pub candidate: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub approve: ::core::primitive::bool,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DefenderVote {
                pub approve: ::core::primitive::bool,
            }
            impl ::subxt::Call for DefenderVote {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "defender_vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Payout {}
            impl ::subxt::Call for Payout {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "payout";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Found {
                pub founder: ::subxt::sp_core::crypto::AccountId32,
                pub max_members: ::core::primitive::u32,
                pub rules: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Found {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "found";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unfound {}
            impl ::subxt::Call for Unfound {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "unfound";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgeSuspendedMember {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub forgive: ::core::primitive::bool,
            }
            impl ::subxt::Call for JudgeSuspendedMember {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "judge_suspended_member";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgeSuspendedCandidate {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub judgement: runtime_types::pallet_society::Judgement,
            }
            impl ::subxt::Call for JudgeSuspendedCandidate {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "judge_suspended_candidate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxMembers {
                pub max: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxMembers {
                const PALLET: &'static str = "Society";
                const FUNCTION: &'static str = "set_max_members";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn bid(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Bid> {
                    let call = Bid { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbid(
                    &self,
                    pos: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Unbid> {
                    let call = Unbid { pos };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vouch(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    value: ::core::primitive::u128,
                    tip: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Vouch> {
                    let call = Vouch { who, value, tip };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unvouch(
                    &self,
                    pos: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Unvouch> {
                    let call = Unvouch { pos };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vote(
                    &self,
                    candidate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, Vote> {
                    let call = Vote { candidate, approve };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn defender_vote(
                    &self,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, DefenderVote> {
                    let call = DefenderVote { approve };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn payout(&self) -> ::subxt::SubmittableExtrinsic<T, Payout> {
                    let call = Payout {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn found(
                    &self,
                    founder: ::subxt::sp_core::crypto::AccountId32,
                    max_members: ::core::primitive::u32,
                    rules: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Found> {
                    let call = Found {
                        founder,
                        max_members,
                        rules,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unfound(&self) -> ::subxt::SubmittableExtrinsic<T, Unfound> {
                    let call = Unfound {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn judge_suspended_member(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    forgive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, JudgeSuspendedMember>
                {
                    let call = JudgeSuspendedMember { who, forgive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn judge_suspended_candidate(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    judgement: runtime_types::pallet_society::Judgement,
                ) -> ::subxt::SubmittableExtrinsic<T, JudgeSuspendedCandidate>
                {
                    let call = JudgeSuspendedCandidate { who, judgement };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_members(
                    &self,
                    max: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxMembers> {
                    let call = SetMaxMembers { max };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_society::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Founded(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Founded {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Founded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bid(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Bid {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Bid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vouch(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Vouch {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Vouch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AutoUnbid(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for AutoUnbid {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "AutoUnbid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unbid(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Unbid {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Unbid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unvouch(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Unvouch {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Unvouch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Inducted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            );
            impl ::subxt::Event for Inducted {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Inducted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SuspendedMemberJudgement(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for SuspendedMemberJudgement {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "SuspendedMemberJudgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateSuspended(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for CandidateSuspended {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "CandidateSuspended";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MemberSuspended(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for MemberSuspended {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "MemberSuspended";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Challenged(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Challenged {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Challenged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vote(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for Vote {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DefenderVote(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for DefenderVote {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "DefenderVote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewMaxMembers(pub ::core::primitive::u32);
            impl ::subxt::Event for NewMaxMembers {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "NewMaxMembers";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unfounded(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Unfounded {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Unfounded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deposit(pub ::core::primitive::u128);
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Society";
                const EVENT: &'static str = "Deposit";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Founder;
            impl ::subxt::StorageEntry for Founder {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Founder";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Rules;
            impl ::subxt::StorageEntry for Rules {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Rules";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Candidates;
            impl ::subxt::StorageEntry for Candidates {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Candidates";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_society::Bid<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SuspendedCandidates(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SuspendedCandidates {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "SuspendedCandidates";
                type Value = (
                    ::core::primitive::u128,
                    runtime_types::pallet_society::BidKind<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Pot;
            impl ::subxt::StorageEntry for Pot {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Pot";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Head;
            impl ::subxt::StorageEntry for Head {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Head";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Members";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SuspendedMembers(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SuspendedMembers {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "SuspendedMembers";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Bids;
            impl ::subxt::StorageEntry for Bids {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Bids";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_society::Bid<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Vouching(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Vouching {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Vouching";
                type Value = runtime_types::pallet_society::VouchingStatus;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Payouts(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Payouts {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Payouts";
                type Value =
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u128)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Strikes(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Strikes {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Strikes";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Votes(
                ::subxt::sp_core::crypto::AccountId32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Votes {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Votes";
                type Value = runtime_types::pallet_society::Vote;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Defender;
            impl ::subxt::StorageEntry for Defender {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "Defender";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DefenderVotes(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for DefenderVotes {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "DefenderVotes";
                type Value = runtime_types::pallet_society::Vote;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct MaxMembers;
            impl ::subxt::StorageEntry for MaxMembers {
                const PALLET: &'static str = "Society";
                const STORAGE: &'static str = "MaxMembers";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn founder(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Founder;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn rules(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = Rules;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn candidates(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_society::Bid<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Candidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn suspended_candidates(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u128,
                        runtime_types::pallet_society::BidKind<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = SuspendedCandidates(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn suspended_candidates_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SuspendedCandidates>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn pot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = Pot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn head(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Head;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn suspended_members(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = SuspendedMembers(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn suspended_members_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SuspendedMembers>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bids(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_society::Bid<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Bids;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn vouching(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_society::VouchingStatus>,
                    ::subxt::Error,
                > {
                    let entry = Vouching(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn vouching_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Vouching>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn payouts(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u128)>,
                    ::subxt::Error,
                > {
                    let entry = Payouts(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn payouts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Payouts>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn strikes(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Strikes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn strikes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Strikes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn votes(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_society::Vote>,
                    ::subxt::Error,
                > {
                    let entry = Votes(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn votes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Votes>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn defender(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Defender;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn defender_votes(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_society::Vote>,
                    ::subxt::Error,
                > {
                    let entry = DefenderVotes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn defender_votes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DefenderVotes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn max_members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = MaxMembers;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod recovery {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsRecovered {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for AsRecovered {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "as_recovered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetRecovered {
                pub lost: ::subxt::sp_core::crypto::AccountId32,
                pub rescuer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetRecovered {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "set_recovered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CreateRecovery {
                pub friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub threshold: ::core::primitive::u16,
                pub delay_period: ::core::primitive::u32,
            }
            impl ::subxt::Call for CreateRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "create_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InitiateRecovery {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for InitiateRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "initiate_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VouchRecovery {
                pub lost: ::subxt::sp_core::crypto::AccountId32,
                pub rescuer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for VouchRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "vouch_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClaimRecovery {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ClaimRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "claim_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CloseRecovery {
                pub rescuer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for CloseRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "close_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveRecovery {}
            impl ::subxt::Call for RemoveRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "remove_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelRecovered {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for CancelRecovered {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "cancel_recovered";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn as_recovered(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsRecovered> {
                    let call = AsRecovered { account, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_recovered(
                    &self,
                    lost: ::subxt::sp_core::crypto::AccountId32,
                    rescuer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetRecovered> {
                    let call = SetRecovered { lost, rescuer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create_recovery(
                    &self,
                    friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    threshold: ::core::primitive::u16,
                    delay_period: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CreateRecovery> {
                    let call = CreateRecovery {
                        friends,
                        threshold,
                        delay_period,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn initiate_recovery(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, InitiateRecovery> {
                    let call = InitiateRecovery { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vouch_recovery(
                    &self,
                    lost: ::subxt::sp_core::crypto::AccountId32,
                    rescuer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, VouchRecovery> {
                    let call = VouchRecovery { lost, rescuer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn claim_recovery(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, ClaimRecovery> {
                    let call = ClaimRecovery { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close_recovery(
                    &self,
                    rescuer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, CloseRecovery> {
                    let call = CloseRecovery { rescuer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_recovery(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveRecovery> {
                    let call = RemoveRecovery {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_recovered(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelRecovered> {
                    let call = CancelRecovered { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_recovery::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryCreated(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for RecoveryCreated {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryInitiated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for RecoveryInitiated {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryInitiated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryVouched(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for RecoveryVouched {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryVouched";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryClosed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for RecoveryClosed {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryClosed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountRecovered(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for AccountRecovered {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "AccountRecovered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryRemoved(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for RecoveryRemoved {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Recoverable(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Recoverable {
                const PALLET: &'static str = "Recovery";
                const STORAGE: &'static str = "Recoverable";
                type Value = runtime_types::pallet_recovery::RecoveryConfig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ActiveRecoveries(
                ::subxt::sp_core::crypto::AccountId32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ActiveRecoveries {
                const PALLET: &'static str = "Recovery";
                const STORAGE: &'static str = "ActiveRecoveries";
                type Value = runtime_types::pallet_recovery::ActiveRecovery<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Proxy(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Proxy {
                const PALLET: &'static str = "Recovery";
                const STORAGE: &'static str = "Proxy";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn recoverable(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_recovery::RecoveryConfig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Recoverable(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn recoverable_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Recoverable>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn active_recoveries(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_recovery::ActiveRecovery<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ActiveRecoveries(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_recoveries_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ActiveRecoveries>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn proxy(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Proxy(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proxy_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proxy>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod vesting {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vest {}
            impl ::subxt::Call for Vest {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestOther {
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for VestOther {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest_other";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestedTransfer {
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for VestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vested_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceVestedTransfer {
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ForceVestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "force_vested_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MergeSchedules {
                pub schedule1_index: ::core::primitive::u32,
                pub schedule2_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for MergeSchedules {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "merge_schedules";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn vest(&self) -> ::subxt::SubmittableExtrinsic<T, Vest> {
                    let call = Vest {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vest_other(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, VestOther> {
                    let call = VestOther { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vested_transfer(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, VestedTransfer> {
                    let call = VestedTransfer { target, schedule };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_vested_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceVestedTransfer>
                {
                    let call = ForceVestedTransfer {
                        source,
                        target,
                        schedule,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn merge_schedules(
                    &self,
                    schedule1_index: ::core::primitive::u32,
                    schedule2_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, MergeSchedules> {
                    let call = MergeSchedules {
                        schedule1_index,
                        schedule2_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_vesting::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestingUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for VestingUpdated {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestingCompleted(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for VestingCompleted {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingCompleted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Vesting(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Vesting {
                const PALLET: &'static str = "Vesting";
                const STORAGE: &'static str = "Vesting";
                type Value =
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_vesting::vesting_info::VestingInfo<
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Vesting";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_vesting::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn vesting(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Vesting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn vesting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Vesting>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_vesting::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod scheduler {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Schedule {
                pub when: ::core::primitive::u32,
                pub maybe_periodic: ::core::option::Option<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for Schedule {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Cancel {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Cancel {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduleNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub when: ::core::primitive::u32,
                pub maybe_periodic: ::core::option::Option<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for ScheduleNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for CancelNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel_named";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduleAfter {
                pub after: ::core::primitive::u32,
                pub maybe_periodic: ::core::option::Option<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for ScheduleAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_after";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduleNamedAfter {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub after: ::core::primitive::u32,
                pub maybe_periodic: ::core::option::Option<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for ScheduleNamedAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named_after";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn schedule(
                    &self,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, Schedule> {
                    let call = Schedule {
                        when,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel(
                    &self,
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Cancel> {
                    let call = Cancel { when, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, ScheduleNamed> {
                    let call = ScheduleNamed {
                        id,
                        when,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelNamed> {
                    let call = CancelNamed { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_after(
                    &self,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, ScheduleAfter> {
                    let call = ScheduleAfter {
                        after,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named_after(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, ScheduleNamedAfter>
                {
                    let call = ScheduleNamedAfter {
                        id,
                        after,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Scheduled(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Canceled(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Dispatched(
                pub (::core::primitive::u32, ::core::primitive::u32),
                pub ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Agenda(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Agenda {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Agenda";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<
                        runtime_types::pallet_scheduler::ScheduledV2<
                            runtime_types::node_runtime::Call,
                            ::core::primitive::u32,
                            runtime_types::node_runtime::OriginCaller,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Lookup(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for Lookup {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Lookup";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_scheduler::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn agenda(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::core::option::Option<
                            runtime_types::pallet_scheduler::ScheduledV2<
                                runtime_types::node_runtime::Call,
                                ::core::primitive::u32,
                                runtime_types::node_runtime::OriginCaller,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Agenda(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn agenda_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Agenda>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn lookup(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Lookup(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn lookup_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Lookup>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_scheduler::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod proxy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Proxy {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type:
                    ::core::option::Option<runtime_types::node_runtime::ProxyType>,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for Proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddProxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::node_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for AddProxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "add_proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveProxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::node_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for RemoveProxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveProxies {}
            impl ::subxt::Call for RemoveProxies {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxies";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Anonymous {
                pub proxy_type: runtime_types::node_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
                pub index: ::core::primitive::u16,
            }
            impl ::subxt::Call for Anonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "anonymous";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillAnonymous {
                pub spawner: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::node_runtime::ProxyType,
                pub index: ::core::primitive::u16,
                #[codec(compact)]
                pub height: ::core::primitive::u32,
                #[codec(compact)]
                pub ext_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillAnonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "kill_anonymous";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Announce {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for Announce {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "announce";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveAnnouncement {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RemoveAnnouncement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_announcement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RejectAnnouncement {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RejectAnnouncement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "reject_announcement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProxyAnnounced {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type:
                    ::core::option::Option<runtime_types::node_runtime::ProxyType>,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for ProxyAnnounced {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy_announced";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn proxy(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::node_runtime::ProxyType,
                    >,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, Proxy> {
                    let call = Proxy {
                        real,
                        force_proxy_type,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::node_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, AddProxy> {
                    let call = AddProxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::node_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveProxy> {
                    let call = RemoveProxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxies(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveProxies> {
                    let call = RemoveProxies {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn anonymous(
                    &self,
                    proxy_type: runtime_types::node_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                    index: ::core::primitive::u16,
                ) -> ::subxt::SubmittableExtrinsic<T, Anonymous> {
                    let call = Anonymous {
                        proxy_type,
                        delay,
                        index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_anonymous(
                    &self,
                    spawner: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::node_runtime::ProxyType,
                    index: ::core::primitive::u16,
                    height: ::core::primitive::u32,
                    ext_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, KillAnonymous> {
                    let call = KillAnonymous {
                        spawner,
                        proxy_type,
                        index,
                        height,
                        ext_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn announce(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, Announce> {
                    let call = Announce { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_announcement(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveAnnouncement>
                {
                    let call = RemoveAnnouncement { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_announcement(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, RejectAnnouncement>
                {
                    let call = RejectAnnouncement {
                        delegate,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn proxy_announced(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::node_runtime::ProxyType,
                    >,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, ProxyAnnounced> {
                    let call = ProxyAnnounced {
                        delegate,
                        real,
                        force_proxy_type,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_proxy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProxyExecuted(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for ProxyExecuted {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AnonymousCreated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::node_runtime::ProxyType,
                pub ::core::primitive::u16,
            );
            impl ::subxt::Event for AnonymousCreated {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "AnonymousCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Announced(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Announced {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "Announced";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProxyAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::node_runtime::ProxyType,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for ProxyAdded {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyAdded";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proxies(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Proxies {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Proxies";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_proxy::ProxyDefinition<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::node_runtime::ProxyType,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Announcements(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Announcements {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Announcements";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_proxy::Announcement<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn proxies(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::ProxyDefinition<
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::node_runtime::ProxyType,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    ),
                    ::subxt::Error,
                > {
                    let entry = Proxies(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proxies_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Proxies>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn announcements(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::Announcement<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    ),
                    ::subxt::Error,
                > {
                    let entry = Announcements(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn announcements_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Announcements>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod multisig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsMultiThreshold1 {
                pub other_signatories:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for AsMultiThreshold1 {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi_threshold1";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call: ::subxt::WrapperKeepOpaque<runtime_types::node_runtime::Call>,
                pub store_call: ::core::primitive::bool,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for AsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for ApproveAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint:
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for CancelAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "cancel_as_multi";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn as_multi_threshold1(
                    &self,
                    other_signatories: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsMultiThreshold1> {
                    let call = AsMultiThreshold1 {
                        other_signatories,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::subxt::WrapperKeepOpaque<runtime_types::node_runtime::Call>,
                    store_call: ::core::primitive::bool,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, AsMulti> {
                    let call = AsMulti {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call,
                        store_call,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveAsMulti> {
                    let call = ApproveAsMulti {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    timepoint: runtime_types::pallet_multisig::Timepoint<
                        ::core::primitive::u32,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<T, CancelAsMulti> {
                    let call = CancelAsMulti {
                        threshold,
                        other_signatories,
                        timepoint,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewMultisig(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigApproval(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigExecuted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigCancelled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(
                ::subxt::sp_core::crypto::AccountId32,
                [::core::primitive::u8; 32usize],
            );
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Calls(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Calls";
                type Value = (
                    ::subxt::WrapperKeepOpaque<runtime_types::node_runtime::Call>,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn multisigs(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn multisigs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Multisigs>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::WrapperKeepOpaque<runtime_types::node_runtime::Call>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Calls>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod bounties {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProposeBounty {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub description: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ProposeBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "propose_bounty";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveBounty {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ApproveBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "approve_bounty";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProposeCurator {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
                pub curator: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for ProposeCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "propose_curator";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnassignCurator {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for UnassignCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "unassign_curator";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AcceptCurator {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for AcceptCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "accept_curator";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AwardBounty {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
                pub beneficiary: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for AwardBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "award_bounty";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClaimBounty {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClaimBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "claim_bounty";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CloseBounty {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for CloseBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "close_bounty";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtendBountyExpiry {
                #[codec(compact)]
                pub bounty_id: ::core::primitive::u32,
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for ExtendBountyExpiry {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "extend_bounty_expiry";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn propose_bounty(
                    &self,
                    value: ::core::primitive::u128,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, ProposeBounty> {
                    let call = ProposeBounty { value, description };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_bounty(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveBounty> {
                    let call = ApproveBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose_curator(
                    &self,
                    bounty_id: ::core::primitive::u32,
                    curator: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ProposeCurator> {
                    let call = ProposeCurator {
                        bounty_id,
                        curator,
                        fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unassign_curator(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, UnassignCurator> {
                    let call = UnassignCurator { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn accept_curator(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, AcceptCurator> {
                    let call = AcceptCurator { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn award_bounty(
                    &self,
                    bounty_id: ::core::primitive::u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, AwardBounty> {
                    let call = AwardBounty {
                        bounty_id,
                        beneficiary,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn claim_bounty(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ClaimBounty> {
                    let call = ClaimBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close_bounty(
                    &self,
                    bounty_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CloseBounty> {
                    let call = CloseBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn extend_bounty_expiry(
                    &self,
                    bounty_id: ::core::primitive::u32,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, ExtendBountyExpiry>
                {
                    let call = ExtendBountyExpiry { bounty_id, remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bounties::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BountyProposed(pub ::core::primitive::u32);
            impl ::subxt::Event for BountyProposed {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyProposed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BountyRejected(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BountyRejected {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyRejected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BountyBecameActive(pub ::core::primitive::u32);
            impl ::subxt::Event for BountyBecameActive {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyBecameActive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BountyAwarded(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for BountyAwarded {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyAwarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BountyClaimed(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for BountyClaimed {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyClaimed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BountyCanceled(pub ::core::primitive::u32);
            impl ::subxt::Event for BountyCanceled {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyCanceled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BountyExtended(pub ::core::primitive::u32);
            impl ::subxt::Event for BountyExtended {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyExtended";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct BountyCount;
            impl ::subxt::StorageEntry for BountyCount {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "BountyCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Bounties(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Bounties {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "Bounties";
                type Value = runtime_types::pallet_bounties::Bounty<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BountyDescriptions(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BountyDescriptions {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "BountyDescriptions";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BountyApprovals;
            impl ::subxt::StorageEntry for BountyApprovals {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "BountyApprovals";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn bounty_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = BountyCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bounties(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_bounties::Bounty<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Bounties(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bounties_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Bounties>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bounty_descriptions(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = BountyDescriptions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bounty_descriptions_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BountyDescriptions>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bounty_approvals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = BountyApprovals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod tips {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportAwesome {
                pub reason: ::std::vec::Vec<::core::primitive::u8>,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ReportAwesome {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "report_awesome";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RetractTip {
                pub hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RetractTip {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "retract_tip";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TipNew {
                pub reason: ::std::vec::Vec<::core::primitive::u8>,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                #[codec(compact)]
                pub tip_value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TipNew {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "tip_new";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Tip {
                pub hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub tip_value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Tip {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "tip";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CloseTip {
                pub hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for CloseTip {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "close_tip";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SlashTip {
                pub hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for SlashTip {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "slash_tip";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn report_awesome(
                    &self,
                    reason: ::std::vec::Vec<::core::primitive::u8>,
                    who: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportAwesome> {
                    let call = ReportAwesome { reason, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn retract_tip(
                    &self,
                    hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, RetractTip> {
                    let call = RetractTip { hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn tip_new(
                    &self,
                    reason: ::std::vec::Vec<::core::primitive::u8>,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    tip_value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, TipNew> {
                    let call = TipNew {
                        reason,
                        who,
                        tip_value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn tip(
                    &self,
                    hash: ::subxt::sp_core::H256,
                    tip_value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Tip> {
                    let call = Tip { hash, tip_value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close_tip(
                    &self,
                    hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, CloseTip> {
                    let call = CloseTip { hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn slash_tip(
                    &self,
                    hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, SlashTip> {
                    let call = SlashTip { hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_tips::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewTip(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for NewTip {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "NewTip";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TipClosing(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for TipClosing {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "TipClosing";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TipClosed(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for TipClosed {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "TipClosed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TipRetracted(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for TipRetracted {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "TipRetracted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TipSlashed(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for TipSlashed {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "TipSlashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Tips(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Tips {
                const PALLET: &'static str = "Tips";
                const STORAGE: &'static str = "Tips";
                type Value = runtime_types::pallet_tips::OpenTip<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Reasons(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Reasons {
                const PALLET: &'static str = "Tips";
                const STORAGE: &'static str = "Reasons";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn tips(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_tips::OpenTip<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Tips(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn tips_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Tips>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reasons(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = Reasons(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reasons_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reasons>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod assets {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Create {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub min_balance: ::core::primitive::u64,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCreate {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub is_sufficient: ::core::primitive::bool,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u64,
            }
            impl ::subxt::Call for ForceCreate {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Destroy {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub witness: runtime_types::pallet_assets::types::DestroyWitness,
            }
            impl ::subxt::Call for Destroy {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Mint {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub beneficiary: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u64,
            }
            impl ::subxt::Call for Mint {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Burn {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u64,
            }
            impl ::subxt::Call for Burn {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u64,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferKeepAlive {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u64,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u64,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Freeze {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Thaw {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for Thaw {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct FreezeAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for FreezeAsset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "freeze_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ThawAsset {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ThawAsset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "thaw_asset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for TransferOwnership {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetTeam {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub issuer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub freezer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetTeam {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
            }
            impl ::subxt::Call for SetMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceSetMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceSetMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_set_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceClearMetadata {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceClearMetadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_clear_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub issuer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub freezer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub min_balance: ::core::primitive::u64,
                pub is_sufficient: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceAssetStatus {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u64,
            }
            impl ::subxt::Call for ApproveTransfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub delegate: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for CancelApproval {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCancelApproval {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub delegate: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ForceCancelApproval {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_cancel_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferApproved {
                #[codec(compact)]
                pub id: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub destination: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u64,
            }
            impl ::subxt::Call for TransferApproved {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_approved";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    id: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    min_balance: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, Create> {
                    let call = Create {
                        id,
                        admin,
                        min_balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_create(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    is_sufficient: ::core::primitive::bool,
                    min_balance: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceCreate> {
                    let call = ForceCreate {
                        id,
                        owner,
                        is_sufficient,
                        min_balance,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn destroy(
                    &self,
                    id: ::core::primitive::u32,
                    witness: runtime_types::pallet_assets::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<T, Destroy> {
                    let call = Destroy { id, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint(
                    &self,
                    id: ::core::primitive::u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, Mint> {
                    let call = Mint {
                        id,
                        beneficiary,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, Burn> {
                    let call = Burn { id, who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferKeepAlive> {
                    let call = TransferKeepAlive { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
                    let call = ForceTransfer {
                        id,
                        source,
                        dest,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Freeze> {
                    let call = Freeze { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Thaw> {
                    let call = Thaw { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, FreezeAsset> {
                    let call = FreezeAsset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ThawAsset> {
                    let call = ThawAsset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_ownership(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferOwnership> {
                    let call = TransferOwnership { id, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_team(
                    &self,
                    id: ::core::primitive::u32,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetTeam> {
                    let call = SetTeam {
                        id,
                        issuer,
                        admin,
                        freezer,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMetadata> {
                    let call = SetMetadata {
                        id,
                        name,
                        symbol,
                        decimals,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearMetadata> {
                    let call = ClearMetadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceSetMetadata> {
                    let call = ForceSetMetadata {
                        id,
                        name,
                        symbol,
                        decimals,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceClearMetadata>
                {
                    let call = ForceClearMetadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_asset_status(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    min_balance: ::core::primitive::u64,
                    is_sufficient: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceAssetStatus> {
                    let call = ForceAssetStatus {
                        id,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        min_balance,
                        is_sufficient,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveTransfer> {
                    let call = ApproveTransfer {
                        id,
                        delegate,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelApproval> {
                    let call = CancelApproval { id, delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceCancelApproval>
                {
                    let call = ForceCancelApproval {
                        id,
                        owner,
                        delegate,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_approved(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    destination: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferApproved> {
                    let call = TransferApproved {
                        id,
                        owner,
                        destination,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Created(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Issued(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transferred(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Burned(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TeamChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OwnerChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Frozen(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Thawed(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetFrozen(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetFrozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetThawed(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetThawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Destroyed(pub ::core::primitive::u32);
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCreated(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MetadataSet(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u8,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MetadataCleared(pub ::core::primitive::u32);
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApprovedTransfer(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApprovalCancelled(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferredApproved(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for TransferredApproved {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetStatusChanged(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Asset(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Asset {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_assets::types::AssetDetails<
                    ::core::primitive::u64,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Account(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_assets::types::AssetBalance<
                    ::core::primitive::u64,
                    (),
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Approvals(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Approvals {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Approvals";
                type Value = runtime_types::pallet_assets::types::Approval<
                    ::core::primitive::u64,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Metadata(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Metadata {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "Metadata";
                type Value = runtime_types::pallet_assets::types::AssetMetadata<
                    ::core::primitive::u128,
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn asset(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::AssetDetails<
                            ::core::primitive::u64,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Asset(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetBalance<
                        ::core::primitive::u64,
                        (),
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn approvals(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    _2: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u64,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Approvals(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn approvals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Approvals>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn metadata(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Metadata(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn metadata_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Metadata>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod mmr {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RootHash;
            impl ::subxt::StorageEntry for RootHash {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "RootHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NumberOfLeaves;
            impl ::subxt::StorageEntry for NumberOfLeaves {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "NumberOfLeaves";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nodes(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Nodes {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "Nodes";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn root_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = RootHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn number_of_leaves(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = NumberOfLeaves;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nodes(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = Nodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nodes>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod lottery {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BuyTicket {
                pub call: runtime_types::node_runtime::Call,
            }
            impl ::subxt::Call for BuyTicket {
                const PALLET: &'static str = "Lottery";
                const FUNCTION: &'static str = "buy_ticket";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCalls {
                pub calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
            }
            impl ::subxt::Call for SetCalls {
                const PALLET: &'static str = "Lottery";
                const FUNCTION: &'static str = "set_calls";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StartLottery {
                pub price: ::core::primitive::u128,
                pub length: ::core::primitive::u32,
                pub delay: ::core::primitive::u32,
                pub repeat: ::core::primitive::bool,
            }
            impl ::subxt::Call for StartLottery {
                const PALLET: &'static str = "Lottery";
                const FUNCTION: &'static str = "start_lottery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StopRepeat {}
            impl ::subxt::Call for StopRepeat {
                const PALLET: &'static str = "Lottery";
                const FUNCTION: &'static str = "stop_repeat";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn buy_ticket(
                    &self,
                    call: runtime_types::node_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, BuyTicket> {
                    let call = BuyTicket { call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_calls(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCalls> {
                    let call = SetCalls { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn start_lottery(
                    &self,
                    price: ::core::primitive::u128,
                    length: ::core::primitive::u32,
                    delay: ::core::primitive::u32,
                    repeat: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, StartLottery> {
                    let call = StartLottery {
                        price,
                        length,
                        delay,
                        repeat,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn stop_repeat(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, StopRepeat> {
                    let call = StopRepeat {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_lottery::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LotteryStarted {}
            impl ::subxt::Event for LotteryStarted {
                const PALLET: &'static str = "Lottery";
                const EVENT: &'static str = "LotteryStarted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CallsUpdated {}
            impl ::subxt::Event for CallsUpdated {
                const PALLET: &'static str = "Lottery";
                const EVENT: &'static str = "CallsUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Winner(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Winner {
                const PALLET: &'static str = "Lottery";
                const EVENT: &'static str = "Winner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TicketBought(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub (::core::primitive::u8, ::core::primitive::u8),
            );
            impl ::subxt::Event for TicketBought {
                const PALLET: &'static str = "Lottery";
                const EVENT: &'static str = "TicketBought";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct LotteryIndex;
            impl ::subxt::StorageEntry for LotteryIndex {
                const PALLET: &'static str = "Lottery";
                const STORAGE: &'static str = "LotteryIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lottery;
            impl ::subxt::StorageEntry for Lottery {
                const PALLET: &'static str = "Lottery";
                const STORAGE: &'static str = "Lottery";
                type Value = runtime_types::pallet_lottery::LotteryConfig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Participants(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Participants {
                const PALLET: &'static str = "Lottery";
                const STORAGE: &'static str = "Participants";
                type Value = (
                    ::core::primitive::u32,
                    ::std::vec::Vec<(::core::primitive::u8, ::core::primitive::u8)>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct TicketsCount;
            impl ::subxt::StorageEntry for TicketsCount {
                const PALLET: &'static str = "Lottery";
                const STORAGE: &'static str = "TicketsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Tickets(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Tickets {
                const PALLET: &'static str = "Lottery";
                const STORAGE: &'static str = "Tickets";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CallIndices;
            impl ::subxt::StorageEntry for CallIndices {
                const PALLET: &'static str = "Lottery";
                const STORAGE: &'static str = "CallIndices";
                type Value =
                    ::std::vec::Vec<(::core::primitive::u8, ::core::primitive::u8)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn lottery_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = LotteryIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lottery(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_lottery::LotteryConfig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Lottery;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn participants(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        ::core::primitive::u32,
                        ::std::vec::Vec<(::core::primitive::u8, ::core::primitive::u8)>,
                    ),
                    ::subxt::Error,
                > {
                    let entry = Participants(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn participants_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Participants>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn tickets_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = TicketsCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn tickets(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Tickets(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn tickets_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Tickets>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn call_indices(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u8, ::core::primitive::u8)>,
                    ::subxt::Error,
                > {
                    let entry = CallIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod gilt {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PlaceBid {
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
                pub duration: ::core::primitive::u32,
            }
            impl ::subxt::Call for PlaceBid {
                const PALLET: &'static str = "Gilt";
                const FUNCTION: &'static str = "place_bid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RetractBid {
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
                pub duration: ::core::primitive::u32,
            }
            impl ::subxt::Call for RetractBid {
                const PALLET: &'static str = "Gilt";
                const FUNCTION: &'static str = "retract_bid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetTarget {
                #[codec(compact)]
                pub target: ::subxt::sp_arithmetic::per_things::Perquintill,
            }
            impl ::subxt::Call for SetTarget {
                const PALLET: &'static str = "Gilt";
                const FUNCTION: &'static str = "set_target";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Thaw {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Thaw {
                const PALLET: &'static str = "Gilt";
                const FUNCTION: &'static str = "thaw";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn place_bid(
                    &self,
                    amount: ::core::primitive::u128,
                    duration: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, PlaceBid> {
                    let call = PlaceBid { amount, duration };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn retract_bid(
                    &self,
                    amount: ::core::primitive::u128,
                    duration: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RetractBid> {
                    let call = RetractBid { amount, duration };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_target(
                    &self,
                    target: ::subxt::sp_arithmetic::per_things::Perquintill,
                ) -> ::subxt::SubmittableExtrinsic<T, SetTarget> {
                    let call = SetTarget { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Thaw> {
                    let call = Thaw { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_gilt::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BidPlaced(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for BidPlaced {
                const PALLET: &'static str = "Gilt";
                const EVENT: &'static str = "BidPlaced";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BidRetracted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for BidRetracted {
                const PALLET: &'static str = "Gilt";
                const EVENT: &'static str = "BidRetracted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct GiltIssued(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for GiltIssued {
                const PALLET: &'static str = "Gilt";
                const EVENT: &'static str = "GiltIssued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct GiltThawed(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for GiltThawed {
                const PALLET: &'static str = "Gilt";
                const EVENT: &'static str = "GiltThawed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct QueueTotals;
            impl ::subxt::StorageEntry for QueueTotals {
                const PALLET: &'static str = "Gilt";
                const STORAGE: &'static str = "QueueTotals";
                type Value =
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u128)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Queues(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Queues {
                const PALLET: &'static str = "Gilt";
                const STORAGE: &'static str = "Queues";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_gilt::pallet::GiltBid<
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ActiveTotal;
            impl ::subxt::StorageEntry for ActiveTotal {
                const PALLET: &'static str = "Gilt";
                const STORAGE: &'static str = "ActiveTotal";
                type Value = runtime_types::pallet_gilt::pallet::ActiveGiltsTotal<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Active(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Active {
                const PALLET: &'static str = "Gilt";
                const STORAGE: &'static str = "Active";
                type Value = runtime_types::pallet_gilt::pallet::ActiveGilt<
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn queue_totals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u128)>,
                    ::subxt::Error,
                > {
                    let entry = QueueTotals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queues(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_gilt::pallet::GiltBid<
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Queues(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queues_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Queues>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn active_total(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_gilt::pallet::ActiveGiltsTotal<
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ActiveTotal;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn active(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_gilt::pallet::ActiveGilt<
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Active(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Active>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod uniques {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Create {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCreate {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub free_holding: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceCreate {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Destroy {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub witness: runtime_types::pallet_uniques::types::DestroyWitness,
            }
            impl ::subxt::Call for Destroy {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Mint {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for Mint {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Burn {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub check_owner: ::core::option::Option<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for Burn {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Redeposit {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub instances: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for Redeposit {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "redeposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Freeze {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Thaw {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for Thaw {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct FreezeClass {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for FreezeClass {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "freeze_class";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ThawClass {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for ThawClass {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "thaw_class";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferOwnership {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for TransferOwnership {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetTeam {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub issuer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub freezer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetTeam {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveTransfer {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub delegate: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ApproveTransfer {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelApproval {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub maybe_check_delegate: ::core::option::Option<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for CancelApproval {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceAssetStatus {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub owner: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub issuer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub admin: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub freezer: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub free_holding: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceAssetStatus {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetAttribute {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for SetAttribute {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_attribute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearAttribute {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for ClearAttribute {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_attribute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for SetMetadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                #[codec(compact)]
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearMetadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetClassMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for SetClassMetadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_class_metadata";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearClassMetadata {
                #[codec(compact)]
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for ClearClassMetadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_class_metadata";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    class: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Create> {
                    let call = Create { class, admin };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_create(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    free_holding: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceCreate> {
                    let call = ForceCreate {
                        class,
                        owner,
                        free_holding,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn destroy(
                    &self,
                    class: ::core::primitive::u32,
                    witness: runtime_types::pallet_uniques::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<T, Destroy> {
                    let call = Destroy { class, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Mint> {
                    let call = Mint {
                        class,
                        instance,
                        owner,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    check_owner: ::core::option::Option<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Burn> {
                    let call = Burn {
                        class,
                        instance,
                        check_owner,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer {
                        class,
                        instance,
                        dest,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn redeposit(
                    &self,
                    class: ::core::primitive::u32,
                    instances: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, Redeposit> {
                    let call = Redeposit { class, instances };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Freeze> {
                    let call = Freeze { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Thaw> {
                    let call = Thaw { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze_class(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, FreezeClass> {
                    let call = FreezeClass { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw_class(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ThawClass> {
                    let call = ThawClass { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_ownership(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferOwnership> {
                    let call = TransferOwnership { class, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_team(
                    &self,
                    class: ::core::primitive::u32,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetTeam> {
                    let call = SetTeam {
                        class,
                        issuer,
                        admin,
                        freezer,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_transfer(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveTransfer> {
                    let call = ApproveTransfer {
                        class,
                        instance,
                        delegate,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_approval(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    maybe_check_delegate: ::core::option::Option<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelApproval> {
                    let call = CancelApproval {
                        class,
                        instance,
                        maybe_check_delegate,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_asset_status(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    free_holding: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceAssetStatus> {
                    let call = ForceAssetStatus {
                        class,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        free_holding,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_attribute(
                    &self,
                    class: ::core::primitive::u32,
                    maybe_instance: ::core::option::Option<::core::primitive::u32>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetAttribute> {
                    let call = SetAttribute {
                        class,
                        maybe_instance,
                        key,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_attribute(
                    &self,
                    class: ::core::primitive::u32,
                    maybe_instance: ::core::option::Option<::core::primitive::u32>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearAttribute> {
                    let call = ClearAttribute {
                        class,
                        maybe_instance,
                        key,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMetadata> {
                    let call = SetMetadata {
                        class,
                        instance,
                        data,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearMetadata> {
                    let call = ClearMetadata { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_class_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, SetClassMetadata> {
                    let call = SetClassMetadata {
                        class,
                        data,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_class_metadata(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearClassMetadata>
                {
                    let call = ClearClassMetadata { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_uniques::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Created(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCreated(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Destroyed(pub ::core::primitive::u32);
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Issued(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Issued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transferred(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Burned(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Burned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Frozen(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Thawed(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClassFrozen(pub ::core::primitive::u32);
            impl ::subxt::Event for ClassFrozen {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ClassFrozen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClassThawed(pub ::core::primitive::u32);
            impl ::subxt::Event for ClassThawed {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ClassThawed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OwnerChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TeamChanged(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApprovedTransfer(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApprovalCancelled(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetStatusChanged(pub ::core::primitive::u32);
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "AssetStatusChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClassMetadataSet(
                pub ::core::primitive::u32,
                pub  runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for ClassMetadataSet {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ClassMetadataSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClassMetadataCleared(pub ::core::primitive::u32);
            impl ::subxt::Event for ClassMetadataCleared {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "ClassMetadataCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MetadataSet(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub  runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MetadataCleared(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Redeposited(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u32>,
            );
            impl ::subxt::Event for Redeposited {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "Redeposited";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AttributeSet(
                pub ::core::primitive::u32,
                pub ::core::option::Option<::core::primitive::u32>,
                pub  runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub  runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            );
            impl ::subxt::Event for AttributeSet {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "AttributeSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AttributeCleared(
                pub ::core::primitive::u32,
                pub ::core::option::Option<::core::primitive::u32>,
                pub  runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            );
            impl ::subxt::Event for AttributeCleared {
                const PALLET: &'static str = "Uniques";
                const EVENT: &'static str = "AttributeCleared";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Class(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Class {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "Class";
                type Value = runtime_types::pallet_uniques::types::ClassDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Account(
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "Account";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Asset(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for Asset {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_uniques::types::InstanceDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ClassMetadataOf(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ClassMetadataOf {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "ClassMetadataOf";
                type Value = runtime_types::pallet_uniques::types::ClassMetadata<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct InstanceMetadataOf(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for InstanceMetadataOf {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "InstanceMetadataOf";
                type Value = runtime_types::pallet_uniques::types::InstanceMetadata<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Attribute(
                ::core::primitive::u32,
                ::core::option::Option<::core::primitive::u32>,
                runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            );
            impl ::subxt::StorageEntry for Attribute {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "Attribute";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn class(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::ClassDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Class(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn class_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Class>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::core::primitive::u32,
                    _2: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::Error>
                {
                    let entry = Account(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::InstanceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Asset(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn class_metadata_of(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::ClassMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ClassMetadataOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn class_metadata_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClassMetadataOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn instance_metadata_of(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::InstanceMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = InstanceMetadataOf(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn instance_metadata_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InstanceMetadataOf>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn attribute(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::option::Option<::core::primitive::u32>,
                    _2: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Attribute(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn attribute_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Attribute>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod transaction_storage {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Store {
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Store {
                const PALLET: &'static str = "TransactionStorage";
                const FUNCTION: &'static str = "store";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Renew {
                pub block: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Renew {
                const PALLET: &'static str = "TransactionStorage";
                const FUNCTION: &'static str = "renew";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CheckProof {
                pub proof:
                    runtime_types::sp_transaction_storage_proof::TransactionStorageProof,
            }
            impl ::subxt::Call for CheckProof {
                const PALLET: &'static str = "TransactionStorage";
                const FUNCTION: &'static str = "check_proof";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn store(
                    &self,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Store> {
                    let call = Store { data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn renew(
                    &self,
                    block: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Renew> {
                    let call = Renew { block, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn check_proof(
                    &self,
                    proof : runtime_types :: sp_transaction_storage_proof :: TransactionStorageProof,
                ) -> ::subxt::SubmittableExtrinsic<T, CheckProof> {
                    let call = CheckProof { proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_transaction_storage::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Stored(pub ::core::primitive::u32);
            impl ::subxt::Event for Stored {
                const PALLET: &'static str = "TransactionStorage";
                const EVENT: &'static str = "Stored";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Renewed(pub ::core::primitive::u32);
            impl ::subxt::Event for Renewed {
                const PALLET: &'static str = "TransactionStorage";
                const EVENT: &'static str = "Renewed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProofChecked {}
            impl ::subxt::Event for ProofChecked {
                const PALLET: &'static str = "TransactionStorage";
                const EVENT: &'static str = "ProofChecked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Transactions(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Transactions {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "Transactions";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_transaction_storage::TransactionInfo,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ChunkCount(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ChunkCount {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "ChunkCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ByteFee;
            impl ::subxt::StorageEntry for ByteFee {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "ByteFee";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EntryFee;
            impl ::subxt::StorageEntry for EntryFee {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "EntryFee";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxTransactionSize;
            impl ::subxt::StorageEntry for MaxTransactionSize {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "MaxTransactionSize";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxBlockTransactions;
            impl ::subxt::StorageEntry for MaxBlockTransactions {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "MaxBlockTransactions";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StoragePeriod;
            impl ::subxt::StorageEntry for StoragePeriod {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "StoragePeriod";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockTransactions;
            impl ::subxt::StorageEntry for BlockTransactions {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "BlockTransactions";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_transaction_storage::TransactionInfo,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ProofChecked;
            impl ::subxt::StorageEntry for ProofChecked {
                const PALLET: &'static str = "TransactionStorage";
                const STORAGE: &'static str = "ProofChecked";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn transactions(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<
                            runtime_types::pallet_transaction_storage::TransactionInfo,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Transactions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn transactions_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Transactions>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn chunk_count(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ChunkCount(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn chunk_count_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ChunkCount>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn byte_fee(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = ByteFee;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn entry_fee(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = EntryFee;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn max_transaction_size(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = MaxTransactionSize;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_block_transactions(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = MaxBlockTransactions;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_period(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = StoragePeriod;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_transactions(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_transaction_storage::TransactionInfo,
                    >,
                    ::subxt::Error,
                > {
                    let entry = BlockTransactions;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proof_checked(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = ProofChecked;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod bags_list {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rebag {
                pub dislocated: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for Rebag {
                const PALLET: &'static str = "BagsList";
                const FUNCTION: &'static str = "rebag";
            }
            pub struct TransactionApi<
                'a,
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            > {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn rebag(
                    &self,
                    dislocated: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, Rebag> {
                    let call = Rebag { dislocated };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bags_list::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rebagged(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Rebagged {
                const PALLET: &'static str = "BagsList";
                const EVENT: &'static str = "Rebagged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct CounterForListNodes;
            impl ::subxt::StorageEntry for CounterForListNodes {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "CounterForListNodes";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ListNodes(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ListNodes {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "ListNodes";
                type Value = runtime_types::pallet_bags_list::list::Node;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ListBags(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for ListBags {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "ListBags";
                type Value = runtime_types::pallet_bags_list::list::Bag;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn counter_for_list_nodes(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CounterForListNodes;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn list_nodes(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_bags_list::list::Node>,
                    ::subxt::Error,
                > {
                    let entry = ListNodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn list_nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ListNodes>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn list_bags(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_bags_list::list::Bag>,
                    ::subxt::Error,
                > {
                    let entry = ListBags(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn list_bags_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ListBags>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_btree_map {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct BoundedBTreeMap<_0, _1>(
                        pub ::std::collections::BTreeMap<_0, _1>,
                    );
                }
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct WrapperKeepOpaque<_0>(::core::primitive::u32, pub _0);
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct WrapperOpaque<_0>(::core::primitive::u32, pub _0);
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                        )]
                        pub enum BalanceStatus {
                            Free,
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum DispatchClass {
                    Normal,
                    Operational,
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Pays {
                    Yes,
                    No,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: ::subxt::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckGenesis {}
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckMortality(
                        pub runtime_types::sp_runtime::generic::era::Era,
                    );
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckNonce(pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckSpecVersion {}
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckTxVersion {}
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct CheckWeight {}
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class:
                        runtime_types::frame_support::weights::PerDispatchClass<
                            runtime_types::frame_system::limits::WeightsPerClass,
                        >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    fill_block { ratio : :: subxt :: sp_arithmetic :: per_things :: Perbill , } , remark { remark : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , set_heap_pages { pages : :: core :: primitive :: u64 , } , set_code { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , set_code_without_checks { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , set_changes_trie_config { changes_trie_config : :: core :: option :: Option < runtime_types :: sp_core :: changes_trie :: ChangesTrieConfiguration > , } , set_storage { items : :: std :: vec :: Vec < (:: std :: vec :: Vec < :: core :: primitive :: u8 > , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) > , } , kill_storage { keys : :: std :: vec :: Vec < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , } , kill_prefix { prefix : :: std :: vec :: Vec < :: core :: primitive :: u8 > , subkeys : :: core :: primitive :: u32 , } , remark_with_event { remark : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InvalidSpecName,
                    SpecVersionNeedsToIncrease,
                    FailedToExtractRuntimeVersion,
                    NonDefaultComposite,
                    NonZeroRefCount,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    CodeUpdated,
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    Remarked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Phase {
                ApplyExtrinsic(::core::primitive::u32),
                Finalization,
                Initialization,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum RawOrigin<_0> {
                Root,
                Signed(_0),
                None,
            }
        }
        pub mod node_runtime {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Call {
                System(runtime_types::frame_system::pallet::Call),
                Utility(runtime_types::pallet_utility::pallet::Call),
                Babe(runtime_types::pallet_babe::pallet::Call),
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                Indices(runtime_types::pallet_indices::pallet::Call),
                Balances(runtime_types::pallet_balances::pallet::Call),
                ElectionProviderMultiPhase(
                    runtime_types::pallet_election_provider_multi_phase::pallet::Call,
                ),
                Staking(runtime_types::pallet_staking::pallet::pallet::Call),
                Session(runtime_types::pallet_session::pallet::Call),
                Democracy(runtime_types::pallet_democracy::pallet::Call),
                Council(runtime_types::pallet_collective::pallet::Call),
                TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
                Elections(runtime_types::pallet_elections_phragmen::pallet::Call),
                TechnicalMembership(runtime_types::pallet_membership::pallet::Call),
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                Treasury(runtime_types::pallet_treasury::pallet::Call),
                Contracts(runtime_types::pallet_contracts::pallet::Call),
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                Identity(runtime_types::pallet_identity::pallet::Call),
                Society(runtime_types::pallet_society::pallet::Call),
                Recovery(runtime_types::pallet_recovery::pallet::Call),
                Vesting(runtime_types::pallet_vesting::pallet::Call),
                Scheduler(runtime_types::pallet_scheduler::pallet::Call),
                Proxy(runtime_types::pallet_proxy::pallet::Call),
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                Bounties(runtime_types::pallet_bounties::pallet::Call),
                Tips(runtime_types::pallet_tips::pallet::Call),
                Assets(runtime_types::pallet_assets::pallet::Call),
                Lottery(runtime_types::pallet_lottery::pallet::Call),
                Gilt(runtime_types::pallet_gilt::pallet::Call),
                Uniques(runtime_types::pallet_uniques::pallet::Call),
                TransactionStorage(
                    runtime_types::pallet_transaction_storage::pallet::Call,
                ),
                BagsList(runtime_types::pallet_bags_list::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Event {
                System(runtime_types::frame_system::pallet::Event),
                Utility(runtime_types::pallet_utility::pallet::Event),
                Indices(runtime_types::pallet_indices::pallet::Event),
                Balances(runtime_types::pallet_balances::pallet::Event),
                ElectionProviderMultiPhase(
                    runtime_types::pallet_election_provider_multi_phase::pallet::Event,
                ),
                Staking(runtime_types::pallet_staking::pallet::pallet::Event),
                Session(runtime_types::pallet_session::pallet::Event),
                Democracy(runtime_types::pallet_democracy::pallet::Event),
                Council(runtime_types::pallet_collective::pallet::Event),
                TechnicalCommittee(runtime_types::pallet_collective::pallet::Event),
                Elections(runtime_types::pallet_elections_phragmen::pallet::Event),
                TechnicalMembership(runtime_types::pallet_membership::pallet::Event),
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                Treasury(runtime_types::pallet_treasury::pallet::Event),
                Contracts(runtime_types::pallet_contracts::pallet::Event),
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                Offences(runtime_types::pallet_offences::pallet::Event),
                Identity(runtime_types::pallet_identity::pallet::Event),
                Society(runtime_types::pallet_society::pallet::Event),
                Recovery(runtime_types::pallet_recovery::pallet::Event),
                Vesting(runtime_types::pallet_vesting::pallet::Event),
                Scheduler(runtime_types::pallet_scheduler::pallet::Event),
                Proxy(runtime_types::pallet_proxy::pallet::Event),
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                Bounties(runtime_types::pallet_bounties::pallet::Event),
                Tips(runtime_types::pallet_tips::pallet::Event),
                Assets(runtime_types::pallet_assets::pallet::Event),
                Lottery(runtime_types::pallet_lottery::pallet::Event),
                Gilt(runtime_types::pallet_gilt::pallet::Event),
                Uniques(runtime_types::pallet_uniques::pallet::Event),
                TransactionStorage(
                    runtime_types::pallet_transaction_storage::pallet::Event,
                ),
                BagsList(runtime_types::pallet_bags_list::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NposSolution16 {
                votes1: ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u16)>,
                votes2: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    (
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ),
                    ::core::primitive::u16,
                )>,
                votes3: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 2usize],
                    ::core::primitive::u16,
                )>,
                votes4: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 3usize],
                    ::core::primitive::u16,
                )>,
                votes5: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 4usize],
                    ::core::primitive::u16,
                )>,
                votes6: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 5usize],
                    ::core::primitive::u16,
                )>,
                votes7: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 6usize],
                    ::core::primitive::u16,
                )>,
                votes8: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 7usize],
                    ::core::primitive::u16,
                )>,
                votes9: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 8usize],
                    ::core::primitive::u16,
                )>,
                votes10: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 9usize],
                    ::core::primitive::u16,
                )>,
                votes11: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 10usize],
                    ::core::primitive::u16,
                )>,
                votes12: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 11usize],
                    ::core::primitive::u16,
                )>,
                votes13: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 12usize],
                    ::core::primitive::u16,
                )>,
                votes14: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 13usize],
                    ::core::primitive::u16,
                )>,
                votes15: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 14usize],
                    ::core::primitive::u16,
                )>,
                votes16: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 15usize],
                    ::core::primitive::u16,
                )>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum OriginCaller {
                system(
                    runtime_types::frame_system::RawOrigin<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ),
                Council(
                    runtime_types::pallet_collective::RawOrigin<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ),
                TechnicalCommittee(
                    runtime_types::pallet_collective::RawOrigin<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ),
                Void(runtime_types::sp_core::Void),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum ProxyType {
                Any,
                NonTransfer,
                Governance,
                Staking,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Runtime {}
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SessionKeys {
                pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                pub babe: runtime_types::sp_consensus_babe::app::Public,
                pub im_online:
                    runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                pub authority_discovery:
                    runtime_types::sp_authority_discovery::app::Public,
            }
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        min_balance: ::core::primitive::u64,
                    },
                    force_create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u64,
                    },
                    destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        witness: runtime_types::pallet_assets::types::DestroyWitness,
                    },
                    mint {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u64,
                    },
                    burn {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u64,
                    },
                    transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u64,
                    },
                    transfer_keep_alive {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u64,
                    },
                    force_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u64,
                    },
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    set_team {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    force_asset_status {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u64,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    approve_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u64,
                    },
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    force_cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    transfer_approved {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        destination: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u64,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    BalanceLow,
                    BalanceZero,
                    NoPermission,
                    Unknown,
                    Frozen,
                    InUse,
                    BadWitness,
                    MinBalanceZero,
                    NoProvider,
                    BadMetadata,
                    Unapproved,
                    WouldDie,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Created(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Issued(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    Transferred(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    Burned(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    TeamChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    OwnerChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Frozen(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Thawed(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    AssetFrozen(::core::primitive::u32),
                    AssetThawed(::core::primitive::u32),
                    Destroyed(::core::primitive::u32),
                    ForceCreated(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    MetadataSet(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u8,
                        ::core::primitive::bool,
                    ),
                    MetadataCleared(::core::primitive::u32),
                    ApprovedTransfer(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    ApprovalCancelled(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    TransferredApproved(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    AssetStatusChanged(::core::primitive::u32),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AssetBalance<_0, _1> {
                    pub balance: _0,
                    pub is_frozen: ::core::primitive::bool,
                    pub sufficient: ::core::primitive::bool,
                    pub extra: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _2,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub accounts: ::core::primitive::u32,
                    #[codec(compact)]
                    pub sufficients: ::core::primitive::u32,
                    #[codec(compact)]
                    pub approvals: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InvalidUncleParent,
                    UnclesAlreadySet,
                    TooManyUncles,
                    GenesisUncle,
                    TooHighUncle,
                    UncleAlreadyIncluded,
                    OldUncle,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum UncleEntryItem<_0, _1, _2> {
                InclusionHeight(_0),
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    report_equivocation { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , report_equivocation_unsigned { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , plan_config_change { config : runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InvalidEquivocationProof,
                    InvalidKeyOwnershipProof,
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_bags_list {
            use super::runtime_types;
            pub mod list {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Bag {
                    pub head:
                        ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    pub tail:
                        ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Node {
                    pub id: ::subxt::sp_core::crypto::AccountId32,
                    pub prev:
                        ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    pub next:
                        ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    pub bag_upper: ::core::primitive::u64,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    rebag {
                        dislocated: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Rebagged(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    VestingBalance,
                    LiquidityRestrictions,
                    InsufficientBalance,
                    ExistentialDeposit,
                    KeepAlive,
                    ExistingVestingSchedule,
                    DeadAccount,
                    TooManyReserves,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Endowed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    DustLost(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    Transfer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    BalanceSet(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    Reserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    Unreserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    ReserveRepatriated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    ),
                    Deposit(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    Withdraw(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    Slashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Reasons {
                Fee,
                Misc,
                All,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                V1_0_0,
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_bounties {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    propose_bounty {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        description: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    approve_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    propose_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        curator: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    unassign_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    accept_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    award_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    claim_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    close_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    extend_bounty_expiry {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InsufficientProposersBalance,
                    InvalidIndex,
                    ReasonTooBig,
                    UnexpectedStatus,
                    RequireCurator,
                    InvalidValue,
                    InvalidFee,
                    PendingPayout,
                    Premature,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    BountyProposed(::core::primitive::u32),
                    BountyRejected(::core::primitive::u32, ::core::primitive::u128),
                    BountyBecameActive(::core::primitive::u32),
                    BountyAwarded(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    BountyClaimed(
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    BountyCanceled(::core::primitive::u32),
                    BountyExtended(::core::primitive::u32),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bounty<_0, _1, _2> {
                pub proposer: _0,
                pub value: _1,
                pub fee: _1,
                pub curator_deposit: _1,
                pub bond: _1,
                pub status: runtime_types::pallet_bounties::BountyStatus<_0, _2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum BountyStatus<_0, _1> {
                Proposed,
                Approved,
                Funded,
                CuratorProposed {
                    curator: _0,
                },
                Active {
                    curator: _0,
                    update_due: _1,
                },
                PendingPayout {
                    curator: _0,
                    beneficiary: _0,
                    unlock_at: _1,
                },
            }
        }
        pub mod pallet_collective {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    set_members {
                        new_members:
                            ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        prime:
                            ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                        old_count: ::core::primitive::u32,
                    },
                    execute {
                        proposal: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    propose {
                        #[codec(compact)]
                        threshold: ::core::primitive::u32,
                        proposal: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    vote {
                        proposal: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        approve: ::core::primitive::bool,
                    },
                    close {
                        proposal_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        proposal_weight_bound: ::core::primitive::u64,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    disapprove_proposal {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    NotMember,
                    DuplicateProposal,
                    ProposalMissing,
                    WrongIndex,
                    DuplicateVote,
                    AlreadyInitialized,
                    TooEarly,
                    TooManyProposals,
                    WrongProposalWeight,
                    WrongProposalLength,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Proposed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    ),
                    Voted(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                        ::core::primitive::bool,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                    Approved(::subxt::sp_core::H256),
                    Disapproved(::subxt::sp_core::H256),
                    Executed(
                        ::subxt::sp_core::H256,
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    MemberExecuted(
                        ::subxt::sp_core::H256,
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    Closed(
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum RawOrigin<_0> {
                Members(::core::primitive::u32, ::core::primitive::u32),
                Member(_0),
                _Phantom,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Votes<_0, _1> {
                pub index: _1,
                pub threshold: _1,
                pub ayes: ::std::vec::Vec<_0>,
                pub nays: ::std::vec::Vec<_0>,
                pub end: _1,
            }
        }
        pub mod pallet_contracts {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    call {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    instantiate_with_code {
                        #[codec(compact)]
                        endowment: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    instantiate {
                        #[codec(compact)]
                        endowment: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        code_hash: ::subxt::sp_core::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InvalidScheduleVersion,
                    OutOfGas,
                    OutputBufferTooSmall,
                    BelowSubsistenceThreshold,
                    NewContractNotFunded,
                    TransferFailed,
                    MaxCallDepthReached,
                    ContractNotFound,
                    CodeTooLarge,
                    CodeNotFound,
                    OutOfBounds,
                    DecodingFailed,
                    ContractTrapped,
                    ValueTooLarge,
                    TerminatedWhileReentrant,
                    InputForwarded,
                    RandomSubjectTooLong,
                    TooManyTopics,
                    DuplicateTopics,
                    NoChainExtension,
                    DeletionQueueFull,
                    StorageExhausted,
                    DuplicateContract,
                    TerminatedInConstructor,
                    DebugMessageInvalidUTF8,
                    ReentranceDenied,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Instantiated {
                        deployer: ::subxt::sp_core::crypto::AccountId32,
                        contract: ::subxt::sp_core::crypto::AccountId32,
                    },
                    Terminated {
                        contract: ::subxt::sp_core::crypto::AccountId32,
                        beneficiary: ::subxt::sp_core::crypto::AccountId32,
                    },
                    CodeStored {
                        code_hash: ::subxt::sp_core::H256,
                    },
                    ScheduleUpdated {
                        version: ::core::primitive::u32,
                    },
                    ContractEmitted {
                        contract: ::subxt::sp_core::crypto::AccountId32,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    CodeRemoved {
                        code_hash: ::subxt::sp_core::H256,
                    },
                }
            }
            pub mod schedule {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct HostFnWeights {
                    pub caller: ::core::primitive::u64,
                    pub address: ::core::primitive::u64,
                    pub gas_left: ::core::primitive::u64,
                    pub balance: ::core::primitive::u64,
                    pub value_transferred: ::core::primitive::u64,
                    pub minimum_balance: ::core::primitive::u64,
                    pub contract_deposit: ::core::primitive::u64,
                    pub block_number: ::core::primitive::u64,
                    pub now: ::core::primitive::u64,
                    pub weight_to_fee: ::core::primitive::u64,
                    pub gas: ::core::primitive::u64,
                    pub input: ::core::primitive::u64,
                    pub input_per_byte: ::core::primitive::u64,
                    pub r#return: ::core::primitive::u64,
                    pub return_per_byte: ::core::primitive::u64,
                    pub terminate: ::core::primitive::u64,
                    pub random: ::core::primitive::u64,
                    pub deposit_event: ::core::primitive::u64,
                    pub deposit_event_per_topic: ::core::primitive::u64,
                    pub deposit_event_per_byte: ::core::primitive::u64,
                    pub debug_message: ::core::primitive::u64,
                    pub set_storage: ::core::primitive::u64,
                    pub set_storage_per_byte: ::core::primitive::u64,
                    pub clear_storage: ::core::primitive::u64,
                    pub get_storage: ::core::primitive::u64,
                    pub get_storage_per_byte: ::core::primitive::u64,
                    pub transfer: ::core::primitive::u64,
                    pub call: ::core::primitive::u64,
                    pub call_transfer_surcharge: ::core::primitive::u64,
                    pub call_per_input_byte: ::core::primitive::u64,
                    pub call_per_output_byte: ::core::primitive::u64,
                    pub instantiate: ::core::primitive::u64,
                    pub instantiate_per_input_byte: ::core::primitive::u64,
                    pub instantiate_per_output_byte: ::core::primitive::u64,
                    pub instantiate_per_salt_byte: ::core::primitive::u64,
                    pub hash_sha2_256: ::core::primitive::u64,
                    pub hash_sha2_256_per_byte: ::core::primitive::u64,
                    pub hash_keccak_256: ::core::primitive::u64,
                    pub hash_keccak_256_per_byte: ::core::primitive::u64,
                    pub hash_blake2_256: ::core::primitive::u64,
                    pub hash_blake2_256_per_byte: ::core::primitive::u64,
                    pub hash_blake2_128: ::core::primitive::u64,
                    pub hash_blake2_128_per_byte: ::core::primitive::u64,
                    pub ecdsa_recover: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct InstructionWeights {
                    pub version: ::core::primitive::u32,
                    pub i64const: ::core::primitive::u32,
                    pub i64load: ::core::primitive::u32,
                    pub i64store: ::core::primitive::u32,
                    pub select: ::core::primitive::u32,
                    pub r#if: ::core::primitive::u32,
                    pub br: ::core::primitive::u32,
                    pub br_if: ::core::primitive::u32,
                    pub br_table: ::core::primitive::u32,
                    pub br_table_per_entry: ::core::primitive::u32,
                    pub call: ::core::primitive::u32,
                    pub call_indirect: ::core::primitive::u32,
                    pub call_indirect_per_param: ::core::primitive::u32,
                    pub local_get: ::core::primitive::u32,
                    pub local_set: ::core::primitive::u32,
                    pub local_tee: ::core::primitive::u32,
                    pub global_get: ::core::primitive::u32,
                    pub global_set: ::core::primitive::u32,
                    pub memory_current: ::core::primitive::u32,
                    pub memory_grow: ::core::primitive::u32,
                    pub i64clz: ::core::primitive::u32,
                    pub i64ctz: ::core::primitive::u32,
                    pub i64popcnt: ::core::primitive::u32,
                    pub i64eqz: ::core::primitive::u32,
                    pub i64extendsi32: ::core::primitive::u32,
                    pub i64extendui32: ::core::primitive::u32,
                    pub i32wrapi64: ::core::primitive::u32,
                    pub i64eq: ::core::primitive::u32,
                    pub i64ne: ::core::primitive::u32,
                    pub i64lts: ::core::primitive::u32,
                    pub i64ltu: ::core::primitive::u32,
                    pub i64gts: ::core::primitive::u32,
                    pub i64gtu: ::core::primitive::u32,
                    pub i64les: ::core::primitive::u32,
                    pub i64leu: ::core::primitive::u32,
                    pub i64ges: ::core::primitive::u32,
                    pub i64geu: ::core::primitive::u32,
                    pub i64add: ::core::primitive::u32,
                    pub i64sub: ::core::primitive::u32,
                    pub i64mul: ::core::primitive::u32,
                    pub i64divs: ::core::primitive::u32,
                    pub i64divu: ::core::primitive::u32,
                    pub i64rems: ::core::primitive::u32,
                    pub i64remu: ::core::primitive::u32,
                    pub i64and: ::core::primitive::u32,
                    pub i64or: ::core::primitive::u32,
                    pub i64xor: ::core::primitive::u32,
                    pub i64shl: ::core::primitive::u32,
                    pub i64shrs: ::core::primitive::u32,
                    pub i64shru: ::core::primitive::u32,
                    pub i64rotl: ::core::primitive::u32,
                    pub i64rotr: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Limits {
                    pub event_topics: ::core::primitive::u32,
                    pub stack_height: ::core::primitive::u32,
                    pub globals: ::core::primitive::u32,
                    pub parameters: ::core::primitive::u32,
                    pub memory_pages: ::core::primitive::u32,
                    pub table_size: ::core::primitive::u32,
                    pub br_table_size: ::core::primitive::u32,
                    pub subject_len: ::core::primitive::u32,
                    pub call_depth: ::core::primitive::u32,
                    pub payload_len: ::core::primitive::u32,
                    pub code_len: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Schedule {
                    pub limits: runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights:
                        runtime_types::pallet_contracts::schedule::InstructionWeights,
                    pub host_fn_weights:
                        runtime_types::pallet_contracts::schedule::HostFnWeights,
                }
            }
            pub mod storage {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DeletedContract {
                    pub trie_id: ::std::vec::Vec<::core::primitive::u8>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RawContractInfo<_0> {
                    pub trie_id: ::std::vec::Vec<::core::primitive::u8>,
                    pub code_hash: _0,
                    pub _reserved: ::core::option::Option<()>,
                }
            }
            pub mod wasm {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct PrefabWasmModule {
                    #[codec(compact)]
                    pub instruction_weights_version: ::core::primitive::u32,
                    #[codec(compact)]
                    pub initial: ::core::primitive::u32,
                    #[codec(compact)]
                    pub maximum: ::core::primitive::u32,
                    #[codec(compact)]
                    pub refcount: ::core::primitive::u64,
                    pub _reserved: ::core::option::Option<()>,
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                    pub original_code_len: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_democracy {
            use super::runtime_types;
            pub mod conviction {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Conviction {
                    None,
                    Locked1x,
                    Locked2x,
                    Locked3x,
                    Locked4x,
                    Locked5x,
                    Locked6x,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    propose {
                        proposal_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    second {
                        #[codec(compact)]
                        proposal: ::core::primitive::u32,
                        #[codec(compact)]
                        seconds_upper_bound: ::core::primitive::u32,
                    },
                    vote {
                        #[codec(compact)]
                        ref_index: ::core::primitive::u32,
                        vote: runtime_types::pallet_democracy::vote::AccountVote<
                            ::core::primitive::u128,
                        >,
                    },
                    emergency_cancel {
                        ref_index: ::core::primitive::u32,
                    },
                    external_propose {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                    external_propose_majority {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                    external_propose_default {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                    fast_track {
                        proposal_hash: ::subxt::sp_core::H256,
                        voting_period: ::core::primitive::u32,
                        delay: ::core::primitive::u32,
                    },
                    veto_external {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                    cancel_referendum {
                        #[codec(compact)]
                        ref_index: ::core::primitive::u32,
                    },
                    cancel_queued {
                        which: ::core::primitive::u32,
                    },
                    delegate {
                        to: ::subxt::sp_core::crypto::AccountId32,
                        conviction:
                            runtime_types::pallet_democracy::conviction::Conviction,
                        balance: ::core::primitive::u128,
                    },
                    undelegate,
                    clear_public_proposals,
                    note_preimage {
                        encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    note_preimage_operational {
                        encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    note_imminent_preimage {
                        encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    note_imminent_preimage_operational {
                        encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    reap_preimage {
                        proposal_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        proposal_len_upper_bound: ::core::primitive::u32,
                    },
                    unlock {
                        target: ::subxt::sp_core::crypto::AccountId32,
                    },
                    remove_vote {
                        index: ::core::primitive::u32,
                    },
                    remove_other_vote {
                        target: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                    },
                    enact_proposal {
                        proposal_hash: ::subxt::sp_core::H256,
                        index: ::core::primitive::u32,
                    },
                    blacklist {
                        proposal_hash: ::subxt::sp_core::H256,
                        maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
                    },
                    cancel_proposal {
                        #[codec(compact)]
                        prop_index: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    ValueLow,
                    ProposalMissing,
                    AlreadyCanceled,
                    DuplicateProposal,
                    ProposalBlacklisted,
                    NotSimpleMajority,
                    InvalidHash,
                    NoProposal,
                    AlreadyVetoed,
                    DuplicatePreimage,
                    NotImminent,
                    TooEarly,
                    Imminent,
                    PreimageMissing,
                    ReferendumInvalid,
                    PreimageInvalid,
                    NoneWaiting,
                    NotVoter,
                    NoPermission,
                    AlreadyDelegating,
                    InsufficientFunds,
                    NotDelegating,
                    VotesExist,
                    InstantNotAllowed,
                    Nonsense,
                    WrongUpperBound,
                    MaxVotesReached,
                    TooManyProposals,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Proposed(::core::primitive::u32, ::core::primitive::u128),
                    Tabled(
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ),
                    ExternalTabled,
                    Started(
                        ::core::primitive::u32,
                        runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                    ),
                    Passed(::core::primitive::u32),
                    NotPassed(::core::primitive::u32),
                    Cancelled(::core::primitive::u32),
                    Executed(
                        ::core::primitive::u32,
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    Delegated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Undelegated(::subxt::sp_core::crypto::AccountId32),
                    Vetoed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    ),
                    PreimageNoted(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    PreimageUsed(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    PreimageInvalid(::subxt::sp_core::H256, ::core::primitive::u32),
                    PreimageMissing(::subxt::sp_core::H256, ::core::primitive::u32),
                    PreimageReaped(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Blacklisted(::subxt::sp_core::H256),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Delegations<_0> {
                    pub votes: _0,
                    pub capital: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ReferendumInfo<_0, _1, _2> {
                    Ongoing(
                        runtime_types::pallet_democracy::types::ReferendumStatus<
                            _0,
                            _1,
                            _2,
                        >,
                    ),
                    Finished {
                        approved: ::core::primitive::bool,
                        end: _0,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ReferendumStatus<_0, _1, _2> {
                    pub end: _0,
                    pub proposal_hash: _1,
                    pub threshold:
                        runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                    pub delay: _0,
                    pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Tally<_0> {
                    pub ayes: _0,
                    pub nays: _0,
                    pub turnout: _0,
                }
            }
            pub mod vote {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum AccountVote<_0> {
                    Standard {
                        vote: runtime_types::pallet_democracy::vote::Vote,
                        balance: _0,
                    },
                    Split {
                        aye: _0,
                        nay: _0,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct PriorLock<_0, _1>(pub _0, pub _1);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Vote(::core::primitive::u8);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Voting<_0, _1, _2> {
                    Direct {
                        votes: ::std::vec::Vec<(
                            _2,
                            runtime_types::pallet_democracy::vote::AccountVote<_0>,
                        )>,
                        delegations:
                            runtime_types::pallet_democracy::types::Delegations<_0>,
                        prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
                    },
                    Delegating {
                        balance: _0,
                        target: _1,
                        conviction:
                            runtime_types::pallet_democracy::conviction::Conviction,
                        delegations:
                            runtime_types::pallet_democracy::types::Delegations<_0>,
                        prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
                    },
                }
            }
            pub mod vote_threshold {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum VoteThreshold {
                    SuperMajorityApprove,
                    SuperMajorityAgainst,
                    SimpleMajority,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum PreimageStatus<_0, _1, _2> {
                Missing(_2),
                Available {
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    provider: _0,
                    deposit: _1,
                    since: _2,
                    expiry: ::core::option::Option<_2>,
                },
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                V1,
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: node_runtime :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < [:: core :: primitive :: u128 ; 3usize] > , } , set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: sp_core :: crypto :: AccountId32 > ,) > , } , submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: node_runtime :: NposSolution16 > > , num_signed_submissions : :: core :: primitive :: u32 , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    PreDispatchEarlySubmission,
                    PreDispatchWrongWinnerCount,
                    PreDispatchWeakSubmission,
                    SignedQueueFull,
                    SignedCannotPayDeposit,
                    SignedInvalidWitness,
                    SignedTooMuchWeight,
                    OcwCallWrongEra,
                    MissingSnapshotMetadata,
                    InvalidSubmissionIndex,
                    CallNotAllowed,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    SolutionStored (runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , :: core :: primitive :: bool ,) , ElectionFinalized (:: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute > ,) , Rewarded (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , Slashed (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 ,) , SignedPhaseStarted (:: core :: primitive :: u32 ,) , UnsignedPhaseStarted (:: core :: primitive :: u32 ,) , }
            }
            pub mod signed {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct SignedSubmission<_0, _1, _2> {
                    pub who: _0,
                    pub deposit: _1,
                    pub raw_solution:
                        runtime_types::pallet_election_provider_multi_phase::RawSolution<
                            _2,
                        >,
                    pub reward: _1,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum ElectionCompute {
                OnChain,
                Signed,
                Unsigned,
                Fallback,
                Emergency,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Phase<_0> {
                Off,
                Signed,
                Unsigned((::core::primitive::bool, _0)),
                Emergency,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: [::core::primitive::u128; 3usize],
                pub round: ::core::primitive::u32,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReadySolution<_0> {
                pub supports:
                    ::std::vec::Vec<(_0, runtime_types::sp_npos_elections::Support<_0>)>,
                pub score: [::core::primitive::u128; 3usize],
                pub compute:
                    runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RoundSnapshot<_0> {
                pub voters:
                    ::std::vec::Vec<(_0, ::core::primitive::u64, ::std::vec::Vec<_0>)>,
                pub targets: ::std::vec::Vec<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SolutionOrSnapshotSize {
                #[codec(compact)]
                pub voters: ::core::primitive::u32,
                #[codec(compact)]
                pub targets: ::core::primitive::u32,
            }
        }
        pub mod pallet_elections_phragmen {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    vote {
                        votes: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    remove_voter,
                    submit_candidacy {
                        #[codec(compact)]
                        candidate_count: ::core::primitive::u32,
                    },
                    renounce_candidacy {
                        renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
                    },
                    remove_member {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        has_replacement: ::core::primitive::bool,
                    },
                    clean_defunct_voters {
                        num_voters: ::core::primitive::u32,
                        num_defunct: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    UnableToVote,
                    NoVotes,
                    TooManyVotes,
                    MaximumVotesExceeded,
                    LowBalance,
                    UnableToPayBond,
                    MustBeVoter,
                    ReportSelf,
                    DuplicatedCandidate,
                    MemberSubmit,
                    RunnerUpSubmit,
                    InsufficientCandidateFunds,
                    NotMember,
                    InvalidWitnessData,
                    InvalidVoteCount,
                    InvalidRenouncing,
                    InvalidReplacement,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    NewTerm(
                        ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    ),
                    EmptyTerm,
                    ElectionError,
                    MemberKicked(::subxt::sp_core::crypto::AccountId32),
                    Renounced(::subxt::sp_core::crypto::AccountId32),
                    CandidateSlashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    SeatHolderSlashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Renouncing {
                Member,
                RunnerUp,
                Candidate(::core::primitive::u32),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SeatHolder<_0, _1> {
                pub who: _0,
                pub stake: _1,
                pub deposit: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Voter<_0, _1> {
                pub votes: ::std::vec::Vec<_0>,
                pub stake: _1,
                pub deposit: _1,
            }
        }
        pub mod pallet_gilt {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ActiveGilt<_0, _1, _2> {
                    pub proportion: ::subxt::sp_arithmetic::per_things::Perquintill,
                    pub amount: _0,
                    pub who: _1,
                    pub expiry: _2,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ActiveGiltsTotal<_0> {
                    pub frozen: _0,
                    pub proportion: ::subxt::sp_arithmetic::per_things::Perquintill,
                    pub index: ::core::primitive::u32,
                    pub target: ::subxt::sp_arithmetic::per_things::Perquintill,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    place_bid {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        duration: ::core::primitive::u32,
                    },
                    retract_bid {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        duration: ::core::primitive::u32,
                    },
                    set_target {
                        #[codec(compact)]
                        target: ::subxt::sp_arithmetic::per_things::Perquintill,
                    },
                    thaw {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    DurationTooSmall,
                    DurationTooBig,
                    AmountTooSmall,
                    BidTooLow,
                    Unknown,
                    NotOwner,
                    NotExpired,
                    NotFound,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    BidPlaced(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    ),
                    BidRetracted(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    ),
                    GiltIssued(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    GiltThawed(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct GiltBid<_0, _1> {
                    pub amount: _0,
                    pub who: _1,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    PauseFailed,
                    ResumeFailed,
                    ChangePending,
                    TooSoon,
                    InvalidKeyOwnershipProof,
                    InvalidEquivocationProof,
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    NewAuthorities(
                        ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    ),
                    Paused,
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StoredPendingChange < _0 > { pub scheduled_at : _0 , pub delay : _0 , pub next_authorities : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_finality_grandpa :: app :: Public , :: core :: primitive :: u64 ,) > , pub forced : :: core :: option :: Option < _0 > , }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum StoredState<_0> {
                Live,
                PendingPause { scheduled_at: _0, delay: _0 },
                Paused,
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_identity {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    add_registrar {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    set_identity {
                        info: ::std::boxed::Box<
                            runtime_types::pallet_identity::types::IdentityInfo,
                        >,
                    },
                    set_subs {
                        subs: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    clear_identity,
                    request_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        #[codec(compact)]
                        max_fee: ::core::primitive::u128,
                    },
                    cancel_request {
                        reg_index: ::core::primitive::u32,
                    },
                    set_fee {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    set_account_id {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        new: ::subxt::sp_core::crypto::AccountId32,
                    },
                    set_fields {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        fields: runtime_types::pallet_identity::types::BitFlags<
                            runtime_types::pallet_identity::types::IdentityField,
                        >,
                    },
                    provide_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        judgement: runtime_types::pallet_identity::types::Judgement<
                            ::core::primitive::u128,
                        >,
                    },
                    kill_identity {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    add_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    rename_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    remove_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    quit_sub,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    TooManySubAccounts,
                    NotFound,
                    NotNamed,
                    EmptyIndex,
                    FeeChanged,
                    NoIdentity,
                    StickyJudgement,
                    JudgementGiven,
                    InvalidJudgement,
                    InvalidIndex,
                    InvalidTarget,
                    TooManyFields,
                    TooManyRegistrars,
                    AlreadyClaimed,
                    NotSub,
                    NotOwned,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    IdentitySet(::subxt::sp_core::crypto::AccountId32),
                    IdentityCleared(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    IdentityKilled(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    JudgementRequested(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    JudgementUnrequested(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    JudgementGiven(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    RegistrarAdded(::core::primitive::u32),
                    SubIdentityAdded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    SubIdentityRemoved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    SubIdentityRevoked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct BitFlags<_0>(
                    pub ::core::primitive::u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Data {
                    None,
                    Raw0([::core::primitive::u8; 0usize]),
                    Raw1([::core::primitive::u8; 1usize]),
                    Raw2([::core::primitive::u8; 2usize]),
                    Raw3([::core::primitive::u8; 3usize]),
                    Raw4([::core::primitive::u8; 4usize]),
                    Raw5([::core::primitive::u8; 5usize]),
                    Raw6([::core::primitive::u8; 6usize]),
                    Raw7([::core::primitive::u8; 7usize]),
                    Raw8([::core::primitive::u8; 8usize]),
                    Raw9([::core::primitive::u8; 9usize]),
                    Raw10([::core::primitive::u8; 10usize]),
                    Raw11([::core::primitive::u8; 11usize]),
                    Raw12([::core::primitive::u8; 12usize]),
                    Raw13([::core::primitive::u8; 13usize]),
                    Raw14([::core::primitive::u8; 14usize]),
                    Raw15([::core::primitive::u8; 15usize]),
                    Raw16([::core::primitive::u8; 16usize]),
                    Raw17([::core::primitive::u8; 17usize]),
                    Raw18([::core::primitive::u8; 18usize]),
                    Raw19([::core::primitive::u8; 19usize]),
                    Raw20([::core::primitive::u8; 20usize]),
                    Raw21([::core::primitive::u8; 21usize]),
                    Raw22([::core::primitive::u8; 22usize]),
                    Raw23([::core::primitive::u8; 23usize]),
                    Raw24([::core::primitive::u8; 24usize]),
                    Raw25([::core::primitive::u8; 25usize]),
                    Raw26([::core::primitive::u8; 26usize]),
                    Raw27([::core::primitive::u8; 27usize]),
                    Raw28([::core::primitive::u8; 28usize]),
                    Raw29([::core::primitive::u8; 29usize]),
                    Raw30([::core::primitive::u8; 30usize]),
                    Raw31([::core::primitive::u8; 31usize]),
                    Raw32([::core::primitive::u8; 32usize]),
                    BlakeTwo256([::core::primitive::u8; 32usize]),
                    Sha256([::core::primitive::u8; 32usize]),
                    Keccak256([::core::primitive::u8; 32usize]),
                    ShaThree256([::core::primitive::u8; 32usize]),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum IdentityField {
                    Display,
                    Legal,
                    Web,
                    Riot,
                    Email,
                    PgpFingerprint,
                    Image,
                    Twitter,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct IdentityInfo {
                    pub additional:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            runtime_types::pallet_identity::types::Data,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    pub display: runtime_types::pallet_identity::types::Data,
                    pub legal: runtime_types::pallet_identity::types::Data,
                    pub web: runtime_types::pallet_identity::types::Data,
                    pub riot: runtime_types::pallet_identity::types::Data,
                    pub email: runtime_types::pallet_identity::types::Data,
                    pub pgp_fingerprint:
                        ::core::option::Option<[::core::primitive::u8; 20usize]>,
                    pub image: runtime_types::pallet_identity::types::Data,
                    pub twitter: runtime_types::pallet_identity::types::Data,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Judgement<_0> {
                    Unknown,
                    FeePaid(_0),
                    Reasonable,
                    KnownGood,
                    OutOfDate,
                    LowQuality,
                    Erroneous,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RegistrarInfo<_0, _1> {
                    pub account: _1,
                    pub fee: _0,
                    pub fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Registration<_0> {
                    pub judgements:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            ::core::primitive::u32,
                            runtime_types::pallet_identity::types::Judgement<_0>,
                        )>,
                    pub deposit: _0,
                    pub info: runtime_types::pallet_identity::types::IdentityInfo,
                }
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    heartbeat { heartbeat : runtime_types :: pallet_im_online :: Heartbeat < :: core :: primitive :: u32 > , signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InvalidKey,
                    DuplicatedHeartbeat,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    HeartbeatReceived(
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    ),
                    AllGood,
                    SomeOffline(
                        ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::pallet_staking::Exposure<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::core::primitive::u128,
                            >,
                        )>,
                    ),
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BoundedOpaqueNetworkState { pub peer_id : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < :: core :: primitive :: u8 > , pub external_addresses : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < :: core :: primitive :: u8 > > , }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_indices {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    claim {
                        index: ::core::primitive::u32,
                    },
                    transfer {
                        new: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                    },
                    free {
                        index: ::core::primitive::u32,
                    },
                    force_transfer {
                        new: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                        freeze: ::core::primitive::bool,
                    },
                    freeze {
                        index: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    NotAssigned,
                    NotOwner,
                    InUse,
                    NotTransfer,
                    Permanent,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    IndexAssigned(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    IndexFreed(::core::primitive::u32),
                    IndexFrozen(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                }
            }
        }
        pub mod pallet_lottery {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    buy_ticket {
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    set_calls {
                        calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
                    },
                    start_lottery {
                        price: ::core::primitive::u128,
                        length: ::core::primitive::u32,
                        delay: ::core::primitive::u32,
                        repeat: ::core::primitive::bool,
                    },
                    stop_repeat,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    NotConfigured,
                    InProgress,
                    AlreadyEnded,
                    InvalidCall,
                    AlreadyParticipating,
                    TooManyCalls,
                    EncodingFailed,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    LotteryStarted,
                    CallsUpdated,
                    Winner(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    TicketBought(
                        ::subxt::sp_core::crypto::AccountId32,
                        (::core::primitive::u8, ::core::primitive::u8),
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LotteryConfig<_0, _1> {
                pub price: _1,
                pub start: _0,
                pub length: _0,
                pub delay: _0,
                pub repeat: ::core::primitive::bool,
            }
        }
        pub mod pallet_membership {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    add_member {
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    remove_member {
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    swap_member {
                        remove: ::subxt::sp_core::crypto::AccountId32,
                        add: ::subxt::sp_core::crypto::AccountId32,
                    },
                    reset_members {
                        members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    },
                    change_key {
                        new: ::subxt::sp_core::crypto::AccountId32,
                    },
                    set_prime {
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    clear_prime,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    AlreadyMember,
                    NotMember,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    MemberAdded,
                    MemberRemoved,
                    MembersSwapped,
                    MembersReset,
                    KeyChanged,
                    Dummy,
                }
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    as_multi_threshold_1 {
                        other_signatories:
                            ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories:
                            ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<
                                ::core::primitive::u32,
                            >,
                        >,
                        call:
                            ::subxt::WrapperKeepOpaque<runtime_types::node_runtime::Call>,
                        store_call: ::core::primitive::bool,
                        max_weight: ::core::primitive::u64,
                    },
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories:
                            ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<
                                ::core::primitive::u32,
                            >,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: ::core::primitive::u64,
                    },
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories:
                            ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        timepoint: runtime_types::pallet_multisig::Timepoint<
                            ::core::primitive::u32,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    MinimumThreshold,
                    AlreadyApproved,
                    NoApprovalsNeeded,
                    TooFewSignatories,
                    TooManySignatories,
                    SignatoriesOutOfOrder,
                    SenderInSignatories,
                    NotFound,
                    NotOwner,
                    NoTimepoint,
                    WrongTimepoint,
                    UnexpectedTimepoint,
                    MaxWeightTooLow,
                    AlreadyStored,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    NewMultisig(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    MultisigApproval(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    MultisigExecuted(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    MultisigCancelled(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: ::std::vec::Vec<_2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_offences {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Offence(
                        [::core::primitive::u8; 16usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                }
            }
        }
        pub mod pallet_proxy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    proxy {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type: ::core::option::Option<
                            runtime_types::node_runtime::ProxyType,
                        >,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    add_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::node_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    remove_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::node_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    remove_proxies,
                    anonymous {
                        proxy_type: runtime_types::node_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                        index: ::core::primitive::u16,
                    },
                    kill_anonymous {
                        spawner: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::node_runtime::ProxyType,
                        index: ::core::primitive::u16,
                        #[codec(compact)]
                        height: ::core::primitive::u32,
                        #[codec(compact)]
                        ext_index: ::core::primitive::u32,
                    },
                    announce {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    remove_announcement {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    reject_announcement {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    proxy_announced {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type: ::core::option::Option<
                            runtime_types::node_runtime::ProxyType,
                        >,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    TooMany,
                    NotFound,
                    NotProxy,
                    Unproxyable,
                    Duplicate,
                    NoPermission,
                    Unannounced,
                    NoSelfProxy,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    ProxyExecuted(
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    AnonymousCreated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::node_runtime::ProxyType,
                        ::core::primitive::u16,
                    ),
                    Announced(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                    ProxyAdded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::node_runtime::ProxyType,
                        ::core::primitive::u32,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Announcement<_0, _1, _2> {
                pub real: _0,
                pub call_hash: _1,
                pub height: _2,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProxyDefinition<_0, _1, _2> {
                pub delegate: _0,
                pub proxy_type: _1,
                pub delay: _2,
            }
        }
        pub mod pallet_recovery {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    as_recovered {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    set_recovered {
                        lost: ::subxt::sp_core::crypto::AccountId32,
                        rescuer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    create_recovery {
                        friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        threshold: ::core::primitive::u16,
                        delay_period: ::core::primitive::u32,
                    },
                    initiate_recovery {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    vouch_recovery {
                        lost: ::subxt::sp_core::crypto::AccountId32,
                        rescuer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    claim_recovery {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    close_recovery {
                        rescuer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    remove_recovery,
                    cancel_recovered {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    NotAllowed,
                    ZeroThreshold,
                    NotEnoughFriends,
                    MaxFriends,
                    NotSorted,
                    NotRecoverable,
                    AlreadyRecoverable,
                    AlreadyStarted,
                    NotStarted,
                    NotFriend,
                    DelayPeriod,
                    AlreadyVouched,
                    Threshold,
                    StillActive,
                    AlreadyProxy,
                    BadState,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    RecoveryCreated(::subxt::sp_core::crypto::AccountId32),
                    RecoveryInitiated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    RecoveryVouched(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    RecoveryClosed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    AccountRecovered(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    RecoveryRemoved(::subxt::sp_core::crypto::AccountId32),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ActiveRecovery<_0, _1, _2> {
                pub created: _0,
                pub deposit: _1,
                pub friends: ::std::vec::Vec<_2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryConfig<_0, _1, _2> {
                pub delay_period: _0,
                pub deposit: _1,
                pub friends: ::std::vec::Vec<_2>,
                pub threshold: ::core::primitive::u16,
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    schedule_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    cancel_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    schedule_named_after {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    FailedToSchedule,
                    NotFound,
                    TargetBlockNumberInPast,
                    RescheduleNoChange,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Scheduled(::core::primitive::u32, ::core::primitive::u32),
                    Canceled(::core::primitive::u32, ::core::primitive::u32),
                    Dispatched(
                        (::core::primitive::u32, ::core::primitive::u32),
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                V1,
                V2,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduledV2<_0, _1, _2, _3> {
                pub maybe_id:
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub priority: ::core::primitive::u8,
                pub call: _0,
                pub maybe_periodic: ::core::option::Option<(_1, _1)>,
                pub origin: _2,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    set_keys {
                        keys: runtime_types::node_runtime::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    purge_keys,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InvalidProof,
                    NoAssociatedValidatorId,
                    DuplicatedKey,
                    NoKeys,
                    NoAccount,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    NewSession(::core::primitive::u32),
                }
            }
        }
        pub mod pallet_society {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    bid {
                        value: ::core::primitive::u128,
                    },
                    unbid {
                        pos: ::core::primitive::u32,
                    },
                    vouch {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        value: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                    unvouch {
                        pos: ::core::primitive::u32,
                    },
                    vote {
                        candidate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        approve: ::core::primitive::bool,
                    },
                    defender_vote {
                        approve: ::core::primitive::bool,
                    },
                    payout,
                    found {
                        founder: ::subxt::sp_core::crypto::AccountId32,
                        max_members: ::core::primitive::u32,
                        rules: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    unfound,
                    judge_suspended_member {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        forgive: ::core::primitive::bool,
                    },
                    judge_suspended_candidate {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        judgement: runtime_types::pallet_society::Judgement,
                    },
                    set_max_members {
                        max: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    BadPosition,
                    NotMember,
                    AlreadyMember,
                    Suspended,
                    NotSuspended,
                    NoPayout,
                    AlreadyFounded,
                    InsufficientPot,
                    AlreadyVouching,
                    NotVouching,
                    Head,
                    Founder,
                    AlreadyBid,
                    AlreadyCandidate,
                    NotCandidate,
                    MaxMembers,
                    NotFounder,
                    NotHead,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Founded(::subxt::sp_core::crypto::AccountId32),
                    Bid(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    Vouch(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    AutoUnbid(::subxt::sp_core::crypto::AccountId32),
                    Unbid(::subxt::sp_core::crypto::AccountId32),
                    Unvouch(::subxt::sp_core::crypto::AccountId32),
                    Inducted(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ),
                    SuspendedMemberJudgement(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::bool,
                    ),
                    CandidateSuspended(::subxt::sp_core::crypto::AccountId32),
                    MemberSuspended(::subxt::sp_core::crypto::AccountId32),
                    Challenged(::subxt::sp_core::crypto::AccountId32),
                    Vote(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::bool,
                    ),
                    DefenderVote(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::bool,
                    ),
                    NewMaxMembers(::core::primitive::u32),
                    Unfounded(::subxt::sp_core::crypto::AccountId32),
                    Deposit(::core::primitive::u128),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bid<_0, _1> {
                pub who: _0,
                pub kind: runtime_types::pallet_society::BidKind<_0, _1>,
                pub value: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum BidKind<_0, _1> {
                Deposit(_1),
                Vouch(_0, _1),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Judgement {
                Rebid,
                Reject,
                Approve,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Vote {
                Skeptic,
                Reject,
                Approve,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum VouchingStatus {
                Vouching,
                Banned,
            }
        }
        pub mod pallet_staking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum Call {
                        bond {
                            controller: ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::core::primitive::u32,
                            >,
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        },
                        bond_extra {
                            #[codec(compact)]
                            max_additional: ::core::primitive::u128,
                        },
                        unbond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        withdraw_unbonded {
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        validate {
                            prefs: runtime_types::pallet_staking::ValidatorPrefs,
                        },
                        nominate {
                            targets: ::std::vec::Vec<
                                ::subxt::sp_runtime::MultiAddress<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    ::core::primitive::u32,
                                >,
                            >,
                        },
                        chill,
                        set_payee {
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        },
                        set_controller {
                            controller: ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::core::primitive::u32,
                            >,
                        },
                        set_validator_count {
                            #[codec(compact)]
                            new: ::core::primitive::u32,
                        },
                        increase_validator_count {
                            #[codec(compact)]
                            additional: ::core::primitive::u32,
                        },
                        scale_validator_count {
                            factor: runtime_types::sp_arithmetic::per_things::Percent,
                        },
                        force_no_eras,
                        force_new_era,
                        set_invulnerables {
                            invulnerables:
                                ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        },
                        force_unstake {
                            stash: ::subxt::sp_core::crypto::AccountId32,
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        force_new_era_always,
                        cancel_deferred_slash {
                            era: ::core::primitive::u32,
                            slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                        },
                        payout_stakers {
                            validator_stash: ::subxt::sp_core::crypto::AccountId32,
                            era: ::core::primitive::u32,
                        },
                        rebond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        set_history_depth {
                            #[codec(compact)]
                            new_history_depth: ::core::primitive::u32,
                            #[codec(compact)]
                            era_items_deleted: ::core::primitive::u32,
                        },
                        reap_stash {
                            stash: ::subxt::sp_core::crypto::AccountId32,
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        kick {
                            who: ::std::vec::Vec<
                                ::subxt::sp_runtime::MultiAddress<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    ::core::primitive::u32,
                                >,
                            >,
                        },
                        set_staking_limits {
                            min_nominator_bond: ::core::primitive::u128,
                            min_validator_bond: ::core::primitive::u128,
                            max_nominator_count:
                                ::core::option::Option<::core::primitive::u32>,
                            max_validator_count:
                                ::core::option::Option<::core::primitive::u32>,
                            threshold: ::core::option::Option<
                                runtime_types::sp_arithmetic::per_things::Percent,
                            >,
                        },
                        chill_other {
                            controller: ::subxt::sp_core::crypto::AccountId32,
                        },
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum Error {
                        NotController,
                        NotStash,
                        AlreadyBonded,
                        AlreadyPaired,
                        EmptyTargets,
                        DuplicateIndex,
                        InvalidSlashIndex,
                        InsufficientBond,
                        NoMoreChunks,
                        NoUnlockChunk,
                        FundedTarget,
                        InvalidEraToReward,
                        InvalidNumberOfNominations,
                        NotSortedAndUnique,
                        AlreadyClaimed,
                        IncorrectHistoryDepth,
                        IncorrectSlashingSpans,
                        BadState,
                        TooManyTargets,
                        BadTarget,
                        CannotChillOther,
                        TooManyNominators,
                        TooManyValidators,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum Event {
                        EraPaid(
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        ),
                        Rewarded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        Slashed(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        OldSlashingReportDiscarded(::core::primitive::u32),
                        StakersElected,
                        Bonded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        Unbonded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        Withdrawn(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        Kicked(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                        StakingElectionFailed,
                        Chilled(::subxt::sp_core::crypto::AccountId32),
                        PayoutStarted(
                            ::core::primitive::u32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                    }
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct SlashingSpans {
                    pub span_index: ::core::primitive::u32,
                    pub last_start: ::core::primitive::u32,
                    pub last_nonzero_slash: ::core::primitive::u32,
                    pub prior: ::std::vec::Vec<::core::primitive::u32>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct SpanRecord<_0> {
                    pub slashed: _0,
                    pub paid_out: _0,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual: ::std::collections::BTreeMap<_0, ::core::primitive::u32>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Exposure<_0, _1> {
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub own: _1,
                pub others: ::std::vec::Vec<
                    runtime_types::pallet_staking::IndividualExposure<_0, _1>,
                >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Forcing {
                NotForcing,
                ForceNew,
                ForceNone,
                ForceAlways,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                #[codec(compact)]
                pub value: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Nominations<_0> {
                pub targets: ::std::vec::Vec<_0>,
                pub submitted_in: ::core::primitive::u32,
                pub suppressed: ::core::primitive::bool,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                V1_0_0Ancient,
                V2_0_0,
                V3_0_0,
                V4_0_0,
                V5_0_0,
                V6_0_0,
                V7_0_0,
                V8_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum RewardDestination<_0> {
                Staked,
                Stash,
                Controller,
                Account(_0),
                None,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakingLedger<_0, _1> {
                pub stash: _0,
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub active: _1,
                pub unlocking:
                    ::std::vec::Vec<runtime_types::pallet_staking::UnlockChunk<_1>>,
                pub claimed_rewards: ::std::vec::Vec<::core::primitive::u32>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnappliedSlash<_0, _1> {
                pub validator: _0,
                pub own: _1,
                pub others: ::std::vec::Vec<(_0, _1)>,
                pub reporters: ::std::vec::Vec<_0>,
                pub payout: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnlockChunk<_0> {
                #[codec(compact)]
                pub value: _0,
                #[codec(compact)]
                pub era: ::core::primitive::u32,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ValidatorPrefs {
                #[codec(compact)]
                pub commission: ::subxt::sp_arithmetic::per_things::Perbill,
                pub blocked: ::core::primitive::bool,
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    sudo {
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Sudid(
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                    KeyChanged(::subxt::sp_core::crypto::AccountId32),
                    SudoAsDone(
                        ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_tips {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    report_awesome {
                        reason: ::std::vec::Vec<::core::primitive::u8>,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    retract_tip {
                        hash: ::subxt::sp_core::H256,
                    },
                    tip_new {
                        reason: ::std::vec::Vec<::core::primitive::u8>,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        #[codec(compact)]
                        tip_value: ::core::primitive::u128,
                    },
                    tip {
                        hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        tip_value: ::core::primitive::u128,
                    },
                    close_tip {
                        hash: ::subxt::sp_core::H256,
                    },
                    slash_tip {
                        hash: ::subxt::sp_core::H256,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    ReasonTooBig,
                    AlreadyKnown,
                    UnknownTip,
                    NotFinder,
                    StillOpen,
                    Premature,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    NewTip(::subxt::sp_core::H256),
                    TipClosing(::subxt::sp_core::H256),
                    TipClosed(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    TipRetracted(::subxt::sp_core::H256),
                    TipSlashed(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenTip<_0, _1, _2, _3> {
                pub reason: _3,
                pub who: _0,
                pub finder: _0,
                pub deposit: _1,
                pub closes: ::core::option::Option<_2>,
                pub tips: ::std::vec::Vec<(_0, _1)>,
                pub finders_fee: ::core::primitive::bool,
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChargeTransactionPayment(pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                V1Ancient,
                V2,
            }
        }
        pub mod pallet_transaction_storage {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    store { data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , renew { block : :: core :: primitive :: u32 , index : :: core :: primitive :: u32 , } , check_proof { proof : runtime_types :: sp_transaction_storage_proof :: TransactionStorageProof , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InsufficientFunds,
                    NotConfigured,
                    RenewedNotFound,
                    EmptyTransaction,
                    UnexpectedProof,
                    InvalidProof,
                    MissingProof,
                    MissingStateData,
                    DoubleCheck,
                    ProofNotChecked,
                    TransactionTooLarge,
                    TooManyTransactions,
                    BadContext,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Stored(::core::primitive::u32),
                    Renewed(::core::primitive::u32),
                    ProofChecked,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransactionInfo {
                pub chunk_root: ::subxt::sp_core::H256,
                pub content_hash: ::subxt::sp_core::H256,
                pub size: ::core::primitive::u32,
                pub block_chunks: ::core::primitive::u32,
            }
        }
        pub mod pallet_treasury {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    propose_spend {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    reject_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                    approve_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    InsufficientProposersBalance,
                    InvalidIndex,
                    TooManyApprovals,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Proposed(::core::primitive::u32),
                    Spending(::core::primitive::u128),
                    Awarded(
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Rejected(::core::primitive::u32, ::core::primitive::u128),
                    Burnt(::core::primitive::u128),
                    Rollover(::core::primitive::u128),
                    Deposit(::core::primitive::u128),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Proposal<_0, _1> {
                pub proposer: _0,
                pub value: _1,
                pub beneficiary: _0,
                pub bond: _1,
            }
        }
        pub mod pallet_uniques {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    create { # [codec (compact)] class : :: core :: primitive :: u32 , admin : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , force_create { # [codec (compact)] class : :: core :: primitive :: u32 , owner : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , free_holding : :: core :: primitive :: bool , } , destroy { # [codec (compact)] class : :: core :: primitive :: u32 , witness : runtime_types :: pallet_uniques :: types :: DestroyWitness , } , mint { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , owner : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , burn { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , check_owner : :: core :: option :: Option < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , transfer { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , dest : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , redeposit { # [codec (compact)] class : :: core :: primitive :: u32 , instances : :: std :: vec :: Vec < :: core :: primitive :: u32 > , } , freeze { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , } , thaw { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , } , freeze_class { # [codec (compact)] class : :: core :: primitive :: u32 , } , thaw_class { # [codec (compact)] class : :: core :: primitive :: u32 , } , transfer_ownership { # [codec (compact)] class : :: core :: primitive :: u32 , owner : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , set_team { # [codec (compact)] class : :: core :: primitive :: u32 , issuer : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , admin : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , freezer : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , approve_transfer { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , delegate : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , cancel_approval { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , maybe_check_delegate : :: core :: option :: Option < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , force_asset_status { # [codec (compact)] class : :: core :: primitive :: u32 , owner : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , issuer : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , admin : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , freezer : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , free_holding : :: core :: primitive :: bool , is_frozen : :: core :: primitive :: bool , } , set_attribute { # [codec (compact)] class : :: core :: primitive :: u32 , maybe_instance : :: core :: option :: Option < :: core :: primitive :: u32 > , key : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , value : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , } , clear_attribute { # [codec (compact)] class : :: core :: primitive :: u32 , maybe_instance : :: core :: option :: Option < :: core :: primitive :: u32 > , key : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , } , set_metadata { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , data : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , is_frozen : :: core :: primitive :: bool , } , clear_metadata { # [codec (compact)] class : :: core :: primitive :: u32 , # [codec (compact)] instance : :: core :: primitive :: u32 , } , set_class_metadata { # [codec (compact)] class : :: core :: primitive :: u32 , data : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , is_frozen : :: core :: primitive :: bool , } , clear_class_metadata { # [codec (compact)] class : :: core :: primitive :: u32 , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    NoPermission,
                    Unknown,
                    AlreadyExists,
                    WrongOwner,
                    BadWitness,
                    InUse,
                    Frozen,
                    WrongDelegate,
                    NoDelegate,
                    Unapproved,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    Created(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    ForceCreated(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Destroyed(::core::primitive::u32),
                    Issued(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Transferred(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Burned(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Frozen(::core::primitive::u32, ::core::primitive::u32),
                    Thawed(::core::primitive::u32, ::core::primitive::u32),
                    ClassFrozen(::core::primitive::u32),
                    ClassThawed(::core::primitive::u32),
                    OwnerChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    TeamChanged(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    ApprovedTransfer(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    ApprovalCancelled(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    AssetStatusChanged(::core::primitive::u32),
                    ClassMetadataSet(
                        ::core::primitive::u32,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        ::core::primitive::bool,
                    ),
                    ClassMetadataCleared(::core::primitive::u32),
                    MetadataSet(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        ::core::primitive::bool,
                    ),
                    MetadataCleared(::core::primitive::u32, ::core::primitive::u32),
                    Redeposited(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u32>,
                    ),
                    AttributeSet(
                        ::core::primitive::u32,
                        ::core::option::Option<::core::primitive::u32>,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    AttributeCleared(
                        ::core::primitive::u32,
                        ::core::option::Option<::core::primitive::u32>,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ClassDetails<_0, _1> {
                    pub owner: _0,
                    pub issuer: _0,
                    pub admin: _0,
                    pub freezer: _0,
                    pub total_deposit: _1,
                    pub free_holding: ::core::primitive::bool,
                    pub instances: ::core::primitive::u32,
                    pub instance_metadatas: ::core::primitive::u32,
                    pub attributes: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ClassMetadata<_0> {
                    pub deposit: _0,
                    pub data:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub instances: ::core::primitive::u32,
                    #[codec(compact)]
                    pub instance_metadatas: ::core::primitive::u32,
                    #[codec(compact)]
                    pub attributes: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct InstanceDetails<_0, _1> {
                    pub owner: _0,
                    pub approved: ::core::option::Option<_0>,
                    pub is_frozen: ::core::primitive::bool,
                    pub deposit: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct InstanceMetadata<_0> {
                    pub deposit: _0,
                    pub data:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    pub is_frozen: ::core::primitive::bool,
                }
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    batch {
                        calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
                    },
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::node_runtime::Call>,
                    },
                    batch_all {
                        calls: ::std::vec::Vec<runtime_types::node_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    TooManyCalls,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    BatchInterrupted(
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::DispatchError,
                    ),
                    BatchCompleted,
                    ItemCompleted,
                }
            }
        }
        pub mod pallet_vesting {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    vest,
                    vest_other {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    vested_transfer {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        schedule:
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            >,
                    },
                    force_vested_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        schedule:
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            >,
                    },
                    merge_schedules {
                        schedule1_index: ::core::primitive::u32,
                        schedule2_index: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    NotVesting,
                    AtMaxVestingSchedules,
                    AmountLow,
                    ScheduleIndexOutOfBounds,
                    InvalidScheduleParams,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    VestingUpdated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    VestingCompleted(::subxt::sp_core::crypto::AccountId32),
                }
            }
            pub mod vesting_info {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct VestingInfo<_0, _1> {
                    pub locked: _0,
                    pub per_block: _0,
                    pub starting_block: _1,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                V0,
                V1,
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct PerU16(pub ::core::primitive::u16);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Percent(pub ::core::primitive::u8);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Permill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Perquintill(pub ::core::primitive::u64);
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum NextConfigDescriptor {
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum AllowedSlots {
                PrimarySlots,
                PrimaryAndSecondaryPlainSlots,
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod changes_trie {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: ::core::primitive::u32,
                    pub digest_levels: ::core::primitive::u32,
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses: ::std::vec::Vec<
                        runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    >,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Equivocation<_0, _1> {
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation:
                    runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_npos_elections {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Support<_0> {
                pub total: ::core::primitive::u128,
                pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum ChangesTrieSignal {
                        NewConfiguration (:: core :: option :: Option < runtime_types :: sp_core :: changes_trie :: ChangesTrieConfiguration > ,) , }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct Digest<_0> {
                        pub logs: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem<_0>,
                        >,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum DigestItem<_0> {
                        ChangesTrieRoot(_0),
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        ChangesTrieSignal(
                            runtime_types::sp_runtime::generic::digest::ChangesTrieSignal,
                        ),
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub enum Era {
                        Immortal,
                        Mortal1(::core::primitive::u8),
                        Mortal2(::core::primitive::u8),
                        Mortal3(::core::primitive::u8),
                        Mortal4(::core::primitive::u8),
                        Mortal5(::core::primitive::u8),
                        Mortal6(::core::primitive::u8),
                        Mortal7(::core::primitive::u8),
                        Mortal8(::core::primitive::u8),
                        Mortal9(::core::primitive::u8),
                        Mortal10(::core::primitive::u8),
                        Mortal11(::core::primitive::u8),
                        Mortal12(::core::primitive::u8),
                        Mortal13(::core::primitive::u8),
                        Mortal14(::core::primitive::u8),
                        Mortal15(::core::primitive::u8),
                        Mortal16(::core::primitive::u8),
                        Mortal17(::core::primitive::u8),
                        Mortal18(::core::primitive::u8),
                        Mortal19(::core::primitive::u8),
                        Mortal20(::core::primitive::u8),
                        Mortal21(::core::primitive::u8),
                        Mortal22(::core::primitive::u8),
                        Mortal23(::core::primitive::u8),
                        Mortal24(::core::primitive::u8),
                        Mortal25(::core::primitive::u8),
                        Mortal26(::core::primitive::u8),
                        Mortal27(::core::primitive::u8),
                        Mortal28(::core::primitive::u8),
                        Mortal29(::core::primitive::u8),
                        Mortal30(::core::primitive::u8),
                        Mortal31(::core::primitive::u8),
                        Mortal32(::core::primitive::u8),
                        Mortal33(::core::primitive::u8),
                        Mortal34(::core::primitive::u8),
                        Mortal35(::core::primitive::u8),
                        Mortal36(::core::primitive::u8),
                        Mortal37(::core::primitive::u8),
                        Mortal38(::core::primitive::u8),
                        Mortal39(::core::primitive::u8),
                        Mortal40(::core::primitive::u8),
                        Mortal41(::core::primitive::u8),
                        Mortal42(::core::primitive::u8),
                        Mortal43(::core::primitive::u8),
                        Mortal44(::core::primitive::u8),
                        Mortal45(::core::primitive::u8),
                        Mortal46(::core::primitive::u8),
                        Mortal47(::core::primitive::u8),
                        Mortal48(::core::primitive::u8),
                        Mortal49(::core::primitive::u8),
                        Mortal50(::core::primitive::u8),
                        Mortal51(::core::primitive::u8),
                        Mortal52(::core::primitive::u8),
                        Mortal53(::core::primitive::u8),
                        Mortal54(::core::primitive::u8),
                        Mortal55(::core::primitive::u8),
                        Mortal56(::core::primitive::u8),
                        Mortal57(::core::primitive::u8),
                        Mortal58(::core::primitive::u8),
                        Mortal59(::core::primitive::u8),
                        Mortal60(::core::primitive::u8),
                        Mortal61(::core::primitive::u8),
                        Mortal62(::core::primitive::u8),
                        Mortal63(::core::primitive::u8),
                        Mortal64(::core::primitive::u8),
                        Mortal65(::core::primitive::u8),
                        Mortal66(::core::primitive::u8),
                        Mortal67(::core::primitive::u8),
                        Mortal68(::core::primitive::u8),
                        Mortal69(::core::primitive::u8),
                        Mortal70(::core::primitive::u8),
                        Mortal71(::core::primitive::u8),
                        Mortal72(::core::primitive::u8),
                        Mortal73(::core::primitive::u8),
                        Mortal74(::core::primitive::u8),
                        Mortal75(::core::primitive::u8),
                        Mortal76(::core::primitive::u8),
                        Mortal77(::core::primitive::u8),
                        Mortal78(::core::primitive::u8),
                        Mortal79(::core::primitive::u8),
                        Mortal80(::core::primitive::u8),
                        Mortal81(::core::primitive::u8),
                        Mortal82(::core::primitive::u8),
                        Mortal83(::core::primitive::u8),
                        Mortal84(::core::primitive::u8),
                        Mortal85(::core::primitive::u8),
                        Mortal86(::core::primitive::u8),
                        Mortal87(::core::primitive::u8),
                        Mortal88(::core::primitive::u8),
                        Mortal89(::core::primitive::u8),
                        Mortal90(::core::primitive::u8),
                        Mortal91(::core::primitive::u8),
                        Mortal92(::core::primitive::u8),
                        Mortal93(::core::primitive::u8),
                        Mortal94(::core::primitive::u8),
                        Mortal95(::core::primitive::u8),
                        Mortal96(::core::primitive::u8),
                        Mortal97(::core::primitive::u8),
                        Mortal98(::core::primitive::u8),
                        Mortal99(::core::primitive::u8),
                        Mortal100(::core::primitive::u8),
                        Mortal101(::core::primitive::u8),
                        Mortal102(::core::primitive::u8),
                        Mortal103(::core::primitive::u8),
                        Mortal104(::core::primitive::u8),
                        Mortal105(::core::primitive::u8),
                        Mortal106(::core::primitive::u8),
                        Mortal107(::core::primitive::u8),
                        Mortal108(::core::primitive::u8),
                        Mortal109(::core::primitive::u8),
                        Mortal110(::core::primitive::u8),
                        Mortal111(::core::primitive::u8),
                        Mortal112(::core::primitive::u8),
                        Mortal113(::core::primitive::u8),
                        Mortal114(::core::primitive::u8),
                        Mortal115(::core::primitive::u8),
                        Mortal116(::core::primitive::u8),
                        Mortal117(::core::primitive::u8),
                        Mortal118(::core::primitive::u8),
                        Mortal119(::core::primitive::u8),
                        Mortal120(::core::primitive::u8),
                        Mortal121(::core::primitive::u8),
                        Mortal122(::core::primitive::u8),
                        Mortal123(::core::primitive::u8),
                        Mortal124(::core::primitive::u8),
                        Mortal125(::core::primitive::u8),
                        Mortal126(::core::primitive::u8),
                        Mortal127(::core::primitive::u8),
                        Mortal128(::core::primitive::u8),
                        Mortal129(::core::primitive::u8),
                        Mortal130(::core::primitive::u8),
                        Mortal131(::core::primitive::u8),
                        Mortal132(::core::primitive::u8),
                        Mortal133(::core::primitive::u8),
                        Mortal134(::core::primitive::u8),
                        Mortal135(::core::primitive::u8),
                        Mortal136(::core::primitive::u8),
                        Mortal137(::core::primitive::u8),
                        Mortal138(::core::primitive::u8),
                        Mortal139(::core::primitive::u8),
                        Mortal140(::core::primitive::u8),
                        Mortal141(::core::primitive::u8),
                        Mortal142(::core::primitive::u8),
                        Mortal143(::core::primitive::u8),
                        Mortal144(::core::primitive::u8),
                        Mortal145(::core::primitive::u8),
                        Mortal146(::core::primitive::u8),
                        Mortal147(::core::primitive::u8),
                        Mortal148(::core::primitive::u8),
                        Mortal149(::core::primitive::u8),
                        Mortal150(::core::primitive::u8),
                        Mortal151(::core::primitive::u8),
                        Mortal152(::core::primitive::u8),
                        Mortal153(::core::primitive::u8),
                        Mortal154(::core::primitive::u8),
                        Mortal155(::core::primitive::u8),
                        Mortal156(::core::primitive::u8),
                        Mortal157(::core::primitive::u8),
                        Mortal158(::core::primitive::u8),
                        Mortal159(::core::primitive::u8),
                        Mortal160(::core::primitive::u8),
                        Mortal161(::core::primitive::u8),
                        Mortal162(::core::primitive::u8),
                        Mortal163(::core::primitive::u8),
                        Mortal164(::core::primitive::u8),
                        Mortal165(::core::primitive::u8),
                        Mortal166(::core::primitive::u8),
                        Mortal167(::core::primitive::u8),
                        Mortal168(::core::primitive::u8),
                        Mortal169(::core::primitive::u8),
                        Mortal170(::core::primitive::u8),
                        Mortal171(::core::primitive::u8),
                        Mortal172(::core::primitive::u8),
                        Mortal173(::core::primitive::u8),
                        Mortal174(::core::primitive::u8),
                        Mortal175(::core::primitive::u8),
                        Mortal176(::core::primitive::u8),
                        Mortal177(::core::primitive::u8),
                        Mortal178(::core::primitive::u8),
                        Mortal179(::core::primitive::u8),
                        Mortal180(::core::primitive::u8),
                        Mortal181(::core::primitive::u8),
                        Mortal182(::core::primitive::u8),
                        Mortal183(::core::primitive::u8),
                        Mortal184(::core::primitive::u8),
                        Mortal185(::core::primitive::u8),
                        Mortal186(::core::primitive::u8),
                        Mortal187(::core::primitive::u8),
                        Mortal188(::core::primitive::u8),
                        Mortal189(::core::primitive::u8),
                        Mortal190(::core::primitive::u8),
                        Mortal191(::core::primitive::u8),
                        Mortal192(::core::primitive::u8),
                        Mortal193(::core::primitive::u8),
                        Mortal194(::core::primitive::u8),
                        Mortal195(::core::primitive::u8),
                        Mortal196(::core::primitive::u8),
                        Mortal197(::core::primitive::u8),
                        Mortal198(::core::primitive::u8),
                        Mortal199(::core::primitive::u8),
                        Mortal200(::core::primitive::u8),
                        Mortal201(::core::primitive::u8),
                        Mortal202(::core::primitive::u8),
                        Mortal203(::core::primitive::u8),
                        Mortal204(::core::primitive::u8),
                        Mortal205(::core::primitive::u8),
                        Mortal206(::core::primitive::u8),
                        Mortal207(::core::primitive::u8),
                        Mortal208(::core::primitive::u8),
                        Mortal209(::core::primitive::u8),
                        Mortal210(::core::primitive::u8),
                        Mortal211(::core::primitive::u8),
                        Mortal212(::core::primitive::u8),
                        Mortal213(::core::primitive::u8),
                        Mortal214(::core::primitive::u8),
                        Mortal215(::core::primitive::u8),
                        Mortal216(::core::primitive::u8),
                        Mortal217(::core::primitive::u8),
                        Mortal218(::core::primitive::u8),
                        Mortal219(::core::primitive::u8),
                        Mortal220(::core::primitive::u8),
                        Mortal221(::core::primitive::u8),
                        Mortal222(::core::primitive::u8),
                        Mortal223(::core::primitive::u8),
                        Mortal224(::core::primitive::u8),
                        Mortal225(::core::primitive::u8),
                        Mortal226(::core::primitive::u8),
                        Mortal227(::core::primitive::u8),
                        Mortal228(::core::primitive::u8),
                        Mortal229(::core::primitive::u8),
                        Mortal230(::core::primitive::u8),
                        Mortal231(::core::primitive::u8),
                        Mortal232(::core::primitive::u8),
                        Mortal233(::core::primitive::u8),
                        Mortal234(::core::primitive::u8),
                        Mortal235(::core::primitive::u8),
                        Mortal236(::core::primitive::u8),
                        Mortal237(::core::primitive::u8),
                        Mortal238(::core::primitive::u8),
                        Mortal239(::core::primitive::u8),
                        Mortal240(::core::primitive::u8),
                        Mortal241(::core::primitive::u8),
                        Mortal242(::core::primitive::u8),
                        Mortal243(::core::primitive::u8),
                        Mortal244(::core::primitive::u8),
                        Mortal245(::core::primitive::u8),
                        Mortal246(::core::primitive::u8),
                        Mortal247(::core::primitive::u8),
                        Mortal248(::core::primitive::u8),
                        Mortal249(::core::primitive::u8),
                        Mortal250(::core::primitive::u8),
                        Mortal251(::core::primitive::u8),
                        Mortal252(::core::primitive::u8),
                        Mortal253(::core::primitive::u8),
                        Mortal254(::core::primitive::u8),
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::sp_core::H256,
                        pub extrinsics_root: ::subxt::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest<
                            ::subxt::sp_core::H256,
                        >,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum MultiAddress<_0, _1> {
                    Id(_0),
                    Index(_1),
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    Address32([::core::primitive::u8; 32usize]),
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlakeTwo256 {}
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum ArithmeticError {
                Underflow,
                Overflow,
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum DispatchError {
                Other,
                CannotLookup,
                BadOrigin,
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                ConsumerRemaining,
                NoProviders,
                Token(runtime_types::sp_runtime::TokenError),
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum MultiSignature {
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum TokenError {
                NoFunds,
                WouldDie,
                BelowMinimum,
                CannotCreate,
                UnknownAsset,
                Frozen,
                Unsupported,
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_staking {
            use super::runtime_types;
            pub mod offence {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OffenceDetails<_0, _1> {
                    pub offender: _1,
                    pub reporters: ::std::vec::Vec<_0>,
                }
            }
        }
        pub mod sp_transaction_storage_proof {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransactionStorageProof {
                pub chunk: ::std::vec::Vec<::core::primitive::u8>,
                pub proof: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::std::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
    }
    #[doc = r" Default configuration of common types for a target Substrate runtime."]
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct DefaultConfig;
    impl ::subxt::Config for DefaultConfig {
        type Index = u32;
        type BlockNumber = u32;
        type Hash = ::subxt::sp_core::H256;
        type Hashing = ::subxt::sp_runtime::traits::BlakeTwo256;
        type AccountId = ::subxt::sp_runtime::AccountId32;
        type Address = ::subxt::sp_runtime::MultiAddress<Self::AccountId, u32>;
        type Header = ::subxt::sp_runtime::generic::Header<
            Self::BlockNumber,
            ::subxt::sp_runtime::traits::BlakeTwo256,
        >;
        type Signature = ::subxt::sp_runtime::MultiSignature;
        type Extrinsic = ::subxt::sp_runtime::OpaqueExtrinsic;
    }
    impl ::subxt::ExtrinsicExtraData<DefaultConfig> for DefaultConfig {
        type AccountData = AccountData;
        type Extra = ::subxt::DefaultExtra<DefaultConfig>;
    }
    pub type AccountData = self::system::storage::Account;
    impl ::subxt::AccountData<DefaultConfig> for AccountData {
        fn nonce(
            result: &<Self as ::subxt::StorageEntry>::Value,
        ) -> <DefaultConfig as ::subxt::Config>::Index {
            result.nonce
        }
        fn storage_entry(
            account_id: <DefaultConfig as ::subxt::Config>::AccountId,
        ) -> Self {
            Self(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        pub client: ::subxt::Client<T>,
    }
    impl<T> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self { client }
        }
    }
    impl<'a, T> RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T> {
            TransactionApi {
                client: &self.client,
            }
        }
    }
    pub struct StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn indices(&self) -> indices::storage::StorageApi<'a, T> {
            indices::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(
            &self,
        ) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::storage::StorageApi<'a, T> {
            election_provider_multi_phase::storage::StorageApi::new(self.client)
        }
        pub fn staking(&self) -> staking::storage::StorageApi<'a, T> {
            staking::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn democracy(&self) -> democracy::storage::StorageApi<'a, T> {
            democracy::storage::StorageApi::new(self.client)
        }
        pub fn council(&self) -> council::storage::StorageApi<'a, T> {
            council::storage::StorageApi::new(self.client)
        }
        pub fn technical_committee(
            &self,
        ) -> technical_committee::storage::StorageApi<'a, T> {
            technical_committee::storage::StorageApi::new(self.client)
        }
        pub fn elections(&self) -> elections::storage::StorageApi<'a, T> {
            elections::storage::StorageApi::new(self.client)
        }
        pub fn technical_membership(
            &self,
        ) -> technical_membership::storage::StorageApi<'a, T> {
            technical_membership::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn treasury(&self) -> treasury::storage::StorageApi<'a, T> {
            treasury::storage::StorageApi::new(self.client)
        }
        pub fn contracts(&self) -> contracts::storage::StorageApi<'a, T> {
            contracts::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
        }
        pub fn offences(&self) -> offences::storage::StorageApi<'a, T> {
            offences::storage::StorageApi::new(self.client)
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn identity(&self) -> identity::storage::StorageApi<'a, T> {
            identity::storage::StorageApi::new(self.client)
        }
        pub fn society(&self) -> society::storage::StorageApi<'a, T> {
            society::storage::StorageApi::new(self.client)
        }
        pub fn recovery(&self) -> recovery::storage::StorageApi<'a, T> {
            recovery::storage::StorageApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::storage::StorageApi<'a, T> {
            vesting::storage::StorageApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
            scheduler::storage::StorageApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::storage::StorageApi<'a, T> {
            proxy::storage::StorageApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi<'a, T> {
            multisig::storage::StorageApi::new(self.client)
        }
        pub fn bounties(&self) -> bounties::storage::StorageApi<'a, T> {
            bounties::storage::StorageApi::new(self.client)
        }
        pub fn tips(&self) -> tips::storage::StorageApi<'a, T> {
            tips::storage::StorageApi::new(self.client)
        }
        pub fn assets(&self) -> assets::storage::StorageApi<'a, T> {
            assets::storage::StorageApi::new(self.client)
        }
        pub fn mmr(&self) -> mmr::storage::StorageApi<'a, T> {
            mmr::storage::StorageApi::new(self.client)
        }
        pub fn lottery(&self) -> lottery::storage::StorageApi<'a, T> {
            lottery::storage::StorageApi::new(self.client)
        }
        pub fn gilt(&self) -> gilt::storage::StorageApi<'a, T> {
            gilt::storage::StorageApi::new(self.client)
        }
        pub fn uniques(&self) -> uniques::storage::StorageApi<'a, T> {
            uniques::storage::StorageApi::new(self.client)
        }
        pub fn transaction_storage(
            &self,
        ) -> transaction_storage::storage::StorageApi<'a, T> {
            transaction_storage::storage::StorageApi::new(self.client)
        }
        pub fn bags_list(&self) -> bags_list::storage::StorageApi<'a, T> {
            bags_list::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> TransactionApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn utility(&self) -> utility::calls::TransactionApi<'a, T> {
            utility::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn indices(&self) -> indices::calls::TransactionApi<'a, T> {
            indices::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::calls::TransactionApi<'a, T> {
            election_provider_multi_phase::calls::TransactionApi::new(self.client)
        }
        pub fn staking(&self) -> staking::calls::TransactionApi<'a, T> {
            staking::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn democracy(&self) -> democracy::calls::TransactionApi<'a, T> {
            democracy::calls::TransactionApi::new(self.client)
        }
        pub fn council(&self) -> council::calls::TransactionApi<'a, T> {
            council::calls::TransactionApi::new(self.client)
        }
        pub fn technical_committee(
            &self,
        ) -> technical_committee::calls::TransactionApi<'a, T> {
            technical_committee::calls::TransactionApi::new(self.client)
        }
        pub fn elections(&self) -> elections::calls::TransactionApi<'a, T> {
            elections::calls::TransactionApi::new(self.client)
        }
        pub fn technical_membership(
            &self,
        ) -> technical_membership::calls::TransactionApi<'a, T> {
            technical_membership::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn treasury(&self) -> treasury::calls::TransactionApi<'a, T> {
            treasury::calls::TransactionApi::new(self.client)
        }
        pub fn contracts(&self) -> contracts::calls::TransactionApi<'a, T> {
            contracts::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T> {
            im_online::calls::TransactionApi::new(self.client)
        }
        pub fn identity(&self) -> identity::calls::TransactionApi<'a, T> {
            identity::calls::TransactionApi::new(self.client)
        }
        pub fn society(&self) -> society::calls::TransactionApi<'a, T> {
            society::calls::TransactionApi::new(self.client)
        }
        pub fn recovery(&self) -> recovery::calls::TransactionApi<'a, T> {
            recovery::calls::TransactionApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::calls::TransactionApi<'a, T> {
            vesting::calls::TransactionApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T> {
            scheduler::calls::TransactionApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::calls::TransactionApi<'a, T> {
            proxy::calls::TransactionApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi<'a, T> {
            multisig::calls::TransactionApi::new(self.client)
        }
        pub fn bounties(&self) -> bounties::calls::TransactionApi<'a, T> {
            bounties::calls::TransactionApi::new(self.client)
        }
        pub fn tips(&self) -> tips::calls::TransactionApi<'a, T> {
            tips::calls::TransactionApi::new(self.client)
        }
        pub fn assets(&self) -> assets::calls::TransactionApi<'a, T> {
            assets::calls::TransactionApi::new(self.client)
        }
        pub fn lottery(&self) -> lottery::calls::TransactionApi<'a, T> {
            lottery::calls::TransactionApi::new(self.client)
        }
        pub fn gilt(&self) -> gilt::calls::TransactionApi<'a, T> {
            gilt::calls::TransactionApi::new(self.client)
        }
        pub fn uniques(&self) -> uniques::calls::TransactionApi<'a, T> {
            uniques::calls::TransactionApi::new(self.client)
        }
        pub fn transaction_storage(
            &self,
        ) -> transaction_storage::calls::TransactionApi<'a, T> {
            transaction_storage::calls::TransactionApi::new(self.client)
        }
        pub fn bags_list(&self) -> bags_list::calls::TransactionApi<'a, T> {
            bags_list::calls::TransactionApi::new(self.client)
        }
    }
}
