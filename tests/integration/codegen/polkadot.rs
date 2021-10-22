#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 1)]
        Scheduler(scheduler::Event),
        #[codec(index = 4)]
        Indices(indices::Event),
        #[codec(index = 5)]
        Balances(balances::Event),
        #[codec(index = 7)]
        Staking(staking::Event),
        #[codec(index = 8)]
        Offences(offences::Event),
        #[codec(index = 9)]
        Session(session::Event),
        #[codec(index = 11)]
        Grandpa(grandpa::Event),
        #[codec(index = 12)]
        ImOnline(im_online::Event),
        #[codec(index = 14)]
        Democracy(democracy::Event),
        #[codec(index = 15)]
        Council(council::Event),
        #[codec(index = 16)]
        TechnicalCommittee(technical_committee::Event),
        #[codec(index = 17)]
        PhragmenElection(phragmen_election::Event),
        #[codec(index = 18)]
        TechnicalMembership(technical_membership::Event),
        #[codec(index = 19)]
        Treasury(treasury::Event),
        #[codec(index = 24)]
        Claims(claims::Event),
        #[codec(index = 25)]
        Vesting(vesting::Event),
        #[codec(index = 26)]
        Utility(utility::Event),
        #[codec(index = 28)]
        Identity(identity::Event),
        #[codec(index = 29)]
        Proxy(proxy::Event),
        #[codec(index = 30)]
        Multisig(multisig::Event),
        #[codec(index = 34)]
        Bounties(bounties::Event),
        #[codec(index = 35)]
        Tips(tips::Event),
        #[codec(index = 36)]
        ElectionProviderMultiPhase(election_provider_multi_phase::Event),
        #[codec(index = 53)]
        ParaInclusion(para_inclusion::Event),
        #[codec(index = 56)]
        Paras(paras::Event),
        #[codec(index = 59)]
        Ump(ump::Event),
        #[codec(index = 60)]
        Hrmp(hrmp::Event),
        #[codec(index = 70)]
        Registrar(registrar::Event),
        #[codec(index = 71)]
        Slots(slots::Event),
        #[codec(index = 72)]
        Auctions(auctions::Event),
        #[codec(index = 73)]
        Crowdloan(crowdloan::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct FillBlock {
                pub ratio: ::subxt::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Remark {
                pub remark: Vec<u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHeapPages {
                pub pages: u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetCode {
                pub code: Vec<u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetCodeWithoutChecks {
                pub code: Vec<u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetChangesTrieConfig {
                pub changes_trie_config: Option<
                    runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                >,
            }
            impl ::subxt::Call for SetChangesTrieConfig {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_changes_trie_config";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetStorage {
                pub items: Vec<(Vec<u8>, Vec<u8>)>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct KillStorage {
                pub keys: Vec<Vec<u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct KillPrefix {
                pub prefix: Vec<u8>,
                pub subkeys: u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemarkWithEvent {
                pub remark: Vec<u8>,
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
                    remark: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Remark> {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHeapPages> {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCode> {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCodeWithoutChecks>
                {
                    let call = SetCodeWithoutChecks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_changes_trie_config(
                    &self,
                    changes_trie_config: Option<
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
                    items: Vec<(Vec<u8>, Vec<u8>)>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetStorage> {
                    let call = SetStorage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: Vec<Vec<u8>>,
                ) -> ::subxt::SubmittableExtrinsic<T, KillStorage> {
                    let call = KillStorage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: Vec<u8>,
                    subkeys: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, KillPrefix> {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, RemarkWithEvent> {
                    let call = RemarkWithEvent { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExtrinsicSuccess(
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CodeUpdated {}
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                    u32,
                    runtime_types::pallet_balances::AccountData<u128>,
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value = runtime_types::frame_support::weights::PerDispatchClass<u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub u32);
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
            pub struct ExtrinsicData(pub u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = Vec<u8>;
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
                type Value = u32;
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
                type Value = Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::polkadot_runtime::Event,
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = Vec<(u32, u32)>;
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
                type Value = bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = bool;
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
                        u32,
                        runtime_types::pallet_balances::AccountData<u128>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<u64>,
                    ::subxt::Error,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Vec<u8>, ::subxt::Error> {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
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
                    Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::polkadot_runtime::Event,
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
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Vec<(u32, u32)>, ::subxt::Error>
                {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                ) -> ::core::result::Result<bool, ::subxt::Error> {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<bool, ::subxt::Error> {
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
    pub mod scheduler {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Schedule {
                pub when: u32,
                pub maybe_periodic: Option<(u32, u32)>,
                pub priority: u8,
                pub call: runtime_types::polkadot_runtime::Call,
            }
            impl ::subxt::Call for Schedule {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Cancel {
                pub when: u32,
                pub index: u32,
            }
            impl ::subxt::Call for Cancel {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScheduleNamed {
                pub id: Vec<u8>,
                pub when: u32,
                pub maybe_periodic: Option<(u32, u32)>,
                pub priority: u8,
                pub call: runtime_types::polkadot_runtime::Call,
            }
            impl ::subxt::Call for ScheduleNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelNamed {
                pub id: Vec<u8>,
            }
            impl ::subxt::Call for CancelNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel_named";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScheduleAfter {
                pub after: u32,
                pub maybe_periodic: Option<(u32, u32)>,
                pub priority: u8,
                pub call: runtime_types::polkadot_runtime::Call,
            }
            impl ::subxt::Call for ScheduleAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_after";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScheduleNamedAfter {
                pub id: Vec<u8>,
                pub after: u32,
                pub maybe_periodic: Option<(u32, u32)>,
                pub priority: u8,
                pub call: runtime_types::polkadot_runtime::Call,
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
                    when: u32,
                    maybe_periodic: Option<(u32, u32)>,
                    priority: u8,
                    call: runtime_types::polkadot_runtime::Call,
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
                    when: u32,
                    index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Cancel> {
                    let call = Cancel { when, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named(
                    &self,
                    id: Vec<u8>,
                    when: u32,
                    maybe_periodic: Option<(u32, u32)>,
                    priority: u8,
                    call: runtime_types::polkadot_runtime::Call,
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
                    id: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelNamed> {
                    let call = CancelNamed { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_after(
                    &self,
                    after: u32,
                    maybe_periodic: Option<(u32, u32)>,
                    priority: u8,
                    call: runtime_types::polkadot_runtime::Call,
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
                    id: Vec<u8>,
                    after: u32,
                    maybe_periodic: Option<(u32, u32)>,
                    priority: u8,
                    call: runtime_types::polkadot_runtime::Call,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Scheduled(pub u32, pub u32);
            impl ::subxt::Event for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Canceled(pub u32, pub u32);
            impl ::subxt::Event for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Dispatched(
                pub (u32, u32),
                pub Option<Vec<u8>>,
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Agenda(pub u32);
            impl ::subxt::StorageEntry for Agenda {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Agenda";
                type Value = Vec<
                    Option<
                        runtime_types::pallet_scheduler::ScheduledV2<
                            runtime_types::polkadot_runtime::Call,
                            u32,
                            runtime_types::polkadot_runtime::OriginCaller,
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
            pub struct Lookup(pub Vec<u8>);
            impl ::subxt::StorageEntry for Lookup {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Lookup";
                type Value = (u32, u32);
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
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<
                        Option<
                            runtime_types::pallet_scheduler::ScheduledV2<
                                runtime_types::polkadot_runtime::Call,
                                u32,
                                runtime_types::polkadot_runtime::OriginCaller,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Agenda(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lookup(
                    &self,
                    _0: Vec<u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(u32, u32)>,
                    ::subxt::Error,
                > {
                    let entry = Lookup(_0);
                    self.client.storage().fetch(&entry, hash).await
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
    pub mod babe {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportEquivocation {
                pub equivocation_proof:
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            u32,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof:
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            u32,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                    equivocation_proof : runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public >,
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
                    equivocation_proof : runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public >,
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
                type Value = u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , u64 ,) > ;
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
                type Value = [u8; 32usize];
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
                type Value = [u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , u64 ,) > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction(pub u32);
            impl ::subxt::StorageEntry for UnderConstruction {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value =
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [u8; 32usize],
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
                type Value = Option<[u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value = Option<[u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (u32, u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = u32;
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
                ) -> ::core::result::Result<u64, ::subxt::Error> {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn authorities (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , u64 ,) > , :: subxt :: Error >{
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
                ) -> ::core::result::Result<[u8; 32usize], ::subxt::Error>
                {
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
                ) -> ::core::result::Result<[u8; 32usize], ::subxt::Error>
                {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn next_authorities (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_consensus_babe :: app :: Public , u64 ,) > , :: subxt :: Error >{
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [u8; 32usize],
                    >,
                    ::subxt::Error,
                > {
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<Option<[u8; 32usize]>>,
                    ::subxt::Error,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Option<[u8; 32usize]>, ::subxt::Error>
                {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(u32, u32), ::subxt::Error> {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Set {
                #[codec(compact)]
                pub now: u64,
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
                pub fn set(&self, now: u64) -> ::subxt::SubmittableExtrinsic<T, Set> {
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
                type Value = u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = bool;
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
                ) -> ::core::result::Result<u64, ::subxt::Error> {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<bool, ::subxt::Error> {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod indices {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Claim {
                pub index: u32,
            }
            impl ::subxt::Call for Claim {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "claim";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Transfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: u32,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Free {
                pub index: u32,
            }
            impl ::subxt::Call for Free {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "free";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceTransfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: u32,
                pub freeze: bool,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Freeze {
                pub index: u32,
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
                    index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Claim> {
                    let call = Claim { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { new, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn free(&self, index: u32) -> ::subxt::SubmittableExtrinsic<T, Free> {
                    let call = Free { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: u32,
                    freeze: bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
                    let call = ForceTransfer { new, index, freeze };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Freeze> {
                    let call = Freeze { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_indices::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct IndexAssigned(pub ::subxt::sp_core::crypto::AccountId32, pub u32);
            impl ::subxt::Event for IndexAssigned {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexAssigned";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct IndexFreed(pub u32);
            impl ::subxt::Event for IndexFreed {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFreed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct IndexFrozen(pub u32, pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for IndexFrozen {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFrozen";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Accounts(pub u32);
            impl ::subxt::StorageEntry for Accounts {
                const PALLET: &'static str = "Indices";
                const STORAGE: &'static str = "Accounts";
                type Value = (::subxt::sp_core::crypto::AccountId32, u128, bool);
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
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                        bool,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Accounts(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Transfer {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetBalance {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub new_free: u128,
                #[codec(compact)]
                pub new_reserved: u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceTransfer {
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransferAll {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub keep_alive: bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceUnreserve {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub amount: u128,
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
                        (),
                    >,
                    value: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: u128,
                    new_reserved: u128,
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
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: u128,
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
                        (),
                    >,
                    value: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferKeepAlive> {
                    let call = TransferKeepAlive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: bool,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferAll> {
                    let call = TransferAll { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceUnreserve> {
                    let call = ForceUnreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Endowed(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct DustLost(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Transfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BalanceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
                pub u128,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Deposit(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Reserved(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unreserved(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReserveRepatriated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
                pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            );
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<u128>;
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
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < u128 > > ;
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
                        runtime_types::pallet_balances::ReserveData<[u8; 8usize], u128>,
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
                ) -> ::core::result::Result<u128, ::subxt::Error> {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<u128>,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn locks (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < u128 > > , :: subxt :: Error >{
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<[u8; 8usize], u128>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
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
    pub mod authorship {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetUncles {
                pub new_uncles: Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        u32,
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
                    new_uncles: Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            u32,
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
                type Value = Vec<
                    runtime_types::pallet_authorship::UncleEntryItem<
                        u32,
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
                type Value = bool;
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
                    Vec<
                        runtime_types::pallet_authorship::UncleEntryItem<
                            u32,
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
                ) -> ::core::result::Result<bool, ::subxt::Error> {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod staking {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Bond {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: u128,
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for Bond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BondExtra {
                #[codec(compact)]
                pub max_additional: u128,
            }
            impl ::subxt::Call for BondExtra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond_extra";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unbond {
                #[codec(compact)]
                pub value: u128,
            }
            impl ::subxt::Call for Unbond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "unbond";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct WithdrawUnbonded {
                pub num_slashing_spans: u32,
            }
            impl ::subxt::Call for WithdrawUnbonded {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "withdraw_unbonded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Validate {
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            impl ::subxt::Call for Validate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "validate";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Nominate {
                pub targets: Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                >,
            }
            impl ::subxt::Call for Nominate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "nominate";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Chill {}
            impl ::subxt::Call for Chill {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetPayee {
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for SetPayee {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_payee";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetController {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            impl ::subxt::Call for SetController {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_controller";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetValidatorCount {
                #[codec(compact)]
                pub new: u32,
            }
            impl ::subxt::Call for SetValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_validator_count";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct IncreaseValidatorCount {
                #[codec(compact)]
                pub additional: u32,
            }
            impl ::subxt::Call for IncreaseValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "increase_validator_count";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScaleValidatorCount {
                pub factor: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for ScaleValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "scale_validator_count";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceNoEras {}
            impl ::subxt::Call for ForceNoEras {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_no_eras";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceNewEra {}
            impl ::subxt::Call for ForceNewEra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetInvulnerables {
                pub invulnerables: Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for SetInvulnerables {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_invulnerables";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceUnstake {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: u32,
            }
            impl ::subxt::Call for ForceUnstake {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_unstake";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceNewEraAlways {}
            impl ::subxt::Call for ForceNewEraAlways {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era_always";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelDeferredSlash {
                pub era: u32,
                pub slash_indices: Vec<u32>,
            }
            impl ::subxt::Call for CancelDeferredSlash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "cancel_deferred_slash";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PayoutStakers {
                pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
                pub era: u32,
            }
            impl ::subxt::Call for PayoutStakers {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "payout_stakers";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Rebond {
                #[codec(compact)]
                pub value: u128,
            }
            impl ::subxt::Call for Rebond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "rebond";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHistoryDepth {
                #[codec(compact)]
                pub new_history_depth: u32,
                #[codec(compact)]
                pub era_items_deleted: u32,
            }
            impl ::subxt::Call for SetHistoryDepth {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_history_depth";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReapStash {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: u32,
            }
            impl ::subxt::Call for ReapStash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "reap_stash";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Kick {
                pub who: Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                >,
            }
            impl ::subxt::Call for Kick {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "kick";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetStakingLimits {
                pub min_nominator_bond: u128,
                pub min_validator_bond: u128,
                pub max_nominator_count: Option<u32>,
                pub max_validator_count: Option<u32>,
                pub threshold: Option<runtime_types::sp_arithmetic::per_things::Percent>,
            }
            impl ::subxt::Call for SetStakingLimits {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_staking_limits";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                        (),
                    >,
                    value: u128,
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
                    max_additional: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, BondExtra> {
                    let call = BondExtra { max_additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbond(
                    &self,
                    value: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Unbond> {
                    let call = Unbond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: u32,
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
                    targets: Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
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
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetController> {
                    let call = SetController { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validator_count(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetValidatorCount> {
                    let call = SetValidatorCount { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_validator_count(
                    &self,
                    additional: u32,
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
                    invulnerables: Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetInvulnerables> {
                    let call = SetInvulnerables { invulnerables };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unstake(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: u32,
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
                    era: u32,
                    slash_indices: Vec<u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelDeferredSlash>
                {
                    let call = CancelDeferredSlash { era, slash_indices };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn payout_stakers(
                    &self,
                    validator_stash: ::subxt::sp_core::crypto::AccountId32,
                    era: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, PayoutStakers> {
                    let call = PayoutStakers {
                        validator_stash,
                        era,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rebond(
                    &self,
                    value: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Rebond> {
                    let call = Rebond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_history_depth(
                    &self,
                    new_history_depth: u32,
                    era_items_deleted: u32,
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
                    num_slashing_spans: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ReapStash> {
                    let call = ReapStash {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kick(
                    &self,
                    who: Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Kick> {
                    let call = Kick { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_staking_limits(
                    &self,
                    min_nominator_bond: u128,
                    min_validator_bond: u128,
                    max_nominator_count: Option<u32>,
                    max_validator_count: Option<u32>,
                    threshold: Option<runtime_types::sp_arithmetic::per_things::Percent>,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct EraPaid(pub u32, pub u128, pub u128);
            impl ::subxt::Event for EraPaid {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Rewarded(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Slashed(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OldSlashingReportDiscarded(pub u32);
            impl ::subxt::Event for OldSlashingReportDiscarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct StakersElected {}
            impl ::subxt::Event for StakersElected {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakersElected";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Bonded(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Bonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unbonded(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Unbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Withdrawn(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Withdrawn {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Kicked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Kicked {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct StakingElectionFailed {}
            impl ::subxt::Event for StakingElectionFailed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakingElectionFailed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Chilled(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Chilled {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Chilled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PayoutStarted(pub u32, pub ::subxt::sp_core::crypto::AccountId32);
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorCount;
            impl ::subxt::StorageEntry for ValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorCount";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinimumValidatorCount;
            impl ::subxt::StorageEntry for MinimumValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinimumValidatorCount";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Invulnerables;
            impl ::subxt::StorageEntry for Invulnerables {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Invulnerables";
                type Value = Vec<::subxt::sp_core::crypto::AccountId32>;
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
                type Value = u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinValidatorBond;
            impl ::subxt::StorageEntry for MinValidatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinValidatorBond";
                type Value = u128;
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
                    u128,
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxValidatorsCount;
            impl ::subxt::StorageEntry for MaxValidatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxValidatorsCount";
                type Value = u32;
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxNominatorsCount;
            impl ::subxt::StorageEntry for MaxNominatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxNominatorsCount";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentEra";
                type Value = u32;
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
            pub struct ErasStartSessionIndex(pub u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasStakers(u32, ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ErasStakers {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakers";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    u128,
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
            pub struct ErasStakersClipped(u32, ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ErasStakersClipped {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakersClipped";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    u128,
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
            pub struct ErasValidatorPrefs(u32, ::subxt::sp_core::crypto::AccountId32);
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
            pub struct ErasValidatorReward(pub u32);
            impl ::subxt::StorageEntry for ErasValidatorReward {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasRewardPoints(pub u32);
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
            pub struct ErasTotalStake(pub u32);
            impl ::subxt::StorageEntry for ErasTotalStake {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = u128;
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
                type Value = u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnappliedSlashes(pub u32);
            impl ::subxt::StorageEntry for UnappliedSlashes {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "UnappliedSlashes";
                type Value = Vec<
                    runtime_types::pallet_staking::UnappliedSlash<
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
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
                type Value = Vec<(u32, u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSlashInEra(u32, ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ValidatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorSlashInEra";
                type Value = (::subxt::sp_arithmetic::per_things::Perbill, u128);
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
            pub struct NominatorSlashInEra(u32, ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NominatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "NominatorSlashInEra";
                type Value = u128;
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
            pub struct SpanSlash(::subxt::sp_core::crypto::AccountId32, u32);
            impl ::subxt::StorageEntry for SpanSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SpanSlash";
                type Value = runtime_types::pallet_staking::slashing::SpanRecord<u128>;
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = u32;
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
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = ValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn minimum_validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = MinimumValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn invulnerables(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<::subxt::sp_core::crypto::AccountId32>,
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
                pub async fn min_nominator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u128, ::subxt::Error> {
                    let entry = MinNominatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn min_validator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u128, ::subxt::Error> {
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
                            u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Ledger(_0);
                    self.client.storage().fetch(&entry, hash).await
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
                pub async fn counter_for_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = CounterForValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_validators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
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
                pub async fn counter_for_nominators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = CounterForNominators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_nominators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = MaxNominatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
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
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_clipped(
                    &self,
                    _0: u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasStakersClipped(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_validator_prefs(
                    &self,
                    _0: u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::Error,
                > {
                    let entry = ErasValidatorPrefs(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u128>, ::subxt::Error>
                {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: u32,
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
                pub async fn eras_total_stake(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u128, ::subxt::Error> {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                ) -> ::core::result::Result<u128, ::subxt::Error> {
                    let entry = CanceledSlashPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<
                        runtime_types::pallet_staking::UnappliedSlash<
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UnappliedSlashes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Vec<(u32, u32)>, ::subxt::Error>
                {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_slash_in_era(
                    &self,
                    _0: u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_arithmetic::per_things::Perbill,
                        u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = ValidatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominator_slash_in_era(
                    &self,
                    _0: u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u128>, ::subxt::Error>
                {
                    let entry = NominatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
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
                pub async fn span_slash(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::slashing::SpanRecord<u128>,
                    ::subxt::Error,
                > {
                    let entry = SpanSlash(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn earliest_unapplied_slash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = EarliestUnappliedSlash;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = CurrentPlannedSession;
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
    pub mod offences {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_offences::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Offence(pub [u8; 16usize], pub Vec<u8>);
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
                            u128,
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
            pub struct ConcurrentReportsIndex([u8; 16usize], Vec<u8>);
            impl ::subxt::StorageEntry for ConcurrentReportsIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ConcurrentReportsIndex";
                type Value = Vec<::subxt::sp_core::H256>;
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
            pub struct ReportsByKindIndex(pub [u8; 16usize]);
            impl ::subxt::StorageEntry for ReportsByKindIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ReportsByKindIndex";
                type Value = Vec<u8>;
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
                                    u128,
                                >,
                            ),
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reports(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn concurrent_reports_index(
                    &self,
                    _0: [u8; 16usize],
                    _1: Vec<u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Vec<::subxt::sp_core::H256>, ::subxt::Error>
                {
                    let entry = ConcurrentReportsIndex(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reports_by_kind_index(
                    &self,
                    _0: [u8; 16usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Vec<u8>, ::subxt::Error> {
                    let entry = ReportsByKindIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetKeys {
                pub keys: runtime_types::polkadot_runtime::SessionKeys,
                pub proof: Vec<u8>,
            }
            impl ::subxt::Call for SetKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                    keys: runtime_types::polkadot_runtime::SessionKeys,
                    proof: Vec<u8>,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewSession(pub u32);
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
                type Value = Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::polkadot_runtime::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = Vec<u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value = runtime_types::polkadot_runtime::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct KeyOwner(runtime_types::sp_core::crypto::KeyTypeId, Vec<u8>);
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
                    Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<bool, ::subxt::Error> {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::polkadot_runtime::SessionKeys,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Vec<u32>, ::subxt::Error> {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_runtime::SessionKeys>,
                    ::subxt::Error,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: Vec<u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportEquivocation {
                pub equivocation_proof:
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        u32,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof:
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        u32,
                    >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NoteStalled {
                pub delay: u32,
                pub best_finalized_block_number: u32,
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
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , u32 >,
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
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , u32 >,
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
                    delay: u32,
                    best_finalized_block_number: u32,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewAuthorities(
                pub Vec<(runtime_types::sp_finality_grandpa::app::Public, u64)>,
            );
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Paused {}
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                type Value = runtime_types::pallet_grandpa::StoredState<u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value = runtime_types::pallet_grandpa::StoredPendingChange<u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (u32, u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = u32;
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
                    runtime_types::pallet_grandpa::StoredState<u32>,
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
                        runtime_types::pallet_grandpa::StoredPendingChange<u32>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(u32, u32)>,
                    ::subxt::Error,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u64, ::subxt::Error> {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod im_online {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Heartbeat {
                pub heartbeat: runtime_types::pallet_im_online::Heartbeat<u32>,
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
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<u32>,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct HeartbeatReceived(
                pub runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            );
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AllGood {}
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SomeOffline(
                pub  Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
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
                type Value = u32;
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
            pub struct ReceivedHeartbeats(u32, u32);
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
            pub struct AuthoredBlocks(u32, ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for AuthoredBlocks {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = u32;
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
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn keys (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public > , :: subxt :: Error >{
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn received_heartbeats(
                    &self,
                    _0: u32,
                    _1: u32,
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
                pub async fn authored_blocks(
                    &self,
                    _0: u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod authority_discovery {
        use super::runtime_types;
    }
    pub mod democracy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Propose {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub value: u128,
            }
            impl ::subxt::Call for Propose {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Second {
                #[codec(compact)]
                pub proposal: u32,
                #[codec(compact)]
                pub seconds_upper_bound: u32,
            }
            impl ::subxt::Call for Second {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "second";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                #[codec(compact)]
                pub ref_index: u32,
                pub vote: runtime_types::pallet_democracy::vote::AccountVote<u128>,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct EmergencyCancel {
                pub ref_index: u32,
            }
            impl ::subxt::Call for EmergencyCancel {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "emergency_cancel";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExternalPropose {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalPropose {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExternalProposeMajority {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalProposeMajority {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose_majority";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExternalProposeDefault {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for ExternalProposeDefault {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "external_propose_default";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct FastTrack {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub voting_period: u32,
                pub delay: u32,
            }
            impl ::subxt::Call for FastTrack {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "fast_track";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct VetoExternal {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for VetoExternal {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "veto_external";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelReferendum {
                #[codec(compact)]
                pub ref_index: u32,
            }
            impl ::subxt::Call for CancelReferendum {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "cancel_referendum";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelQueued {
                pub which: u32,
            }
            impl ::subxt::Call for CancelQueued {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "cancel_queued";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Delegate {
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub conviction: runtime_types::pallet_democracy::conviction::Conviction,
                pub balance: u128,
            }
            impl ::subxt::Call for Delegate {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "delegate";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Undelegate {}
            impl ::subxt::Call for Undelegate {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "undelegate";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearPublicProposals {}
            impl ::subxt::Call for ClearPublicProposals {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "clear_public_proposals";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NotePreimage {
                pub encoded_proposal: Vec<u8>,
            }
            impl ::subxt::Call for NotePreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_preimage";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NotePreimageOperational {
                pub encoded_proposal: Vec<u8>,
            }
            impl ::subxt::Call for NotePreimageOperational {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_preimage_operational";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NoteImminentPreimage {
                pub encoded_proposal: Vec<u8>,
            }
            impl ::subxt::Call for NoteImminentPreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_imminent_preimage";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NoteImminentPreimageOperational {
                pub encoded_proposal: Vec<u8>,
            }
            impl ::subxt::Call for NoteImminentPreimageOperational {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "note_imminent_preimage_operational";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReapPreimage {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub proposal_len_upper_bound: u32,
            }
            impl ::subxt::Call for ReapPreimage {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "reap_preimage";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unlock {
                pub target: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for Unlock {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "unlock";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveVote {
                pub index: u32,
            }
            impl ::subxt::Call for RemoveVote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "remove_vote";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveOtherVote {
                pub target: ::subxt::sp_core::crypto::AccountId32,
                pub index: u32,
            }
            impl ::subxt::Call for RemoveOtherVote {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "remove_other_vote";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct EnactProposal {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub index: u32,
            }
            impl ::subxt::Call for EnactProposal {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "enact_proposal";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Blacklist {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub maybe_ref_index: Option<u32>,
            }
            impl ::subxt::Call for Blacklist {
                const PALLET: &'static str = "Democracy";
                const FUNCTION: &'static str = "blacklist";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelProposal {
                #[codec(compact)]
                pub prop_index: u32,
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
                    value: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Propose> {
                    let call = Propose {
                        proposal_hash,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn second(
                    &self,
                    proposal: u32,
                    seconds_upper_bound: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Second> {
                    let call = Second {
                        proposal,
                        seconds_upper_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vote(
                    &self,
                    ref_index: u32,
                    vote: runtime_types::pallet_democracy::vote::AccountVote<u128>,
                ) -> ::subxt::SubmittableExtrinsic<T, Vote> {
                    let call = Vote { ref_index, vote };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn emergency_cancel(
                    &self,
                    ref_index: u32,
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
                    voting_period: u32,
                    delay: u32,
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
                    ref_index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelReferendum> {
                    let call = CancelReferendum { ref_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_queued(
                    &self,
                    which: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelQueued> {
                    let call = CancelQueued { which };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn delegate(
                    &self,
                    to: ::subxt::sp_core::crypto::AccountId32,
                    conviction: runtime_types::pallet_democracy::conviction::Conviction,
                    balance: u128,
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
                    encoded_proposal: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, NotePreimage> {
                    let call = NotePreimage { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_preimage_operational(
                    &self,
                    encoded_proposal: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, NotePreimageOperational>
                {
                    let call = NotePreimageOperational { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_imminent_preimage(
                    &self,
                    encoded_proposal: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, NoteImminentPreimage>
                {
                    let call = NoteImminentPreimage { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_imminent_preimage_operational(
                    &self,
                    encoded_proposal: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, NoteImminentPreimageOperational>
                {
                    let call = NoteImminentPreimageOperational { encoded_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reap_preimage(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    proposal_len_upper_bound: u32,
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
                    index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveVote> {
                    let call = RemoveVote { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_other_vote(
                    &self,
                    target: ::subxt::sp_core::crypto::AccountId32,
                    index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveOtherVote> {
                    let call = RemoveOtherVote { target, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn enact_proposal(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    index: u32,
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
                    maybe_ref_index: Option<u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, Blacklist> {
                    let call = Blacklist {
                        proposal_hash,
                        maybe_ref_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_proposal(
                    &self,
                    prop_index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelProposal> {
                    let call = CancelProposal { prop_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_democracy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Proposed(pub u32, pub u128);
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Tabled(
                pub u32,
                pub u128,
                pub Vec<::subxt::sp_core::crypto::AccountId32>,
            );
            impl ::subxt::Event for Tabled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Tabled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExternalTabled {}
            impl ::subxt::Event for ExternalTabled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "ExternalTabled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Started(
                pub u32,
                pub runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
            );
            impl ::subxt::Event for Started {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Started";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Passed(pub u32);
            impl ::subxt::Event for Passed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Passed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NotPassed(pub u32);
            impl ::subxt::Event for NotPassed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "NotPassed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Cancelled(pub u32);
            impl ::subxt::Event for Cancelled {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Cancelled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Executed(
                pub u32,
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Executed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Delegated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Delegated {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Delegated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Undelegated(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Undelegated {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Undelegated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vetoed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub u32,
            );
            impl ::subxt::Event for Vetoed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "Vetoed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PreimageNoted(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for PreimageNoted {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageNoted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PreimageUsed(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for PreimageUsed {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageUsed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PreimageInvalid(pub ::subxt::sp_core::H256, pub u32);
            impl ::subxt::Event for PreimageInvalid {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageInvalid";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PreimageMissing(pub ::subxt::sp_core::H256, pub u32);
            impl ::subxt::Event for PreimageMissing {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageMissing";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PreimageReaped(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for PreimageReaped {
                const PALLET: &'static str = "Democracy";
                const EVENT: &'static str = "PreimageReaped";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PublicProps;
            impl ::subxt::StorageEntry for PublicProps {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "PublicProps";
                type Value = Vec<(
                    u32,
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DepositOf(pub u32);
            impl ::subxt::StorageEntry for DepositOf {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "DepositOf";
                type Value = (Vec<::subxt::sp_core::crypto::AccountId32>, u128);
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
                    u128,
                    u32,
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LowestUnbaked;
            impl ::subxt::StorageEntry for LowestUnbaked {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "LowestUnbaked";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReferendumInfoOf(pub u32);
            impl ::subxt::StorageEntry for ReferendumInfoOf {
                const PALLET: &'static str = "Democracy";
                const STORAGE: &'static str = "ReferendumInfoOf";
                type Value = runtime_types::pallet_democracy::types::ReferendumInfo<
                    u32,
                    ::subxt::sp_core::H256,
                    u128,
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
                    u128,
                    ::subxt::sp_core::crypto::AccountId32,
                    u32,
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
                type Value = u32;
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
                type Value = bool;
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
                type Value = (u32, Vec<::subxt::sp_core::crypto::AccountId32>);
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
                type Value = bool;
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
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = PublicPropCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn public_props(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<(
                        u32,
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
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        Vec<::subxt::sp_core::crypto::AccountId32>,
                        u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = DepositOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn preimages(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_democracy::PreimageStatus<
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
                            u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Preimages(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn referendum_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = ReferendumCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lowest_unbaked(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = LowestUnbaked;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn referendum_info_of(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_democracy::types::ReferendumInfo<
                            u32,
                            ::subxt::sp_core::H256,
                            u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ReferendumInfoOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn voting_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_democracy::vote::Voting<
                        u128,
                        ::subxt::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = VotingOf(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = Locks(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn last_tabled_was_external(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<bool, ::subxt::Error> {
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
                        u32,
                        Vec<::subxt::sp_core::crypto::AccountId32>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Blacklist(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn cancellations(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<bool, ::subxt::Error> {
                    let entry = Cancellations(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMembers {
                pub new_members: Vec<::subxt::sp_core::crypto::AccountId32>,
                pub prime: Option<::subxt::sp_core::crypto::AccountId32>,
                pub old_count: u32,
            }
            impl ::subxt::Call for SetMembers {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "set_members";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Execute {
                pub proposal: runtime_types::polkadot_runtime::Call,
                #[codec(compact)]
                pub length_bound: u32,
            }
            impl ::subxt::Call for Execute {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Propose {
                #[codec(compact)]
                pub threshold: u32,
                pub proposal: runtime_types::polkadot_runtime::Call,
                #[codec(compact)]
                pub length_bound: u32,
            }
            impl ::subxt::Call for Propose {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                pub proposal: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: u32,
                pub approve: bool,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Close {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: u32,
                #[codec(compact)]
                pub proposal_weight_bound: u64,
                #[codec(compact)]
                pub length_bound: u32,
            }
            impl ::subxt::Call for Close {
                const PALLET: &'static str = "Council";
                const FUNCTION: &'static str = "close";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                    new_members: Vec<::subxt::sp_core::crypto::AccountId32>,
                    prime: Option<::subxt::sp_core::crypto::AccountId32>,
                    old_count: u32,
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
                    proposal: runtime_types::polkadot_runtime::Call,
                    length_bound: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Execute> {
                    let call = Execute {
                        proposal,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose(
                    &self,
                    threshold: u32,
                    proposal: runtime_types::polkadot_runtime::Call,
                    length_bound: u32,
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
                    index: u32,
                    approve: bool,
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
                    index: u32,
                    proposal_weight_bound: u64,
                    length_bound: u32,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Proposed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u32,
                pub ::subxt::sp_core::H256,
                pub u32,
            );
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Voted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub bool,
                pub u32,
                pub u32,
            );
            impl ::subxt::Event for Voted {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Voted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Approved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Approved {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Approved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Disapproved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Disapproved {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Executed(
                pub ::subxt::sp_core::H256,
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Executed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MemberExecuted(
                pub ::subxt::sp_core::H256,
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MemberExecuted {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "MemberExecuted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Closed(pub ::subxt::sp_core::H256, pub u32, pub u32);
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
                type Value = runtime_types::polkadot_runtime::Call;
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
                    u32,
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "Council";
                const STORAGE: &'static str = "Members";
                type Value = Vec<::subxt::sp_core::crypto::AccountId32>;
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
                    ::core::option::Option<runtime_types::polkadot_runtime::Call>,
                    ::subxt::Error,
                > {
                    let entry = ProposalOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn voting(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<::subxt::sp_core::crypto::AccountId32>,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMembers {
                pub new_members: Vec<::subxt::sp_core::crypto::AccountId32>,
                pub prime: Option<::subxt::sp_core::crypto::AccountId32>,
                pub old_count: u32,
            }
            impl ::subxt::Call for SetMembers {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "set_members";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Execute {
                pub proposal: runtime_types::polkadot_runtime::Call,
                #[codec(compact)]
                pub length_bound: u32,
            }
            impl ::subxt::Call for Execute {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Propose {
                #[codec(compact)]
                pub threshold: u32,
                pub proposal: runtime_types::polkadot_runtime::Call,
                #[codec(compact)]
                pub length_bound: u32,
            }
            impl ::subxt::Call for Propose {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                pub proposal: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: u32,
                pub approve: bool,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Close {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: u32,
                #[codec(compact)]
                pub proposal_weight_bound: u64,
                #[codec(compact)]
                pub length_bound: u32,
            }
            impl ::subxt::Call for Close {
                const PALLET: &'static str = "TechnicalCommittee";
                const FUNCTION: &'static str = "close";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                    new_members: Vec<::subxt::sp_core::crypto::AccountId32>,
                    prime: Option<::subxt::sp_core::crypto::AccountId32>,
                    old_count: u32,
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
                    proposal: runtime_types::polkadot_runtime::Call,
                    length_bound: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Execute> {
                    let call = Execute {
                        proposal,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose(
                    &self,
                    threshold: u32,
                    proposal: runtime_types::polkadot_runtime::Call,
                    length_bound: u32,
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
                    index: u32,
                    approve: bool,
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
                    index: u32,
                    proposal_weight_bound: u64,
                    length_bound: u32,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Proposed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u32,
                pub ::subxt::sp_core::H256,
                pub u32,
            );
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Voted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
                pub bool,
                pub u32,
                pub u32,
            );
            impl ::subxt::Event for Voted {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Voted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Approved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Approved {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Approved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Disapproved(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for Disapproved {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Executed(
                pub ::subxt::sp_core::H256,
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "Executed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MemberExecuted(
                pub ::subxt::sp_core::H256,
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MemberExecuted {
                const PALLET: &'static str = "TechnicalCommittee";
                const EVENT: &'static str = "MemberExecuted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Closed(pub ::subxt::sp_core::H256, pub u32, pub u32);
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
                type Value = runtime_types::polkadot_runtime::Call;
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
                    u32,
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "TechnicalCommittee";
                const STORAGE: &'static str = "Members";
                type Value = Vec<::subxt::sp_core::crypto::AccountId32>;
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
                    ::core::option::Option<runtime_types::polkadot_runtime::Call>,
                    ::subxt::Error,
                > {
                    let entry = ProposalOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn voting(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<::subxt::sp_core::crypto::AccountId32>,
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
    pub mod phragmen_election {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                pub votes: Vec<::subxt::sp_core::crypto::AccountId32>,
                #[codec(compact)]
                pub value: u128,
            }
            impl ::subxt::Call for Vote {
                const PALLET: &'static str = "PhragmenElection";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveVoter {}
            impl ::subxt::Call for RemoveVoter {
                const PALLET: &'static str = "PhragmenElection";
                const FUNCTION: &'static str = "remove_voter";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SubmitCandidacy {
                #[codec(compact)]
                pub candidate_count: u32,
            }
            impl ::subxt::Call for SubmitCandidacy {
                const PALLET: &'static str = "PhragmenElection";
                const FUNCTION: &'static str = "submit_candidacy";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RenounceCandidacy {
                pub renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
            }
            impl ::subxt::Call for RenounceCandidacy {
                const PALLET: &'static str = "PhragmenElection";
                const FUNCTION: &'static str = "renounce_candidacy";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveMember {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub has_replacement: bool,
            }
            impl ::subxt::Call for RemoveMember {
                const PALLET: &'static str = "PhragmenElection";
                const FUNCTION: &'static str = "remove_member";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CleanDefunctVoters {
                pub num_voters: u32,
                pub num_defunct: u32,
            }
            impl ::subxt::Call for CleanDefunctVoters {
                const PALLET: &'static str = "PhragmenElection";
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
                    votes: Vec<::subxt::sp_core::crypto::AccountId32>,
                    value: u128,
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
                    candidate_count: u32,
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
                        (),
                    >,
                    has_replacement: bool,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveMember> {
                    let call = RemoveMember {
                        who,
                        has_replacement,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clean_defunct_voters(
                    &self,
                    num_voters: u32,
                    num_defunct: u32,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewTerm(pub Vec<(::subxt::sp_core::crypto::AccountId32, u128)>);
            impl ::subxt::Event for NewTerm {
                const PALLET: &'static str = "PhragmenElection";
                const EVENT: &'static str = "NewTerm";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct EmptyTerm {}
            impl ::subxt::Event for EmptyTerm {
                const PALLET: &'static str = "PhragmenElection";
                const EVENT: &'static str = "EmptyTerm";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ElectionError {}
            impl ::subxt::Event for ElectionError {
                const PALLET: &'static str = "PhragmenElection";
                const EVENT: &'static str = "ElectionError";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MemberKicked(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for MemberKicked {
                const PALLET: &'static str = "PhragmenElection";
                const EVENT: &'static str = "MemberKicked";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Renounced(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Renounced {
                const PALLET: &'static str = "PhragmenElection";
                const EVENT: &'static str = "Renounced";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CandidateSlashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for CandidateSlashed {
                const PALLET: &'static str = "PhragmenElection";
                const EVENT: &'static str = "CandidateSlashed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SeatHolderSlashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for SeatHolderSlashed {
                const PALLET: &'static str = "PhragmenElection";
                const EVENT: &'static str = "SeatHolderSlashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "PhragmenElection";
                const STORAGE: &'static str = "Members";
                type Value = Vec<
                    runtime_types::pallet_elections_phragmen::SeatHolder<
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RunnersUp;
            impl ::subxt::StorageEntry for RunnersUp {
                const PALLET: &'static str = "PhragmenElection";
                const STORAGE: &'static str = "RunnersUp";
                type Value = Vec<
                    runtime_types::pallet_elections_phragmen::SeatHolder<
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Candidates;
            impl ::subxt::StorageEntry for Candidates {
                const PALLET: &'static str = "PhragmenElection";
                const STORAGE: &'static str = "Candidates";
                type Value = Vec<(::subxt::sp_core::crypto::AccountId32, u128)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ElectionRounds;
            impl ::subxt::StorageEntry for ElectionRounds {
                const PALLET: &'static str = "PhragmenElection";
                const STORAGE: &'static str = "ElectionRounds";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Voting(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Voting {
                const PALLET: &'static str = "PhragmenElection";
                const STORAGE: &'static str = "Voting";
                type Value = runtime_types::pallet_elections_phragmen::Voter<
                    ::subxt::sp_core::crypto::AccountId32,
                    u128,
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
                    Vec<
                        runtime_types::pallet_elections_phragmen::SeatHolder<
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
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
                    Vec<
                        runtime_types::pallet_elections_phragmen::SeatHolder<
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
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
                    Vec<(::subxt::sp_core::crypto::AccountId32, u128)>,
                    ::subxt::Error,
                > {
                    let entry = Candidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn election_rounds(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
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
                        u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod technical_membership {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddMember {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for AddMember {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "add_member";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveMember {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for RemoveMember {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "remove_member";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SwapMember {
                pub remove: ::subxt::sp_core::crypto::AccountId32,
                pub add: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SwapMember {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "swap_member";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ResetMembers {
                pub members: Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for ResetMembers {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "reset_members";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ChangeKey {
                pub new: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ChangeKey {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "change_key";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetPrime {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetPrime {
                const PALLET: &'static str = "TechnicalMembership";
                const FUNCTION: &'static str = "set_prime";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                    members: Vec<::subxt::sp_core::crypto::AccountId32>,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MemberAdded {}
            impl ::subxt::Event for MemberAdded {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "MemberAdded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MemberRemoved {}
            impl ::subxt::Event for MemberRemoved {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "MemberRemoved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MembersSwapped {}
            impl ::subxt::Event for MembersSwapped {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "MembersSwapped";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MembersReset {}
            impl ::subxt::Event for MembersReset {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "MembersReset";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct KeyChanged {}
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "TechnicalMembership";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                type Value = Vec<::subxt::sp_core::crypto::AccountId32>;
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
                    Vec<::subxt::sp_core::crypto::AccountId32>,
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
    pub mod treasury {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProposeSpend {
                #[codec(compact)]
                pub value: u128,
                pub beneficiary: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            impl ::subxt::Call for ProposeSpend {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "propose_spend";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RejectProposal {
                #[codec(compact)]
                pub proposal_id: u32,
            }
            impl ::subxt::Call for RejectProposal {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "reject_proposal";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ApproveProposal {
                #[codec(compact)]
                pub proposal_id: u32,
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
                    value: u128,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ProposeSpend> {
                    let call = ProposeSpend { value, beneficiary };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_proposal(
                    &self,
                    proposal_id: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RejectProposal> {
                    let call = RejectProposal { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_proposal(
                    &self,
                    proposal_id: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveProposal> {
                    let call = ApproveProposal { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_treasury::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Proposed(pub u32);
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Spending(pub u128);
            impl ::subxt::Event for Spending {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Spending";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Awarded(
                pub u32,
                pub u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Awarded {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Awarded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Rejected(pub u32, pub u128);
            impl ::subxt::Event for Rejected {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rejected";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Burnt(pub u128);
            impl ::subxt::Event for Burnt {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Burnt";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Rollover(pub u128);
            impl ::subxt::Event for Rollover {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rollover";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Deposit(pub u128);
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Proposals(pub u32);
            impl ::subxt::StorageEntry for Proposals {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "Proposals";
                type Value = runtime_types::pallet_treasury::Proposal<
                    ::subxt::sp_core::crypto::AccountId32,
                    u128,
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
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<u32>;
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
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposals(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_treasury::Proposal<
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Proposals(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn approvals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<u32>,
                    ::subxt::Error,
                > {
                    let entry = Approvals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod claims {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Claim {
                pub dest: ::subxt::sp_core::crypto::AccountId32,
                pub ethereum_signature:
                    runtime_types::polkadot_runtime_common::claims::EcdsaSignature,
            }
            impl ::subxt::Call for Claim {
                const PALLET: &'static str = "Claims";
                const FUNCTION: &'static str = "claim";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MintClaim {
                pub who: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                pub value: u128,
                pub vesting_schedule: Option<(u128, u128, u32)>,
                pub statement:
                    Option<runtime_types::polkadot_runtime_common::claims::StatementKind>,
            }
            impl ::subxt::Call for MintClaim {
                const PALLET: &'static str = "Claims";
                const FUNCTION: &'static str = "mint_claim";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClaimAttest {
                pub dest: ::subxt::sp_core::crypto::AccountId32,
                pub ethereum_signature:
                    runtime_types::polkadot_runtime_common::claims::EcdsaSignature,
                pub statement: Vec<u8>,
            }
            impl ::subxt::Call for ClaimAttest {
                const PALLET: &'static str = "Claims";
                const FUNCTION: &'static str = "claim_attest";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Attest {
                pub statement: Vec<u8>,
            }
            impl ::subxt::Call for Attest {
                const PALLET: &'static str = "Claims";
                const FUNCTION: &'static str = "attest";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MoveClaim {
                pub old: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                pub new: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                pub maybe_preclaim: Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for MoveClaim {
                const PALLET: &'static str = "Claims";
                const FUNCTION: &'static str = "move_claim";
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
                    dest: ::subxt::sp_core::crypto::AccountId32,
                    ethereum_signature : runtime_types :: polkadot_runtime_common :: claims :: EcdsaSignature,
                ) -> ::subxt::SubmittableExtrinsic<T, Claim> {
                    let call = Claim {
                        dest,
                        ethereum_signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint_claim(
                    &self,
                    who: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                    value: u128,
                    vesting_schedule: Option<(u128, u128, u32)>,
                    statement: Option<
                        runtime_types::polkadot_runtime_common::claims::StatementKind,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, MintClaim> {
                    let call = MintClaim {
                        who,
                        value,
                        vesting_schedule,
                        statement,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn claim_attest(
                    &self,
                    dest: ::subxt::sp_core::crypto::AccountId32,
                    ethereum_signature : runtime_types :: polkadot_runtime_common :: claims :: EcdsaSignature,
                    statement: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, ClaimAttest> {
                    let call = ClaimAttest {
                        dest,
                        ethereum_signature,
                        statement,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn attest(
                    &self,
                    statement: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Attest> {
                    let call = Attest { statement };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn move_claim(
                    &self,
                    old: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                    new: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                    maybe_preclaim: Option<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<T, MoveClaim> {
                    let call = MoveClaim {
                        old,
                        new,
                        maybe_preclaim,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::claims::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Claimed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                pub u128,
            );
            impl ::subxt::Event for Claimed {
                const PALLET: &'static str = "Claims";
                const EVENT: &'static str = "Claimed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Claims(
                pub runtime_types::polkadot_runtime_common::claims::EthereumAddress,
            );
            impl ::subxt::StorageEntry for Claims {
                const PALLET: &'static str = "Claims";
                const STORAGE: &'static str = "Claims";
                type Value = u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct Total;
            impl ::subxt::StorageEntry for Total {
                const PALLET: &'static str = "Claims";
                const STORAGE: &'static str = "Total";
                type Value = u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Vesting(
                pub runtime_types::polkadot_runtime_common::claims::EthereumAddress,
            );
            impl ::subxt::StorageEntry for Vesting {
                const PALLET: &'static str = "Claims";
                const STORAGE: &'static str = "Vesting";
                type Value = (u128, u128, u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct Signing(
                pub runtime_types::polkadot_runtime_common::claims::EthereumAddress,
            );
            impl ::subxt::StorageEntry for Signing {
                const PALLET: &'static str = "Claims";
                const STORAGE: &'static str = "Signing";
                type Value =
                    runtime_types::polkadot_runtime_common::claims::StatementKind;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct Preclaims(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Preclaims {
                const PALLET: &'static str = "Claims";
                const STORAGE: &'static str = "Preclaims";
                type Value =
                    runtime_types::polkadot_runtime_common::claims::EthereumAddress;
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
                pub async fn claims(
                    &self,
                    _0: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u128>, ::subxt::Error>
                {
                    let entry = Claims(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn total(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u128, ::subxt::Error> {
                    let entry = Total;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn vesting(
                    &self,
                    _0: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(u128, u128, u32)>,
                    ::subxt::Error,
                > {
                    let entry = Vesting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signing(
                    &self,
                    _0: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_common::claims::StatementKind,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Signing(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn preclaims(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Preclaims(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod vesting {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vest {}
            impl ::subxt::Call for Vest {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct VestOther {
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            impl ::subxt::Call for VestOther {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest_other";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct VestedTransfer {
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub schedule:
                    runtime_types::pallet_vesting::vesting_info::VestingInfo<u128, u32>,
            }
            impl ::subxt::Call for VestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vested_transfer";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceVestedTransfer {
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub schedule:
                    runtime_types::pallet_vesting::vesting_info::VestingInfo<u128, u32>,
            }
            impl ::subxt::Call for ForceVestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "force_vested_transfer";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MergeSchedules {
                pub schedule1_index: u32,
                pub schedule2_index: u32,
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
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, VestOther> {
                    let call = VestOther { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vested_transfer(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        u128,
                        u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, VestedTransfer> {
                    let call = VestedTransfer { target, schedule };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_vested_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        u128,
                        u32,
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
                    schedule1_index: u32,
                    schedule2_index: u32,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct VestingUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for VestingUpdated {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingUpdated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                            u128,
                            u32,
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
                                u128,
                                u32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Vesting(_0);
                    self.client.storage().fetch(&entry, hash).await
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
    pub mod utility {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Batch {
                pub calls: Vec<runtime_types::polkadot_runtime::Call>,
            }
            impl ::subxt::Call for Batch {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AsDerivative {
                pub index: u16,
                pub call: runtime_types::polkadot_runtime::Call,
            }
            impl ::subxt::Call for AsDerivative {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "as_derivative";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BatchAll {
                pub calls: Vec<runtime_types::polkadot_runtime::Call>,
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
                    calls: Vec<runtime_types::polkadot_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, Batch> {
                    let call = Batch { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_derivative(
                    &self,
                    index: u16,
                    call: runtime_types::polkadot_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsDerivative> {
                    let call = AsDerivative { index, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn batch_all(
                    &self,
                    calls: Vec<runtime_types::polkadot_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, BatchAll> {
                    let call = BatchAll { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BatchInterrupted(
                pub u32,
                pub runtime_types::sp_runtime::DispatchError,
            );
            impl ::subxt::Event for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BatchCompleted {}
            impl ::subxt::Event for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ItemCompleted {}
            impl ::subxt::Event for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
        }
    }
    pub mod identity {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddRegistrar {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for AddRegistrar {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "add_registrar";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetIdentity {
                pub info: runtime_types::pallet_identity::types::IdentityInfo,
            }
            impl ::subxt::Call for SetIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_identity";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetSubs {
                pub subs: Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_identity::types::Data,
                )>,
            }
            impl ::subxt::Call for SetSubs {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_subs";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearIdentity {}
            impl ::subxt::Call for ClearIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "clear_identity";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RequestJudgement {
                #[codec(compact)]
                pub reg_index: u32,
                #[codec(compact)]
                pub max_fee: u128,
            }
            impl ::subxt::Call for RequestJudgement {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "request_judgement";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelRequest {
                pub reg_index: u32,
            }
            impl ::subxt::Call for CancelRequest {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "cancel_request";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetFee {
                #[codec(compact)]
                pub index: u32,
                #[codec(compact)]
                pub fee: u128,
            }
            impl ::subxt::Call for SetFee {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_fee";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetAccountId {
                #[codec(compact)]
                pub index: u32,
                pub new: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetAccountId {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_account_id";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetFields {
                #[codec(compact)]
                pub index: u32,
                pub fields: runtime_types::pallet_identity::types::BitFlags<
                    runtime_types::pallet_identity::types::IdentityField,
                >,
            }
            impl ::subxt::Call for SetFields {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_fields";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProvideJudgement {
                #[codec(compact)]
                pub reg_index: u32,
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub judgement: runtime_types::pallet_identity::types::Judgement<u128>,
            }
            impl ::subxt::Call for ProvideJudgement {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "provide_judgement";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct KillIdentity {
                pub target: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            impl ::subxt::Call for KillIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "kill_identity";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddSub {
                pub sub: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            impl ::subxt::Call for AddSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "add_sub";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RenameSub {
                pub sub: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            impl ::subxt::Call for RenameSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "rename_sub";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveSub {
                pub sub: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            impl ::subxt::Call for RemoveSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "remove_sub";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                    subs: Vec<(
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
                    reg_index: u32,
                    max_fee: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, RequestJudgement> {
                    let call = RequestJudgement { reg_index, max_fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_request(
                    &self,
                    reg_index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelRequest> {
                    let call = CancelRequest { reg_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_fee(
                    &self,
                    index: u32,
                    fee: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetFee> {
                    let call = SetFee { index, fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_account_id(
                    &self,
                    index: u32,
                    new: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetAccountId> {
                    let call = SetAccountId { index, new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_fields(
                    &self,
                    index: u32,
                    fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetFields> {
                    let call = SetFields { index, fields };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn provide_judgement(
                    &self,
                    reg_index: u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    judgement: runtime_types::pallet_identity::types::Judgement<u128>,
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
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, KillIdentity> {
                    let call = KillIdentity { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
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
                        (),
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
                        (),
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct IdentitySet(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for IdentitySet {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentitySet";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct IdentityCleared(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for IdentityCleared {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityCleared";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct IdentityKilled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for IdentityKilled {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityKilled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct JudgementRequested(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u32,
            );
            impl ::subxt::Event for JudgementRequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementRequested";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct JudgementUnrequested(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u32,
            );
            impl ::subxt::Event for JudgementUnrequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementUnrequested";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct JudgementGiven(pub ::subxt::sp_core::crypto::AccountId32, pub u32);
            impl ::subxt::Event for JudgementGiven {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementGiven";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RegistrarAdded(pub u32);
            impl ::subxt::Event for RegistrarAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "RegistrarAdded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SubIdentityAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for SubIdentityAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityAdded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SubIdentityRemoved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for SubIdentityRemoved {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRemoved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SubIdentityRevoked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
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
                type Value = runtime_types::pallet_identity::types::Registration<u128>;
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
                    u128,
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
                        Option<
                            runtime_types::pallet_identity::types::RegistrarInfo<
                                u128,
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
                        runtime_types::pallet_identity::types::Registration<u128>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = IdentityOf(_0);
                    self.client.storage().fetch(&entry, hash).await
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
                pub async fn subs_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    ),
                    ::subxt::Error,
                > {
                    let entry = SubsOf(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn registrars(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        Option<
                            runtime_types::pallet_identity::types::RegistrarInfo<
                                u128,
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
    pub mod proxy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Proxy {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type: Option<runtime_types::polkadot_runtime::ProxyType>,
                pub call: runtime_types::polkadot_runtime::Call,
            }
            impl ::subxt::Call for Proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddProxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::polkadot_runtime::ProxyType,
                pub delay: u32,
            }
            impl ::subxt::Call for AddProxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "add_proxy";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveProxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::polkadot_runtime::ProxyType,
                pub delay: u32,
            }
            impl ::subxt::Call for RemoveProxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxy";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveProxies {}
            impl ::subxt::Call for RemoveProxies {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxies";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Anonymous {
                pub proxy_type: runtime_types::polkadot_runtime::ProxyType,
                pub delay: u32,
                pub index: u16,
            }
            impl ::subxt::Call for Anonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "anonymous";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct KillAnonymous {
                pub spawner: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::polkadot_runtime::ProxyType,
                pub index: u16,
                #[codec(compact)]
                pub height: u32,
                #[codec(compact)]
                pub ext_index: u32,
            }
            impl ::subxt::Call for KillAnonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "kill_anonymous";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Announce {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for Announce {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "announce";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveAnnouncement {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RemoveAnnouncement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_announcement";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RejectAnnouncement {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RejectAnnouncement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "reject_announcement";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProxyAnnounced {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type: Option<runtime_types::polkadot_runtime::ProxyType>,
                pub call: runtime_types::polkadot_runtime::Call,
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
                    force_proxy_type: Option<runtime_types::polkadot_runtime::ProxyType>,
                    call: runtime_types::polkadot_runtime::Call,
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
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    delay: u32,
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
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    delay: u32,
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
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    delay: u32,
                    index: u16,
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
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    index: u16,
                    height: u32,
                    ext_index: u32,
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
                    force_proxy_type: Option<runtime_types::polkadot_runtime::ProxyType>,
                    call: runtime_types::polkadot_runtime::Call,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProxyExecuted(
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for ProxyExecuted {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyExecuted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AnonymousCreated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_runtime::ProxyType,
                pub u16,
            );
            impl ::subxt::Event for AnonymousCreated {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "AnonymousCreated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Announced(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Announced {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "Announced";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProxyAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_runtime::ProxyType,
                pub u32,
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
                            runtime_types::polkadot_runtime::ProxyType,
                            u32,
                        >,
                    >,
                    u128,
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
                            u32,
                        >,
                    >,
                    u128,
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
                                runtime_types::polkadot_runtime::ProxyType,
                                u32,
                            >,
                        >,
                        u128,
                    ),
                    ::subxt::Error,
                > {
                    let entry = Proxies(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                                u32,
                            >,
                        >,
                        u128,
                    ),
                    ::subxt::Error,
                > {
                    let entry = Announcements(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod multisig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AsMultiThreshold1 {
                pub other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: runtime_types::polkadot_runtime::Call,
            }
            impl ::subxt::Call for AsMultiThreshold1 {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi_threshold1";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AsMulti {
                pub threshold: u16,
                pub other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint:
                    Option<runtime_types::pallet_multisig::Timepoint<u32>>,
                pub call: Vec<u8>,
                pub store_call: bool,
                pub max_weight: u64,
            }
            impl ::subxt::Call for AsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ApproveAsMulti {
                pub threshold: u16,
                pub other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint:
                    Option<runtime_types::pallet_multisig::Timepoint<u32>>,
                pub call_hash: [u8; 32usize],
                pub max_weight: u64,
            }
            impl ::subxt::Call for ApproveAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelAsMulti {
                pub threshold: u16,
                pub other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<u32>,
                pub call_hash: [u8; 32usize],
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
                    other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                    call: runtime_types::polkadot_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsMultiThreshold1> {
                    let call = AsMultiThreshold1 {
                        other_signatories,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_multi(
                    &self,
                    threshold: u16,
                    other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: Option<
                        runtime_types::pallet_multisig::Timepoint<u32>,
                    >,
                    call: Vec<u8>,
                    store_call: bool,
                    max_weight: u64,
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
                    threshold: u16,
                    other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: Option<
                        runtime_types::pallet_multisig::Timepoint<u32>,
                    >,
                    call_hash: [u8; 32usize],
                    max_weight: u64,
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
                    threshold: u16,
                    other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<u32>,
                    call_hash: [u8; 32usize],
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewMultisig(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [u8; 32usize],
            );
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MultisigApproval(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [u8; 32usize],
            );
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MultisigExecuted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [u8; 32usize],
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MultisigCancelled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [u8; 32usize],
            );
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(::subxt::sp_core::crypto::AccountId32, [u8; 32usize]);
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    u32,
                    u128,
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
            pub struct Calls(pub [u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Calls";
                type Value = (Vec<u8>, ::subxt::sp_core::crypto::AccountId32, u128);
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
                    _1: [u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            u32,
                            u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        Vec<u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod bounties {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProposeBounty {
                #[codec(compact)]
                pub value: u128,
                pub description: Vec<u8>,
            }
            impl ::subxt::Call for ProposeBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "propose_bounty";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ApproveBounty {
                #[codec(compact)]
                pub bounty_id: u32,
            }
            impl ::subxt::Call for ApproveBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "approve_bounty";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProposeCurator {
                #[codec(compact)]
                pub bounty_id: u32,
                pub curator: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub fee: u128,
            }
            impl ::subxt::Call for ProposeCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "propose_curator";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct UnassignCurator {
                #[codec(compact)]
                pub bounty_id: u32,
            }
            impl ::subxt::Call for UnassignCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "unassign_curator";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AcceptCurator {
                #[codec(compact)]
                pub bounty_id: u32,
            }
            impl ::subxt::Call for AcceptCurator {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "accept_curator";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AwardBounty {
                #[codec(compact)]
                pub bounty_id: u32,
                pub beneficiary: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            impl ::subxt::Call for AwardBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "award_bounty";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClaimBounty {
                #[codec(compact)]
                pub bounty_id: u32,
            }
            impl ::subxt::Call for ClaimBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "claim_bounty";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CloseBounty {
                #[codec(compact)]
                pub bounty_id: u32,
            }
            impl ::subxt::Call for CloseBounty {
                const PALLET: &'static str = "Bounties";
                const FUNCTION: &'static str = "close_bounty";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExtendBountyExpiry {
                #[codec(compact)]
                pub bounty_id: u32,
                pub remark: Vec<u8>,
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
                    value: u128,
                    description: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, ProposeBounty> {
                    let call = ProposeBounty { value, description };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_bounty(
                    &self,
                    bounty_id: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveBounty> {
                    let call = ApproveBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose_curator(
                    &self,
                    bounty_id: u32,
                    curator: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    fee: u128,
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
                    bounty_id: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, UnassignCurator> {
                    let call = UnassignCurator { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn accept_curator(
                    &self,
                    bounty_id: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, AcceptCurator> {
                    let call = AcceptCurator { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn award_bounty(
                    &self,
                    bounty_id: u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
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
                    bounty_id: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ClaimBounty> {
                    let call = ClaimBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close_bounty(
                    &self,
                    bounty_id: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CloseBounty> {
                    let call = CloseBounty { bounty_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn extend_bounty_expiry(
                    &self,
                    bounty_id: u32,
                    remark: Vec<u8>,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BountyProposed(pub u32);
            impl ::subxt::Event for BountyProposed {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyProposed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BountyRejected(pub u32, pub u128);
            impl ::subxt::Event for BountyRejected {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyRejected";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BountyBecameActive(pub u32);
            impl ::subxt::Event for BountyBecameActive {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyBecameActive";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BountyAwarded(pub u32, pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for BountyAwarded {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyAwarded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BountyClaimed(
                pub u32,
                pub u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for BountyClaimed {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyClaimed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BountyCanceled(pub u32);
            impl ::subxt::Event for BountyCanceled {
                const PALLET: &'static str = "Bounties";
                const EVENT: &'static str = "BountyCanceled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BountyExtended(pub u32);
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Bounties(pub u32);
            impl ::subxt::StorageEntry for Bounties {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "Bounties";
                type Value = runtime_types::pallet_bounties::Bounty<
                    ::subxt::sp_core::crypto::AccountId32,
                    u128,
                    u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BountyDescriptions(pub u32);
            impl ::subxt::StorageEntry for BountyDescriptions {
                const PALLET: &'static str = "Bounties";
                const STORAGE: &'static str = "BountyDescriptions";
                type Value = Vec<u8>;
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
                type Value = Vec<u32>;
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
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = BountyCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bounties(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_bounties::Bounty<
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
                            u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Bounties(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bounty_descriptions(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<Vec<u8>>, ::subxt::Error>
                {
                    let entry = BountyDescriptions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bounty_approvals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Vec<u32>, ::subxt::Error> {
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportAwesome {
                pub reason: Vec<u8>,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ReportAwesome {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "report_awesome";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RetractTip {
                pub hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RetractTip {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "retract_tip";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct TipNew {
                pub reason: Vec<u8>,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                #[codec(compact)]
                pub tip_value: u128,
            }
            impl ::subxt::Call for TipNew {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "tip_new";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Tip {
                pub hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub tip_value: u128,
            }
            impl ::subxt::Call for Tip {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "tip";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CloseTip {
                pub hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for CloseTip {
                const PALLET: &'static str = "Tips";
                const FUNCTION: &'static str = "close_tip";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                    reason: Vec<u8>,
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
                    reason: Vec<u8>,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    tip_value: u128,
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
                    tip_value: u128,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewTip(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for NewTip {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "NewTip";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct TipClosing(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for TipClosing {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "TipClosing";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct TipClosed(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for TipClosed {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "TipClosed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct TipRetracted(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for TipRetracted {
                const PALLET: &'static str = "Tips";
                const EVENT: &'static str = "TipRetracted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct TipSlashed(
                pub ::subxt::sp_core::H256,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
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
                    u128,
                    u32,
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
                type Value = Vec<u8>;
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
                            u128,
                            u32,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Tips(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reasons(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<Vec<u8>>, ::subxt::Error>
                {
                    let entry = Reasons(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod election_provider_multi_phase {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SubmitUnsigned { pub raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: polkadot_runtime :: NposCompactSolution16 > , pub witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize }
            impl ::subxt::Call for SubmitUnsigned {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit_unsigned";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMinimumUntrustedScore {
                pub maybe_next_score: Option<[u128; 3usize]>,
            }
            impl ::subxt::Call for SetMinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_minimum_untrusted_score";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetEmergencyElectionResult {
                pub supports: Vec<(
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Submit {
                pub raw_solution:
                    runtime_types::pallet_election_provider_multi_phase::RawSolution<
                        runtime_types::polkadot_runtime::NposCompactSolution16,
                    >,
                pub num_signed_submissions: u32,
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
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: polkadot_runtime :: NposCompactSolution16 >,
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
                    maybe_next_score: Option<[u128; 3usize]>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMinimumUntrustedScore>
                {
                    let call = SetMinimumUntrustedScore { maybe_next_score };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_emergency_election_result(
                    &self,
                    supports: Vec<(
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
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: polkadot_runtime :: NposCompactSolution16 >,
                    num_signed_submissions: u32,
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SolutionStored(
                pub runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                pub bool,
            );
            impl ::subxt::Event for SolutionStored {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SolutionStored";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ElectionFinalized(
                pub  Option<
                    runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                >,
            );
            impl ::subxt::Event for ElectionFinalized {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "ElectionFinalized";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Rewarded(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Slashed(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SignedPhaseStarted(pub u32);
            impl ::subxt::Event for SignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SignedPhaseStarted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct UnsignedPhaseStarted(pub u32);
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPhase;
            impl ::subxt::StorageEntry for CurrentPhase {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "CurrentPhase";
                type Value =
                    runtime_types::pallet_election_provider_multi_phase::Phase<u32>;
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
                type Value = u32;
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
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionIndices;
            impl ::subxt::StorageEntry for SignedSubmissionIndices {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionIndices";
                type Value = runtime_types :: frame_support :: storage :: bounded_btree_map :: BoundedBTreeMap < [u128 ; 3usize] , u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionsMap(pub u32);
            impl ::subxt::StorageEntry for SignedSubmissionsMap {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionsMap";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , u128 , runtime_types :: polkadot_runtime :: NposCompactSolution16 > ;
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
                type Value = [u128; 3usize];
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
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = Round;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_election_provider_multi_phase::Phase<u32>,
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
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = DesiredTargets;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn snapshot_metadata (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize > , :: subxt :: Error >{
                    let entry = SnapshotMetadata;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_submission_next_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = SignedSubmissionNextIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submission_indices (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_btree_map :: BoundedBTreeMap < [u128 ; 3usize] , u32 > , :: subxt :: Error >{
                    let entry = SignedSubmissionIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submissions_map (& self , _0 : u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , u128 , runtime_types :: polkadot_runtime :: NposCompactSolution16 > , :: subxt :: Error >{
                    let entry = SignedSubmissionsMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn minimum_untrusted_score(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[u128; 3usize]>,
                    ::subxt::Error,
                > {
                    let entry = MinimumUntrustedScore;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod parachains_origin {
        use super::runtime_types;
    }
    pub mod configuration {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetValidationUpgradeFrequency {
                pub new: u32,
            }
            impl ::subxt::Call for SetValidationUpgradeFrequency {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_validation_upgrade_frequency";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetValidationUpgradeDelay {
                pub new: u32,
            }
            impl ::subxt::Call for SetValidationUpgradeDelay {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_validation_upgrade_delay";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetCodeRetentionPeriod {
                pub new: u32,
            }
            impl ::subxt::Call for SetCodeRetentionPeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_code_retention_period";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxCodeSize {
                pub new: u32,
            }
            impl ::subxt::Call for SetMaxCodeSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_code_size";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxPovSize {
                pub new: u32,
            }
            impl ::subxt::Call for SetMaxPovSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_pov_size";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxHeadDataSize {
                pub new: u32,
            }
            impl ::subxt::Call for SetMaxHeadDataSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_head_data_size";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetParathreadCores {
                pub new: u32,
            }
            impl ::subxt::Call for SetParathreadCores {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_parathread_cores";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetParathreadRetries {
                pub new: u32,
            }
            impl ::subxt::Call for SetParathreadRetries {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_parathread_retries";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetGroupRotationFrequency {
                pub new: u32,
            }
            impl ::subxt::Call for SetGroupRotationFrequency {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_group_rotation_frequency";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetChainAvailabilityPeriod {
                pub new: u32,
            }
            impl ::subxt::Call for SetChainAvailabilityPeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_chain_availability_period";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetThreadAvailabilityPeriod {
                pub new: u32,
            }
            impl ::subxt::Call for SetThreadAvailabilityPeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_thread_availability_period";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetSchedulingLookahead {
                pub new: u32,
            }
            impl ::subxt::Call for SetSchedulingLookahead {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_scheduling_lookahead";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxValidatorsPerCore {
                pub new: Option<u32>,
            }
            impl ::subxt::Call for SetMaxValidatorsPerCore {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_validators_per_core";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxValidators {
                pub new: Option<u32>,
            }
            impl ::subxt::Call for SetMaxValidators {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_validators";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetDisputePeriod {
                pub new: u32,
            }
            impl ::subxt::Call for SetDisputePeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_period";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetDisputePostConclusionAcceptancePeriod {
                pub new: u32,
            }
            impl ::subxt::Call for SetDisputePostConclusionAcceptancePeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str =
                    "set_dispute_post_conclusion_acceptance_period";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetDisputeMaxSpamSlots {
                pub new: u32,
            }
            impl ::subxt::Call for SetDisputeMaxSpamSlots {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_max_spam_slots";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetDisputeConclusionByTimeOutPeriod {
                pub new: u32,
            }
            impl ::subxt::Call for SetDisputeConclusionByTimeOutPeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str =
                    "set_dispute_conclusion_by_time_out_period";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetNoShowSlots {
                pub new: u32,
            }
            impl ::subxt::Call for SetNoShowSlots {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_no_show_slots";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetNDelayTranches {
                pub new: u32,
            }
            impl ::subxt::Call for SetNDelayTranches {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_n_delay_tranches";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetZerothDelayTrancheWidth {
                pub new: u32,
            }
            impl ::subxt::Call for SetZerothDelayTrancheWidth {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_zeroth_delay_tranche_width";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetNeededApprovals {
                pub new: u32,
            }
            impl ::subxt::Call for SetNeededApprovals {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_needed_approvals";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetRelayVrfModuloSamples {
                pub new: u32,
            }
            impl ::subxt::Call for SetRelayVrfModuloSamples {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_relay_vrf_modulo_samples";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxUpwardQueueCount {
                pub new: u32,
            }
            impl ::subxt::Call for SetMaxUpwardQueueCount {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_queue_count";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxUpwardQueueSize {
                pub new: u32,
            }
            impl ::subxt::Call for SetMaxUpwardQueueSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_queue_size";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxDownwardMessageSize {
                pub new: u32,
            }
            impl ::subxt::Call for SetMaxDownwardMessageSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_downward_message_size";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetUmpServiceTotalWeight {
                pub new: u64,
            }
            impl ::subxt::Call for SetUmpServiceTotalWeight {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_ump_service_total_weight";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxUpwardMessageSize {
                pub new: u32,
            }
            impl ::subxt::Call for SetMaxUpwardMessageSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_message_size";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxUpwardMessageNumPerCandidate {
                pub new: u32,
            }
            impl ::subxt::Call for SetMaxUpwardMessageNumPerCandidate {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_message_num_per_candidate";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpOpenRequestTtl {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpOpenRequestTtl {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_open_request_ttl";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpSenderDeposit {
                pub new: u128,
            }
            impl ::subxt::Call for SetHrmpSenderDeposit {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_sender_deposit";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpRecipientDeposit {
                pub new: u128,
            }
            impl ::subxt::Call for SetHrmpRecipientDeposit {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_recipient_deposit";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpChannelMaxCapacity {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpChannelMaxCapacity {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_capacity";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpChannelMaxTotalSize {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpChannelMaxTotalSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_total_size";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpMaxParachainInboundChannels {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpMaxParachainInboundChannels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parachain_inbound_channels";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpMaxParathreadInboundChannels {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpMaxParathreadInboundChannels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parathread_inbound_channels";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpChannelMaxMessageSize {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpChannelMaxMessageSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_message_size";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpMaxParachainOutboundChannels {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpMaxParachainOutboundChannels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parachain_outbound_channels";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpMaxParathreadOutboundChannels {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpMaxParathreadOutboundChannels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str =
                    "set_hrmp_max_parathread_outbound_channels";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHrmpMaxMessageNumPerCandidate {
                pub new: u32,
            }
            impl ::subxt::Call for SetHrmpMaxMessageNumPerCandidate {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_message_num_per_candidate";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetUmpMaxIndividualWeight {
                pub new: u64,
            }
            impl ::subxt::Call for SetUmpMaxIndividualWeight {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_ump_max_individual_weight";
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
                pub fn set_validation_upgrade_frequency(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetValidationUpgradeFrequency>
                {
                    let call = SetValidationUpgradeFrequency { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validation_upgrade_delay(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetValidationUpgradeDelay>
                {
                    let call = SetValidationUpgradeDelay { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_retention_period(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCodeRetentionPeriod>
                {
                    let call = SetCodeRetentionPeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_code_size(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxCodeSize> {
                    let call = SetMaxCodeSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_pov_size(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxPovSize> {
                    let call = SetMaxPovSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_head_data_size(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxHeadDataSize>
                {
                    let call = SetMaxHeadDataSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_parathread_cores(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetParathreadCores>
                {
                    let call = SetParathreadCores { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_parathread_retries(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetParathreadRetries>
                {
                    let call = SetParathreadRetries { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_group_rotation_frequency(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetGroupRotationFrequency>
                {
                    let call = SetGroupRotationFrequency { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_chain_availability_period(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetChainAvailabilityPeriod>
                {
                    let call = SetChainAvailabilityPeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_thread_availability_period(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetThreadAvailabilityPeriod>
                {
                    let call = SetThreadAvailabilityPeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_scheduling_lookahead(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetSchedulingLookahead>
                {
                    let call = SetSchedulingLookahead { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_validators_per_core(
                    &self,
                    new: Option<u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxValidatorsPerCore>
                {
                    let call = SetMaxValidatorsPerCore { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_validators(
                    &self,
                    new: Option<u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxValidators> {
                    let call = SetMaxValidators { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_period(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetDisputePeriod> {
                    let call = SetDisputePeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_post_conclusion_acceptance_period(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    T,
                    SetDisputePostConclusionAcceptancePeriod,
                > {
                    let call = SetDisputePostConclusionAcceptancePeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_max_spam_slots(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetDisputeMaxSpamSlots>
                {
                    let call = SetDisputeMaxSpamSlots { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_conclusion_by_time_out_period(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetDisputeConclusionByTimeOutPeriod>
                {
                    let call = SetDisputeConclusionByTimeOutPeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_no_show_slots(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetNoShowSlots> {
                    let call = SetNoShowSlots { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_n_delay_tranches(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetNDelayTranches> {
                    let call = SetNDelayTranches { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_zeroth_delay_tranche_width(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetZerothDelayTrancheWidth>
                {
                    let call = SetZerothDelayTrancheWidth { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_needed_approvals(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetNeededApprovals>
                {
                    let call = SetNeededApprovals { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_relay_vrf_modulo_samples(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetRelayVrfModuloSamples>
                {
                    let call = SetRelayVrfModuloSamples { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_queue_count(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxUpwardQueueCount>
                {
                    let call = SetMaxUpwardQueueCount { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_queue_size(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxUpwardQueueSize>
                {
                    let call = SetMaxUpwardQueueSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_downward_message_size(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxDownwardMessageSize>
                {
                    let call = SetMaxDownwardMessageSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_ump_service_total_weight(
                    &self,
                    new: u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SetUmpServiceTotalWeight>
                {
                    let call = SetUmpServiceTotalWeight { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_message_size(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxUpwardMessageSize>
                {
                    let call = SetMaxUpwardMessageSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_message_num_per_candidate(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxUpwardMessageNumPerCandidate>
                {
                    let call = SetMaxUpwardMessageNumPerCandidate { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_open_request_ttl(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpOpenRequestTtl>
                {
                    let call = SetHrmpOpenRequestTtl { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_sender_deposit(
                    &self,
                    new: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpSenderDeposit>
                {
                    let call = SetHrmpSenderDeposit { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_recipient_deposit(
                    &self,
                    new: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpRecipientDeposit>
                {
                    let call = SetHrmpRecipientDeposit { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_capacity(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpChannelMaxCapacity>
                {
                    let call = SetHrmpChannelMaxCapacity { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_total_size(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpChannelMaxTotalSize>
                {
                    let call = SetHrmpChannelMaxTotalSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parachain_inbound_channels(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxParachainInboundChannels>
                {
                    let call = SetHrmpMaxParachainInboundChannels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parathread_inbound_channels(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxParathreadInboundChannels>
                {
                    let call = SetHrmpMaxParathreadInboundChannels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_message_size(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpChannelMaxMessageSize>
                {
                    let call = SetHrmpChannelMaxMessageSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parachain_outbound_channels(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxParachainOutboundChannels>
                {
                    let call = SetHrmpMaxParachainOutboundChannels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parathread_outbound_channels(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxParathreadOutboundChannels>
                {
                    let call = SetHrmpMaxParathreadOutboundChannels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_message_num_per_candidate(
                    &self,
                    new: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxMessageNumPerCandidate>
                {
                    let call = SetHrmpMaxMessageNumPerCandidate { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_ump_max_individual_weight(
                    &self,
                    new: u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SetUmpMaxIndividualWeight>
                {
                    let call = SetUmpMaxIndividualWeight { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ActiveConfig;
            impl ::subxt::StorageEntry for ActiveConfig {
                const PALLET: &'static str = "Configuration";
                const STORAGE: &'static str = "ActiveConfig";
                type Value = runtime_types :: polkadot_runtime_parachains :: configuration :: HostConfiguration < u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingConfig(pub u32);
            impl ::subxt::StorageEntry for PendingConfig {
                const PALLET: &'static str = "Configuration";
                const STORAGE: &'static str = "PendingConfig";
                type Value = runtime_types :: polkadot_runtime_parachains :: configuration :: HostConfiguration < u32 > ;
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
                }                pub async fn active_config (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: polkadot_runtime_parachains :: configuration :: HostConfiguration < u32 > , :: subxt :: Error >{
                    let entry = ActiveConfig;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn pending_config (& self , _0 : u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: configuration :: HostConfiguration < u32 > > , :: subxt :: Error >{
                    let entry = PendingConfig(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod paras_shared {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
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
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct CurrentSessionIndex;
            impl ::subxt::StorageEntry for CurrentSessionIndex {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "CurrentSessionIndex";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveValidatorIndices;
            impl ::subxt::StorageEntry for ActiveValidatorIndices {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "ActiveValidatorIndices";
                type Value = Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveValidatorKeys;
            impl ::subxt::StorageEntry for ActiveValidatorKeys {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "ActiveValidatorKeys";
                type Value =
                    Vec<runtime_types::polkadot_primitives::v0::validator_app::Public>;
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
                pub async fn current_session_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = CurrentSessionIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn active_validator_indices(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                    ::subxt::Error,
                > {
                    let entry = ActiveValidatorIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn active_validator_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_primitives::v0::validator_app::Public>,
                    ::subxt::Error,
                > {
                    let entry = ActiveValidatorKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod para_inclusion {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
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
            }
        }
        pub type Event =
            runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CandidateBacked(
                pub  runtime_types::polkadot_primitives::v1::CandidateReceipt<
                    ::subxt::sp_core::H256,
                >,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
                pub runtime_types::polkadot_primitives::v1::GroupIndex,
            );
            impl ::subxt::Event for CandidateBacked {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateBacked";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CandidateIncluded(
                pub  runtime_types::polkadot_primitives::v1::CandidateReceipt<
                    ::subxt::sp_core::H256,
                >,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
                pub runtime_types::polkadot_primitives::v1::GroupIndex,
            );
            impl ::subxt::Event for CandidateIncluded {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateIncluded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CandidateTimedOut(
                pub  runtime_types::polkadot_primitives::v1::CandidateReceipt<
                    ::subxt::sp_core::H256,
                >,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
            );
            impl ::subxt::Event for CandidateTimedOut {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateTimedOut";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AvailabilityBitfields(
                pub runtime_types::polkadot_primitives::v0::ValidatorIndex,
            );
            impl ::subxt::StorageEntry for AvailabilityBitfields {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "AvailabilityBitfields";
                type Value = runtime_types :: polkadot_runtime_parachains :: inclusion :: AvailabilityBitfieldRecord < u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PendingAvailability(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for PendingAvailability {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "PendingAvailability";
                type Value = runtime_types :: polkadot_runtime_parachains :: inclusion :: CandidatePendingAvailability < :: subxt :: sp_core :: H256 , u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PendingAvailabilityCommitments(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for PendingAvailabilityCommitments {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "PendingAvailabilityCommitments";
                type Value =
                    runtime_types::polkadot_primitives::v1::CandidateCommitments<u32>;
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
                }                pub async fn availability_bitfields (& self , _0 : runtime_types :: polkadot_primitives :: v0 :: ValidatorIndex , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: inclusion :: AvailabilityBitfieldRecord < u32 > > , :: subxt :: Error >{
                    let entry = AvailabilityBitfields(_0);
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn pending_availability (& self , _0 : runtime_types :: polkadot_parachain :: primitives :: Id , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: inclusion :: CandidatePendingAvailability < :: subxt :: sp_core :: H256 , u32 > > , :: subxt :: Error >{
                    let entry = PendingAvailability(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_availability_commitments(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::CandidateCommitments<u32>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingAvailabilityCommitments(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod para_inherent {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Enter {
                pub data: runtime_types::polkadot_primitives::v1::InherentData<
                    runtime_types::sp_runtime::generic::header::Header<
                        u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for Enter {
                const PALLET: &'static str = "ParaInherent";
                const FUNCTION: &'static str = "enter";
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
                pub fn enter(
                    &self,
                    data: runtime_types::polkadot_primitives::v1::InherentData<
                        runtime_types::sp_runtime::generic::header::Header<
                            u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Enter> {
                    let call = Enter { data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Included;
            impl ::subxt::StorageEntry for Included {
                const PALLET: &'static str = "ParaInherent";
                const STORAGE: &'static str = "Included";
                type Value = ();
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
                pub async fn included(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::Error>
                {
                    let entry = Included;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod para_scheduler {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct ValidatorGroups;
            impl ::subxt::StorageEntry for ValidatorGroups {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ValidatorGroups";
                type Value =
                    Vec<Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParathreadQueue;
            impl ::subxt::StorageEntry for ParathreadQueue {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ParathreadQueue";
                type Value = runtime_types :: polkadot_runtime_parachains :: scheduler :: ParathreadClaimQueue ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AvailabilityCores;
            impl ::subxt::StorageEntry for AvailabilityCores {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "AvailabilityCores";
                type Value =
                    Vec<Option<runtime_types::polkadot_primitives::v1::CoreOccupied>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParathreadClaimIndex;
            impl ::subxt::StorageEntry for ParathreadClaimIndex {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ParathreadClaimIndex";
                type Value = Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SessionStartBlock;
            impl ::subxt::StorageEntry for SessionStartBlock {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "SessionStartBlock";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Scheduled;
            impl ::subxt::StorageEntry for Scheduled {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "Scheduled";
                type Value = Vec<
                    runtime_types::polkadot_runtime_parachains::scheduler::CoreAssignment,
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
                pub async fn validator_groups(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>>,
                    ::subxt::Error,
                > {
                    let entry = ValidatorGroups;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn parathread_queue (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: polkadot_runtime_parachains :: scheduler :: ParathreadClaimQueue , :: subxt :: Error >{
                    let entry = ParathreadQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn availability_cores(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<Option<runtime_types::polkadot_primitives::v1::CoreOccupied>>,
                    ::subxt::Error,
                > {
                    let entry = AvailabilityCores;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parathread_claim_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = ParathreadClaimIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn session_start_block(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = SessionStartBlock;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn scheduled (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < Vec < runtime_types :: polkadot_runtime_parachains :: scheduler :: CoreAssignment > , :: subxt :: Error >{
                    let entry = Scheduled;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod paras {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceSetCurrentCode {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_code:
                    runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for ForceSetCurrentCode {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_set_current_code";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceSetCurrentHead {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
            }
            impl ::subxt::Call for ForceSetCurrentHead {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_set_current_head";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceScheduleCodeUpgrade {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_code:
                    runtime_types::polkadot_parachain::primitives::ValidationCode,
                pub relay_parent_number: u32,
            }
            impl ::subxt::Call for ForceScheduleCodeUpgrade {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_schedule_code_upgrade";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceNoteNewHead {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
            }
            impl ::subxt::Call for ForceNoteNewHead {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_note_new_head";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceQueueAction {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for ForceQueueAction {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_queue_action";
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
                pub fn force_set_current_code(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_code : runtime_types :: polkadot_parachain :: primitives :: ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceSetCurrentCode>
                {
                    let call = ForceSetCurrentCode { para, new_code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_current_head(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceSetCurrentHead>
                {
                    let call = ForceSetCurrentHead { para, new_head };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_schedule_code_upgrade(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_code : runtime_types :: polkadot_parachain :: primitives :: ValidationCode,
                    relay_parent_number: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceScheduleCodeUpgrade>
                {
                    let call = ForceScheduleCodeUpgrade {
                        para,
                        new_code,
                        relay_parent_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_note_new_head(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceNoteNewHead> {
                    let call = ForceNoteNewHead { para, new_head };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_queue_action(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceQueueAction> {
                    let call = ForceQueueAction { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::paras::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CurrentCodeUpdated(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for CurrentCodeUpdated {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CurrentCodeUpdated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CurrentHeadUpdated(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for CurrentHeadUpdated {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CurrentHeadUpdated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CodeUpgradeScheduled(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for CodeUpgradeScheduled {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CodeUpgradeScheduled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewHeadNoted(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for NewHeadNoted {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "NewHeadNoted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ActionQueued(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub u32,
            );
            impl ::subxt::Event for ActionQueued {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "ActionQueued";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Parachains;
            impl ::subxt::StorageEntry for Parachains {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "Parachains";
                type Value = Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParaLifecycles(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for ParaLifecycles {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "ParaLifecycles";
                type Value =
                    runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Heads(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Heads {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "Heads";
                type Value = runtime_types::polkadot_parachain::primitives::HeadData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CurrentCodeHash(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for CurrentCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CurrentCodeHash";
                type Value =
                    runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodeHash(
                runtime_types::polkadot_parachain::primitives::Id,
                u32,
            );
            impl ::subxt::StorageEntry for PastCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodeHash";
                type Value =
                    runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodeMeta(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for PastCodeMeta {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodeMeta";
                type Value =
                    runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
                        u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodePruning;
            impl ::subxt::StorageEntry for PastCodePruning {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodePruning";
                type Value =
                    Vec<(runtime_types::polkadot_parachain::primitives::Id, u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct FutureCodeUpgrades(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for FutureCodeUpgrades {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "FutureCodeUpgrades";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct FutureCodeHash(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for FutureCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "FutureCodeHash";
                type Value =
                    runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeGoAheadSignal(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for UpgradeGoAheadSignal {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeGoAheadSignal";
                type Value = runtime_types::polkadot_primitives::v1::UpgradeGoAhead;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeRestrictionSignal(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for UpgradeRestrictionSignal {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeRestrictionSignal";
                type Value = runtime_types::polkadot_primitives::v1::UpgradeRestriction;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeCooldowns;
            impl ::subxt::StorageEntry for UpgradeCooldowns {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeCooldowns";
                type Value =
                    Vec<(runtime_types::polkadot_parachain::primitives::Id, u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpcomingUpgrades;
            impl ::subxt::StorageEntry for UpcomingUpgrades {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpcomingUpgrades";
                type Value =
                    Vec<(runtime_types::polkadot_parachain::primitives::Id, u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActionsQueue(pub u32);
            impl ::subxt::StorageEntry for ActionsQueue {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "ActionsQueue";
                type Value = Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpcomingParasGenesis(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for UpcomingParasGenesis {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpcomingParasGenesis";
                type Value =
                    runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CodeByHashRefs(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            );
            impl ::subxt::StorageEntry for CodeByHashRefs {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CodeByHashRefs";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CodeByHash(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            );
            impl ::subxt::StorageEntry for CodeByHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CodeByHash";
                type Value =
                    runtime_types::polkadot_parachain::primitives::ValidationCode;
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
                pub async fn parachains(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = Parachains;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn para_lifecycles(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ParaLifecycles(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn heads(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::HeadData,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Heads(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentCodeHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn past_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    _1: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PastCodeHash(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn past_code_meta(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
                        u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PastCodeMeta(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn past_code_pruning(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<(runtime_types::polkadot_parachain::primitives::Id, u32)>,
                    ::subxt::Error,
                > {
                    let entry = PastCodePruning;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn future_code_upgrades(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = FutureCodeUpgrades(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn future_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::Error,
                > {
                    let entry = FutureCodeHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgrade_go_ahead_signal(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::UpgradeGoAhead,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UpgradeGoAheadSignal(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgrade_restriction_signal(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::UpgradeRestriction,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UpgradeRestrictionSignal(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgrade_cooldowns(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<(runtime_types::polkadot_parachain::primitives::Id, u32)>,
                    ::subxt::Error,
                > {
                    let entry = UpgradeCooldowns;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upcoming_upgrades(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<(runtime_types::polkadot_parachain::primitives::Id, u32)>,
                    ::subxt::Error,
                > {
                    let entry = UpcomingUpgrades;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn actions_queue(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = ActionsQueue(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn upcoming_paras_genesis (& self , _0 : runtime_types :: polkadot_parachain :: primitives :: Id , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: paras :: ParaGenesisArgs > , :: subxt :: Error >{
                    let entry = UpcomingParasGenesis(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn code_by_hash_refs(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = CodeByHashRefs(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn code_by_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CodeByHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod initializer {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceApprove {
                pub up_to: u32,
            }
            impl ::subxt::Call for ForceApprove {
                const PALLET: &'static str = "Initializer";
                const FUNCTION: &'static str = "force_approve";
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
                pub fn force_approve(
                    &self,
                    up_to: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceApprove> {
                    let call = ForceApprove { up_to };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HasInitialized;
            impl ::subxt::StorageEntry for HasInitialized {
                const PALLET: &'static str = "Initializer";
                const STORAGE: &'static str = "HasInitialized";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BufferedSessionChanges;
            impl ::subxt::StorageEntry for BufferedSessionChanges {
                const PALLET: &'static str = "Initializer";
                const STORAGE: &'static str = "BufferedSessionChanges";
                type Value = Vec < runtime_types :: polkadot_runtime_parachains :: initializer :: BufferedSessionChange > ;
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
                pub async fn has_initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::Error>
                {
                    let entry = HasInitialized;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn buffered_session_changes (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < Vec < runtime_types :: polkadot_runtime_parachains :: initializer :: BufferedSessionChange > , :: subxt :: Error >{
                    let entry = BufferedSessionChanges;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod dmp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
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
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct DownwardMessageQueues(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for DownwardMessageQueues {
                const PALLET: &'static str = "Dmp";
                const STORAGE: &'static str = "DownwardMessageQueues";
                type Value = Vec<
                    runtime_types::polkadot_core_primitives::InboundDownwardMessage<u32>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct DownwardMessageQueueHeads(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for DownwardMessageQueueHeads {
                const PALLET: &'static str = "Dmp";
                const STORAGE: &'static str = "DownwardMessageQueueHeads";
                type Value = ::subxt::sp_core::H256;
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
                pub async fn downward_message_queues(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<
                        runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                            u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = DownwardMessageQueues(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn downward_message_queue_heads(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = DownwardMessageQueueHeads(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod ump {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ServiceOverweight {
                pub index: u64,
                pub weight_limit: u64,
            }
            impl ::subxt::Call for ServiceOverweight {
                const PALLET: &'static str = "Ump";
                const FUNCTION: &'static str = "service_overweight";
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
                pub fn service_overweight(
                    &self,
                    index: u64,
                    weight_limit: u64,
                ) -> ::subxt::SubmittableExtrinsic<T, ServiceOverweight> {
                    let call = ServiceOverweight {
                        index,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::ump::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct InvalidFormat(pub [u8; 32usize]);
            impl ::subxt::Event for InvalidFormat {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct UnsupportedVersion(pub [u8; 32usize]);
            impl ::subxt::Event for UnsupportedVersion {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExecutedUpward(
                pub [u8; 32usize],
                pub runtime_types::xcm::v2::traits::Outcome,
            );
            impl ::subxt::Event for ExecutedUpward {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "ExecutedUpward";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct WeightExhausted(pub [u8; 32usize], pub u64, pub u64);
            impl ::subxt::Event for WeightExhausted {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "WeightExhausted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct UpwardMessagesReceived(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub u32,
                pub u32,
            );
            impl ::subxt::Event for UpwardMessagesReceived {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "UpwardMessagesReceived";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OverweightEnqueued(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub [u8; 32usize],
                pub u64,
                pub u64,
            );
            impl ::subxt::Event for OverweightEnqueued {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OverweightServiced(pub u64, pub u64);
            impl ::subxt::Event for OverweightServiced {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RelayDispatchQueues(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for RelayDispatchQueues {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "RelayDispatchQueues";
                type Value = Vec<Vec<u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct RelayDispatchQueueSize(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for RelayDispatchQueueSize {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "RelayDispatchQueueSize";
                type Value = (u32, u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NeedsDispatch;
            impl ::subxt::StorageEntry for NeedsDispatch {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "NeedsDispatch";
                type Value = Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextDispatchRoundStartWith;
            impl ::subxt::StorageEntry for NextDispatchRoundStartWith {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "NextDispatchRoundStartWith";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Overweight(pub u64);
            impl ::subxt::StorageEntry for Overweight {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "Overweight";
                type Value = (runtime_types::polkadot_parachain::primitives::Id, Vec<u8>);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct OverweightCount;
            impl ::subxt::StorageEntry for OverweightCount {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "OverweightCount";
                type Value = u64;
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
                pub async fn relay_dispatch_queues(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<Vec<Vec<u8>>, ::subxt::Error>
                {
                    let entry = RelayDispatchQueues(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn relay_dispatch_queue_size(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(u32, u32), ::subxt::Error> {
                    let entry = RelayDispatchQueueSize(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn needs_dispatch(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = NeedsDispatch;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_dispatch_round_start_with(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::Id,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NextDispatchRoundStartWith;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight(
                    &self,
                    _0: u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        Vec<u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Overweight(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u64, ::subxt::Error> {
                    let entry = OverweightCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod hrmp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct HrmpInitOpenChannel {
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                pub proposed_max_capacity: u32,
                pub proposed_max_message_size: u32,
            }
            impl ::subxt::Call for HrmpInitOpenChannel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_init_open_channel";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct HrmpAcceptOpenChannel {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for HrmpAcceptOpenChannel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_accept_open_channel";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct HrmpCloseChannel {
                pub channel_id:
                    runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            }
            impl ::subxt::Call for HrmpCloseChannel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_close_channel";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceCleanHrmp {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for ForceCleanHrmp {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_clean_hrmp";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceProcessHrmpOpen {}
            impl ::subxt::Call for ForceProcessHrmpOpen {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_process_hrmp_open";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceProcessHrmpClose {}
            impl ::subxt::Call for ForceProcessHrmpClose {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_process_hrmp_close";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct HrmpCancelOpenRequest {
                pub channel_id:
                    runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            }
            impl ::subxt::Call for HrmpCancelOpenRequest {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_cancel_open_request";
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
                pub fn hrmp_init_open_channel(
                    &self,
                    recipient: runtime_types::polkadot_parachain::primitives::Id,
                    proposed_max_capacity: u32,
                    proposed_max_message_size: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, HrmpInitOpenChannel>
                {
                    let call = HrmpInitOpenChannel {
                        recipient,
                        proposed_max_capacity,
                        proposed_max_message_size,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_accept_open_channel(
                    &self,
                    sender: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, HrmpAcceptOpenChannel>
                {
                    let call = HrmpAcceptOpenChannel { sender };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_close_channel(
                    &self,
                    channel_id : runtime_types :: polkadot_parachain :: primitives :: HrmpChannelId,
                ) -> ::subxt::SubmittableExtrinsic<T, HrmpCloseChannel> {
                    let call = HrmpCloseChannel { channel_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_clean_hrmp(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceCleanHrmp> {
                    let call = ForceCleanHrmp { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_process_hrmp_open(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceProcessHrmpOpen>
                {
                    let call = ForceProcessHrmpOpen {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_process_hrmp_close(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceProcessHrmpClose>
                {
                    let call = ForceProcessHrmpClose {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_cancel_open_request(
                    &self,
                    channel_id : runtime_types :: polkadot_parachain :: primitives :: HrmpChannelId,
                ) -> ::subxt::SubmittableExtrinsic<T, HrmpCancelOpenRequest>
                {
                    let call = HrmpCancelOpenRequest { channel_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OpenChannelRequested(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub u32,
                pub u32,
            );
            impl ::subxt::Event for OpenChannelRequested {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelRequested";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OpenChannelCanceled(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::Event for OpenChannelCanceled {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelCanceled";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OpenChannelAccepted(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for OpenChannelAccepted {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelAccepted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ChannelClosed(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::Event for ChannelClosed {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "ChannelClosed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HrmpOpenChannelRequests(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpOpenChannelRequests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequests";
                type Value = runtime_types :: polkadot_runtime_parachains :: hrmp :: HrmpOpenChannelRequest ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpOpenChannelRequestsList;
            impl ::subxt::StorageEntry for HrmpOpenChannelRequestsList {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequestsList";
                type Value =
                    Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpOpenChannelRequestCount(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpOpenChannelRequestCount {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequestCount";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpAcceptedChannelRequestCount(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpAcceptedChannelRequestCount {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpAcceptedChannelRequestCount";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpCloseChannelRequests(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpCloseChannelRequests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpCloseChannelRequests";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpCloseChannelRequestsList;
            impl ::subxt::StorageEntry for HrmpCloseChannelRequestsList {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpCloseChannelRequestsList";
                type Value =
                    Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpWatermarks(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpWatermarks {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpWatermarks";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannels(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpChannels {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannels";
                type Value =
                    runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpIngressChannelsIndex(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpIngressChannelsIndex {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpIngressChannelsIndex";
                type Value = Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpEgressChannelsIndex(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpEgressChannelsIndex {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpEgressChannelsIndex";
                type Value = Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannelContents(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpChannelContents {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannelContents";
                type Value =
                    Vec<runtime_types::polkadot_core_primitives::InboundHrmpMessage<u32>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannelDigests(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpChannelDigests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannelDigests";
                type Value =
                    Vec<(u32, Vec<runtime_types::polkadot_parachain::primitives::Id>)>;
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
                }                pub async fn hrmp_open_channel_requests (& self , _0 : runtime_types :: polkadot_parachain :: primitives :: HrmpChannelId , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: hrmp :: HrmpOpenChannelRequest > , :: subxt :: Error >{
                    let entry = HrmpOpenChannelRequests(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_open_channel_requests_list(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>,
                    ::subxt::Error,
                > {
                    let entry = HrmpOpenChannelRequestsList;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_open_channel_request_count(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = HrmpOpenChannelRequestCount(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_accepted_channel_request_count(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = HrmpAcceptedChannelRequestCount(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_close_channel_requests(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::Error>
                {
                    let entry = HrmpCloseChannelRequests(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_close_channel_requests_list(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>,
                    ::subxt::Error,
                > {
                    let entry = HrmpCloseChannelRequestsList;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_watermarks(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u32>, ::subxt::Error>
                {
                    let entry = HrmpWatermarks(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_channels(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel,
                    >,
                    ::subxt::Error,
                > {
                    let entry = HrmpChannels(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_ingress_channels_index(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = HrmpIngressChannelsIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_egress_channels_index(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = HrmpEgressChannelsIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_channel_contents(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_core_primitives::InboundHrmpMessage<u32>>,
                    ::subxt::Error,
                > {
                    let entry = HrmpChannelContents(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_channel_digests(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<(u32, Vec<runtime_types::polkadot_parachain::primitives::Id>)>,
                    ::subxt::Error,
                > {
                    let entry = HrmpChannelDigests(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod para_session_info {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct AssignmentKeysUnsafe;
            impl ::subxt::StorageEntry for AssignmentKeysUnsafe {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "AssignmentKeysUnsafe";
                type Value =
                    Vec<runtime_types::polkadot_primitives::v1::assignment_app::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EarliestStoredSession;
            impl ::subxt::StorageEntry for EarliestStoredSession {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "EarliestStoredSession";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Sessions(pub u32);
            impl ::subxt::StorageEntry for Sessions {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "Sessions";
                type Value = runtime_types::polkadot_primitives::v1::SessionInfo;
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
                pub async fn assignment_keys_unsafe(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_primitives::v1::assignment_app::Public>,
                    ::subxt::Error,
                > {
                    let entry = AssignmentKeysUnsafe;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn earliest_stored_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = EarliestStoredSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn sessions(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::SessionInfo,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Sessions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod registrar {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Register {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                pub validation_code:
                    runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for Register {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "register";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceRegister {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub deposit: u128,
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                pub validation_code:
                    runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for ForceRegister {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "force_register";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Deregister {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Deregister {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "deregister";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Swap {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub other: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Swap {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "swap";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceRemoveLock {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for ForceRemoveLock {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "force_remove_lock";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Reserve {}
            impl ::subxt::Call for Reserve {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "reserve";
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
                pub fn register(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    validation_code : runtime_types :: polkadot_parachain :: primitives :: ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<T, Register> {
                    let call = Register {
                        id,
                        genesis_head,
                        validation_code,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_register(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    deposit: u128,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    validation_code : runtime_types :: polkadot_parachain :: primitives :: ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceRegister> {
                    let call = ForceRegister {
                        who,
                        deposit,
                        id,
                        genesis_head,
                        validation_code,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deregister(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Deregister> {
                    let call = Deregister { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn swap(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    other: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Swap> {
                    let call = Swap { id, other };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_remove_lock(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceRemoveLock> {
                    let call = ForceRemoveLock { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reserve(&self) -> ::subxt::SubmittableExtrinsic<T, Reserve> {
                    let call = Reserve {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event =
            runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Registered(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Registered {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Registered";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Deregistered(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for Deregistered {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Deregistered";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Reserved(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Reserved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PendingSwap(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for PendingSwap {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "PendingSwap";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Paras(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Paras {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "Paras";
                type Value =
                    runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NextFreeParaId;
            impl ::subxt::StorageEntry for NextFreeParaId {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "NextFreeParaId";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
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
                pub async fn pending_swap(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::Id,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingSwap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn paras(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Paras(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_free_para_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::subxt::Error,
                > {
                    let entry = NextFreeParaId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod slots {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceLease {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub leaser: ::subxt::sp_core::crypto::AccountId32,
                pub amount: u128,
                pub period_begin: u32,
                pub period_count: u32,
            }
            impl ::subxt::Call for ForceLease {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "force_lease";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearAllLeases {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for ClearAllLeases {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "clear_all_leases";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct TriggerOnboard {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for TriggerOnboard {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "trigger_onboard";
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
                pub fn force_lease(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    leaser: ::subxt::sp_core::crypto::AccountId32,
                    amount: u128,
                    period_begin: u32,
                    period_count: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceLease> {
                    let call = ForceLease {
                        para,
                        leaser,
                        amount,
                        period_begin,
                        period_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_all_leases(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearAllLeases> {
                    let call = ClearAllLeases { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn trigger_onboard(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, TriggerOnboard> {
                    let call = TriggerOnboard { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::slots::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewLeasePeriod(pub u32);
            impl ::subxt::Event for NewLeasePeriod {
                const PALLET: &'static str = "Slots";
                const EVENT: &'static str = "NewLeasePeriod";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Leased(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u32,
                pub u32,
                pub u128,
                pub u128,
            );
            impl ::subxt::Event for Leased {
                const PALLET: &'static str = "Slots";
                const EVENT: &'static str = "Leased";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Leases(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Leases {
                const PALLET: &'static str = "Slots";
                const STORAGE: &'static str = "Leases";
                type Value = Vec<Option<(::subxt::sp_core::crypto::AccountId32, u128)>>;
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
                pub async fn leases(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<Option<(::subxt::sp_core::crypto::AccountId32, u128)>>,
                    ::subxt::Error,
                > {
                    let entry = Leases(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod auctions {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NewAuction {
                #[codec(compact)]
                pub duration: u32,
                #[codec(compact)]
                pub lease_period_index: u32,
            }
            impl ::subxt::Call for NewAuction {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "new_auction";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Bid {
                #[codec(compact)]
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub auction_index: u32,
                #[codec(compact)]
                pub first_slot: u32,
                #[codec(compact)]
                pub last_slot: u32,
                #[codec(compact)]
                pub amount: u128,
            }
            impl ::subxt::Call for Bid {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "bid";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelAuction {}
            impl ::subxt::Call for CancelAuction {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "cancel_auction";
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
                pub fn new_auction(
                    &self,
                    duration: u32,
                    lease_period_index: u32,
                ) -> ::subxt::SubmittableExtrinsic<T, NewAuction> {
                    let call = NewAuction {
                        duration,
                        lease_period_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bid(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    auction_index: u32,
                    first_slot: u32,
                    last_slot: u32,
                    amount: u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Bid> {
                    let call = Bid {
                        para,
                        auction_index,
                        first_slot,
                        last_slot,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_auction(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelAuction> {
                    let call = CancelAuction {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::auctions::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AuctionStarted(pub u32, pub u32, pub u32);
            impl ::subxt::Event for AuctionStarted {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "AuctionStarted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AuctionClosed(pub u32);
            impl ::subxt::Event for AuctionClosed {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "AuctionClosed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
                pub u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unreserved(pub ::subxt::sp_core::crypto::AccountId32, pub u128);
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReserveConfiscated(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub u128,
            );
            impl ::subxt::Event for ReserveConfiscated {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "ReserveConfiscated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BidAccepted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub u128,
                pub u32,
                pub u32,
            );
            impl ::subxt::Event for BidAccepted {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "BidAccepted";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct WinningOffset(pub u32, pub u32);
            impl ::subxt::Event for WinningOffset {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "WinningOffset";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AuctionCounter;
            impl ::subxt::StorageEntry for AuctionCounter {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "AuctionCounter";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuctionInfo;
            impl ::subxt::StorageEntry for AuctionInfo {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "AuctionInfo";
                type Value = (u32, u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedAmounts(
                ::subxt::sp_core::crypto::AccountId32,
                runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for ReservedAmounts {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "ReservedAmounts";
                type Value = u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Winning(pub u32);
            impl ::subxt::StorageEntry for Winning {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "Winning";
                type Value = [Option<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::polkadot_parachain::primitives::Id,
                    u128,
                )>; 36usize];
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
                pub async fn auction_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = AuctionCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn auction_info(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(u32, u32)>,
                    ::subxt::Error,
                > {
                    let entry = AuctionInfo;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reserved_amounts(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<u128>, ::subxt::Error>
                {
                    let entry = ReservedAmounts(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn winning(
                    &self,
                    _0: u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        [Option<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            u128,
                        )>; 36usize],
                    >,
                    ::subxt::Error,
                > {
                    let entry = Winning(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod crowdloan {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Create {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub cap: u128,
                #[codec(compact)]
                pub first_period: u32,
                #[codec(compact)]
                pub last_period: u32,
                #[codec(compact)]
                pub end: u32,
                pub verifier: Option<runtime_types::sp_runtime::MultiSigner>,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "create";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Contribute {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub value: u128,
                pub signature: Option<runtime_types::sp_runtime::MultiSignature>,
            }
            impl ::subxt::Call for Contribute {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "contribute";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Withdraw {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "withdraw";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Refund {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Refund {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "refund";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Dissolve {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Dissolve {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "dissolve";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Edit {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub cap: u128,
                #[codec(compact)]
                pub first_period: u32,
                #[codec(compact)]
                pub last_period: u32,
                #[codec(compact)]
                pub end: u32,
                pub verifier: Option<runtime_types::sp_runtime::MultiSigner>,
            }
            impl ::subxt::Call for Edit {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "edit";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddMemo {
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                pub memo: Vec<u8>,
            }
            impl ::subxt::Call for AddMemo {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "add_memo";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Poke {
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Poke {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "poke";
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
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    cap: u128,
                    first_period: u32,
                    last_period: u32,
                    end: u32,
                    verifier: Option<runtime_types::sp_runtime::MultiSigner>,
                ) -> ::subxt::SubmittableExtrinsic<T, Create> {
                    let call = Create {
                        index,
                        cap,
                        first_period,
                        last_period,
                        end,
                        verifier,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn contribute(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    value: u128,
                    signature: Option<runtime_types::sp_runtime::MultiSignature>,
                ) -> ::subxt::SubmittableExtrinsic<T, Contribute> {
                    let call = Contribute {
                        index,
                        value,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Withdraw> {
                    let call = Withdraw { who, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn refund(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Refund> {
                    let call = Refund { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn dissolve(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Dissolve> {
                    let call = Dissolve { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn edit(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    cap: u128,
                    first_period: u32,
                    last_period: u32,
                    end: u32,
                    verifier: Option<runtime_types::sp_runtime::MultiSigner>,
                ) -> ::subxt::SubmittableExtrinsic<T, Edit> {
                    let call = Edit {
                        index,
                        cap,
                        first_period,
                        last_period,
                        end,
                        verifier,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_memo(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    memo: Vec<u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, AddMemo> {
                    let call = AddMemo { index, memo };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn poke(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Poke> {
                    let call = Poke { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::crowdloan::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Created(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Created";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Contributed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub u128,
            );
            impl ::subxt::Event for Contributed {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Contributed";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Withdrew(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub u128,
            );
            impl ::subxt::Event for Withdrew {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Withdrew";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PartiallyRefunded(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for PartiallyRefunded {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "PartiallyRefunded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AllRefunded(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for AllRefunded {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "AllRefunded";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Dissolved(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Dissolved {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Dissolved";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct HandleBidResult(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for HandleBidResult {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "HandleBidResult";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Edited(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Edited {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Edited";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MemoUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub Vec<u8>,
            );
            impl ::subxt::Event for MemoUpdated {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "MemoUpdated";
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddedToNewRaise(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for AddedToNewRaise {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "AddedToNewRaise";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Funds(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Funds {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "Funds";
                type Value = runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
                    ::subxt::sp_core::crypto::AccountId32,
                    u128,
                    u32,
                    u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NewRaise;
            impl ::subxt::StorageEntry for NewRaise {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "NewRaise";
                type Value = Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EndingsCount;
            impl ::subxt::StorageEntry for EndingsCount {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "EndingsCount";
                type Value = u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextTrieIndex;
            impl ::subxt::StorageEntry for NextTrieIndex {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "NextTrieIndex";
                type Value = u32;
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
                pub async fn funds(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
                            u32,
                            u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Funds(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn new_raise(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = NewRaise;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn endings_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = EndingsCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_trie_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<u32, ::subxt::Error> {
                    let entry = NextTrieIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bitvec {
            use super::runtime_types;
            pub mod order {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Lsb0 {}
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct BoundedBTreeMap<_0, _1>(
                        pub std::collections::BTreeMap<_0, _1>,
                    );
                }
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct BoundedVec<_0>(pub Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct WeakBoundedVec<_0>(pub Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct WrapperOpaque<_0>(u32, pub _0);
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum DispatchClass {
                    Normal,
                    Operational,
                    Mandatory,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct DispatchInfo {
                    pub weight: u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Pays {
                    Yes,
                    No,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct RuntimeDbWeight {
                    pub read: u64,
                    pub write: u64,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: ::subxt::sp_arithmetic::per_things::Perbill,
                    pub negative: bool,
                    pub degree: u8,
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct PalletId(pub [u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct CheckGenesis {}
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct CheckMortality(
                        pub runtime_types::sp_runtime::generic::era::Era,
                    );
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct CheckNonce(pub u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct CheckSpecVersion {}
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct CheckTxVersion {}
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct CheckWeight {}
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<u32>,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct BlockWeights {
                    pub base_block: u64,
                    pub max_block: u64,
                    pub per_class:
                        runtime_types::frame_support::weights::PerDispatchClass<
                            runtime_types::frame_system::limits::WeightsPerClass,
                        >,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: u64,
                    pub max_extrinsic: Option<u64>,
                    pub max_total: Option<u64>,
                    pub reserved: Option<u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    fill_block { ratio : :: subxt :: sp_arithmetic :: per_things :: Perbill , } , remark { remark : Vec < u8 > , } , set_heap_pages { pages : u64 , } , set_code { code : Vec < u8 > , } , set_code_without_checks { code : Vec < u8 > , } , set_changes_trie_config { changes_trie_config : Option < runtime_types :: sp_core :: changes_trie :: ChangesTrieConfiguration > , } , set_storage { items : Vec < (Vec < u8 > , Vec < u8 > ,) > , } , kill_storage { keys : Vec < Vec < u8 > > , } , kill_prefix { prefix : Vec < u8 > , subkeys : u32 , } , remark_with_event { remark : Vec < u8 > , } , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    InvalidSpecName,
                    SpecVersionNeedsToIncrease,
                    FailedToExtractRuntimeVersion,
                    NonDefaultComposite,
                    NonZeroRefCount,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: Vec<_1>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: u32,
                pub spec_name: String,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Phase {
                ApplyExtrinsic(u32),
                Finalization,
                Initialization,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawOrigin<_0> {
                Root,
                Signed(_0),
                None,
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    set_uncles {
                        new_uncles: Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum UncleEntryItem<_0, _1, _2> {
                InclusionHeight(_0),
                Uncle(_1, Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    report_equivocation { equivocation_proof : std :: boxed :: Box < runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , report_equivocation_unsigned { equivocation_proof : std :: boxed :: Box < runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , plan_config_change { config : runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor , } , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    InvalidEquivocationProof,
                    InvalidKeyOwnershipProof,
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: u128,
                    },
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: u128,
                        #[codec(compact)]
                        new_reserved: u128,
                    },
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: u128,
                    },
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: u128,
                    },
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: bool,
                    },
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: u128,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    Endowed(::subxt::sp_core::crypto::AccountId32, u128),
                    DustLost(::subxt::sp_core::crypto::AccountId32, u128),
                    Transfer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    BalanceSet(::subxt::sp_core::crypto::AccountId32, u128, u128),
                    Deposit(::subxt::sp_core::crypto::AccountId32, u128),
                    Reserved(::subxt::sp_core::crypto::AccountId32, u128),
                    Unreserved(::subxt::sp_core::crypto::AccountId32, u128),
                    ReserveRepatriated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                        runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    ),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BalanceLock<_0> {
                pub id: [u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Reasons {
                Fee,
                Misc,
                All,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1_0_0,
                V2_0_0,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_bounties {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    propose_bounty {
                        #[codec(compact)]
                        value: u128,
                        description: Vec<u8>,
                    },
                    approve_bounty {
                        #[codec(compact)]
                        bounty_id: u32,
                    },
                    propose_curator {
                        #[codec(compact)]
                        bounty_id: u32,
                        curator: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        fee: u128,
                    },
                    unassign_curator {
                        #[codec(compact)]
                        bounty_id: u32,
                    },
                    accept_curator {
                        #[codec(compact)]
                        bounty_id: u32,
                    },
                    award_bounty {
                        #[codec(compact)]
                        bounty_id: u32,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    claim_bounty {
                        #[codec(compact)]
                        bounty_id: u32,
                    },
                    close_bounty {
                        #[codec(compact)]
                        bounty_id: u32,
                    },
                    extend_bounty_expiry {
                        #[codec(compact)]
                        bounty_id: u32,
                        remark: Vec<u8>,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    BountyProposed(u32),
                    BountyRejected(u32, u128),
                    BountyBecameActive(u32),
                    BountyAwarded(u32, ::subxt::sp_core::crypto::AccountId32),
                    BountyClaimed(u32, u128, ::subxt::sp_core::crypto::AccountId32),
                    BountyCanceled(u32),
                    BountyExtended(u32),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Bounty<_0, _1, _2> {
                pub proposer: _0,
                pub value: _1,
                pub fee: _1,
                pub curator_deposit: _1,
                pub bond: _1,
                pub status: runtime_types::pallet_bounties::BountyStatus<_0, _2>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    set_members {
                        new_members: Vec<::subxt::sp_core::crypto::AccountId32>,
                        prime: Option<::subxt::sp_core::crypto::AccountId32>,
                        old_count: u32,
                    },
                    execute {
                        proposal: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                        #[codec(compact)]
                        length_bound: u32,
                    },
                    propose {
                        #[codec(compact)]
                        threshold: u32,
                        proposal: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                        #[codec(compact)]
                        length_bound: u32,
                    },
                    vote {
                        proposal: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        index: u32,
                        approve: bool,
                    },
                    close {
                        proposal_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        index: u32,
                        #[codec(compact)]
                        proposal_weight_bound: u64,
                        #[codec(compact)]
                        length_bound: u32,
                    },
                    disapprove_proposal {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    Proposed(
                        ::subxt::sp_core::crypto::AccountId32,
                        u32,
                        ::subxt::sp_core::H256,
                        u32,
                    ),
                    Voted(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                        bool,
                        u32,
                        u32,
                    ),
                    Approved(::subxt::sp_core::H256),
                    Disapproved(::subxt::sp_core::H256),
                    Executed(
                        ::subxt::sp_core::H256,
                        Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                    MemberExecuted(
                        ::subxt::sp_core::H256,
                        Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                    Closed(::subxt::sp_core::H256, u32, u32),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawOrigin<_0> {
                Members(u32, u32),
                Member(_0),
                _Phantom,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Votes<_0, _1> {
                pub index: _1,
                pub threshold: _1,
                pub ayes: Vec<_0>,
                pub nays: Vec<_0>,
                pub end: _1,
            }
        }
        pub mod pallet_democracy {
            use super::runtime_types;
            pub mod conviction {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    propose {
                        proposal_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        value: u128,
                    },
                    second {
                        #[codec(compact)]
                        proposal: u32,
                        #[codec(compact)]
                        seconds_upper_bound: u32,
                    },
                    vote {
                        #[codec(compact)]
                        ref_index: u32,
                        vote: runtime_types::pallet_democracy::vote::AccountVote<u128>,
                    },
                    emergency_cancel {
                        ref_index: u32,
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
                        voting_period: u32,
                        delay: u32,
                    },
                    veto_external {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                    cancel_referendum {
                        #[codec(compact)]
                        ref_index: u32,
                    },
                    cancel_queued {
                        which: u32,
                    },
                    delegate {
                        to: ::subxt::sp_core::crypto::AccountId32,
                        conviction:
                            runtime_types::pallet_democracy::conviction::Conviction,
                        balance: u128,
                    },
                    undelegate,
                    clear_public_proposals,
                    note_preimage {
                        encoded_proposal: Vec<u8>,
                    },
                    note_preimage_operational {
                        encoded_proposal: Vec<u8>,
                    },
                    note_imminent_preimage {
                        encoded_proposal: Vec<u8>,
                    },
                    note_imminent_preimage_operational {
                        encoded_proposal: Vec<u8>,
                    },
                    reap_preimage {
                        proposal_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        proposal_len_upper_bound: u32,
                    },
                    unlock {
                        target: ::subxt::sp_core::crypto::AccountId32,
                    },
                    remove_vote {
                        index: u32,
                    },
                    remove_other_vote {
                        target: ::subxt::sp_core::crypto::AccountId32,
                        index: u32,
                    },
                    enact_proposal {
                        proposal_hash: ::subxt::sp_core::H256,
                        index: u32,
                    },
                    blacklist {
                        proposal_hash: ::subxt::sp_core::H256,
                        maybe_ref_index: Option<u32>,
                    },
                    cancel_proposal {
                        #[codec(compact)]
                        prop_index: u32,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    Proposed(u32, u128),
                    Tabled(u32, u128, Vec<::subxt::sp_core::crypto::AccountId32>),
                    ExternalTabled,
                    Started(
                        u32,
                        runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                    ),
                    Passed(u32),
                    NotPassed(u32),
                    Cancelled(u32),
                    Executed(u32, Result<(), runtime_types::sp_runtime::DispatchError>),
                    Delegated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Undelegated(::subxt::sp_core::crypto::AccountId32),
                    Vetoed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                        u32,
                    ),
                    PreimageNoted(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    PreimageUsed(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    PreimageInvalid(::subxt::sp_core::H256, u32),
                    PreimageMissing(::subxt::sp_core::H256, u32),
                    PreimageReaped(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    Blacklisted(::subxt::sp_core::H256),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Delegations<_0> {
                    pub votes: _0,
                    pub capital: _0,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum ReferendumInfo<_0, _1, _2> {
                    Ongoing(
                        runtime_types::pallet_democracy::types::ReferendumStatus<
                            _0,
                            _1,
                            _2,
                        >,
                    ),
                    Finished {
                        approved: bool,
                        end: _0,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ReferendumStatus<_0, _1, _2> {
                    pub end: _0,
                    pub proposal_hash: _1,
                    pub threshold:
                        runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                    pub delay: _0,
                    pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Tally<_0> {
                    pub ayes: _0,
                    pub nays: _0,
                    pub turnout: _0,
                }
            }
            pub mod vote {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct PriorLock<_0, _1>(pub _0, pub _1);
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct Vote(u8);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Voting<_0, _1, _2> {
                    Direct {
                        votes: Vec<(
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum VoteThreshold {
                    SuperMajorityApprove,
                    SuperMajorityAgainst,
                    SimpleMajority,
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum PreimageStatus<_0, _1, _2> {
                Missing(_2),
                Available {
                    data: Vec<u8>,
                    provider: _0,
                    deposit: _1,
                    since: _2,
                    expiry: Option<_2>,
                },
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1,
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    submit_unsigned { raw_solution : std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: polkadot_runtime :: NposCompactSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , set_minimum_untrusted_score { maybe_next_score : Option < [u128 ; 3usize] > , } , set_emergency_election_result { supports : Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: sp_core :: crypto :: AccountId32 > ,) > , } , submit { raw_solution : std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: polkadot_runtime :: NposCompactSolution16 > > , num_signed_submissions : u32 , } , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    SolutionStored (runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , bool ,) , ElectionFinalized (Option < runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute > ,) , Rewarded (:: subxt :: sp_core :: crypto :: AccountId32 , u128 ,) , Slashed (:: subxt :: sp_core :: crypto :: AccountId32 , u128 ,) , SignedPhaseStarted (u32 ,) , UnsignedPhaseStarted (u32 ,) , }
            }
            pub mod signed {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum ElectionCompute {
                OnChain,
                Signed,
                Unsigned,
                Fallback,
                Emergency,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Phase<_0> {
                Off,
                Signed,
                Unsigned((bool, _0)),
                Emergency,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: [u128; 3usize],
                pub round: u32,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReadySolution<_0> {
                pub supports: Vec<(_0, runtime_types::sp_npos_elections::Support<_0>)>,
                pub score: [u128; 3usize],
                pub compute:
                    runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RoundSnapshot<_0> {
                pub voters: Vec<(_0, u64, Vec<_0>)>,
                pub targets: Vec<_0>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SolutionOrSnapshotSize {
                #[codec(compact)]
                pub voters: u32,
                #[codec(compact)]
                pub targets: u32,
            }
        }
        pub mod pallet_elections_phragmen {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    vote {
                        votes: Vec<::subxt::sp_core::crypto::AccountId32>,
                        #[codec(compact)]
                        value: u128,
                    },
                    remove_voter,
                    submit_candidacy {
                        #[codec(compact)]
                        candidate_count: u32,
                    },
                    renounce_candidacy {
                        renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
                    },
                    remove_member {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        has_replacement: bool,
                    },
                    clean_defunct_voters {
                        num_voters: u32,
                        num_defunct: u32,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    NewTerm(Vec<(::subxt::sp_core::crypto::AccountId32, u128)>),
                    EmptyTerm,
                    ElectionError,
                    MemberKicked(::subxt::sp_core::crypto::AccountId32),
                    Renounced(::subxt::sp_core::crypto::AccountId32),
                    CandidateSlashed(::subxt::sp_core::crypto::AccountId32, u128),
                    SeatHolderSlashed(::subxt::sp_core::crypto::AccountId32, u128),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Renouncing {
                Member,
                RunnerUp,
                Candidate(u32),
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SeatHolder<_0, _1> {
                pub who: _0,
                pub stake: _1,
                pub deposit: _1,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Voter<_0, _1> {
                pub votes: Vec<_0>,
                pub stake: _1,
                pub deposit: _1,
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    report_equivocation {
                        equivocation_proof: std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    report_equivocation_unsigned {
                        equivocation_proof: std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    note_stalled {
                        delay: u32,
                        best_finalized_block_number: u32,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    PauseFailed,
                    ResumeFailed,
                    ChangePending,
                    TooSoon,
                    InvalidKeyOwnershipProof,
                    InvalidEquivocationProof,
                    DuplicateOffenceReport,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    NewAuthorities(
                        Vec<(runtime_types::sp_finality_grandpa::app::Public, u64)>,
                    ),
                    Paused,
                    Resumed,
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct StoredPendingChange < _0 > { pub scheduled_at : _0 , pub delay : _0 , pub next_authorities : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_finality_grandpa :: app :: Public , u64 ,) > , pub forced : Option < _0 > , }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    add_registrar {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    set_identity {
                        info: std::boxed::Box<
                            runtime_types::pallet_identity::types::IdentityInfo,
                        >,
                    },
                    set_subs {
                        subs: Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    clear_identity,
                    request_judgement {
                        #[codec(compact)]
                        reg_index: u32,
                        #[codec(compact)]
                        max_fee: u128,
                    },
                    cancel_request {
                        reg_index: u32,
                    },
                    set_fee {
                        #[codec(compact)]
                        index: u32,
                        #[codec(compact)]
                        fee: u128,
                    },
                    set_account_id {
                        #[codec(compact)]
                        index: u32,
                        new: ::subxt::sp_core::crypto::AccountId32,
                    },
                    set_fields {
                        #[codec(compact)]
                        index: u32,
                        fields: runtime_types::pallet_identity::types::BitFlags<
                            runtime_types::pallet_identity::types::IdentityField,
                        >,
                    },
                    provide_judgement {
                        #[codec(compact)]
                        reg_index: u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        judgement: runtime_types::pallet_identity::types::Judgement<u128>,
                    },
                    kill_identity {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    add_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    rename_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    remove_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    quit_sub,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    IdentitySet(::subxt::sp_core::crypto::AccountId32),
                    IdentityCleared(::subxt::sp_core::crypto::AccountId32, u128),
                    IdentityKilled(::subxt::sp_core::crypto::AccountId32, u128),
                    JudgementRequested(::subxt::sp_core::crypto::AccountId32, u32),
                    JudgementUnrequested(::subxt::sp_core::crypto::AccountId32, u32),
                    JudgementGiven(::subxt::sp_core::crypto::AccountId32, u32),
                    RegistrarAdded(u32),
                    SubIdentityAdded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    SubIdentityRemoved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    SubIdentityRevoked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    ),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct BitFlags<_0>(
                    pub u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Data {
                    None,
                    Raw0([u8; 0usize]),
                    Raw1([u8; 1usize]),
                    Raw2([u8; 2usize]),
                    Raw3([u8; 3usize]),
                    Raw4([u8; 4usize]),
                    Raw5([u8; 5usize]),
                    Raw6([u8; 6usize]),
                    Raw7([u8; 7usize]),
                    Raw8([u8; 8usize]),
                    Raw9([u8; 9usize]),
                    Raw10([u8; 10usize]),
                    Raw11([u8; 11usize]),
                    Raw12([u8; 12usize]),
                    Raw13([u8; 13usize]),
                    Raw14([u8; 14usize]),
                    Raw15([u8; 15usize]),
                    Raw16([u8; 16usize]),
                    Raw17([u8; 17usize]),
                    Raw18([u8; 18usize]),
                    Raw19([u8; 19usize]),
                    Raw20([u8; 20usize]),
                    Raw21([u8; 21usize]),
                    Raw22([u8; 22usize]),
                    Raw23([u8; 23usize]),
                    Raw24([u8; 24usize]),
                    Raw25([u8; 25usize]),
                    Raw26([u8; 26usize]),
                    Raw27([u8; 27usize]),
                    Raw28([u8; 28usize]),
                    Raw29([u8; 29usize]),
                    Raw30([u8; 30usize]),
                    Raw31([u8; 31usize]),
                    Raw32([u8; 32usize]),
                    BlakeTwo256([u8; 32usize]),
                    Sha256([u8; 32usize]),
                    Keccak256([u8; 32usize]),
                    ShaThree256([u8; 32usize]),
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                    pub pgp_fingerprint: Option<[u8; 20usize]>,
                    pub image: runtime_types::pallet_identity::types::Data,
                    pub twitter: runtime_types::pallet_identity::types::Data,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Judgement<_0> {
                    Unknown,
                    FeePaid(_0),
                    Reasonable,
                    KnownGood,
                    OutOfDate,
                    LowQuality,
                    Erroneous,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct RegistrarInfo<_0, _1> {
                    pub account: _1,
                    pub fee: _0,
                    pub fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Registration<_0> {
                    pub judgements:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            u32,
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    heartbeat { heartbeat : runtime_types :: pallet_im_online :: Heartbeat < u32 > , signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature , } , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    InvalidKey,
                    DuplicatedHeartbeat,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    HeartbeatReceived(
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    ),
                    AllGood,
                    SomeOffline(
                        Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::pallet_staking::Exposure<
                                ::subxt::sp_core::crypto::AccountId32,
                                u128,
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
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BoundedOpaqueNetworkState { pub peer_id : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < u8 > , pub external_addresses : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < u8 > > , }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    claim {
                        index: u32,
                    },
                    transfer {
                        new: ::subxt::sp_core::crypto::AccountId32,
                        index: u32,
                    },
                    free {
                        index: u32,
                    },
                    force_transfer {
                        new: ::subxt::sp_core::crypto::AccountId32,
                        index: u32,
                        freeze: bool,
                    },
                    freeze {
                        index: u32,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    NotAssigned,
                    NotOwner,
                    InUse,
                    NotTransfer,
                    Permanent,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    IndexAssigned(::subxt::sp_core::crypto::AccountId32, u32),
                    IndexFreed(u32),
                    IndexFrozen(u32, ::subxt::sp_core::crypto::AccountId32),
                }
            }
        }
        pub mod pallet_membership {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                        members: Vec<::subxt::sp_core::crypto::AccountId32>,
                    },
                    change_key {
                        new: ::subxt::sp_core::crypto::AccountId32,
                    },
                    set_prime {
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    clear_prime,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    AlreadyMember,
                    NotMember,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    as_multi_threshold_1 {
                        other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                        call: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                    },
                    as_multi {
                        threshold: u16,
                        other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint:
                            Option<runtime_types::pallet_multisig::Timepoint<u32>>,
                        call: Vec<u8>,
                        store_call: bool,
                        max_weight: u64,
                    },
                    approve_as_multi {
                        threshold: u16,
                        other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint:
                            Option<runtime_types::pallet_multisig::Timepoint<u32>>,
                        call_hash: [u8; 32usize],
                        max_weight: u64,
                    },
                    cancel_as_multi {
                        threshold: u16,
                        other_signatories: Vec<::subxt::sp_core::crypto::AccountId32>,
                        timepoint: runtime_types::pallet_multisig::Timepoint<u32>,
                        call_hash: [u8; 32usize],
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    NewMultisig(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        [u8; 32usize],
                    ),
                    MultisigApproval(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [u8; 32usize],
                    ),
                    MultisigExecuted(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [u8; 32usize],
                        Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                    MultisigCancelled(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [u8; 32usize],
                    ),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: Vec<_2>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_offences {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    Offence([u8; 16usize], Vec<u8>),
                }
            }
        }
        pub mod pallet_proxy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    proxy {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type:
                            Option<runtime_types::polkadot_runtime::ProxyType>,
                        call: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                    },
                    add_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::polkadot_runtime::ProxyType,
                        delay: u32,
                    },
                    remove_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::polkadot_runtime::ProxyType,
                        delay: u32,
                    },
                    remove_proxies,
                    anonymous {
                        proxy_type: runtime_types::polkadot_runtime::ProxyType,
                        delay: u32,
                        index: u16,
                    },
                    kill_anonymous {
                        spawner: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::polkadot_runtime::ProxyType,
                        index: u16,
                        #[codec(compact)]
                        height: u32,
                        #[codec(compact)]
                        ext_index: u32,
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
                        force_proxy_type:
                            Option<runtime_types::polkadot_runtime::ProxyType>,
                        call: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
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
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    ProxyExecuted(Result<(), runtime_types::sp_runtime::DispatchError>),
                    AnonymousCreated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::polkadot_runtime::ProxyType,
                        u16,
                    ),
                    Announced(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                    ProxyAdded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::polkadot_runtime::ProxyType,
                        u32,
                    ),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Announcement<_0, _1, _2> {
                pub real: _0,
                pub call_hash: _1,
                pub height: _2,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProxyDefinition<_0, _1, _2> {
                pub delegate: _0,
                pub proxy_type: _1,
                pub delay: _2,
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    schedule {
                        when: u32,
                        maybe_periodic: Option<(u32, u32)>,
                        priority: u8,
                        call: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                    },
                    cancel {
                        when: u32,
                        index: u32,
                    },
                    schedule_named {
                        id: Vec<u8>,
                        when: u32,
                        maybe_periodic: Option<(u32, u32)>,
                        priority: u8,
                        call: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                    },
                    cancel_named {
                        id: Vec<u8>,
                    },
                    schedule_after {
                        after: u32,
                        maybe_periodic: Option<(u32, u32)>,
                        priority: u8,
                        call: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                    },
                    schedule_named_after {
                        id: Vec<u8>,
                        after: u32,
                        maybe_periodic: Option<(u32, u32)>,
                        priority: u8,
                        call: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    FailedToSchedule,
                    NotFound,
                    TargetBlockNumberInPast,
                    RescheduleNoChange,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    Scheduled(u32, u32),
                    Canceled(u32, u32),
                    Dispatched(
                        (u32, u32),
                        Option<Vec<u8>>,
                        Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1,
                V2,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScheduledV2<_0, _1, _2, _3> {
                pub maybe_id: Option<Vec<u8>>,
                pub priority: u8,
                pub call: _0,
                pub maybe_periodic: Option<(_1, _1)>,
                pub origin: _2,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    set_keys {
                        keys: runtime_types::polkadot_runtime::SessionKeys,
                        proof: Vec<u8>,
                    },
                    purge_keys,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    InvalidProof,
                    NoAssociatedValidatorId,
                    DuplicatedKey,
                    NoKeys,
                    NoAccount,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    NewSession(u32),
                }
            }
        }
        pub mod pallet_staking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        bond {
                            controller: ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                            #[codec(compact)]
                            value: u128,
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        },
                        bond_extra {
                            #[codec(compact)]
                            max_additional: u128,
                        },
                        unbond {
                            #[codec(compact)]
                            value: u128,
                        },
                        withdraw_unbonded {
                            num_slashing_spans: u32,
                        },
                        validate {
                            prefs: runtime_types::pallet_staking::ValidatorPrefs,
                        },
                        nominate {
                            targets: Vec<
                                ::subxt::sp_runtime::MultiAddress<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    (),
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
                                (),
                            >,
                        },
                        set_validator_count {
                            #[codec(compact)]
                            new: u32,
                        },
                        increase_validator_count {
                            #[codec(compact)]
                            additional: u32,
                        },
                        scale_validator_count {
                            factor: runtime_types::sp_arithmetic::per_things::Percent,
                        },
                        force_no_eras,
                        force_new_era,
                        set_invulnerables {
                            invulnerables: Vec<::subxt::sp_core::crypto::AccountId32>,
                        },
                        force_unstake {
                            stash: ::subxt::sp_core::crypto::AccountId32,
                            num_slashing_spans: u32,
                        },
                        force_new_era_always,
                        cancel_deferred_slash {
                            era: u32,
                            slash_indices: Vec<u32>,
                        },
                        payout_stakers {
                            validator_stash: ::subxt::sp_core::crypto::AccountId32,
                            era: u32,
                        },
                        rebond {
                            #[codec(compact)]
                            value: u128,
                        },
                        set_history_depth {
                            #[codec(compact)]
                            new_history_depth: u32,
                            #[codec(compact)]
                            era_items_deleted: u32,
                        },
                        reap_stash {
                            stash: ::subxt::sp_core::crypto::AccountId32,
                            num_slashing_spans: u32,
                        },
                        kick {
                            who: Vec<
                                ::subxt::sp_runtime::MultiAddress<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    (),
                                >,
                            >,
                        },
                        set_staking_limits {
                            min_nominator_bond: u128,
                            min_validator_bond: u128,
                            max_nominator_count: Option<u32>,
                            max_validator_count: Option<u32>,
                            threshold:
                                Option<runtime_types::sp_arithmetic::per_things::Percent>,
                        },
                        chill_other {
                            controller: ::subxt::sp_core::crypto::AccountId32,
                        },
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
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
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        EraPaid(u32, u128, u128),
                        Rewarded(::subxt::sp_core::crypto::AccountId32, u128),
                        Slashed(::subxt::sp_core::crypto::AccountId32, u128),
                        OldSlashingReportDiscarded(u32),
                        StakersElected,
                        Bonded(::subxt::sp_core::crypto::AccountId32, u128),
                        Unbonded(::subxt::sp_core::crypto::AccountId32, u128),
                        Withdrawn(::subxt::sp_core::crypto::AccountId32, u128),
                        Kicked(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                        StakingElectionFailed,
                        Chilled(::subxt::sp_core::crypto::AccountId32),
                        PayoutStarted(u32, ::subxt::sp_core::crypto::AccountId32),
                    }
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct SlashingSpans {
                    pub span_index: u32,
                    pub last_start: u32,
                    pub last_nonzero_slash: u32,
                    pub prior: Vec<u32>,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct SpanRecord<_0> {
                    pub slashed: _0,
                    pub paid_out: _0,
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ActiveEraInfo {
                pub index: u32,
                pub start: Option<u64>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct EraRewardPoints<_0> {
                pub total: u32,
                pub individual: std::collections::BTreeMap<_0, u32>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Exposure<_0, _1> {
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub own: _1,
                pub others:
                    Vec<runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Forcing {
                NotForcing,
                ForceNew,
                ForceNone,
                ForceAlways,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                #[codec(compact)]
                pub value: _1,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Nominations<_0> {
                pub targets: Vec<_0>,
                pub submitted_in: u32,
                pub suppressed: bool,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum RewardDestination<_0> {
                Staked,
                Stash,
                Controller,
                Account(_0),
                None,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct StakingLedger<_0, _1> {
                pub stash: _0,
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub active: _1,
                pub unlocking: Vec<runtime_types::pallet_staking::UnlockChunk<_1>>,
                pub claimed_rewards: Vec<u32>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct UnappliedSlash<_0, _1> {
                pub validator: _0,
                pub own: _1,
                pub others: Vec<(_0, _1)>,
                pub reporters: Vec<_0>,
                pub payout: _1,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct UnlockChunk<_0> {
                #[codec(compact)]
                pub value: _0,
                #[codec(compact)]
                pub era: u32,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ValidatorPrefs {
                #[codec(compact)]
                pub commission: ::subxt::sp_arithmetic::per_things::Perbill,
                pub blocked: bool,
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    set {
                        #[codec(compact)]
                        now: u64,
                    },
                }
            }
        }
        pub mod pallet_tips {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    report_awesome {
                        reason: Vec<u8>,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    retract_tip {
                        hash: ::subxt::sp_core::H256,
                    },
                    tip_new {
                        reason: Vec<u8>,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        #[codec(compact)]
                        tip_value: u128,
                    },
                    tip {
                        hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        tip_value: u128,
                    },
                    close_tip {
                        hash: ::subxt::sp_core::H256,
                    },
                    slash_tip {
                        hash: ::subxt::sp_core::H256,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    ReasonTooBig,
                    AlreadyKnown,
                    UnknownTip,
                    NotFinder,
                    StillOpen,
                    Premature,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    NewTip(::subxt::sp_core::H256),
                    TipClosing(::subxt::sp_core::H256),
                    TipClosed(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    TipRetracted(::subxt::sp_core::H256),
                    TipSlashed(
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        u128,
                    ),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OpenTip<_0, _1, _2, _3> {
                pub reason: _3,
                pub who: _0,
                pub finder: _0,
                pub deposit: _1,
                pub closes: Option<_2>,
                pub tips: Vec<(_0, _1)>,
                pub finders_fee: bool,
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct ChargeTransactionPayment(pub u128);
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1Ancient,
                V2,
            }
        }
        pub mod pallet_treasury {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    propose_spend {
                        #[codec(compact)]
                        value: u128,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    reject_proposal {
                        #[codec(compact)]
                        proposal_id: u32,
                    },
                    approve_proposal {
                        #[codec(compact)]
                        proposal_id: u32,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    InsufficientProposersBalance,
                    InvalidIndex,
                    TooManyApprovals,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    Proposed(u32),
                    Spending(u128),
                    Awarded(u32, u128, ::subxt::sp_core::crypto::AccountId32),
                    Rejected(u32, u128),
                    Burnt(u128),
                    Rollover(u128),
                    Deposit(u128),
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Proposal<_0, _1> {
                pub proposer: _0,
                pub value: _1,
                pub beneficiary: _0,
                pub bond: _1,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    batch {
                        calls: Vec<runtime_types::polkadot_runtime::Call>,
                    },
                    as_derivative {
                        index: u16,
                        call: std::boxed::Box<runtime_types::polkadot_runtime::Call>,
                    },
                    batch_all {
                        calls: Vec<runtime_types::polkadot_runtime::Call>,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    TooManyCalls,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    BatchInterrupted(u32, runtime_types::sp_runtime::DispatchError),
                    BatchCompleted,
                    ItemCompleted,
                }
            }
        }
        pub mod pallet_vesting {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Call {
                    vest,
                    vest_other {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    vested_transfer {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        schedule:
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                u128,
                                u32,
                            >,
                    },
                    force_vested_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        schedule:
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                u128,
                                u32,
                            >,
                    },
                    merge_schedules {
                        schedule1_index: u32,
                        schedule2_index: u32,
                    },
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Error {
                    NotVesting,
                    AtMaxVestingSchedules,
                    AmountLow,
                    ScheduleIndexOutOfBounds,
                    InvalidScheduleParams,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum Event {
                    VestingUpdated(::subxt::sp_core::crypto::AccountId32, u128),
                    VestingCompleted(::subxt::sp_core::crypto::AccountId32),
                }
            }
            pub mod vesting_info {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct VestingInfo<_0, _1> {
                    pub locked: _0,
                    pub per_block: _0,
                    pub starting_block: _1,
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V0,
                V1,
            }
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct CandidateHash(pub ::subxt::sp_core::H256);
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: Vec<u8>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: Vec<u8>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: Vec<u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct HeadData(pub Vec<u8>);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct HrmpChannelId {
                    pub sender: runtime_types::polkadot_parachain::primitives::Id,
                    pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                }
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct Id(pub u32);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ValidationCode(pub Vec<u8>);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ValidationCodeHash(pub ::subxt::sp_core::H256);
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v0 {
                use super::runtime_types;
                pub mod collator_app {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
                pub mod validator_app {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct ValidatorIndex(pub u32);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum ValidityAttestation {
                    Implicit(
                        runtime_types::polkadot_primitives::v0::validator_app::Signature,
                    ),
                    Explicit(
                        runtime_types::polkadot_primitives::v0::validator_app::Signature,
                    ),
                }
            }
            pub mod v1 {
                use super::runtime_types;
                pub mod assignment_app {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
                pub mod signed {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct UncheckedSigned < _0 , _1 > { pub payload : _0 , pub validator_index : runtime_types :: polkadot_primitives :: v0 :: ValidatorIndex , pub signature : runtime_types :: polkadot_primitives :: v0 :: validator_app :: Signature , # [codec (skip)] pub __subxt_unused_type_params : :: core :: marker :: PhantomData < _1 > , }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct AvailabilityBitfield(
                    pub ::subxt::bitvec::vec::BitVec<::subxt::bitvec::order::Lsb0, u8>,
                );
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct BackedCandidate<_0> {
                    pub candidate:
                        runtime_types::polkadot_primitives::v1::CommittedCandidateReceipt<
                            _0,
                        >,
                    pub validity_votes:
                        Vec<runtime_types::polkadot_primitives::v0::ValidityAttestation>,
                    pub validator_indices:
                        ::subxt::bitvec::vec::BitVec<::subxt::bitvec::order::Lsb0, u8>,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct CandidateCommitments<_0> {
                    pub upward_messages: Vec<Vec<u8>>,
                    pub horizontal_messages: Vec<
                        runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                            runtime_types::polkadot_parachain::primitives::Id,
                        >,
                    >,
                    pub new_validation_code: Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                    pub head_data:
                        runtime_types::polkadot_parachain::primitives::HeadData,
                    pub processed_downward_messages: _0,
                    pub hrmp_watermark: _0,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct CandidateDescriptor<_0> {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub relay_parent: _0,
                    pub collator:
                        runtime_types::polkadot_primitives::v0::collator_app::Public,
                    pub persisted_validation_data_hash: _0,
                    pub pov_hash: _0,
                    pub erasure_root: _0,
                    pub signature:
                        runtime_types::polkadot_primitives::v0::collator_app::Signature,
                    pub para_head: _0,
                    pub validation_code_hash:
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct CandidateReceipt<_0> {
                    pub descriptor:
                        runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub commitments_hash: _0,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct CommittedCandidateReceipt<_0> {
                    pub descriptor:
                        runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub commitments:
                        runtime_types::polkadot_primitives::v1::CandidateCommitments<u32>,
                }
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct CoreIndex(pub u32);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum CoreOccupied {
                    Parathread(runtime_types::polkadot_primitives::v1::ParathreadEntry),
                    Parachain,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum DisputeStatement {
                    Valid (runtime_types :: polkadot_primitives :: v1 :: ValidDisputeStatementKind ,) , Invalid (runtime_types :: polkadot_primitives :: v1 :: InvalidDisputeStatementKind ,) , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct DisputeStatementSet {
                    pub candidate_hash:
                        runtime_types::polkadot_core_primitives::CandidateHash,
                    pub session: u32,
                    pub statements: Vec<(
                        runtime_types::polkadot_primitives::v1::DisputeStatement,
                        runtime_types::polkadot_primitives::v0::ValidatorIndex,
                        runtime_types::polkadot_primitives::v0::validator_app::Signature,
                    )>,
                }
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct GroupIndex(pub u32);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct InherentData<_0> {
                    pub bitfields: Vec<
                        runtime_types::polkadot_primitives::v1::signed::UncheckedSigned<
                            runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                            runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                        >,
                    >,
                    pub backed_candidates: Vec<
                        runtime_types::polkadot_primitives::v1::BackedCandidate<
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    pub disputes:
                        Vec<runtime_types::polkadot_primitives::v1::DisputeStatementSet>,
                    pub parent_header: _0,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum InvalidDisputeStatementKind {
                    Explicit,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ParathreadClaim(
                    pub runtime_types::polkadot_parachain::primitives::Id,
                    pub runtime_types::polkadot_primitives::v0::collator_app::Public,
                );
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ParathreadEntry {
                    pub claim: runtime_types::polkadot_primitives::v1::ParathreadClaim,
                    pub retries: u32,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct SessionInfo {
                    pub validators: Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub discovery_keys:
                        Vec<runtime_types::sp_authority_discovery::app::Public>,
                    pub assignment_keys: Vec<
                        runtime_types::polkadot_primitives::v1::assignment_app::Public,
                    >,
                    pub validator_groups:
                        Vec<Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>>,
                    pub n_cores: u32,
                    pub zeroth_delay_tranche_width: u32,
                    pub relay_vrf_modulo_samples: u32,
                    pub n_delay_tranches: u32,
                    pub no_show_slots: u32,
                    pub needed_approvals: u32,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum UpgradeGoAhead {
                    Abort,
                    GoAhead,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum UpgradeRestriction {
                    Present,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum ValidDisputeStatementKind {
                    Explicit,
                    BackingSeconded(::subxt::sp_core::H256),
                    BackingValid(::subxt::sp_core::H256),
                    ApprovalChecking,
                }
            }
        }
        pub mod polkadot_runtime {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Call {
                System (runtime_types :: frame_system :: pallet :: Call ,) , Scheduler (runtime_types :: pallet_scheduler :: pallet :: Call ,) , Babe (runtime_types :: pallet_babe :: pallet :: Call ,) , Timestamp (runtime_types :: pallet_timestamp :: pallet :: Call ,) , Indices (runtime_types :: pallet_indices :: pallet :: Call ,) , Balances (runtime_types :: pallet_balances :: pallet :: Call ,) , Authorship (runtime_types :: pallet_authorship :: pallet :: Call ,) , Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Call ,) , Session (runtime_types :: pallet_session :: pallet :: Call ,) , Grandpa (runtime_types :: pallet_grandpa :: pallet :: Call ,) , ImOnline (runtime_types :: pallet_im_online :: pallet :: Call ,) , Democracy (runtime_types :: pallet_democracy :: pallet :: Call ,) , Council (runtime_types :: pallet_collective :: pallet :: Call ,) , TechnicalCommittee (runtime_types :: pallet_collective :: pallet :: Call ,) , PhragmenElection (runtime_types :: pallet_elections_phragmen :: pallet :: Call ,) , TechnicalMembership (runtime_types :: pallet_membership :: pallet :: Call ,) , Treasury (runtime_types :: pallet_treasury :: pallet :: Call ,) , Claims (runtime_types :: polkadot_runtime_common :: claims :: pallet :: Call ,) , Vesting (runtime_types :: pallet_vesting :: pallet :: Call ,) , Utility (runtime_types :: pallet_utility :: pallet :: Call ,) , Identity (runtime_types :: pallet_identity :: pallet :: Call ,) , Proxy (runtime_types :: pallet_proxy :: pallet :: Call ,) , Multisig (runtime_types :: pallet_multisig :: pallet :: Call ,) , Bounties (runtime_types :: pallet_bounties :: pallet :: Call ,) , Tips (runtime_types :: pallet_tips :: pallet :: Call ,) , ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Call ,) , Configuration (runtime_types :: polkadot_runtime_parachains :: configuration :: pallet :: Call ,) , ParasShared (runtime_types :: polkadot_runtime_parachains :: shared :: pallet :: Call ,) , ParaInclusion (runtime_types :: polkadot_runtime_parachains :: inclusion :: pallet :: Call ,) , ParaInherent (runtime_types :: polkadot_runtime_parachains :: paras_inherent :: pallet :: Call ,) , Paras (runtime_types :: polkadot_runtime_parachains :: paras :: pallet :: Call ,) , Initializer (runtime_types :: polkadot_runtime_parachains :: initializer :: pallet :: Call ,) , Dmp (runtime_types :: polkadot_runtime_parachains :: dmp :: pallet :: Call ,) , Ump (runtime_types :: polkadot_runtime_parachains :: ump :: pallet :: Call ,) , Hrmp (runtime_types :: polkadot_runtime_parachains :: hrmp :: pallet :: Call ,) , Registrar (runtime_types :: polkadot_runtime_common :: paras_registrar :: pallet :: Call ,) , Slots (runtime_types :: polkadot_runtime_common :: slots :: pallet :: Call ,) , Auctions (runtime_types :: polkadot_runtime_common :: auctions :: pallet :: Call ,) , Crowdloan (runtime_types :: polkadot_runtime_common :: crowdloan :: pallet :: Call ,) , }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Event {
                System (runtime_types :: frame_system :: pallet :: Event ,) , Scheduler (runtime_types :: pallet_scheduler :: pallet :: Event ,) , Indices (runtime_types :: pallet_indices :: pallet :: Event ,) , Balances (runtime_types :: pallet_balances :: pallet :: Event ,) , Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Event ,) , Offences (runtime_types :: pallet_offences :: pallet :: Event ,) , Session (runtime_types :: pallet_session :: pallet :: Event ,) , Grandpa (runtime_types :: pallet_grandpa :: pallet :: Event ,) , ImOnline (runtime_types :: pallet_im_online :: pallet :: Event ,) , Democracy (runtime_types :: pallet_democracy :: pallet :: Event ,) , Council (runtime_types :: pallet_collective :: pallet :: Event ,) , TechnicalCommittee (runtime_types :: pallet_collective :: pallet :: Event ,) , PhragmenElection (runtime_types :: pallet_elections_phragmen :: pallet :: Event ,) , TechnicalMembership (runtime_types :: pallet_membership :: pallet :: Event ,) , Treasury (runtime_types :: pallet_treasury :: pallet :: Event ,) , Claims (runtime_types :: polkadot_runtime_common :: claims :: pallet :: Event ,) , Vesting (runtime_types :: pallet_vesting :: pallet :: Event ,) , Utility (runtime_types :: pallet_utility :: pallet :: Event ,) , Identity (runtime_types :: pallet_identity :: pallet :: Event ,) , Proxy (runtime_types :: pallet_proxy :: pallet :: Event ,) , Multisig (runtime_types :: pallet_multisig :: pallet :: Event ,) , Bounties (runtime_types :: pallet_bounties :: pallet :: Event ,) , Tips (runtime_types :: pallet_tips :: pallet :: Event ,) , ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Event ,) , ParaInclusion (runtime_types :: polkadot_runtime_parachains :: inclusion :: pallet :: Event ,) , Paras (runtime_types :: polkadot_runtime_parachains :: paras :: pallet :: Event ,) , Ump (runtime_types :: polkadot_runtime_parachains :: ump :: pallet :: Event ,) , Hrmp (runtime_types :: polkadot_runtime_parachains :: hrmp :: pallet :: Event ,) , Registrar (runtime_types :: polkadot_runtime_common :: paras_registrar :: pallet :: Event ,) , Slots (runtime_types :: polkadot_runtime_common :: slots :: pallet :: Event ,) , Auctions (runtime_types :: polkadot_runtime_common :: auctions :: pallet :: Event ,) , Crowdloan (runtime_types :: polkadot_runtime_common :: crowdloan :: pallet :: Event ,) , }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct NposCompactSolution16 {
                votes1: Vec<(u32, u16)>,
                votes2: Vec<(
                    u32,
                    (u16, runtime_types::sp_arithmetic::per_things::PerU16),
                    u16,
                )>,
                votes3: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 2usize],
                    u16,
                )>,
                votes4: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 3usize],
                    u16,
                )>,
                votes5: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 4usize],
                    u16,
                )>,
                votes6: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 5usize],
                    u16,
                )>,
                votes7: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 6usize],
                    u16,
                )>,
                votes8: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 7usize],
                    u16,
                )>,
                votes9: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 8usize],
                    u16,
                )>,
                votes10: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 9usize],
                    u16,
                )>,
                votes11: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 10usize],
                    u16,
                )>,
                votes12: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 11usize],
                    u16,
                )>,
                votes13: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 12usize],
                    u16,
                )>,
                votes14: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 13usize],
                    u16,
                )>,
                votes15: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 14usize],
                    u16,
                )>,
                votes16: Vec<(
                    u32,
                    [(u16, runtime_types::sp_arithmetic::per_things::PerU16); 15usize],
                    u16,
                )>,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
                ParachainsOrigin(
                    runtime_types::polkadot_runtime_parachains::origin::pallet::Origin,
                ),
                Void(runtime_types::sp_core::Void),
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum ProxyType {
                Any,
                NonTransfer,
                Governance,
                Staking,
                IdentityJudgement,
                CancelProxy,
                Auction,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Runtime {}
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct SessionKeys {
                pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                pub babe: runtime_types::sp_consensus_babe::app::Public,
                pub im_online:
                    runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                pub para_validator:
                    runtime_types::polkadot_primitives::v0::validator_app::Public,
                pub para_assignment:
                    runtime_types::polkadot_primitives::v1::assignment_app::Public,
                pub authority_discovery:
                    runtime_types::sp_authority_discovery::app::Public,
            }
        }
        pub mod polkadot_runtime_common {
            use super::runtime_types;
            pub mod auctions {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        new_auction {
                            #[codec(compact)]
                            duration: u32,
                            #[codec(compact)]
                            lease_period_index: u32,
                        },
                        bid {
                            #[codec(compact)]
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            auction_index: u32,
                            #[codec(compact)]
                            first_slot: u32,
                            #[codec(compact)]
                            last_slot: u32,
                            #[codec(compact)]
                            amount: u128,
                        },
                        cancel_auction,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        AuctionInProgress,
                        LeasePeriodInPast,
                        ParaNotRegistered,
                        NotCurrentAuction,
                        NotAuction,
                        AuctionEnded,
                        AlreadyLeasedOut,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        AuctionStarted(u32, u32, u32),
                        AuctionClosed(u32),
                        Reserved(::subxt::sp_core::crypto::AccountId32, u128, u128),
                        Unreserved(::subxt::sp_core::crypto::AccountId32, u128),
                        ReserveConfiscated(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                            u128,
                        ),
                        BidAccepted(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            u128,
                            u32,
                            u32,
                        ),
                        WinningOffset(u32, u32),
                    }
                }
            }
            pub mod claims {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        claim { dest : :: subxt :: sp_core :: crypto :: AccountId32 , ethereum_signature : runtime_types :: polkadot_runtime_common :: claims :: EcdsaSignature , } , mint_claim { who : runtime_types :: polkadot_runtime_common :: claims :: EthereumAddress , value : u128 , vesting_schedule : Option < (u128 , u128 , u32 ,) > , statement : Option < runtime_types :: polkadot_runtime_common :: claims :: StatementKind > , } , claim_attest { dest : :: subxt :: sp_core :: crypto :: AccountId32 , ethereum_signature : runtime_types :: polkadot_runtime_common :: claims :: EcdsaSignature , statement : Vec < u8 > , } , attest { statement : Vec < u8 > , } , move_claim { old : runtime_types :: polkadot_runtime_common :: claims :: EthereumAddress , new : runtime_types :: polkadot_runtime_common :: claims :: EthereumAddress , maybe_preclaim : Option < :: subxt :: sp_core :: crypto :: AccountId32 > , } , }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        InvalidEthereumSignature,
                        SignerHasNoClaim,
                        SenderHasNoClaim,
                        PotUnderflow,
                        InvalidStatement,
                        VestedBalanceExists,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        Claimed (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: polkadot_runtime_common :: claims :: EthereumAddress , u128 ,) , }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct EcdsaSignature(pub [u8; 65usize]);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct EthereumAddress(pub [u8; 20usize]);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct PrevalidateAttests {}
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum StatementKind {
                    Regular,
                    Saft,
                }
            }
            pub mod crowdloan {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        create {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            cap: u128,
                            #[codec(compact)]
                            first_period: u32,
                            #[codec(compact)]
                            last_period: u32,
                            #[codec(compact)]
                            end: u32,
                            verifier: Option<runtime_types::sp_runtime::MultiSigner>,
                        },
                        contribute {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            value: u128,
                            signature: Option<runtime_types::sp_runtime::MultiSignature>,
                        },
                        withdraw {
                            who: ::subxt::sp_core::crypto::AccountId32,
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        refund {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        dissolve {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        edit {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            cap: u128,
                            #[codec(compact)]
                            first_period: u32,
                            #[codec(compact)]
                            last_period: u32,
                            #[codec(compact)]
                            end: u32,
                            verifier: Option<runtime_types::sp_runtime::MultiSigner>,
                        },
                        add_memo {
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            memo: Vec<u8>,
                        },
                        poke {
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        FirstPeriodInPast,
                        FirstPeriodTooFarInFuture,
                        LastPeriodBeforeFirstPeriod,
                        LastPeriodTooFarInFuture,
                        CannotEndInPast,
                        EndTooFarInFuture,
                        Overflow,
                        ContributionTooSmall,
                        InvalidParaId,
                        CapExceeded,
                        ContributionPeriodOver,
                        InvalidOrigin,
                        NotParachain,
                        LeaseActive,
                        BidOrLeaseActive,
                        FundNotEnded,
                        NoContributions,
                        NotReadyToDissolve,
                        InvalidSignature,
                        MemoTooLarge,
                        AlreadyInNewRaise,
                        VrfDelayInProgress,
                        NoLeasePeriod,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        Created(runtime_types::polkadot_parachain::primitives::Id),
                        Contributed(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            u128,
                        ),
                        Withdrew(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            u128,
                        ),
                        PartiallyRefunded(
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        AllRefunded(runtime_types::polkadot_parachain::primitives::Id),
                        Dissolved(runtime_types::polkadot_parachain::primitives::Id),
                        HandleBidResult(
                            runtime_types::polkadot_parachain::primitives::Id,
                            Result<(), runtime_types::sp_runtime::DispatchError>,
                        ),
                        Edited(runtime_types::polkadot_parachain::primitives::Id),
                        MemoUpdated(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            Vec<u8>,
                        ),
                        AddedToNewRaise(
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                    }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct FundInfo < _0 , _1 , _2 , _3 > { pub depositor : _0 , pub verifier : Option < runtime_types :: sp_runtime :: MultiSigner > , pub deposit : _1 , pub raised : _1 , pub end : _2 , pub cap : _1 , pub last_contribution : runtime_types :: polkadot_runtime_common :: crowdloan :: LastContribution < _2 > , pub first_period : _2 , pub last_period : _2 , pub trie_index : _2 , # [codec (skip)] pub __subxt_unused_type_params : :: core :: marker :: PhantomData < _3 > , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum LastContribution<_0> {
                    Never,
                    PreEnding(_0),
                    Ending(_0),
                }
            }
            pub mod paras_registrar {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        register { id : runtime_types :: polkadot_parachain :: primitives :: Id , genesis_head : runtime_types :: polkadot_parachain :: primitives :: HeadData , validation_code : runtime_types :: polkadot_parachain :: primitives :: ValidationCode , } , force_register { who : :: subxt :: sp_core :: crypto :: AccountId32 , deposit : u128 , id : runtime_types :: polkadot_parachain :: primitives :: Id , genesis_head : runtime_types :: polkadot_parachain :: primitives :: HeadData , validation_code : runtime_types :: polkadot_parachain :: primitives :: ValidationCode , } , deregister { id : runtime_types :: polkadot_parachain :: primitives :: Id , } , swap { id : runtime_types :: polkadot_parachain :: primitives :: Id , other : runtime_types :: polkadot_parachain :: primitives :: Id , } , force_remove_lock { para : runtime_types :: polkadot_parachain :: primitives :: Id , } , reserve , }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        NotRegistered,
                        AlreadyRegistered,
                        NotOwner,
                        CodeTooLarge,
                        HeadDataTooLarge,
                        NotParachain,
                        NotParathread,
                        CannotDeregister,
                        CannotDowngrade,
                        CannotUpgrade,
                        ParaLocked,
                        NotReserved,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        Registered(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                        Deregistered(runtime_types::polkadot_parachain::primitives::Id),
                        Reserved(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                    }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ParaInfo<_0, _1> {
                    pub manager: _0,
                    pub deposit: _1,
                    pub locked: bool,
                }
            }
            pub mod slots {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        force_lease {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            leaser: ::subxt::sp_core::crypto::AccountId32,
                            amount: u128,
                            period_begin: u32,
                            period_count: u32,
                        },
                        clear_all_leases {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        trigger_onboard {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        ParaNotOnboarding,
                        LeaseError,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        NewLeasePeriod(u32),
                        Leased(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                            u32,
                            u32,
                            u128,
                            u128,
                        ),
                    }
                }
            }
        }
        pub mod polkadot_runtime_parachains {
            use super::runtime_types;
            pub mod configuration {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        set_validation_upgrade_frequency { new: u32 },
                        set_validation_upgrade_delay { new: u32 },
                        set_code_retention_period { new: u32 },
                        set_max_code_size { new: u32 },
                        set_max_pov_size { new: u32 },
                        set_max_head_data_size { new: u32 },
                        set_parathread_cores { new: u32 },
                        set_parathread_retries { new: u32 },
                        set_group_rotation_frequency { new: u32 },
                        set_chain_availability_period { new: u32 },
                        set_thread_availability_period { new: u32 },
                        set_scheduling_lookahead { new: u32 },
                        set_max_validators_per_core { new: Option<u32> },
                        set_max_validators { new: Option<u32> },
                        set_dispute_period { new: u32 },
                        set_dispute_post_conclusion_acceptance_period { new: u32 },
                        set_dispute_max_spam_slots { new: u32 },
                        set_dispute_conclusion_by_time_out_period { new: u32 },
                        set_no_show_slots { new: u32 },
                        set_n_delay_tranches { new: u32 },
                        set_zeroth_delay_tranche_width { new: u32 },
                        set_needed_approvals { new: u32 },
                        set_relay_vrf_modulo_samples { new: u32 },
                        set_max_upward_queue_count { new: u32 },
                        set_max_upward_queue_size { new: u32 },
                        set_max_downward_message_size { new: u32 },
                        set_ump_service_total_weight { new: u64 },
                        set_max_upward_message_size { new: u32 },
                        set_max_upward_message_num_per_candidate { new: u32 },
                        set_hrmp_open_request_ttl { new: u32 },
                        set_hrmp_sender_deposit { new: u128 },
                        set_hrmp_recipient_deposit { new: u128 },
                        set_hrmp_channel_max_capacity { new: u32 },
                        set_hrmp_channel_max_total_size { new: u32 },
                        set_hrmp_max_parachain_inbound_channels { new: u32 },
                        set_hrmp_max_parathread_inbound_channels { new: u32 },
                        set_hrmp_channel_max_message_size { new: u32 },
                        set_hrmp_max_parachain_outbound_channels { new: u32 },
                        set_hrmp_max_parathread_outbound_channels { new: u32 },
                        set_hrmp_max_message_num_per_candidate { new: u32 },
                        set_ump_max_individual_weight { new: u64 },
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        InvalidNewValue,
                    }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct HostConfiguration<_0> {
                    pub max_code_size: _0,
                    pub max_head_data_size: _0,
                    pub max_upward_queue_count: _0,
                    pub max_upward_queue_size: _0,
                    pub max_upward_message_size: _0,
                    pub max_upward_message_num_per_candidate: _0,
                    pub hrmp_max_message_num_per_candidate: _0,
                    pub validation_upgrade_frequency: _0,
                    pub validation_upgrade_delay: _0,
                    pub max_pov_size: _0,
                    pub max_downward_message_size: _0,
                    pub ump_service_total_weight: u64,
                    pub hrmp_max_parachain_outbound_channels: _0,
                    pub hrmp_max_parathread_outbound_channels: _0,
                    pub hrmp_sender_deposit: u128,
                    pub hrmp_recipient_deposit: u128,
                    pub hrmp_channel_max_capacity: _0,
                    pub hrmp_channel_max_total_size: _0,
                    pub hrmp_max_parachain_inbound_channels: _0,
                    pub hrmp_max_parathread_inbound_channels: _0,
                    pub hrmp_channel_max_message_size: _0,
                    pub code_retention_period: _0,
                    pub parathread_cores: _0,
                    pub parathread_retries: _0,
                    pub group_rotation_frequency: _0,
                    pub chain_availability_period: _0,
                    pub thread_availability_period: _0,
                    pub scheduling_lookahead: _0,
                    pub max_validators_per_core: Option<_0>,
                    pub max_validators: Option<_0>,
                    pub dispute_period: _0,
                    pub dispute_post_conclusion_acceptance_period: _0,
                    pub dispute_max_spam_slots: _0,
                    pub dispute_conclusion_by_time_out_period: _0,
                    pub no_show_slots: _0,
                    pub n_delay_tranches: _0,
                    pub zeroth_delay_tranche_width: _0,
                    pub needed_approvals: _0,
                    pub relay_vrf_modulo_samples: _0,
                    pub ump_max_individual_weight: u64,
                }
            }
            pub mod dmp {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {}
                }
            }
            pub mod hrmp {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        hrmp_init_open_channel { recipient : runtime_types :: polkadot_parachain :: primitives :: Id , proposed_max_capacity : u32 , proposed_max_message_size : u32 , } , hrmp_accept_open_channel { sender : runtime_types :: polkadot_parachain :: primitives :: Id , } , hrmp_close_channel { channel_id : runtime_types :: polkadot_parachain :: primitives :: HrmpChannelId , } , force_clean_hrmp { para : runtime_types :: polkadot_parachain :: primitives :: Id , } , force_process_hrmp_open , force_process_hrmp_close , hrmp_cancel_open_request { channel_id : runtime_types :: polkadot_parachain :: primitives :: HrmpChannelId , } , }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        OpenHrmpChannelToSelf,
                        OpenHrmpChannelInvalidRecipient,
                        OpenHrmpChannelZeroCapacity,
                        OpenHrmpChannelCapacityExceedsLimit,
                        OpenHrmpChannelZeroMessageSize,
                        OpenHrmpChannelMessageSizeExceedsLimit,
                        OpenHrmpChannelAlreadyExists,
                        OpenHrmpChannelAlreadyRequested,
                        OpenHrmpChannelLimitExceeded,
                        AcceptHrmpChannelDoesntExist,
                        AcceptHrmpChannelAlreadyConfirmed,
                        AcceptHrmpChannelLimitExceeded,
                        CloseHrmpChannelUnauthorized,
                        CloseHrmpChannelDoesntExist,
                        CloseHrmpChannelAlreadyUnderway,
                        CancelHrmpOpenChannelUnauthorized,
                        OpenHrmpChannelDoesntExist,
                        OpenHrmpChannelAlreadyConfirmed,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        OpenChannelRequested(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::Id,
                            u32,
                            u32,
                        ),
                        OpenChannelCanceled(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        ),
                        OpenChannelAccepted(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        ChannelClosed(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        ),
                    }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct HrmpChannel {
                    pub max_capacity: u32,
                    pub max_total_size: u32,
                    pub max_message_size: u32,
                    pub msg_count: u32,
                    pub total_size: u32,
                    pub mqc_head: Option<::subxt::sp_core::H256>,
                    pub sender_deposit: u128,
                    pub recipient_deposit: u128,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct HrmpOpenChannelRequest {
                    pub confirmed: bool,
                    pub _age: u32,
                    pub sender_deposit: u128,
                    pub max_message_size: u32,
                    pub max_capacity: u32,
                    pub max_total_size: u32,
                }
            }
            pub mod inclusion {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {}
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        WrongBitfieldSize,
                        BitfieldDuplicateOrUnordered,
                        ValidatorIndexOutOfBounds,
                        InvalidBitfieldSignature,
                        UnscheduledCandidate,
                        CandidateScheduledBeforeParaFree,
                        WrongCollator,
                        ScheduledOutOfOrder,
                        HeadDataTooLarge,
                        PrematureCodeUpgrade,
                        NewCodeTooLarge,
                        CandidateNotInParentContext,
                        UnoccupiedBitInBitfield,
                        InvalidGroupIndex,
                        InsufficientBacking,
                        InvalidBacking,
                        NotCollatorSigned,
                        ValidationDataHashMismatch,
                        InternalError,
                        IncorrectDownwardMessageHandling,
                        InvalidUpwardMessages,
                        HrmpWatermarkMishandling,
                        InvalidOutboundHrmp,
                        InvalidValidationCodeHash,
                        ParaHeadMismatch,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        CandidateBacked(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                            runtime_types::polkadot_primitives::v1::GroupIndex,
                        ),
                        CandidateIncluded(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                            runtime_types::polkadot_primitives::v1::GroupIndex,
                        ),
                        CandidateTimedOut(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                        ),
                    }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct AvailabilityBitfieldRecord<_0> {
                    pub bitfield:
                        runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                    pub submitted_at: _0,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct CandidatePendingAvailability<_0, _1> {
                    pub core: runtime_types::polkadot_primitives::v1::CoreIndex,
                    pub hash: runtime_types::polkadot_core_primitives::CandidateHash,
                    pub descriptor:
                        runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub availability_votes:
                        ::subxt::bitvec::vec::BitVec<::subxt::bitvec::order::Lsb0, u8>,
                    pub backers:
                        ::subxt::bitvec::vec::BitVec<::subxt::bitvec::order::Lsb0, u8>,
                    pub relay_parent_number: _1,
                    pub backed_in_number: _1,
                    pub backing_group: runtime_types::polkadot_primitives::v1::GroupIndex,
                }
            }
            pub mod initializer {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        force_approve { up_to: u32 },
                    }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct BufferedSessionChange {
                    pub validators: Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub queued: Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub session_index: u32,
                }
            }
            pub mod origin {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Origin {
                        Parachain(runtime_types::polkadot_parachain::primitives::Id),
                    }
                }
            }
            pub mod paras {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        force_set_current_code { para : runtime_types :: polkadot_parachain :: primitives :: Id , new_code : runtime_types :: polkadot_parachain :: primitives :: ValidationCode , } , force_set_current_head { para : runtime_types :: polkadot_parachain :: primitives :: Id , new_head : runtime_types :: polkadot_parachain :: primitives :: HeadData , } , force_schedule_code_upgrade { para : runtime_types :: polkadot_parachain :: primitives :: Id , new_code : runtime_types :: polkadot_parachain :: primitives :: ValidationCode , relay_parent_number : u32 , } , force_note_new_head { para : runtime_types :: polkadot_parachain :: primitives :: Id , new_head : runtime_types :: polkadot_parachain :: primitives :: HeadData , } , force_queue_action { para : runtime_types :: polkadot_parachain :: primitives :: Id , } , }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        NotRegistered,
                        CannotOnboard,
                        CannotOffboard,
                        CannotUpgrade,
                        CannotDowngrade,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        CurrentCodeUpdated(
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        CurrentHeadUpdated(
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        CodeUpgradeScheduled(
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        NewHeadNoted(runtime_types::polkadot_parachain::primitives::Id),
                        ActionQueued(
                            runtime_types::polkadot_parachain::primitives::Id,
                            u32,
                        ),
                    }
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ParaGenesisArgs {
                    pub genesis_head:
                        runtime_types::polkadot_parachain::primitives::HeadData,
                    pub validation_code:
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    pub parachain: bool,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum ParaLifecycle {
                    Onboarding,
                    Parathread,
                    Parachain,
                    UpgradingParathread,
                    DowngradingParachain,
                    OffboardingParathread,
                    OffboardingParachain,
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ParaPastCodeMeta < _0 > { pub upgrade_times : Vec < runtime_types :: polkadot_runtime_parachains :: paras :: ReplacementTimes < _0 > > , pub last_pruned : Option < _0 > , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ReplacementTimes<_0> {
                    pub expected_at: _0,
                    pub activated_at: _0,
                }
            }
            pub mod paras_inherent {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        enter {
                            data: runtime_types::polkadot_primitives::v1::InherentData<
                                runtime_types::sp_runtime::generic::header::Header<
                                    u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                            >,
                        },
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        TooManyInclusionInherents,
                        InvalidParentHeader,
                        CandidateConcludedInvalid,
                    }
                }
            }
            pub mod scheduler {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum AssignmentKind {
                    Parachain,
                    Parathread(
                        runtime_types::polkadot_primitives::v0::collator_app::Public,
                        u32,
                    ),
                }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct CoreAssignment { pub core : runtime_types :: polkadot_primitives :: v1 :: CoreIndex , pub para_id : runtime_types :: polkadot_parachain :: primitives :: Id , pub kind : runtime_types :: polkadot_runtime_parachains :: scheduler :: AssignmentKind , pub group_idx : runtime_types :: polkadot_primitives :: v1 :: GroupIndex , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ParathreadClaimQueue { pub queue : Vec < runtime_types :: polkadot_runtime_parachains :: scheduler :: QueuedParathread > , pub next_core_offset : u32 , }
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct QueuedParathread {
                    pub claim: runtime_types::polkadot_primitives::v1::ParathreadEntry,
                    pub core_offset: u32,
                }
            }
            pub mod shared {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {}
                }
            }
            pub mod ump {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Call {
                        service_overweight { index: u64, weight_limit: u64 },
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        UnknownMessageIndex,
                        WeightOverLimit,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Event {
                        InvalidFormat([u8; 32usize]),
                        UnsupportedVersion([u8; 32usize]),
                        ExecutedUpward(
                            [u8; 32usize],
                            runtime_types::xcm::v2::traits::Outcome,
                        ),
                        WeightExhausted([u8; 32usize], u64, u64),
                        UpwardMessagesReceived(
                            runtime_types::polkadot_parachain::primitives::Id,
                            u32,
                            u32,
                        ),
                        OverweightEnqueued(
                            runtime_types::polkadot_parachain::primitives::Id,
                            [u8; 32usize],
                            u64,
                            u64,
                        ),
                        OverweightServiced(u64, u64),
                    }
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct H256(pub [u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct FixedU128(pub u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct PerU16(pub u16);
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct Perbill(pub u32);
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct Percent(pub u8);
                #[derive(
                    Debug,
                    Eq,
                    PartialEq,
                    :: codec :: Encode,
                    :: codec :: Decode,
                    :: codec :: CompactAs,
                )]
                pub struct Permill(pub u32);
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum NextConfigDescriptor {
                    V1 {
                        c: (u64, u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum AllowedSlots {
                PrimarySlots,
                PrimaryAndSecondaryPlainSlots,
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct BabeEpochConfiguration {
                pub c: (u64, u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                Debug,
                Eq,
                PartialEq,
                :: codec :: Encode,
                :: codec :: Decode,
                :: codec :: CompactAs,
            )]
            pub struct Slot(pub u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod changes_trie {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: u32,
                    pub digest_levels: u32,
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct AccountId32(pub [u8; 32usize]);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct KeyTypeId(pub [u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Public(pub [u8; 33usize]);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Signature(pub [u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Public(pub [u8; 32usize]);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Signature(pub [u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct OpaqueMultiaddr(pub Vec<u8>);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Public(pub [u8; 32usize]);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Signature(pub [u8; 64usize]);
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct OpaquePeerId(pub Vec<u8>);
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: u64,
                pub equivocation:
                    runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_npos_elections {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct Support<_0> {
                pub total: u128,
                pub voters: Vec<(_0, u128)>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum ChangesTrieSignal {
                        NewConfiguration (Option < runtime_types :: sp_core :: changes_trie :: ChangesTrieConfiguration > ,) , }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct Digest<_0> {
                        pub logs: Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem<_0>,
                        >,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum DigestItem<_0> {
                        ChangesTrieRoot(_0),
                        PreRuntime([u8; 4usize], Vec<u8>),
                        Consensus([u8; 4usize], Vec<u8>),
                        Seal([u8; 4usize], Vec<u8>),
                        ChangesTrieSignal(
                            runtime_types::sp_runtime::generic::digest::ChangesTrieSignal,
                        ),
                        Other(Vec<u8>),
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Era {
                        Immortal,
                        Mortal1(u8),
                        Mortal2(u8),
                        Mortal3(u8),
                        Mortal4(u8),
                        Mortal5(u8),
                        Mortal6(u8),
                        Mortal7(u8),
                        Mortal8(u8),
                        Mortal9(u8),
                        Mortal10(u8),
                        Mortal11(u8),
                        Mortal12(u8),
                        Mortal13(u8),
                        Mortal14(u8),
                        Mortal15(u8),
                        Mortal16(u8),
                        Mortal17(u8),
                        Mortal18(u8),
                        Mortal19(u8),
                        Mortal20(u8),
                        Mortal21(u8),
                        Mortal22(u8),
                        Mortal23(u8),
                        Mortal24(u8),
                        Mortal25(u8),
                        Mortal26(u8),
                        Mortal27(u8),
                        Mortal28(u8),
                        Mortal29(u8),
                        Mortal30(u8),
                        Mortal31(u8),
                        Mortal32(u8),
                        Mortal33(u8),
                        Mortal34(u8),
                        Mortal35(u8),
                        Mortal36(u8),
                        Mortal37(u8),
                        Mortal38(u8),
                        Mortal39(u8),
                        Mortal40(u8),
                        Mortal41(u8),
                        Mortal42(u8),
                        Mortal43(u8),
                        Mortal44(u8),
                        Mortal45(u8),
                        Mortal46(u8),
                        Mortal47(u8),
                        Mortal48(u8),
                        Mortal49(u8),
                        Mortal50(u8),
                        Mortal51(u8),
                        Mortal52(u8),
                        Mortal53(u8),
                        Mortal54(u8),
                        Mortal55(u8),
                        Mortal56(u8),
                        Mortal57(u8),
                        Mortal58(u8),
                        Mortal59(u8),
                        Mortal60(u8),
                        Mortal61(u8),
                        Mortal62(u8),
                        Mortal63(u8),
                        Mortal64(u8),
                        Mortal65(u8),
                        Mortal66(u8),
                        Mortal67(u8),
                        Mortal68(u8),
                        Mortal69(u8),
                        Mortal70(u8),
                        Mortal71(u8),
                        Mortal72(u8),
                        Mortal73(u8),
                        Mortal74(u8),
                        Mortal75(u8),
                        Mortal76(u8),
                        Mortal77(u8),
                        Mortal78(u8),
                        Mortal79(u8),
                        Mortal80(u8),
                        Mortal81(u8),
                        Mortal82(u8),
                        Mortal83(u8),
                        Mortal84(u8),
                        Mortal85(u8),
                        Mortal86(u8),
                        Mortal87(u8),
                        Mortal88(u8),
                        Mortal89(u8),
                        Mortal90(u8),
                        Mortal91(u8),
                        Mortal92(u8),
                        Mortal93(u8),
                        Mortal94(u8),
                        Mortal95(u8),
                        Mortal96(u8),
                        Mortal97(u8),
                        Mortal98(u8),
                        Mortal99(u8),
                        Mortal100(u8),
                        Mortal101(u8),
                        Mortal102(u8),
                        Mortal103(u8),
                        Mortal104(u8),
                        Mortal105(u8),
                        Mortal106(u8),
                        Mortal107(u8),
                        Mortal108(u8),
                        Mortal109(u8),
                        Mortal110(u8),
                        Mortal111(u8),
                        Mortal112(u8),
                        Mortal113(u8),
                        Mortal114(u8),
                        Mortal115(u8),
                        Mortal116(u8),
                        Mortal117(u8),
                        Mortal118(u8),
                        Mortal119(u8),
                        Mortal120(u8),
                        Mortal121(u8),
                        Mortal122(u8),
                        Mortal123(u8),
                        Mortal124(u8),
                        Mortal125(u8),
                        Mortal126(u8),
                        Mortal127(u8),
                        Mortal128(u8),
                        Mortal129(u8),
                        Mortal130(u8),
                        Mortal131(u8),
                        Mortal132(u8),
                        Mortal133(u8),
                        Mortal134(u8),
                        Mortal135(u8),
                        Mortal136(u8),
                        Mortal137(u8),
                        Mortal138(u8),
                        Mortal139(u8),
                        Mortal140(u8),
                        Mortal141(u8),
                        Mortal142(u8),
                        Mortal143(u8),
                        Mortal144(u8),
                        Mortal145(u8),
                        Mortal146(u8),
                        Mortal147(u8),
                        Mortal148(u8),
                        Mortal149(u8),
                        Mortal150(u8),
                        Mortal151(u8),
                        Mortal152(u8),
                        Mortal153(u8),
                        Mortal154(u8),
                        Mortal155(u8),
                        Mortal156(u8),
                        Mortal157(u8),
                        Mortal158(u8),
                        Mortal159(u8),
                        Mortal160(u8),
                        Mortal161(u8),
                        Mortal162(u8),
                        Mortal163(u8),
                        Mortal164(u8),
                        Mortal165(u8),
                        Mortal166(u8),
                        Mortal167(u8),
                        Mortal168(u8),
                        Mortal169(u8),
                        Mortal170(u8),
                        Mortal171(u8),
                        Mortal172(u8),
                        Mortal173(u8),
                        Mortal174(u8),
                        Mortal175(u8),
                        Mortal176(u8),
                        Mortal177(u8),
                        Mortal178(u8),
                        Mortal179(u8),
                        Mortal180(u8),
                        Mortal181(u8),
                        Mortal182(u8),
                        Mortal183(u8),
                        Mortal184(u8),
                        Mortal185(u8),
                        Mortal186(u8),
                        Mortal187(u8),
                        Mortal188(u8),
                        Mortal189(u8),
                        Mortal190(u8),
                        Mortal191(u8),
                        Mortal192(u8),
                        Mortal193(u8),
                        Mortal194(u8),
                        Mortal195(u8),
                        Mortal196(u8),
                        Mortal197(u8),
                        Mortal198(u8),
                        Mortal199(u8),
                        Mortal200(u8),
                        Mortal201(u8),
                        Mortal202(u8),
                        Mortal203(u8),
                        Mortal204(u8),
                        Mortal205(u8),
                        Mortal206(u8),
                        Mortal207(u8),
                        Mortal208(u8),
                        Mortal209(u8),
                        Mortal210(u8),
                        Mortal211(u8),
                        Mortal212(u8),
                        Mortal213(u8),
                        Mortal214(u8),
                        Mortal215(u8),
                        Mortal216(u8),
                        Mortal217(u8),
                        Mortal218(u8),
                        Mortal219(u8),
                        Mortal220(u8),
                        Mortal221(u8),
                        Mortal222(u8),
                        Mortal223(u8),
                        Mortal224(u8),
                        Mortal225(u8),
                        Mortal226(u8),
                        Mortal227(u8),
                        Mortal228(u8),
                        Mortal229(u8),
                        Mortal230(u8),
                        Mortal231(u8),
                        Mortal232(u8),
                        Mortal233(u8),
                        Mortal234(u8),
                        Mortal235(u8),
                        Mortal236(u8),
                        Mortal237(u8),
                        Mortal238(u8),
                        Mortal239(u8),
                        Mortal240(u8),
                        Mortal241(u8),
                        Mortal242(u8),
                        Mortal243(u8),
                        Mortal244(u8),
                        Mortal245(u8),
                        Mortal246(u8),
                        Mortal247(u8),
                        Mortal248(u8),
                        Mortal249(u8),
                        Mortal250(u8),
                        Mortal251(u8),
                        Mortal252(u8),
                        Mortal253(u8),
                        Mortal254(u8),
                        Mortal255(u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
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
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        Vec<u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub enum MultiAddress<_0, _1> {
                    Id(_0),
                    Index(_1),
                    Raw(Vec<u8>),
                    Address32([u8; 32usize]),
                    Address20([u8; 20usize]),
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct BlakeTwo256 {}
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum ArithmeticError {
                Underflow,
                Overflow,
                DivisionByZero,
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum DispatchError {
                Other,
                CannotLookup,
                BadOrigin,
                Module { index: u8, error: u8 },
                ConsumerRemaining,
                NoProviders,
                Token(runtime_types::sp_runtime::TokenError),
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum MultiSignature {
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub enum MultiSigner {
                Ed25519(runtime_types::sp_core::ed25519::Public),
                Sr25519(runtime_types::sp_core::sr25519::Public),
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
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
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct MembershipProof {
                pub session: u32,
                pub trie_nodes: Vec<Vec<u8>>,
                pub validator_count: u32,
            }
        }
        pub mod sp_staking {
            use super::runtime_types;
            pub mod offence {
                use super::runtime_types;
                #[derive(
                    Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                )]
                pub struct OffenceDetails<_0, _1> {
                    pub offender: _1,
                    pub reporters: Vec<_0>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode)]
            pub struct RuntimeVersion {
                pub spec_name: String,
                pub impl_name: String,
                pub authoring_version: u32,
                pub spec_version: u32,
                pub impl_version: u32,
                pub apis: Vec<([u8; 8usize], u32)>,
                pub transaction_version: u32,
            }
        }
        pub mod xcm {
            use super::runtime_types;
            pub mod v2 {
                use super::runtime_types;
                pub mod traits {
                    use super::runtime_types;
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Error {
                        Overflow,
                        Unimplemented,
                        UntrustedReserveLocation,
                        UntrustedTeleportLocation,
                        MultiLocationFull,
                        MultiLocationNotInvertible,
                        BadOrigin,
                        InvalidLocation,
                        AssetNotFound,
                        FailedToTransactAsset,
                        NotWithdrawable,
                        LocationCannotHold,
                        ExceedsMaxMessageSize,
                        DestinationUnsupported,
                        Transport,
                        Unroutable,
                        UnknownClaim,
                        FailedToDecode,
                        TooMuchWeightRequired,
                        NotHoldingFees,
                        TooExpensive,
                        Trap(u64),
                        UnhandledXcmVersion,
                        WeightLimitReached(u64),
                        Barrier,
                        WeightNotComputable,
                    }
                    #[derive(
                        Debug, Eq, PartialEq, :: codec :: Encode, :: codec :: Decode,
                    )]
                    pub enum Outcome {
                        Complete(u64),
                        Incomplete(u64, runtime_types::xcm::v2::traits::Error),
                        Error(runtime_types::xcm::v2::traits::Error),
                    }
                }
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
        pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
            scheduler::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
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
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn staking(&self) -> staking::storage::StorageApi<'a, T> {
            staking::storage::StorageApi::new(self.client)
        }
        pub fn offences(&self) -> offences::storage::StorageApi<'a, T> {
            offences::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
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
        pub fn phragmen_election(&self) -> phragmen_election::storage::StorageApi<'a, T> {
            phragmen_election::storage::StorageApi::new(self.client)
        }
        pub fn technical_membership(
            &self,
        ) -> technical_membership::storage::StorageApi<'a, T> {
            technical_membership::storage::StorageApi::new(self.client)
        }
        pub fn treasury(&self) -> treasury::storage::StorageApi<'a, T> {
            treasury::storage::StorageApi::new(self.client)
        }
        pub fn claims(&self) -> claims::storage::StorageApi<'a, T> {
            claims::storage::StorageApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::storage::StorageApi<'a, T> {
            vesting::storage::StorageApi::new(self.client)
        }
        pub fn identity(&self) -> identity::storage::StorageApi<'a, T> {
            identity::storage::StorageApi::new(self.client)
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
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::storage::StorageApi<'a, T> {
            election_provider_multi_phase::storage::StorageApi::new(self.client)
        }
        pub fn configuration(&self) -> configuration::storage::StorageApi<'a, T> {
            configuration::storage::StorageApi::new(self.client)
        }
        pub fn paras_shared(&self) -> paras_shared::storage::StorageApi<'a, T> {
            paras_shared::storage::StorageApi::new(self.client)
        }
        pub fn para_inclusion(&self) -> para_inclusion::storage::StorageApi<'a, T> {
            para_inclusion::storage::StorageApi::new(self.client)
        }
        pub fn para_inherent(&self) -> para_inherent::storage::StorageApi<'a, T> {
            para_inherent::storage::StorageApi::new(self.client)
        }
        pub fn para_scheduler(&self) -> para_scheduler::storage::StorageApi<'a, T> {
            para_scheduler::storage::StorageApi::new(self.client)
        }
        pub fn paras(&self) -> paras::storage::StorageApi<'a, T> {
            paras::storage::StorageApi::new(self.client)
        }
        pub fn initializer(&self) -> initializer::storage::StorageApi<'a, T> {
            initializer::storage::StorageApi::new(self.client)
        }
        pub fn dmp(&self) -> dmp::storage::StorageApi<'a, T> {
            dmp::storage::StorageApi::new(self.client)
        }
        pub fn ump(&self) -> ump::storage::StorageApi<'a, T> {
            ump::storage::StorageApi::new(self.client)
        }
        pub fn hrmp(&self) -> hrmp::storage::StorageApi<'a, T> {
            hrmp::storage::StorageApi::new(self.client)
        }
        pub fn para_session_info(&self) -> para_session_info::storage::StorageApi<'a, T> {
            para_session_info::storage::StorageApi::new(self.client)
        }
        pub fn registrar(&self) -> registrar::storage::StorageApi<'a, T> {
            registrar::storage::StorageApi::new(self.client)
        }
        pub fn slots(&self) -> slots::storage::StorageApi<'a, T> {
            slots::storage::StorageApi::new(self.client)
        }
        pub fn auctions(&self) -> auctions::storage::StorageApi<'a, T> {
            auctions::storage::StorageApi::new(self.client)
        }
        pub fn crowdloan(&self) -> crowdloan::storage::StorageApi<'a, T> {
            crowdloan::storage::StorageApi::new(self.client)
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
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T> {
            scheduler::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn indices(&self) -> indices::calls::TransactionApi<'a, T> {
            indices::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn staking(&self) -> staking::calls::TransactionApi<'a, T> {
            staking::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T> {
            im_online::calls::TransactionApi::new(self.client)
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
        pub fn phragmen_election(
            &self,
        ) -> phragmen_election::calls::TransactionApi<'a, T> {
            phragmen_election::calls::TransactionApi::new(self.client)
        }
        pub fn technical_membership(
            &self,
        ) -> technical_membership::calls::TransactionApi<'a, T> {
            technical_membership::calls::TransactionApi::new(self.client)
        }
        pub fn treasury(&self) -> treasury::calls::TransactionApi<'a, T> {
            treasury::calls::TransactionApi::new(self.client)
        }
        pub fn claims(&self) -> claims::calls::TransactionApi<'a, T> {
            claims::calls::TransactionApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::calls::TransactionApi<'a, T> {
            vesting::calls::TransactionApi::new(self.client)
        }
        pub fn utility(&self) -> utility::calls::TransactionApi<'a, T> {
            utility::calls::TransactionApi::new(self.client)
        }
        pub fn identity(&self) -> identity::calls::TransactionApi<'a, T> {
            identity::calls::TransactionApi::new(self.client)
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
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::calls::TransactionApi<'a, T> {
            election_provider_multi_phase::calls::TransactionApi::new(self.client)
        }
        pub fn configuration(&self) -> configuration::calls::TransactionApi<'a, T> {
            configuration::calls::TransactionApi::new(self.client)
        }
        pub fn paras_shared(&self) -> paras_shared::calls::TransactionApi<'a, T> {
            paras_shared::calls::TransactionApi::new(self.client)
        }
        pub fn para_inclusion(&self) -> para_inclusion::calls::TransactionApi<'a, T> {
            para_inclusion::calls::TransactionApi::new(self.client)
        }
        pub fn para_inherent(&self) -> para_inherent::calls::TransactionApi<'a, T> {
            para_inherent::calls::TransactionApi::new(self.client)
        }
        pub fn paras(&self) -> paras::calls::TransactionApi<'a, T> {
            paras::calls::TransactionApi::new(self.client)
        }
        pub fn initializer(&self) -> initializer::calls::TransactionApi<'a, T> {
            initializer::calls::TransactionApi::new(self.client)
        }
        pub fn dmp(&self) -> dmp::calls::TransactionApi<'a, T> {
            dmp::calls::TransactionApi::new(self.client)
        }
        pub fn ump(&self) -> ump::calls::TransactionApi<'a, T> {
            ump::calls::TransactionApi::new(self.client)
        }
        pub fn hrmp(&self) -> hrmp::calls::TransactionApi<'a, T> {
            hrmp::calls::TransactionApi::new(self.client)
        }
        pub fn registrar(&self) -> registrar::calls::TransactionApi<'a, T> {
            registrar::calls::TransactionApi::new(self.client)
        }
        pub fn slots(&self) -> slots::calls::TransactionApi<'a, T> {
            slots::calls::TransactionApi::new(self.client)
        }
        pub fn auctions(&self) -> auctions::calls::TransactionApi<'a, T> {
            auctions::calls::TransactionApi::new(self.client)
        }
        pub fn crowdloan(&self) -> crowdloan::calls::TransactionApi<'a, T> {
            crowdloan::calls::TransactionApi::new(self.client)
        }
    }
}
