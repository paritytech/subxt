#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
    use super::api as root_mod;
    pub static PALLETS: [&str; 1usize] = ["Balances"];
    #[derive(
        :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
    )]
    pub enum Event {
        #[codec(index = 6)]
        Balances(balances::Event),
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct Transfer {
                pub dest: ::subxt::utils::MultiAddress<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetBalance {
                pub who: ::subxt::utils::MultiAddress<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ForceTransfer {
                pub source: ::subxt::utils::MultiAddress<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u32,
                >,
                pub dest: ::subxt::utils::MultiAddress<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::utils::MultiAddress<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct TransferAll {
                pub dest: ::subxt::utils::MultiAddress<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u32,
                >,
                pub keep_alive: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ForceUnreserve {
                pub who: ::subxt::utils::MultiAddress<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u32,
                >,
                pub amount: ::core::primitive::u128,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                #[doc = "  types. See related functions below."]
                #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                #[doc = "  computation."]
                #[doc = ""]
                #[doc = "Related functions:"]
                #[doc = ""]
                #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                #[doc = "    that the transfer will not kill the origin account."]
                pub fn transfer(
                    &self,
                    dest: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer",
                        Transfer { dest, value },
                        [
                            255u8, 181u8, 144u8, 248u8, 64u8, 167u8, 5u8, 134u8, 208u8,
                            20u8, 223u8, 103u8, 235u8, 35u8, 66u8, 184u8, 27u8, 94u8,
                            176u8, 60u8, 233u8, 236u8, 145u8, 218u8, 44u8, 138u8, 240u8,
                            224u8, 16u8, 193u8, 220u8, 95u8,
                        ],
                    )
                }
                #[doc = "Set the balances of a given account."]
                #[doc = ""]
                #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                #[doc = "If the new free or reserved balance is below the existential deposit,"]
                #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn set_balance(
                    &self,
                    who: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<SetBalance> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "set_balance",
                        SetBalance {
                            who,
                            new_free,
                            new_reserved,
                        },
                        [
                            174u8, 34u8, 80u8, 252u8, 193u8, 51u8, 228u8, 236u8, 234u8,
                            16u8, 173u8, 214u8, 122u8, 21u8, 254u8, 7u8, 49u8, 176u8,
                            18u8, 128u8, 122u8, 68u8, 72u8, 181u8, 119u8, 90u8, 167u8,
                            46u8, 203u8, 220u8, 109u8, 110u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                #[doc = "specified."]
                #[doc = "## Complexity"]
                #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                #[doc = "  assumed to be in the overlay."]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    dest: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_transfer",
                        ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            56u8, 80u8, 186u8, 45u8, 134u8, 147u8, 200u8, 19u8, 53u8,
                            221u8, 213u8, 32u8, 13u8, 51u8, 130u8, 42u8, 244u8, 85u8,
                            50u8, 246u8, 189u8, 51u8, 93u8, 1u8, 108u8, 142u8, 112u8,
                            245u8, 104u8, 255u8, 15u8, 62u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                #[doc = "origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_keep_alive",
                        TransferKeepAlive { dest, value },
                        [
                            202u8, 239u8, 204u8, 0u8, 52u8, 57u8, 158u8, 8u8, 252u8,
                            178u8, 91u8, 197u8, 238u8, 186u8, 205u8, 56u8, 217u8, 250u8,
                            21u8, 44u8, 239u8, 66u8, 79u8, 99u8, 25u8, 106u8, 70u8,
                            226u8, 50u8, 255u8, 176u8, 71u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true). ## Complexity"]
                #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<TransferAll> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_all",
                        TransferAll { dest, keep_alive },
                        [
                            118u8, 215u8, 198u8, 243u8, 4u8, 173u8, 108u8, 224u8, 113u8,
                            203u8, 149u8, 23u8, 130u8, 176u8, 53u8, 205u8, 112u8, 147u8,
                            88u8, 167u8, 197u8, 32u8, 104u8, 117u8, 201u8, 168u8, 144u8,
                            230u8, 120u8, 29u8, 122u8, 159u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_unreserve",
                        ForceUnreserve { who, amount },
                        [
                            39u8, 229u8, 111u8, 44u8, 147u8, 80u8, 7u8, 26u8, 185u8,
                            121u8, 149u8, 25u8, 151u8, 37u8, 124u8, 46u8, 108u8, 136u8,
                            167u8, 145u8, 103u8, 65u8, 33u8, 168u8, 36u8, 214u8, 126u8,
                            237u8, 180u8, 61u8, 108u8, 110u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                Debug,
            )]
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
                Debug,
            )]
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
                Debug,
            )]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::utils::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                Debug,
            )]
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
                Debug,
            )]
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
                Debug,
            )]
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
                Debug,
            )]
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
                Debug,
            )]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8,
                            115u8, 51u8, 156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8,
                            25u8, 164u8, 235u8, 20u8, 86u8, 242u8, 124u8, 23u8, 28u8,
                            140u8, 26u8, 73u8, 231u8, 51u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            74u8, 203u8, 111u8, 142u8, 225u8, 104u8, 173u8, 51u8, 226u8,
                            12u8, 85u8, 135u8, 41u8, 206u8, 177u8, 238u8, 94u8, 246u8,
                            184u8, 250u8, 140u8, 213u8, 91u8, 118u8, 163u8, 111u8, 211u8,
                            46u8, 204u8, 160u8, 154u8, 21u8,
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
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_balances::AccountData<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8,
                            80u8, 40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8,
                            141u8, 240u8, 172u8, 217u8, 215u8, 109u8, 87u8, 135u8, 248u8,
                            57u8, 98u8, 185u8, 22u8, 4u8,
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
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_balances::AccountData<
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        Vec::new(),
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8,
                            80u8, 40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8,
                            141u8, 240u8, 172u8, 217u8, 215u8, 109u8, 87u8, 135u8, 248u8,
                            57u8, 98u8, 185u8, 22u8, 4u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]                pub fn locks (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: utils :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: bounded_collections :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8,
                            134u8, 195u8, 58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8,
                            4u8, 122u8, 90u8, 212u8, 136u8, 14u8, 127u8, 232u8, 8u8,
                            192u8, 40u8, 233u8, 18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]                pub fn locks_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: bounded_collections :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        Vec::new(),
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8,
                            134u8, 195u8, 58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8,
                            4u8, 122u8, 90u8, 212u8, 136u8, 14u8, 127u8, 232u8, 8u8,
                            192u8, 40u8, 233u8, 18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8,
                            250u8, 128u8, 167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8,
                            16u8, 147u8, 74u8, 55u8, 183u8, 94u8, 160u8, 177u8, 26u8,
                            187u8, 71u8, 197u8, 187u8, 163u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        Vec::new(),
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8,
                            250u8, 128u8, 167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8,
                            16u8, 147u8, 74u8, 55u8, 183u8, 94u8, 160u8, 177u8, 26u8,
                            187u8, 71u8, 197u8, 187u8, 163u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open."]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8,
                            200u8, 214u8, 27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8,
                            101u8, 54u8, 210u8, 136u8, 71u8, 63u8, 49u8, 237u8, 234u8,
                            15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8,
                            157u8, 125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8,
                            98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8, 200u8,
                            90u8, 203u8, 100u8, 41u8, 145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8,
                            157u8, 125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8,
                            98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8, 200u8,
                            90u8, 203u8, 100u8, 41u8, 145u8,
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
                    Debug,
                )]
                pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_benchmarking_pallet_pov {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    emit_event,
                    #[codec(index = 1)]
                    noop,
                }
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum RawOrigin<_0> {
                    #[codec(index = 0)]
                    Root,
                    #[codec(index = 1)]
                    Signed(_0),
                    #[codec(index = 2)]
                    None,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod preimages {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Bounded<_0> {
                        #[codec(index = 0)]
                        Legacy {
                            hash: ::subxt::utils::H256,
                        },
                        #[codec(index = 1)]
                        Inline(
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        ),
                        #[codec(index = 2)]
                        Lookup {
                            hash: ::subxt::utils::H256,
                            len: ::core::primitive::u32,
                        },
                        __Ignore(::core::marker::PhantomData<_0>),
                    }
                }
                pub mod schedule {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum DispatchTime<_0> {
                        #[codec(index = 0)]
                        At(_0),
                        #[codec(index = 1)]
                        After(_0),
                    }
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub enum AttributeNamespace<_0> {
                            #[codec(index = 0)]
                            Pallet,
                            #[codec(index = 1)]
                            CollectionOwner,
                            #[codec(index = 2)]
                            ItemOwner,
                            #[codec(index = 3)]
                            Account(_0),
                        }
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
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
                        Debug,
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckMortality(
                        pub runtime_types::sp_runtime::generic::era::Era,
                    );
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
        }
        pub mod kitchensink_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct NposSolution16 {
                pub votes1: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes2: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    (
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ),
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes3: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 2usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes4: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 3usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes5: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 4usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes6: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 5usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes7: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 6usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes8: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 7usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes9: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 8usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes10: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 9usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes11: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 10usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes12: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 11usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes13: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 12usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes14: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 13usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes15: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 14usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes16: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 15usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_support::dispatch::RawOrigin<
                        ::subxt::utils::AccountId32,
                    >,
                ),
                #[codec(index = 13)]
                Council(
                    runtime_types::pallet_collective::RawOrigin<
                        ::subxt::utils::AccountId32,
                    >,
                ),
                #[codec(index = 14)]
                TechnicalCommittee(
                    runtime_types::pallet_collective::RawOrigin<
                        ::subxt::utils::AccountId32,
                    >,
                ),
                #[codec(index = 52)]
                AllianceMotion(
                    runtime_types::pallet_collective::RawOrigin<
                        ::subxt::utils::AccountId32,
                    >,
                ),
                #[codec(index = 4)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum ProxyType {
                #[codec(index = 0)]
                Any,
                #[codec(index = 1)]
                NonTransfer,
                #[codec(index = 2)]
                Governance,
                #[codec(index = 3)]
                Staking,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 2)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 3)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 5)]
                Indices(runtime_types::pallet_indices::pallet::Call),
                #[codec(index = 6)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 9)]
                ElectionProviderMultiPhase(
                    runtime_types::pallet_election_provider_multi_phase::pallet::Call,
                ),
                #[codec(index = 10)]
                Staking(runtime_types::pallet_staking::pallet::pallet::Call),
                #[codec(index = 11)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 12)]
                Democracy(runtime_types::pallet_democracy::pallet::Call),
                #[codec(index = 13)]
                Council(runtime_types::pallet_collective::pallet::Call),
                #[codec(index = 14)]
                TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
                #[codec(index = 15)]
                Elections(runtime_types::pallet_elections_phragmen::pallet::Call),
                #[codec(index = 16)]
                TechnicalMembership(runtime_types::pallet_membership::pallet::Call),
                #[codec(index = 17)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 18)]
                Treasury(runtime_types::pallet_treasury::pallet::Call),
                #[codec(index = 19)]
                Contracts(runtime_types::pallet_contracts::pallet::Call),
                #[codec(index = 20)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 21)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 26)]
                Identity(runtime_types::pallet_identity::pallet::Call),
                #[codec(index = 27)]
                Society(runtime_types::pallet_society::pallet::Call),
                #[codec(index = 28)]
                Recovery(runtime_types::pallet_recovery::pallet::Call),
                #[codec(index = 29)]
                Vesting(runtime_types::pallet_vesting::pallet::Call),
                #[codec(index = 30)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Call),
                #[codec(index = 31)]
                Glutton(runtime_types::pallet_glutton::pallet::Call),
                #[codec(index = 32)]
                Preimage(runtime_types::pallet_preimage::pallet::Call),
                #[codec(index = 33)]
                Proxy(runtime_types::pallet_proxy::pallet::Call),
                #[codec(index = 34)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 35)]
                Bounties(runtime_types::pallet_bounties::pallet::Call),
                #[codec(index = 36)]
                Tips(runtime_types::pallet_tips::pallet::Call),
                #[codec(index = 37)]
                Assets(runtime_types::pallet_assets::pallet::Call),
                #[codec(index = 39)]
                Lottery(runtime_types::pallet_lottery::pallet::Call),
                #[codec(index = 40)]
                Nis(runtime_types::pallet_nis::pallet::Call),
                #[codec(index = 41)]
                Uniques(runtime_types::pallet_uniques::pallet::Call),
                #[codec(index = 42)]
                Nfts(runtime_types::pallet_nfts::pallet::Call),
                #[codec(index = 43)]
                TransactionStorage(
                    runtime_types::pallet_transaction_storage::pallet::Call,
                ),
                #[codec(index = 44)]
                VoterList(runtime_types::pallet_bags_list::pallet::Call),
                #[codec(index = 45)]
                StateTrieMigration(
                    runtime_types::pallet_state_trie_migration::pallet::Call,
                ),
                #[codec(index = 46)]
                ChildBounties(runtime_types::pallet_child_bounties::pallet::Call),
                #[codec(index = 47)]
                Referenda(runtime_types::pallet_referenda::pallet::Call),
                #[codec(index = 48)]
                Remark(runtime_types::pallet_remark::pallet::Call),
                #[codec(index = 49)]
                RootTesting(runtime_types::pallet_root_testing::pallet::Call),
                #[codec(index = 50)]
                ConvictionVoting(runtime_types::pallet_conviction_voting::pallet::Call),
                #[codec(index = 51)]
                Whitelist(runtime_types::pallet_whitelist::pallet::Call),
                #[codec(index = 52)]
                AllianceMotion(runtime_types::pallet_collective::pallet::Call),
                #[codec(index = 53)]
                Alliance(runtime_types::pallet_alliance::pallet::Call),
                #[codec(index = 54)]
                NominationPools(runtime_types::pallet_nomination_pools::pallet::Call),
                #[codec(index = 55)]
                RankedPolls(runtime_types::pallet_referenda::pallet::Call),
                #[codec(index = 56)]
                RankedCollective(runtime_types::pallet_ranked_collective::pallet::Call),
                #[codec(index = 57)]
                FastUnstake(runtime_types::pallet_fast_unstake::pallet::Call),
                #[codec(index = 58)]
                MessageQueue(runtime_types::pallet_message_queue::pallet::Call),
                #[codec(index = 59)]
                Pov(runtime_types::frame_benchmarking_pallet_pov::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SessionKeys {
                pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
                pub babe: runtime_types::sp_consensus_babe::app::Public,
                pub im_online:
                    runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                pub authority_discovery:
                    runtime_types::sp_authority_discovery::app::Public,
            }
        }
        pub mod pallet_alliance {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Add a new proposal to be voted on."] # [doc = ""] # [doc = "Must be called by a Fellow."] propose { # [codec (compact)] threshold : :: core :: primitive :: u32 , proposal : :: std :: boxed :: Box < runtime_types :: kitchensink_runtime :: RuntimeCall > , # [codec (compact)] length_bound : :: core :: primitive :: u32 , } , # [codec (index = 1)] # [doc = "Add an aye or nay vote for the sender to the given proposal."] # [doc = ""] # [doc = "Must be called by a Fellow."] vote { proposal : :: subxt :: utils :: H256 , # [codec (compact)] index : :: core :: primitive :: u32 , approve : :: core :: primitive :: bool , } , # [codec (index = 2)] # [doc = "Close a vote that is either approved, disapproved, or whose voting period has ended."] # [doc = ""] # [doc = "Must be called by a Fellow."] close_old_weight { proposal_hash : :: subxt :: utils :: H256 , # [codec (compact)] index : :: core :: primitive :: u32 , # [codec (compact)] proposal_weight_bound : runtime_types :: sp_weights :: OldWeight , # [codec (compact)] length_bound : :: core :: primitive :: u32 , } , # [codec (index = 3)] # [doc = "Initialize the Alliance, onboard fellows and allies."] # [doc = ""] # [doc = "The Alliance must be empty, and the call must provide some founding members."] # [doc = ""] # [doc = "Must be called by the Root origin."] init_members { fellows : :: std :: vec :: Vec < :: subxt :: utils :: AccountId32 > , allies : :: std :: vec :: Vec < :: subxt :: utils :: AccountId32 > , } , # [codec (index = 4)] # [doc = "Disband the Alliance, remove all active members and unreserve deposits."] # [doc = ""] # [doc = "Witness data must be set."] disband { witness : runtime_types :: pallet_alliance :: types :: DisbandWitness , } , # [codec (index = 5)] # [doc = "Set a new IPFS CID to the alliance rule."] set_rule { rule : runtime_types :: pallet_alliance :: types :: Cid , } , # [codec (index = 6)] # [doc = "Make an announcement of a new IPFS CID about alliance issues."] announce { announcement : runtime_types :: pallet_alliance :: types :: Cid , } , # [codec (index = 7)] # [doc = "Remove an announcement."] remove_announcement { announcement : runtime_types :: pallet_alliance :: types :: Cid , } , # [codec (index = 8)] # [doc = "Submit oneself for candidacy. A fixed deposit is reserved."] join_alliance , # [codec (index = 9)] # [doc = "A Fellow can nominate someone to join the alliance as an Ally. There is no deposit"] # [doc = "required from the nominator or nominee."] nominate_ally { who : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 10)] # [doc = "Elevate an Ally to Fellow."] elevate_ally { ally : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 11)] # [doc = "As a member, give a retirement notice and start a retirement period required to pass in"] # [doc = "order to retire."] give_retirement_notice , # [codec (index = 12)] # [doc = "As a member, retire from the Alliance and unreserve the deposit."] # [doc = ""] # [doc = "This can only be done once you have called `give_retirement_notice` and the"] # [doc = "`RetirementPeriod` has passed."] retire , # [codec (index = 13)] # [doc = "Kick a member from the Alliance and slash its deposit."] kick_member { who : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 14)] # [doc = "Add accounts or websites to the list of unscrupulous items."] add_unscrupulous_items { items : :: std :: vec :: Vec < runtime_types :: pallet_alliance :: UnscrupulousItem < :: subxt :: utils :: AccountId32 , runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > > , } , # [codec (index = 15)] # [doc = "Deem some items no longer unscrupulous."] remove_unscrupulous_items { items : :: std :: vec :: Vec < runtime_types :: pallet_alliance :: UnscrupulousItem < :: subxt :: utils :: AccountId32 , runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > > , } , # [codec (index = 16)] # [doc = "Close a vote that is either approved, disapproved, or whose voting period has ended."] # [doc = ""] # [doc = "Must be called by a Fellow."] close { proposal_hash : :: subxt :: utils :: H256 , # [codec (compact)] index : :: core :: primitive :: u32 , proposal_weight_bound : runtime_types :: sp_weights :: weight_v2 :: Weight , # [codec (compact)] length_bound : :: core :: primitive :: u32 , } , # [codec (index = 17)] # [doc = "Abdicate one's position as a voting member and just be an Ally. May be used by Fellows"] # [doc = "who do not want to leave the Alliance but do not have the capacity to participate"] # [doc = "operationally for some time."] abdicate_fellow_status , }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Cid {
                    pub version: runtime_types::pallet_alliance::types::Version,
                    pub codec: ::core::primitive::u64,
                    pub hash: runtime_types::pallet_alliance::types::Multihash,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct DisbandWitness {
                    #[codec(compact)]
                    pub fellow_members: ::core::primitive::u32,
                    #[codec(compact)]
                    pub ally_members: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Multihash {
                    pub code: ::core::primitive::u64,
                    pub digest:
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum Version {
                    #[codec(index = 0)]
                    V0,
                    #[codec(index = 1)]
                    V1,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum UnscrupulousItem<_0, _1> {
                #[codec(index = 0)]
                AccountId(_0),
                #[codec(index = 1)]
                Website(_1),
            }
        }
        pub mod pallet_asset_tx_payment {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ChargeAssetTxPayment {
                #[codec(compact)]
                pub tip: ::core::primitive::u128,
                pub asset_id: ::core::option::Option<::core::primitive::u32>,
            }
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Issue a new class of fungible assets from a public origin."]
                    #[doc = ""]
                    #[doc = "This new asset class has no assets initially and its owner is the origin."]
                    #[doc = ""]
                    #[doc = "The origin must conform to the configured `CreateOrigin` and have sufficient funds free."]
                    #[doc = ""]
                    #[doc = "Funds of sender are reserved by `AssetDeposit`."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
                    #[doc = "an existing asset."]
                    #[doc = "- `admin`: The admin of this class of assets. The admin is the initial address of each"]
                    #[doc = "member of the asset class's admin team."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = ""]
                    #[doc = "Emits `Created` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        admin: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Issue a new class of fungible assets from a privileged origin."]
                    #[doc = ""]
                    #[doc = "This new asset class has no assets initially."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "Unlike `create`, no funds are reserved."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
                    #[doc = "an existing asset."]
                    #[doc = "- `owner`: The owner of this class of assets. The owner has full superuser permissions"]
                    #[doc = "over this asset, but may later change and configure the permissions using"]
                    #[doc = "`transfer_ownership` and `set_team`."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = ""]
                    #[doc = "Emits `ForceCreated` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Start the process of destroying a fungible asset class."]
                    #[doc = ""]
                    #[doc = "`start_destroy` is the first in a series of extrinsics that should be called, to allow"]
                    #[doc = "destruction of an asset class."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin` or must be `Signed` by the asset's `owner`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "  asset."]
                    #[doc = ""]
                    #[doc = "The asset class must be frozen before calling `start_destroy`."]
                    start_destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Destroy all accounts associated with a given asset."]
                    #[doc = ""]
                    #[doc = "`destroy_accounts` should only be called after `start_destroy` has been called, and the"]
                    #[doc = "asset is in a `Destroying` state."]
                    #[doc = ""]
                    #[doc = "Due to weight restrictions, this function may need to be called multiple times to fully"]
                    #[doc = "destroy all accounts. It will destroy `RemoveItemsLimit` accounts at a time."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "  asset."]
                    #[doc = ""]
                    #[doc = "Each call emits the `Event::DestroyedAccounts` event."]
                    destroy_accounts {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "Destroy all approvals associated with a given asset up to the max (T::RemoveItemsLimit)."]
                    #[doc = ""]
                    #[doc = "`destroy_approvals` should only be called after `start_destroy` has been called, and the"]
                    #[doc = "asset is in a `Destroying` state."]
                    #[doc = ""]
                    #[doc = "Due to weight restrictions, this function may need to be called multiple times to fully"]
                    #[doc = "destroy all approvals. It will destroy `RemoveItemsLimit` approvals at a time."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "  asset."]
                    #[doc = ""]
                    #[doc = "Each call emits the `Event::DestroyedApprovals` event."]
                    destroy_approvals {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Complete destroying asset and unreserve currency."]
                    #[doc = ""]
                    #[doc = "`finish_destroy` should only be called after `start_destroy` has been called, and the"]
                    #[doc = "asset is in a `Destroying` state. All accounts or approvals should be destroyed before"]
                    #[doc = "hand."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "  asset."]
                    #[doc = ""]
                    #[doc = "Each successful call emits the `Event::Destroyed` event."]
                    finish_destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Mint assets of a particular class."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed and the sender must be the Issuer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount minted."]
                    #[doc = "- `beneficiary`: The account to be credited with the minted assets."]
                    #[doc = "- `amount`: The amount of the asset to be minted."]
                    #[doc = ""]
                    #[doc = "Emits `Issued` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existing balance of `beneficiary`; Account pre-existence of `beneficiary`."]
                    mint {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "Reduce the balance of `who` by as much as possible up to `amount` assets of `id`."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Manager of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Bails with `NoAccount` if the `who` is already dead."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount burned."]
                    #[doc = "- `who`: The account to be debited from."]
                    #[doc = "- `amount`: The maximum amount by which `who`'s balance should be reduced."]
                    #[doc = ""]
                    #[doc = "Emits `Burned` with the actual amount burned. If this takes the balance to below the"]
                    #[doc = "minimum for the asset, then the amount burned is increased to take it to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Post-existence of `who`; Pre & post Zombie-status of `who`."]
                    burn {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Move some assets from the sender account to another."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `target`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
                    #[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
                    #[doc = "the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
                    #[doc = "`target`."]
                    transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Move some assets from the sender account to another, keeping the sender account alive."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `target`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
                    #[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
                    #[doc = "the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
                    #[doc = "`target`."]
                    transfer_keep_alive {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Move some assets from one account to another."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `source`: The account to be debited."]
                    #[doc = "- `dest`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the `source`'s balance of assets should be reduced and"]
                    #[doc = "`dest`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the `source` balance above zero but"]
                    #[doc = "below the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `dest`; Post-existence of `source`; Account pre-existence of"]
                    #[doc = "`dest`."]
                    force_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        source: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Disallow further unprivileged transfers from an account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `who`: The account to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `Frozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 12)]
                    #[doc = "Allow unprivileged transfers from an account again."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `who`: The account to be unfrozen."]
                    #[doc = ""]
                    #[doc = "Emits `Thawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 13)]
                    #[doc = "Disallow further unprivileged transfers for the asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `Frozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 14)]
                    #[doc = "Allow unprivileged transfers for the asset again."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be thawed."]
                    #[doc = ""]
                    #[doc = "Emits `Thawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 15)]
                    #[doc = "Change the Owner of an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The new Owner of this asset."]
                    #[doc = ""]
                    #[doc = "Emits `OwnerChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 16)]
                    #[doc = "Change the Issuer, Admin and Freezer of an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `issuer`: The new Issuer of this asset."]
                    #[doc = "- `admin`: The new Admin of this asset."]
                    #[doc = "- `freezer`: The new Freezer of this asset."]
                    #[doc = ""]
                    #[doc = "Emits `TeamChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_team {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        issuer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        admin: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        freezer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 17)]
                    #[doc = "Set the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Funds of sender are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + MetadataDepositPerByte * (name.len + symbol.len)` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to update."]
                    #[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 18)]
                    #[doc = "Clear the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the asset owner."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to clear."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 19)]
                    #[doc = "Force the metadata for an asset to some value."]
                    #[doc = ""]
                    #[doc = "Origin must be ForceOrigin."]
                    #[doc = ""]
                    #[doc = "Any deposit is left alone."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to update."]
                    #[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(N + S)` where N and S are the length of the name and symbol respectively."]
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 20)]
                    #[doc = "Clear the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be ForceOrigin."]
                    #[doc = ""]
                    #[doc = "Any deposit is returned."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to clear."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 21)]
                    #[doc = "Alter the attributes of a given asset."]
                    #[doc = ""]
                    #[doc = "Origin must be `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The new Owner of this asset."]
                    #[doc = "- `issuer`: The new Issuer of this asset."]
                    #[doc = "- `admin`: The new Admin of this asset."]
                    #[doc = "- `freezer`: The new Freezer of this asset."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = "- `is_sufficient`: Whether a non-zero balance of this asset is deposit of sufficient"]
                    #[doc = "value to account for the state bloat associated with its balance storage. If set to"]
                    #[doc = "`true`, then non-zero balances may be stored without a `consumer` reference (and thus"]
                    #[doc = "an ED in the Balances pallet or whatever else is used to control user-account state"]
                    #[doc = "growth)."]
                    #[doc = "- `is_frozen`: Whether this asset class is frozen except for permissioned/admin"]
                    #[doc = "instructions."]
                    #[doc = ""]
                    #[doc = "Emits `AssetStatusChanged` with the identity of the asset."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_asset_status {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        issuer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        admin: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        freezer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 22)]
                    #[doc = "Approve an amount of asset for transfer by a delegated third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "Ensures that `ApprovalDeposit` worth of `Currency` is reserved from signing account"]
                    #[doc = "for the purpose of holding the approval. If some non-zero amount of assets is already"]
                    #[doc = "approved from signing account to `delegate`, then it is topped up or unreserved to"]
                    #[doc = "meet the right value."]
                    #[doc = ""]
                    #[doc = "NOTE: The signing account does not need to own `amount` of assets at the point of"]
                    #[doc = "making this call."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account to delegate permission to transfer asset."]
                    #[doc = "- `amount`: The amount of asset that may be transferred by `delegate`. If there is"]
                    #[doc = "already an approval in place, then this acts additively."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovedTransfer` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    approve_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    #[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and there must be an approval in place between signer and"]
                    #[doc = "`delegate`."]
                    #[doc = ""]
                    #[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account delegated permission to transfer asset."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovalCancelled` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 24)]
                    #[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be either ForceOrigin or Signed origin with the signer being the Admin"]
                    #[doc = "account of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account delegated permission to transfer asset."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovalCancelled` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        delegate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 25)]
                    #[doc = "Transfer some asset balance from a previously delegated account to some third-party"]
                    #[doc = "account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and there must be an approval in place by the `owner` to the"]
                    #[doc = "signer."]
                    #[doc = ""]
                    #[doc = "If the entire amount approved for transfer is transferred, then any deposit previously"]
                    #[doc = "reserved by `approve_transfer` is unreserved."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The account which previously approved for a transfer of at least `amount` and"]
                    #[doc = "from which the asset balance will be withdrawn."]
                    #[doc = "- `destination`: The account to which the asset balance of `amount` will be transferred."]
                    #[doc = "- `amount`: The amount of assets to transfer."]
                    #[doc = ""]
                    #[doc = "Emits `TransferredApproved` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer_approved {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        destination: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 26)]
                    #[doc = "Create an asset account for non-provider assets."]
                    #[doc = ""]
                    #[doc = "A deposit will be taken from the signer account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Signed; the signer account must have sufficient funds for a deposit"]
                    #[doc = "  to be taken."]
                    #[doc = "- `id`: The identifier of the asset for the account to be created."]
                    #[doc = ""]
                    #[doc = "Emits `Touched` event when successful."]
                    touch {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 27)]
                    #[doc = "Return the deposit (if any) of an asset account."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset for the account to be created."]
                    #[doc = "- `allow_burn`: If `true` then assets may be destroyed in order to complete the refund."]
                    #[doc = ""]
                    #[doc = "Emits `Refunded` event when successful."]
                    refund {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        allow_burn: ::core::primitive::bool,
                    },
                }
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Report authority equivocation/misbehavior. This method will verify"] # [doc = "the equivocation proof and validate the given key ownership proof"] # [doc = "against the extracted offender. If both are valid, the offence will"] # [doc = "be reported."] report_equivocation { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , # [codec (index = 1)] # [doc = "Report authority equivocation/misbehavior. This method will verify"] # [doc = "the equivocation proof and validate the given key ownership proof"] # [doc = "against the extracted offender. If both are valid, the offence will"] # [doc = "be reported."] # [doc = "This extrinsic must be called unsigned and it is expected that only"] # [doc = "block authors will call it (validated in `ValidateUnsigned`), as such"] # [doc = "if the block author is defined it will be defined as the equivocation"] # [doc = "reporter."] report_equivocation_unsigned { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_consensus_slots :: EquivocationProof < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > , runtime_types :: sp_consensus_babe :: app :: Public > > , key_owner_proof : runtime_types :: sp_session :: MembershipProof , } , # [codec (index = 2)] # [doc = "Plan an epoch config change. The epoch config change is recorded and will be enacted on"] # [doc = "the next call to `enact_epoch_change`. The config will be activated one epoch after."] # [doc = "Multiple calls to this method will replace any existing planned config change that had"] # [doc = "not been enacted yet."] plan_config_change { config : runtime_types :: sp_consensus_babe :: digests :: NextConfigDescriptor , } , }
            }
        }
        pub mod pallet_bags_list {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Declare that some `dislocated` account has, through rewards or penalties, sufficiently"]
                    #[doc = "changed its score that it should properly fall into a different bag than its current"]
                    #[doc = "one."]
                    #[doc = ""]
                    #[doc = "Anyone can call this function about any potentially dislocated account."]
                    #[doc = ""]
                    #[doc = "Will always update the stored score of `dislocated` to the correct score, based on"]
                    #[doc = "`ScoreProvider`."]
                    #[doc = ""]
                    #[doc = "If `dislocated` does not exists, it returns an error."]
                    rebag {
                        dislocated: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Move the caller's Id directly in front of `lighter`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and can only be called by the Id of"]
                    #[doc = "the account going in front of `lighter`."]
                    #[doc = ""]
                    #[doc = "Only works if"]
                    #[doc = "- both nodes are within the same bag,"]
                    #[doc = "- and `origin` has a greater `Score` than `lighter`."]
                    put_in_front_of {
                        lighter: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                    #[doc = "  types. See related functions below."]
                    #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                    #[doc = "  computation."]
                    #[doc = ""]
                    #[doc = "Related functions:"]
                    #[doc = ""]
                    #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                    #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                    #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                    #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                    #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                    #[doc = "    that the transfer will not kill the origin account."]
                    transfer {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the balances of a given account."]
                    #[doc = ""]
                    #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                    #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                    #[doc = "If the new free or reserved balance is below the existential deposit,"]
                    #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    set_balance {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                    #[doc = "specified."]
                    #[doc = "## Complexity"]
                    #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                    #[doc = "  assumed to be in the overlay."]
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                    #[doc = "origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true). ## Complexity"]
                    #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value"]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal"]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit"]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account"]
                    KeepAlive,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account"]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist"]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed MaxReserves"]
                    TooManyReserves,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    # [codec (index = 0)] # [doc = "An account was created with some free balance."] Endowed { account : :: subxt :: utils :: AccountId32 , free_balance : :: core :: primitive :: u128 , } , # [codec (index = 1)] # [doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"] # [doc = "resulting in an outright loss."] DustLost { account : :: subxt :: utils :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 2)] # [doc = "Transfer succeeded."] Transfer { from : :: subxt :: utils :: AccountId32 , to : :: subxt :: utils :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 3)] # [doc = "A balance was set by root."] BalanceSet { who : :: subxt :: utils :: AccountId32 , free : :: core :: primitive :: u128 , reserved : :: core :: primitive :: u128 , } , # [codec (index = 4)] # [doc = "Some balance was reserved (moved from free to reserved)."] Reserved { who : :: subxt :: utils :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 5)] # [doc = "Some balance was unreserved (moved from reserved to free)."] Unreserved { who : :: subxt :: utils :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 6)] # [doc = "Some balance was moved from the reserve of the first account to the second account."] # [doc = "Final argument indicates the destination balance type."] ReserveRepatriated { from : :: subxt :: utils :: AccountId32 , to : :: subxt :: utils :: AccountId32 , amount : :: core :: primitive :: u128 , destination_status : runtime_types :: frame_support :: traits :: tokens :: misc :: BalanceStatus , } , # [codec (index = 7)] # [doc = "Some amount was deposited (e.g. for transaction fees)."] Deposit { who : :: subxt :: utils :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 8)] # [doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."] Withdraw { who : :: subxt :: utils :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 9)] # [doc = "Some amount was removed from the account (e.g. for misbehavior)."] Slashed { who : :: subxt :: utils :: AccountId32 , amount : :: core :: primitive :: u128 , } , }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                Debug,
            )]
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Propose a new bounty."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Payment: `TipReportDepositBase` will be reserved from the origin account, as well as"]
                    #[doc = "`DataDepositPerByte` for each byte in `reason`. It will be unreserved upon approval,"]
                    #[doc = "or slashed when rejected."]
                    #[doc = ""]
                    #[doc = "- `curator`: The curator account whom will manage this bounty."]
                    #[doc = "- `fee`: The curator fee."]
                    #[doc = "- `value`: The total payment amount of this bounty, curator fee included."]
                    #[doc = "- `description`: The description of this bounty."]
                    propose_bounty {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        description: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Approve a bounty proposal. At a later time, the bounty will be funded and become active"]
                    #[doc = "and the original deposit will be returned."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::SpendOrigin`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    approve_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Assign a curator to a funded bounty."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::SpendOrigin`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    propose_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        curator: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Unassign curator from a bounty."]
                    #[doc = ""]
                    #[doc = "This function can only be called by the `RejectOrigin` a signed origin."]
                    #[doc = ""]
                    #[doc = "If this function is called by the `RejectOrigin`, we assume that the curator is"]
                    #[doc = "malicious or inactive. As a result, we will slash the curator when possible."]
                    #[doc = ""]
                    #[doc = "If the origin is the curator, we take this as a sign they are unable to do their job and"]
                    #[doc = "they willingly give up. We could slash them, but for now we allow them to recover their"]
                    #[doc = "deposit and exit without issue. (We may want to change this if it is abused.)"]
                    #[doc = ""]
                    #[doc = "Finally, the origin can be anyone if and only if the curator is \"inactive\". This allows"]
                    #[doc = "anyone in the community to call out that a curator is not doing their due diligence, and"]
                    #[doc = "we should pick a new curator. In this case the curator should also be slashed."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    unassign_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "Accept the curator role for a bounty."]
                    #[doc = "A deposit will be reserved from curator and refund upon successful payout."]
                    #[doc = ""]
                    #[doc = "May only be called from the curator."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    accept_curator {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Award bounty to a beneficiary account. The beneficiary will be able to claim the funds"]
                    #[doc = "after a delay."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the curator of this bounty."]
                    #[doc = ""]
                    #[doc = "- `bounty_id`: Bounty ID to award."]
                    #[doc = "- `beneficiary`: The beneficiary account whom will receive the payout."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    award_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        beneficiary: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Claim the payout from an awarded bounty after payout delay."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the beneficiary of this bounty."]
                    #[doc = ""]
                    #[doc = "- `bounty_id`: Bounty ID to claim."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    claim_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Cancel a proposed or active bounty. All the funds will be sent to treasury and"]
                    #[doc = "the curator deposit will be unreserved if possible."]
                    #[doc = ""]
                    #[doc = "Only `T::RejectOrigin` is able to cancel a bounty."]
                    #[doc = ""]
                    #[doc = "- `bounty_id`: Bounty ID to cancel."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    close_bounty {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Extend the expiry time of an active bounty."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the curator of this bounty."]
                    #[doc = ""]
                    #[doc = "- `bounty_id`: Bounty ID to extend."]
                    #[doc = "- `remark`: additional information."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    extend_bounty_expiry {
                        #[codec(compact)]
                        bounty_id: ::core::primitive::u32,
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
        }
        pub mod pallet_child_bounties {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a new child-bounty."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the curator of parent"]
                    #[doc = "bounty and the parent bounty must be in \"active\" state."]
                    #[doc = ""]
                    #[doc = "Child-bounty gets added successfully & fund gets transferred from"]
                    #[doc = "parent bounty to child-bounty account, if parent bounty has enough"]
                    #[doc = "funds, else the call fails."]
                    #[doc = ""]
                    #[doc = "Upper bound to maximum number of active  child bounties that can be"]
                    #[doc = "added are managed via runtime trait config"]
                    #[doc = "[`Config::MaxActiveChildBountyCount`]."]
                    #[doc = ""]
                    #[doc = "If the call is success, the status of child-bounty is updated to"]
                    #[doc = "\"Added\"."]
                    #[doc = ""]
                    #[doc = "- `parent_bounty_id`: Index of parent bounty for which child-bounty is being added."]
                    #[doc = "- `value`: Value for executing the proposal."]
                    #[doc = "- `description`: Text description for the child-bounty."]
                    add_child_bounty {
                        #[codec(compact)]
                        parent_bounty_id: ::core::primitive::u32,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        description: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Propose curator for funded child-bounty."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be curator of parent bounty."]
                    #[doc = ""]
                    #[doc = "Parent bounty must be in active state, for this child-bounty call to"]
                    #[doc = "work."]
                    #[doc = ""]
                    #[doc = "Child-bounty must be in \"Added\" state, for processing the call. And"]
                    #[doc = "state of child-bounty is moved to \"CuratorProposed\" on successful"]
                    #[doc = "call completion."]
                    #[doc = ""]
                    #[doc = "- `parent_bounty_id`: Index of parent bounty."]
                    #[doc = "- `child_bounty_id`: Index of child bounty."]
                    #[doc = "- `curator`: Address of child-bounty curator."]
                    #[doc = "- `fee`: payment fee to child-bounty curator for execution."]
                    propose_curator {
                        #[codec(compact)]
                        parent_bounty_id: ::core::primitive::u32,
                        #[codec(compact)]
                        child_bounty_id: ::core::primitive::u32,
                        curator: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Accept the curator role for the child-bounty."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the curator of this"]
                    #[doc = "child-bounty."]
                    #[doc = ""]
                    #[doc = "A deposit will be reserved from the curator and refund upon"]
                    #[doc = "successful payout or cancellation."]
                    #[doc = ""]
                    #[doc = "Fee for curator is deducted from curator fee of parent bounty."]
                    #[doc = ""]
                    #[doc = "Parent bounty must be in active state, for this child-bounty call to"]
                    #[doc = "work."]
                    #[doc = ""]
                    #[doc = "Child-bounty must be in \"CuratorProposed\" state, for processing the"]
                    #[doc = "call. And state of child-bounty is moved to \"Active\" on successful"]
                    #[doc = "call completion."]
                    #[doc = ""]
                    #[doc = "- `parent_bounty_id`: Index of parent bounty."]
                    #[doc = "- `child_bounty_id`: Index of child bounty."]
                    accept_curator {
                        #[codec(compact)]
                        parent_bounty_id: ::core::primitive::u32,
                        #[codec(compact)]
                        child_bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Unassign curator from a child-bounty."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call can be either `RejectOrigin`, or"]
                    #[doc = "the curator of the parent bounty, or any signed origin."]
                    #[doc = ""]
                    #[doc = "For the origin other than T::RejectOrigin and the child-bounty"]
                    #[doc = "curator, parent bounty must be in active state, for this call to"]
                    #[doc = "work. We allow child-bounty curator and T::RejectOrigin to execute"]
                    #[doc = "this call irrespective of the parent bounty state."]
                    #[doc = ""]
                    #[doc = "If this function is called by the `RejectOrigin` or the"]
                    #[doc = "parent bounty curator, we assume that the child-bounty curator is"]
                    #[doc = "malicious or inactive. As a result, child-bounty curator deposit is"]
                    #[doc = "slashed."]
                    #[doc = ""]
                    #[doc = "If the origin is the child-bounty curator, we take this as a sign"]
                    #[doc = "that they are unable to do their job, and are willingly giving up."]
                    #[doc = "We could slash the deposit, but for now we allow them to unreserve"]
                    #[doc = "their deposit and exit without issue. (We may want to change this if"]
                    #[doc = "it is abused.)"]
                    #[doc = ""]
                    #[doc = "Finally, the origin can be anyone iff the child-bounty curator is"]
                    #[doc = "\"inactive\". Expiry update due of parent bounty is used to estimate"]
                    #[doc = "inactive state of child-bounty curator."]
                    #[doc = ""]
                    #[doc = "This allows anyone in the community to call out that a child-bounty"]
                    #[doc = "curator is not doing their due diligence, and we should pick a new"]
                    #[doc = "one. In this case the child-bounty curator deposit is slashed."]
                    #[doc = ""]
                    #[doc = "State of child-bounty is moved to Added state on successful call"]
                    #[doc = "completion."]
                    #[doc = ""]
                    #[doc = "- `parent_bounty_id`: Index of parent bounty."]
                    #[doc = "- `child_bounty_id`: Index of child bounty."]
                    unassign_curator {
                        #[codec(compact)]
                        parent_bounty_id: ::core::primitive::u32,
                        #[codec(compact)]
                        child_bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "Award child-bounty to a beneficiary."]
                    #[doc = ""]
                    #[doc = "The beneficiary will be able to claim the funds after a delay."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the parent curator or"]
                    #[doc = "curator of this child-bounty."]
                    #[doc = ""]
                    #[doc = "Parent bounty must be in active state, for this child-bounty call to"]
                    #[doc = "work."]
                    #[doc = ""]
                    #[doc = "Child-bounty must be in active state, for processing the call. And"]
                    #[doc = "state of child-bounty is moved to \"PendingPayout\" on successful call"]
                    #[doc = "completion."]
                    #[doc = ""]
                    #[doc = "- `parent_bounty_id`: Index of parent bounty."]
                    #[doc = "- `child_bounty_id`: Index of child bounty."]
                    #[doc = "- `beneficiary`: Beneficiary account."]
                    award_child_bounty {
                        #[codec(compact)]
                        parent_bounty_id: ::core::primitive::u32,
                        #[codec(compact)]
                        child_bounty_id: ::core::primitive::u32,
                        beneficiary: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Claim the payout from an awarded child-bounty after payout delay."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call may be any signed origin."]
                    #[doc = ""]
                    #[doc = "Call works independent of parent bounty state, No need for parent"]
                    #[doc = "bounty to be in active state."]
                    #[doc = ""]
                    #[doc = "The Beneficiary is paid out with agreed bounty value. Curator fee is"]
                    #[doc = "paid & curator deposit is unreserved."]
                    #[doc = ""]
                    #[doc = "Child-bounty must be in \"PendingPayout\" state, for processing the"]
                    #[doc = "call. And instance of child-bounty is removed from the state on"]
                    #[doc = "successful call completion."]
                    #[doc = ""]
                    #[doc = "- `parent_bounty_id`: Index of parent bounty."]
                    #[doc = "- `child_bounty_id`: Index of child bounty."]
                    claim_child_bounty {
                        #[codec(compact)]
                        parent_bounty_id: ::core::primitive::u32,
                        #[codec(compact)]
                        child_bounty_id: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Cancel a proposed or active child-bounty. Child-bounty account funds"]
                    #[doc = "are transferred to parent bounty account. The child-bounty curator"]
                    #[doc = "deposit may be unreserved if possible."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be either parent curator or"]
                    #[doc = "`T::RejectOrigin`."]
                    #[doc = ""]
                    #[doc = "If the state of child-bounty is `Active`, curator deposit is"]
                    #[doc = "unreserved."]
                    #[doc = ""]
                    #[doc = "If the state of child-bounty is `PendingPayout`, call fails &"]
                    #[doc = "returns `PendingPayout` error."]
                    #[doc = ""]
                    #[doc = "For the origin other than T::RejectOrigin, parent bounty must be in"]
                    #[doc = "active state, for this child-bounty call to work. For origin"]
                    #[doc = "T::RejectOrigin execution is forced."]
                    #[doc = ""]
                    #[doc = "Instance of child-bounty is removed from the state on successful"]
                    #[doc = "call completion."]
                    #[doc = ""]
                    #[doc = "- `parent_bounty_id`: Index of parent bounty."]
                    #[doc = "- `child_bounty_id`: Index of child bounty."]
                    close_child_bounty {
                        #[codec(compact)]
                        parent_bounty_id: ::core::primitive::u32,
                        #[codec(compact)]
                        child_bounty_id: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_collective {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the collective's membership."]
                    #[doc = ""]
                    #[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
                    #[doc = "- `prime`: The prime member whose vote sets the default."]
                    #[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
                    #[doc = "  weight estimation."]
                    #[doc = ""]
                    #[doc = "The dispatch of this call must be `SetMembersOrigin`."]
                    #[doc = ""]
                    #[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
                    #[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
                    #[doc = ""]
                    #[doc = "# WARNING:"]
                    #[doc = ""]
                    #[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
                    #[doc = "implementation of the trait [`ChangeMembers`]."]
                    #[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
                    #[doc = "with other logic managing the member set."]
                    #[doc = ""]
                    #[doc = "## Complexity:"]
                    #[doc = "- `O(MP + N)` where:"]
                    #[doc = "  - `M` old-members-count (code- and governance-bounded)"]
                    #[doc = "  - `N` new-members-count (code- and governance-bounded)"]
                    #[doc = "  - `P` proposals-count (code-bounded)"]
                    set_members {
                        new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        prime: ::core::option::Option<::subxt::utils::AccountId32>,
                        old_count: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Dispatch a proposal from a member using the `Member` origin."]
                    #[doc = ""]
                    #[doc = "Origin must be a member of the collective."]
                    #[doc = ""]
                    #[doc = "## Complexity:"]
                    #[doc = "- `O(B + M + P)` where:"]
                    #[doc = "- `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "- `M` members-count (code-bounded)"]
                    #[doc = "- `P` complexity of dispatching `proposal`"]
                    execute {
                        proposal: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Add a new proposal to either be voted on or executed directly."]
                    #[doc = ""]
                    #[doc = "Requires the sender to be member."]
                    #[doc = ""]
                    #[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
                    #[doc = "or put up for voting."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
                    #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                    #[doc = "  - branching is influenced by `threshold` where:"]
                    #[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
                    #[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
                    propose {
                        #[codec(compact)]
                        threshold: ::core::primitive::u32,
                        proposal: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Add an aye or nay vote for the sender to the given proposal."]
                    #[doc = ""]
                    #[doc = "Requires the sender to be a member."]
                    #[doc = ""]
                    #[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
                    #[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
                    #[doc = "fee."]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
                    vote {
                        proposal: ::subxt::utils::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        approve: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    #[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
                    #[doc = ""]
                    #[doc = "May be called by any signed account in order to finish voting and close the proposal."]
                    #[doc = ""]
                    #[doc = "If called before the end of the voting period it will only close the vote if it is"]
                    #[doc = "has enough votes to be approved or disapproved."]
                    #[doc = ""]
                    #[doc = "If called after the end of the voting period abstentions are counted as rejections"]
                    #[doc = "unless there is a prime member set and the prime member cast an approval."]
                    #[doc = ""]
                    #[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
                    #[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
                    #[doc = ""]
                    #[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
                    #[doc = "proposal."]
                    #[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
                    #[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(B + M + P1 + P2)` where:"]
                    #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                    #[doc = "  - `P1` is the complexity of `proposal` preimage."]
                    #[doc = "  - `P2` is proposal-count (code-bounded)"]
                    close_old_weight {
                        proposal_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        proposal_weight_bound: runtime_types::sp_weights::OldWeight,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
                    #[doc = "state."]
                    #[doc = ""]
                    #[doc = "Must be called by the Root origin."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "O(P) where P is the number of max proposals"]
                    disapprove_proposal { proposal_hash: ::subxt::utils::H256 },
                    #[codec(index = 6)]
                    #[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
                    #[doc = ""]
                    #[doc = "May be called by any signed account in order to finish voting and close the proposal."]
                    #[doc = ""]
                    #[doc = "If called before the end of the voting period it will only close the vote if it is"]
                    #[doc = "has enough votes to be approved or disapproved."]
                    #[doc = ""]
                    #[doc = "If called after the end of the voting period abstentions are counted as rejections"]
                    #[doc = "unless there is a prime member set and the prime member cast an approval."]
                    #[doc = ""]
                    #[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
                    #[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
                    #[doc = ""]
                    #[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
                    #[doc = "proposal."]
                    #[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
                    #[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(B + M + P1 + P2)` where:"]
                    #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                    #[doc = "  - `P1` is the complexity of `proposal` preimage."]
                    #[doc = "  - `P2` is proposal-count (code-bounded)"]
                    close {
                        proposal_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        proposal_weight_bound:
                            runtime_types::sp_weights::weight_v2::Weight,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Members(::core::primitive::u32, ::core::primitive::u32),
                #[codec(index = 1)]
                Member(_0),
                #[codec(index = 2)]
                _Phantom,
            }
        }
        pub mod pallet_contracts {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Deprecated version if [`Self::call`] for use in an in-storage `Call`."]
                    call_old_weight {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: runtime_types::sp_weights::OldWeight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Deprecated version if [`Self::instantiate_with_code`] for use in an in-storage `Call`."]
                    instantiate_with_code_old_weight {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: runtime_types::sp_weights::OldWeight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Deprecated version if [`Self::instantiate`] for use in an in-storage `Call`."]
                    instantiate_old_weight {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: runtime_types::sp_weights::OldWeight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code_hash: ::subxt::utils::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Upload new `code` without instantiating a contract from it."]
                    #[doc = ""]
                    #[doc = "If the code does not already exist a deposit is reserved from the caller"]
                    #[doc = "and unreserved only when [`Self::remove_code`] is called. The size of the reserve"]
                    #[doc = "depends on the instrumented size of the the supplied `code`."]
                    #[doc = ""]
                    #[doc = "If the code already exists in storage it will still return `Ok` and upgrades"]
                    #[doc = "the in storage version to the current"]
                    #[doc = "[`InstructionWeights::version`](InstructionWeights)."]
                    #[doc = ""]
                    #[doc = "- `determinism`: If this is set to any other value but [`Determinism::Deterministic`]"]
                    #[doc = "  then the only way to use this code is to delegate call into it from an offchain"]
                    #[doc = "  execution. Set to [`Determinism::Deterministic`] if in doubt."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "Anyone can instantiate a contract from any uploaded code and thus prevent its removal."]
                    #[doc = "To avoid this situation a constructor could employ access control so that it can"]
                    #[doc = "only be instantiated by permissioned entities. The same is true when uploading"]
                    #[doc = "through [`Self::instantiate_with_code`]."]
                    upload_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        determinism: runtime_types::pallet_contracts::wasm::Determinism,
                    },
                    #[codec(index = 4)]
                    #[doc = "Remove the code stored under `code_hash` and refund the deposit to its owner."]
                    #[doc = ""]
                    #[doc = "A code can only be removed by its original uploader (its owner) and only if it is"]
                    #[doc = "not used by any contract."]
                    remove_code { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 5)]
                    #[doc = "Privileged function that changes the code of an existing contract."]
                    #[doc = ""]
                    #[doc = "This takes care of updating refcounts and all other necessary operations. Returns"]
                    #[doc = "an error if either the `code_hash` or `dest` do not exist."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "This does **not** change the address of the contract in question. This means"]
                    #[doc = "that the contract address is no longer derived from its code hash after calling"]
                    #[doc = "this dispatchable."]
                    set_code {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        code_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "Makes a call to an account, optionally transferring some balance."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = ""]
                    #[doc = "* `dest`: Address of the contract to call."]
                    #[doc = "* `value`: The balance to transfer from the `origin` to `dest`."]
                    #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                    #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged from the"]
                    #[doc = "  caller to pay for the storage consumed."]
                    #[doc = "* `data`: The input data to pass to the contract."]
                    #[doc = ""]
                    #[doc = "* If the account is a smart-contract account, the associated code will be"]
                    #[doc = "executed and any value will be transferred."]
                    #[doc = "* If the account is a regular account, any value will be transferred."]
                    #[doc = "* If no account exists and the call value is not less than `existential_deposit`,"]
                    #[doc = "a regular account will be created and any value will be transferred."]
                    call {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Instantiates a new contract from the supplied `code` optionally transferring"]
                    #[doc = "some balance."]
                    #[doc = ""]
                    #[doc = "This dispatchable has the same effect as calling [`Self::upload_code`] +"]
                    #[doc = "[`Self::instantiate`]. Bundling them together provides efficiency gains. Please"]
                    #[doc = "also check the documentation of [`Self::upload_code`]."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = ""]
                    #[doc = "* `value`: The balance to transfer from the `origin` to the newly created contract."]
                    #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                    #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged/reserved"]
                    #[doc = "  from the caller to pay for the storage consumed."]
                    #[doc = "* `code`: The contract code to deploy in raw bytes."]
                    #[doc = "* `data`: The input data to pass to the contract constructor."]
                    #[doc = "* `salt`: Used for the address derivation. See [`Pallet::contract_address`]."]
                    #[doc = ""]
                    #[doc = "Instantiation is executed as follows:"]
                    #[doc = ""]
                    #[doc = "- The supplied `code` is instrumented, deployed, and a `code_hash` is created for that"]
                    #[doc = "  code."]
                    #[doc = "- If the `code_hash` already exists on the chain the underlying `code` will be shared."]
                    #[doc = "- The destination address is computed based on the sender, code_hash and the salt."]
                    #[doc = "- The smart-contract account is created at the computed address."]
                    #[doc = "- The `value` is transferred to the new account."]
                    #[doc = "- The `deploy` function is executed in the context of the newly-created account."]
                    instantiate_with_code {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    #[doc = "Instantiates a contract from a previously deployed wasm binary."]
                    #[doc = ""]
                    #[doc = "This function is identical to [`Self::instantiate_with_code`] but without the"]
                    #[doc = "code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary"]
                    #[doc = "must be supplied."]
                    instantiate {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code_hash: ::subxt::utils::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
            pub mod wasm {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum Determinism {
                    #[codec(index = 0)]
                    Deterministic,
                    #[codec(index = 1)]
                    AllowIndeterminism,
                }
            }
        }
        pub mod pallet_conviction_voting {
            use super::runtime_types;
            pub mod conviction {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum Conviction {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Locked1x,
                    #[codec(index = 2)]
                    Locked2x,
                    #[codec(index = 3)]
                    Locked3x,
                    #[codec(index = 4)]
                    Locked4x,
                    #[codec(index = 5)]
                    Locked5x,
                    #[codec(index = 6)]
                    Locked6x,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Vote in a poll. If `vote.is_aye()`, the vote is to enact the proposal;"] # [doc = "otherwise it is a vote to keep the status quo."] # [doc = ""] # [doc = "The dispatch origin of this call must be _Signed_."] # [doc = ""] # [doc = "- `poll_index`: The index of the poll to vote for."] # [doc = "- `vote`: The vote configuration."] # [doc = ""] # [doc = "Weight: `O(R)` where R is the number of polls the voter has voted on."] vote { # [codec (compact)] poll_index : :: core :: primitive :: u32 , vote : runtime_types :: pallet_conviction_voting :: vote :: AccountVote < :: core :: primitive :: u128 > , } , # [codec (index = 1)] # [doc = "Delegate the voting power (with some given conviction) of the sending account for a"] # [doc = "particular class of polls."] # [doc = ""] # [doc = "The balance delegated is locked for as long as it's delegated, and thereafter for the"] # [doc = "time appropriate for the conviction's lock period."] # [doc = ""] # [doc = "The dispatch origin of this call must be _Signed_, and the signing account must either:"] # [doc = "  - be delegating already; or"] # [doc = "  - have no voting activity (if there is, then it will need to be removed/consolidated"] # [doc = "    through `reap_vote` or `unvote`)."] # [doc = ""] # [doc = "- `to`: The account whose voting the `target` account's voting power will follow."] # [doc = "- `class`: The class of polls to delegate. To delegate multiple classes, multiple calls"] # [doc = "  to this function are required."] # [doc = "- `conviction`: The conviction that will be attached to the delegated votes. When the"] # [doc = "  account is undelegated, the funds will be locked for the corresponding period."] # [doc = "- `balance`: The amount of the account's balance to be used in delegating. This must not"] # [doc = "  be more than the account's current balance."] # [doc = ""] # [doc = "Emits `Delegated`."] # [doc = ""] # [doc = "Weight: `O(R)` where R is the number of polls the voter delegating to has"] # [doc = "  voted on. Weight is initially charged as if maximum votes, but is refunded later."] delegate { class : :: core :: primitive :: u16 , to : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , conviction : runtime_types :: pallet_conviction_voting :: conviction :: Conviction , balance : :: core :: primitive :: u128 , } , # [codec (index = 2)] # [doc = "Undelegate the voting power of the sending account for a particular class of polls."] # [doc = ""] # [doc = "Tokens may be unlocked following once an amount of time consistent with the lock period"] # [doc = "of the conviction with which the delegation was issued has passed."] # [doc = ""] # [doc = "The dispatch origin of this call must be _Signed_ and the signing account must be"] # [doc = "currently delegating."] # [doc = ""] # [doc = "- `class`: The class of polls to remove the delegation from."] # [doc = ""] # [doc = "Emits `Undelegated`."] # [doc = ""] # [doc = "Weight: `O(R)` where R is the number of polls the voter delegating to has"] # [doc = "  voted on. Weight is initially charged as if maximum votes, but is refunded later."] undelegate { class : :: core :: primitive :: u16 , } , # [codec (index = 3)] # [doc = "Remove the lock caused by prior voting/delegating which has expired within a particular"] # [doc = "class."] # [doc = ""] # [doc = "The dispatch origin of this call must be _Signed_."] # [doc = ""] # [doc = "- `class`: The class of polls to unlock."] # [doc = "- `target`: The account to remove the lock on."] # [doc = ""] # [doc = "Weight: `O(R)` with R number of vote of target."] unlock { class : :: core :: primitive :: u16 , target : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 4)] # [doc = "Remove a vote for a poll."] # [doc = ""] # [doc = "If:"] # [doc = "- the poll was cancelled, or"] # [doc = "- the poll is ongoing, or"] # [doc = "- the poll has ended such that"] # [doc = "  - the vote of the account was in opposition to the result; or"] # [doc = "  - there was no conviction to the account's vote; or"] # [doc = "  - the account made a split vote"] # [doc = "...then the vote is removed cleanly and a following call to `unlock` may result in more"] # [doc = "funds being available."] # [doc = ""] # [doc = "If, however, the poll has ended and:"] # [doc = "- it finished corresponding to the vote of the account, and"] # [doc = "- the account made a standard vote with conviction, and"] # [doc = "- the lock period of the conviction is not over"] # [doc = "...then the lock will be aggregated into the overall account's lock, which may involve"] # [doc = "*overlocking* (where the two locks are combined into a single lock that is the maximum"] # [doc = "of both the amount locked and the time is it locked for)."] # [doc = ""] # [doc = "The dispatch origin of this call must be _Signed_, and the signer must have a vote"] # [doc = "registered for poll `index`."] # [doc = ""] # [doc = "- `index`: The index of poll of the vote to be removed."] # [doc = "- `class`: Optional parameter, if given it indicates the class of the poll. For polls"] # [doc = "  which have finished or are cancelled, this must be `Some`."] # [doc = ""] # [doc = "Weight: `O(R + log R)` where R is the number of polls that `target` has voted on."] # [doc = "  Weight is calculated for the maximum number of vote."] remove_vote { class : :: core :: option :: Option < :: core :: primitive :: u16 > , index : :: core :: primitive :: u32 , } , # [codec (index = 5)] # [doc = "Remove a vote for a poll."] # [doc = ""] # [doc = "If the `target` is equal to the signer, then this function is exactly equivalent to"] # [doc = "`remove_vote`. If not equal to the signer, then the vote must have expired,"] # [doc = "either because the poll was cancelled, because the voter lost the poll or"] # [doc = "because the conviction period is over."] # [doc = ""] # [doc = "The dispatch origin of this call must be _Signed_."] # [doc = ""] # [doc = "- `target`: The account of the vote to be removed; this account must have voted for poll"] # [doc = "  `index`."] # [doc = "- `index`: The index of poll of the vote to be removed."] # [doc = "- `class`: The class of the poll."] # [doc = ""] # [doc = "Weight: `O(R + log R)` where R is the number of polls that `target` has voted on."] # [doc = "  Weight is calculated for the maximum number of vote."] remove_other_vote { target : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , class : :: core :: primitive :: u16 , index : :: core :: primitive :: u32 , } , }
            }
            pub mod vote {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum AccountVote<_0> {
                    #[codec(index = 0)]
                    Standard {
                        vote: runtime_types::pallet_conviction_voting::vote::Vote,
                        balance: _0,
                    },
                    #[codec(index = 1)]
                    Split { aye: _0, nay: _0 },
                    #[codec(index = 2)]
                    SplitAbstain { aye: _0, nay: _0, abstain: _0 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Vote(pub ::core::primitive::u8);
            }
        }
        pub mod pallet_democracy {
            use super::runtime_types;
            pub mod conviction {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum Conviction {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Locked1x,
                    #[codec(index = 2)]
                    Locked2x,
                    #[codec(index = 3)]
                    Locked3x,
                    #[codec(index = 4)]
                    Locked4x,
                    #[codec(index = 5)]
                    Locked5x,
                    #[codec(index = 6)]
                    Locked6x,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Propose a sensitive action to be taken."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Signed_ and the sender must"]
                    #[doc = "have funds to cover the deposit."]
                    #[doc = ""]
                    #[doc = "- `proposal_hash`: The hash of the proposal preimage."]
                    #[doc = "- `value`: The amount of deposit (must be at least `MinimumDeposit`)."]
                    #[doc = ""]
                    #[doc = "Emits `Proposed`."]
                    propose {
                        proposal:
                            runtime_types::frame_support::traits::preimages::Bounded<
                                runtime_types::kitchensink_runtime::RuntimeCall,
                            >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Signals agreement with a particular proposal."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Signed_ and the sender"]
                    #[doc = "must have funds to cover the deposit, equal to the original deposit."]
                    #[doc = ""]
                    #[doc = "- `proposal`: The index of the proposal to second."]
                    second {
                        #[codec(compact)]
                        proposal: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;"]
                    #[doc = "otherwise it is a vote to keep the status quo."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `ref_index`: The index of the referendum to vote for."]
                    #[doc = "- `vote`: The vote configuration."]
                    vote {
                        #[codec(compact)]
                        ref_index: ::core::primitive::u32,
                        vote: runtime_types::pallet_democracy::vote::AccountVote<
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Schedule an emergency cancellation of a referendum. Cannot happen twice to the same"]
                    #[doc = "referendum."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be `CancellationOrigin`."]
                    #[doc = ""]
                    #[doc = "-`ref_index`: The index of the referendum to cancel."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`."]
                    emergency_cancel { ref_index: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "Schedule a referendum to be tabled once it is legal to schedule an external"]
                    #[doc = "referendum."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be `ExternalOrigin`."]
                    #[doc = ""]
                    #[doc = "- `proposal_hash`: The preimage hash of the proposal."]
                    external_propose {
                        proposal:
                            runtime_types::frame_support::traits::preimages::Bounded<
                                runtime_types::kitchensink_runtime::RuntimeCall,
                            >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Schedule a majority-carries referendum to be tabled next once it is legal to schedule"]
                    #[doc = "an external referendum."]
                    #[doc = ""]
                    #[doc = "The dispatch of this call must be `ExternalMajorityOrigin`."]
                    #[doc = ""]
                    #[doc = "- `proposal_hash`: The preimage hash of the proposal."]
                    #[doc = ""]
                    #[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
                    #[doc = "pre-scheduled `external_propose` call."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    external_propose_majority {
                        proposal:
                            runtime_types::frame_support::traits::preimages::Bounded<
                                runtime_types::kitchensink_runtime::RuntimeCall,
                            >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Schedule a negative-turnout-bias referendum to be tabled next once it is legal to"]
                    #[doc = "schedule an external referendum."]
                    #[doc = ""]
                    #[doc = "The dispatch of this call must be `ExternalDefaultOrigin`."]
                    #[doc = ""]
                    #[doc = "- `proposal_hash`: The preimage hash of the proposal."]
                    #[doc = ""]
                    #[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
                    #[doc = "pre-scheduled `external_propose` call."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    external_propose_default {
                        proposal:
                            runtime_types::frame_support::traits::preimages::Bounded<
                                runtime_types::kitchensink_runtime::RuntimeCall,
                            >,
                    },
                    #[codec(index = 7)]
                    #[doc = "Schedule the currently externally-proposed majority-carries referendum to be tabled"]
                    #[doc = "immediately. If there is no externally-proposed referendum currently, or if there is one"]
                    #[doc = "but it is not a majority-carries referendum then it fails."]
                    #[doc = ""]
                    #[doc = "The dispatch of this call must be `FastTrackOrigin`."]
                    #[doc = ""]
                    #[doc = "- `proposal_hash`: The hash of the current external proposal."]
                    #[doc = "- `voting_period`: The period that is allowed for voting on this proposal. Increased to"]
                    #[doc = "\tMust be always greater than zero."]
                    #[doc = "\tFor `FastTrackOrigin` must be equal or greater than `FastTrackVotingPeriod`."]
                    #[doc = "- `delay`: The number of block after voting has ended in approval and this should be"]
                    #[doc = "  enacted. This doesn't have a minimum amount."]
                    #[doc = ""]
                    #[doc = "Emits `Started`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    fast_track {
                        proposal_hash: ::subxt::utils::H256,
                        voting_period: ::core::primitive::u32,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Veto and blacklist the external proposal hash."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be `VetoOrigin`."]
                    #[doc = ""]
                    #[doc = "- `proposal_hash`: The preimage hash of the proposal to veto and blacklist."]
                    #[doc = ""]
                    #[doc = "Emits `Vetoed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(V + log(V))` where V is number of `existing vetoers`"]
                    veto_external { proposal_hash: ::subxt::utils::H256 },
                    #[codec(index = 9)]
                    #[doc = "Remove a referendum."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "- `ref_index`: The index of the referendum to cancel."]
                    #[doc = ""]
                    #[doc = "# Weight: `O(1)`."]
                    cancel_referendum {
                        #[codec(compact)]
                        ref_index: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    #[doc = "Delegate the voting power (with some given conviction) of the sending account."]
                    #[doc = ""]
                    #[doc = "The balance delegated is locked for as long as it's delegated, and thereafter for the"]
                    #[doc = "time appropriate for the conviction's lock period."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Signed_, and the signing account must either:"]
                    #[doc = "  - be delegating already; or"]
                    #[doc = "  - have no voting activity (if there is, then it will need to be removed/consolidated"]
                    #[doc = "    through `reap_vote` or `unvote`)."]
                    #[doc = ""]
                    #[doc = "- `to`: The account whose voting the `target` account's voting power will follow."]
                    #[doc = "- `conviction`: The conviction that will be attached to the delegated votes. When the"]
                    #[doc = "  account is undelegated, the funds will be locked for the corresponding period."]
                    #[doc = "- `balance`: The amount of the account's balance to be used in delegating. This must not"]
                    #[doc = "  be more than the account's current balance."]
                    #[doc = ""]
                    #[doc = "Emits `Delegated`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
                    #[doc = "  voted on. Weight is charged as if maximum votes."]
                    delegate {
                        to: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        conviction:
                            runtime_types::pallet_democracy::conviction::Conviction,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Undelegate the voting power of the sending account."]
                    #[doc = ""]
                    #[doc = "Tokens may be unlocked following once an amount of time consistent with the lock period"]
                    #[doc = "of the conviction with which the delegation was issued."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Signed_ and the signing account must be"]
                    #[doc = "currently delegating."]
                    #[doc = ""]
                    #[doc = "Emits `Undelegated`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
                    #[doc = "  voted on. Weight is charged as if maximum votes."]
                    undelegate,
                    #[codec(index = 12)]
                    #[doc = "Clears all public proposals."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`."]
                    clear_public_proposals,
                    #[codec(index = 13)]
                    #[doc = "Unlock tokens that have an expired lock."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `target`: The account to remove the lock on."]
                    #[doc = ""]
                    #[doc = "Weight: `O(R)` with R number of vote of target."]
                    unlock {
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 14)]
                    #[doc = "Remove a vote for a referendum."]
                    #[doc = ""]
                    #[doc = "If:"]
                    #[doc = "- the referendum was cancelled, or"]
                    #[doc = "- the referendum is ongoing, or"]
                    #[doc = "- the referendum has ended such that"]
                    #[doc = "  - the vote of the account was in opposition to the result; or"]
                    #[doc = "  - there was no conviction to the account's vote; or"]
                    #[doc = "  - the account made a split vote"]
                    #[doc = "...then the vote is removed cleanly and a following call to `unlock` may result in more"]
                    #[doc = "funds being available."]
                    #[doc = ""]
                    #[doc = "If, however, the referendum has ended and:"]
                    #[doc = "- it finished corresponding to the vote of the account, and"]
                    #[doc = "- the account made a standard vote with conviction, and"]
                    #[doc = "- the lock period of the conviction is not over"]
                    #[doc = "...then the lock will be aggregated into the overall account's lock, which may involve"]
                    #[doc = "*overlocking* (where the two locks are combined into a single lock that is the maximum"]
                    #[doc = "of both the amount locked and the time is it locked for)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Signed_, and the signer must have a vote"]
                    #[doc = "registered for referendum `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: The index of referendum of the vote to be removed."]
                    #[doc = ""]
                    #[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
                    #[doc = "  Weight is calculated for the maximum number of vote."]
                    remove_vote { index: ::core::primitive::u32 },
                    #[codec(index = 15)]
                    #[doc = "Remove a vote for a referendum."]
                    #[doc = ""]
                    #[doc = "If the `target` is equal to the signer, then this function is exactly equivalent to"]
                    #[doc = "`remove_vote`. If not equal to the signer, then the vote must have expired,"]
                    #[doc = "either because the referendum was cancelled, because the voter lost the referendum or"]
                    #[doc = "because the conviction period is over."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `target`: The account of the vote to be removed; this account must have voted for"]
                    #[doc = "  referendum `index`."]
                    #[doc = "- `index`: The index of referendum of the vote to be removed."]
                    #[doc = ""]
                    #[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
                    #[doc = "  Weight is calculated for the maximum number of vote."]
                    remove_other_vote {
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    #[doc = "Permanently place a proposal into the blacklist. This prevents it from ever being"]
                    #[doc = "proposed again."]
                    #[doc = ""]
                    #[doc = "If called on a queued public or external proposal, then this will result in it being"]
                    #[doc = "removed. If the `ref_index` supplied is an active referendum with the proposal hash,"]
                    #[doc = "then it will be cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be `BlacklistOrigin`."]
                    #[doc = ""]
                    #[doc = "- `proposal_hash`: The proposal hash to blacklist permanently."]
                    #[doc = "- `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be"]
                    #[doc = "cancelled."]
                    #[doc = ""]
                    #[doc = "Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a"]
                    #[doc = "  reasonable value)."]
                    blacklist {
                        proposal_hash: ::subxt::utils::H256,
                        maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 17)]
                    #[doc = "Remove a proposal."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be `CancelProposalOrigin`."]
                    #[doc = ""]
                    #[doc = "- `prop_index`: The index of the proposal to cancel."]
                    #[doc = ""]
                    #[doc = "Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`"]
                    cancel_proposal {
                        #[codec(compact)]
                        prop_index: ::core::primitive::u32,
                    },
                    #[codec(index = 18)]
                    #[doc = "Set or clear a metadata of a proposal or a referendum."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `origin`: Must correspond to the `MetadataOwner`."]
                    #[doc = "    - `ExternalOrigin` for an external proposal with the `SuperMajorityApprove`"]
                    #[doc = "      threshold."]
                    #[doc = "    - `ExternalDefaultOrigin` for an external proposal with the `SuperMajorityAgainst`"]
                    #[doc = "      threshold."]
                    #[doc = "    - `ExternalMajorityOrigin` for an external proposal with the `SimpleMajority`"]
                    #[doc = "      threshold."]
                    #[doc = "    - `Signed` by a creator for a public proposal."]
                    #[doc = "    - `Signed` to clear a metadata for a finished referendum."]
                    #[doc = "    - `Root` to set a metadata for an ongoing referendum."]
                    #[doc = "- `owner`: an identifier of a metadata owner."]
                    #[doc = "- `maybe_hash`: The hash of an on-chain stored preimage. `None` to clear a metadata."]
                    set_metadata {
                        owner: runtime_types::pallet_democracy::types::MetadataOwner,
                        maybe_hash: ::core::option::Option<::subxt::utils::H256>,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum MetadataOwner {
                    #[codec(index = 0)]
                    External,
                    #[codec(index = 1)]
                    Proposal(::core::primitive::u32),
                    #[codec(index = 2)]
                    Referendum(::core::primitive::u32),
                }
            }
            pub mod vote {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum AccountVote<_0> {
                    #[codec(index = 0)]
                    Standard {
                        vote: runtime_types::pallet_democracy::vote::Vote,
                        balance: _0,
                    },
                    #[codec(index = 1)]
                    Split { aye: _0, nay: _0 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Vote(pub ::core::primitive::u8);
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Submit a solution for the unsigned phase."] # [doc = ""] # [doc = "The dispatch origin fo this call must be __none__."] # [doc = ""] # [doc = "This submission is checked on the fly. Moreover, this unsigned solution is only"] # [doc = "validated when submitted to the pool from the **local** node. Effectively, this means"] # [doc = "that only active validators can submit this transaction when authoring a block (similar"] # [doc = "to an inherent)."] # [doc = ""] # [doc = "To prevent any incorrect solution (and thus wasted time/weight), this transaction will"] # [doc = "panic if the solution submitted by the validator is invalid in any way, effectively"] # [doc = "putting their authoring reward at risk."] # [doc = ""] # [doc = "No deposit or reward is associated with this submission."] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: kitchensink_runtime :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] # [doc = "Set a new value for `MinimumUntrustedScore`."] # [doc = ""] # [doc = "Dispatch origin must be aligned with `T::ForceOrigin`."] # [doc = ""] # [doc = "This check can be turned off by setting the value to `None`."] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < runtime_types :: sp_npos_elections :: ElectionScore > , } , # [codec (index = 2)] # [doc = "Set a solution in the queue, to be handed out to the client of this pallet in the next"] # [doc = "call to `ElectionProvider::elect`."] # [doc = ""] # [doc = "This can only be set by `T::ForceOrigin`, and only when the phase is `Emergency`."] # [doc = ""] # [doc = "The solution is not checked for any feasibility and is assumed to be trustworthy, as any"] # [doc = "feasibility check itself can in principle cause the election process to fail (due to"] # [doc = "memory/weight constrains)."] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: utils :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: utils :: AccountId32 > ,) > , } , # [codec (index = 3)] # [doc = "Submit a solution for the signed phase."] # [doc = ""] # [doc = "The dispatch origin fo this call must be __signed__."] # [doc = ""] # [doc = "The solution is potentially queued, based on the claimed score and processed at the end"] # [doc = "of the signed phase."] # [doc = ""] # [doc = "A deposit is reserved and recorded for the solution. Based on the outcome, the solution"] # [doc = "might be rewarded, slashed, or get all or a part of the deposit back."] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: kitchensink_runtime :: NposSolution16 > > , } , # [codec (index = 4)] # [doc = "Trigger the governance fallback."] # [doc = ""] # [doc = "This can only be called when [`Phase::Emergency`] is enabled, as an alternative to"] # [doc = "calling [`Call::set_emergency_election_result`]."] governance_fallback { maybe_max_voters : :: core :: option :: Option < :: core :: primitive :: u32 > , maybe_max_targets : :: core :: option :: Option < :: core :: primitive :: u32 > , } , }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: runtime_types::sp_npos_elections::ElectionScore,
                pub round: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Vote for a set of candidates for the upcoming round of election. This can be called to"]
                    #[doc = "set the initial votes, or update already existing votes."]
                    #[doc = ""]
                    #[doc = "Upon initial voting, `value` units of `who`'s balance is locked and a deposit amount is"]
                    #[doc = "reserved. The deposit is based on the number of votes and can be updated over time."]
                    #[doc = ""]
                    #[doc = "The `votes` should:"]
                    #[doc = "  - not be empty."]
                    #[doc = "  - be less than the number of possible candidates. Note that all current members and"]
                    #[doc = "    runners-up are also automatically candidates for the next round."]
                    #[doc = ""]
                    #[doc = "If `value` is more than `who`'s free balance, then the maximum of the two is used."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed."]
                    #[doc = ""]
                    #[doc = "### Warning"]
                    #[doc = ""]
                    #[doc = "It is the responsibility of the caller to **NOT** place all of their balance into the"]
                    #[doc = "lock and keep some for further operations."]
                    vote {
                        votes: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Remove `origin` as a voter."]
                    #[doc = ""]
                    #[doc = "This removes the lock and returns the deposit."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed and be a voter."]
                    remove_voter,
                    #[codec(index = 2)]
                    #[doc = "Submit oneself for candidacy. A fixed amount of deposit is recorded."]
                    #[doc = ""]
                    #[doc = "All candidates are wiped at the end of the term. They either become a member/runner-up,"]
                    #[doc = "or leave the system while their deposit is slashed."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed."]
                    #[doc = ""]
                    #[doc = "### Warning"]
                    #[doc = ""]
                    #[doc = "Even if a candidate ends up being a member, they must call [`Call::renounce_candidacy`]"]
                    #[doc = "to get their deposit back. Losing the spot in an election will always lead to a slash."]
                    #[doc = ""]
                    #[doc = "The number of current candidates must be provided as witness data."]
                    #[doc = "## Complexity"]
                    #[doc = "O(C + log(C)) where C is candidate_count."]
                    submit_candidacy {
                        #[codec(compact)]
                        candidate_count: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Renounce one's intention to be a candidate for the next election round. 3 potential"]
                    #[doc = "outcomes exist:"]
                    #[doc = ""]
                    #[doc = "- `origin` is a candidate and not elected in any set. In this case, the deposit is"]
                    #[doc = "  unreserved, returned and origin is removed as a candidate."]
                    #[doc = "- `origin` is a current runner-up. In this case, the deposit is unreserved, returned and"]
                    #[doc = "  origin is removed as a runner-up."]
                    #[doc = "- `origin` is a current member. In this case, the deposit is unreserved and origin is"]
                    #[doc = "  removed as a member, consequently not being a candidate for the next round anymore."]
                    #[doc = "  Similar to [`remove_member`](Self::remove_member), if replacement runners exists, they"]
                    #[doc = "  are immediately used. If the prime is renouncing, then no prime will exist until the"]
                    #[doc = "  next round."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed, and have one of the above roles."]
                    #[doc = "The type of renouncing must be provided as witness data."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "  - Renouncing::Candidate(count): O(count + log(count))"]
                    #[doc = "  - Renouncing::Member: O(1)"]
                    #[doc = "  - Renouncing::RunnerUp: O(1)"]
                    renounce_candidacy {
                        renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
                    },
                    #[codec(index = 4)]
                    #[doc = "Remove a particular member from the set. This is effective immediately and the bond of"]
                    #[doc = "the outgoing member is slashed."]
                    #[doc = ""]
                    #[doc = "If a runner-up is available, then the best runner-up will be removed and replaces the"]
                    #[doc = "outgoing member. Otherwise, if `rerun_election` is `true`, a new phragmen election is"]
                    #[doc = "started, else, nothing happens."]
                    #[doc = ""]
                    #[doc = "If `slash_bond` is set to true, the bond of the member being removed is slashed. Else,"]
                    #[doc = "it is returned."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be root."]
                    #[doc = ""]
                    #[doc = "Note that this does not affect the designated block number of the next election."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- Check details of remove_and_replace_member() and do_phragmen()."]
                    remove_member {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        slash_bond: ::core::primitive::bool,
                        rerun_election: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Clean all voters who are defunct (i.e. they do not serve any purpose at all). The"]
                    #[doc = "deposit of the removed voters are returned."]
                    #[doc = ""]
                    #[doc = "This is an root function to be used only for cleaning the state."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be root."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- Check is_defunct_voter() details."]
                    clean_defunct_voters {
                        num_voters: ::core::primitive::u32,
                        num_defunct: ::core::primitive::u32,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum Renouncing {
                #[codec(index = 0)]
                Member,
                #[codec(index = 1)]
                RunnerUp,
                #[codec(index = 2)]
                Candidate(#[codec(compact)] ::core::primitive::u32),
            }
        }
        pub mod pallet_fast_unstake {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Register oneself for fast-unstake."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed by the controller account, similar to"]
                    #[doc = "`staking::unbond`."]
                    #[doc = ""]
                    #[doc = "The stash associated with the origin must have no ongoing unlocking chunks. If"]
                    #[doc = "successful, this will fully unbond and chill the stash. Then, it will enqueue the stash"]
                    #[doc = "to be checked in further blocks."]
                    #[doc = ""]
                    #[doc = "If by the time this is called, the stash is actually eligible for fast-unstake, then"]
                    #[doc = "they are guaranteed to remain eligible, because the call will chill them as well."]
                    #[doc = ""]
                    #[doc = "If the check works, the entire staking data is removed, i.e. the stash is fully"]
                    #[doc = "unstaked."]
                    #[doc = ""]
                    #[doc = "If the check fails, the stash remains chilled and waiting for being unbonded as in with"]
                    #[doc = "the normal staking system, but they lose part of their unbonding chunks due to consuming"]
                    #[doc = "the chain's resources."]
                    register_fast_unstake,
                    #[codec(index = 1)]
                    #[doc = "Deregister oneself from the fast-unstake."]
                    #[doc = ""]
                    #[doc = "This is useful if one is registered, they are still waiting, and they change their mind."]
                    #[doc = ""]
                    #[doc = "Note that the associated stash is still fully unbonded and chilled as a consequence of"]
                    #[doc = "calling `register_fast_unstake`. This should probably be followed by a call to"]
                    #[doc = "`Staking::rebond`."]
                    deregister,
                    #[codec(index = 2)]
                    #[doc = "Control the operation of this pallet."]
                    #[doc = ""]
                    #[doc = "Dispatch origin must be signed by the [`Config::ControlOrigin`]."]
                    control {
                        eras_to_check: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_glutton {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Initializes the pallet by writing into `TrashData`."]
                    #[doc = ""]
                    #[doc = "Only callable by Root. A good default for `trash_count` is `5_000`."]
                    initialize_pallet {
                        new_count: ::core::primitive::u32,
                        witness_count: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the `Compute` storage value that determines how much of the"]
                    #[doc = "block's weight `ref_time` to use during `on_idle`."]
                    #[doc = ""]
                    #[doc = "Only callable by Root."]
                    set_compute {
                        compute: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 2)]
                    #[doc = "Set the `Storage` storage value that determines the PoV size usage"]
                    #[doc = "for each block."]
                    #[doc = ""]
                    #[doc = "Only callable by Root."]
                    set_storage {
                        storage: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                    #[doc = ""]
                    #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                    #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                    #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                    #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                    #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                    #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                    #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                    #[doc = "block of all validators of the new authority set."]
                    #[doc = ""]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_identity {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Identity pallet declaration."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a registrar to the system."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `T::RegistrarOrigin`."]
                    #[doc = ""]
                    #[doc = "- `account`: the account of the registrar."]
                    #[doc = ""]
                    #[doc = "Emits `RegistrarAdded` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)` where `R` registrar-count (governance-bounded and code-bounded)."]
                    add_registrar {
                        account: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set an account's identity information and reserve the appropriate deposit."]
                    #[doc = ""]
                    #[doc = "If the account already has identity information, the deposit is taken as part payment"]
                    #[doc = "for the new deposit."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `info`: The identity information."]
                    #[doc = ""]
                    #[doc = "Emits `IdentitySet` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(X + X' + R)`"]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)"]
                    #[doc = "  - where `R` judgements-count (registrar-count-bounded)"]
                    set_identity {
                        info: ::std::boxed::Box<
                            runtime_types::pallet_identity::types::IdentityInfo,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Set the sub-accounts of the sender."]
                    #[doc = ""]
                    #[doc = "Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned"]
                    #[doc = "and an amount `SubAccountDeposit` will be reserved for each item in `subs`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "identity."]
                    #[doc = ""]
                    #[doc = "- `subs`: The identity's (new) sub-accounts."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(P + S)`"]
                    #[doc = "  - where `P` old-subs-count (hard- and deposit-bounded)."]
                    #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                    set_subs {
                        subs: ::std::vec::Vec<(
                            ::subxt::utils::AccountId32,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Clear an account's identity info and all sub-accounts and return all deposits."]
                    #[doc = ""]
                    #[doc = "Payment: All reserved balances on the account are returned."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "identity."]
                    #[doc = ""]
                    #[doc = "Emits `IdentityCleared` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + S + X)`"]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    clear_identity,
                    #[codec(index = 4)]
                    #[doc = "Request a judgement from a registrar."]
                    #[doc = ""]
                    #[doc = "Payment: At most `max_fee` will be reserved for payment to the registrar if judgement"]
                    #[doc = "given."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                    #[doc = "registered identity."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: The index of the registrar whose judgement is requested."]
                    #[doc = "- `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:"]
                    #[doc = ""]
                    #[doc = "```nocompile"]
                    #[doc = "Self::registrars().get(reg_index).unwrap().fee"]
                    #[doc = "```"]
                    #[doc = ""]
                    #[doc = "Emits `JudgementRequested` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + X)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    request_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        #[codec(compact)]
                        max_fee: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Cancel a previous request."]
                    #[doc = ""]
                    #[doc = "Payment: A previously reserved deposit is returned on success."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                    #[doc = "registered identity."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: The index of the registrar whose judgement is no longer requested."]
                    #[doc = ""]
                    #[doc = "Emits `JudgementUnrequested` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + X)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    cancel_request { reg_index: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "Set the fee required for a judgement to be requested from a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `fee`: the new fee."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    set_fee {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "Change the account associated with a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `new`: the new account ID."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    set_account_id {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        new: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the field information for a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `fields`: the fields that the registrar concerns themselves with."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    set_fields {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        fields: runtime_types::pallet_identity::types::BitFlags<
                            runtime_types::pallet_identity::types::IdentityField,
                        >,
                    },
                    #[codec(index = 9)]
                    #[doc = "Provide a judgement for an account's identity."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `reg_index`."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: the index of the registrar whose judgement is being made."]
                    #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                    #[doc = "  with a registered identity."]
                    #[doc = "- `judgement`: the judgement of the registrar of index `reg_index` about `target`."]
                    #[doc = "- `identity`: The hash of the [`IdentityInfo`] for that the judgement is provided."]
                    #[doc = ""]
                    #[doc = "Emits `JudgementGiven` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + X)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    provide_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        judgement: runtime_types::pallet_identity::types::Judgement<
                            ::core::primitive::u128,
                        >,
                        identity: ::subxt::utils::H256,
                    },
                    #[codec(index = 10)]
                    #[doc = "Remove an account's identity and sub-account information and slash the deposits."]
                    #[doc = ""]
                    #[doc = "Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by"]
                    #[doc = "`Slash`. Verification request deposits are not returned; they should be cancelled"]
                    #[doc = "manually using `cancel_request`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                    #[doc = "  with a registered identity."]
                    #[doc = ""]
                    #[doc = "Emits `IdentityKilled` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + S + X)`"]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    kill_identity {
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 11)]
                    #[doc = "Add the given account to the sender's subs."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    add_sub {
                        sub: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 12)]
                    #[doc = "Alter the associated name of the given sub-account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    rename_sub {
                        sub: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 13)]
                    #[doc = "Remove the given account from the sender's subs."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    remove_sub {
                        sub: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 14)]
                    #[doc = "Remove the sender as a sub-account."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender (*not* the original depositor)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "super-identity."]
                    #[doc = ""]
                    #[doc = "NOTE: This should not normally be used, but is provided in the case that the non-"]
                    #[doc = "controller of an account is maliciously registered as a sub-account."]
                    quit_sub,
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct BitFlags<_0>(
                    pub ::core::primitive::u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum Data {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Raw0([::core::primitive::u8; 0usize]),
                    #[codec(index = 2)]
                    Raw1([::core::primitive::u8; 1usize]),
                    #[codec(index = 3)]
                    Raw2([::core::primitive::u8; 2usize]),
                    #[codec(index = 4)]
                    Raw3([::core::primitive::u8; 3usize]),
                    #[codec(index = 5)]
                    Raw4([::core::primitive::u8; 4usize]),
                    #[codec(index = 6)]
                    Raw5([::core::primitive::u8; 5usize]),
                    #[codec(index = 7)]
                    Raw6([::core::primitive::u8; 6usize]),
                    #[codec(index = 8)]
                    Raw7([::core::primitive::u8; 7usize]),
                    #[codec(index = 9)]
                    Raw8([::core::primitive::u8; 8usize]),
                    #[codec(index = 10)]
                    Raw9([::core::primitive::u8; 9usize]),
                    #[codec(index = 11)]
                    Raw10([::core::primitive::u8; 10usize]),
                    #[codec(index = 12)]
                    Raw11([::core::primitive::u8; 11usize]),
                    #[codec(index = 13)]
                    Raw12([::core::primitive::u8; 12usize]),
                    #[codec(index = 14)]
                    Raw13([::core::primitive::u8; 13usize]),
                    #[codec(index = 15)]
                    Raw14([::core::primitive::u8; 14usize]),
                    #[codec(index = 16)]
                    Raw15([::core::primitive::u8; 15usize]),
                    #[codec(index = 17)]
                    Raw16([::core::primitive::u8; 16usize]),
                    #[codec(index = 18)]
                    Raw17([::core::primitive::u8; 17usize]),
                    #[codec(index = 19)]
                    Raw18([::core::primitive::u8; 18usize]),
                    #[codec(index = 20)]
                    Raw19([::core::primitive::u8; 19usize]),
                    #[codec(index = 21)]
                    Raw20([::core::primitive::u8; 20usize]),
                    #[codec(index = 22)]
                    Raw21([::core::primitive::u8; 21usize]),
                    #[codec(index = 23)]
                    Raw22([::core::primitive::u8; 22usize]),
                    #[codec(index = 24)]
                    Raw23([::core::primitive::u8; 23usize]),
                    #[codec(index = 25)]
                    Raw24([::core::primitive::u8; 24usize]),
                    #[codec(index = 26)]
                    Raw25([::core::primitive::u8; 25usize]),
                    #[codec(index = 27)]
                    Raw26([::core::primitive::u8; 26usize]),
                    #[codec(index = 28)]
                    Raw27([::core::primitive::u8; 27usize]),
                    #[codec(index = 29)]
                    Raw28([::core::primitive::u8; 28usize]),
                    #[codec(index = 30)]
                    Raw29([::core::primitive::u8; 29usize]),
                    #[codec(index = 31)]
                    Raw30([::core::primitive::u8; 30usize]),
                    #[codec(index = 32)]
                    Raw31([::core::primitive::u8; 31usize]),
                    #[codec(index = 33)]
                    Raw32([::core::primitive::u8; 32usize]),
                    #[codec(index = 34)]
                    BlakeTwo256([::core::primitive::u8; 32usize]),
                    #[codec(index = 35)]
                    Sha256([::core::primitive::u8; 32usize]),
                    #[codec(index = 36)]
                    Keccak256([::core::primitive::u8; 32usize]),
                    #[codec(index = 37)]
                    ShaThree256([::core::primitive::u8; 32usize]),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum IdentityField {
                    #[codec(index = 1)]
                    Display,
                    #[codec(index = 2)]
                    Legal,
                    #[codec(index = 4)]
                    Web,
                    #[codec(index = 8)]
                    Riot,
                    #[codec(index = 16)]
                    Email,
                    #[codec(index = 32)]
                    PgpFingerprint,
                    #[codec(index = 64)]
                    Image,
                    #[codec(index = 128)]
                    Twitter,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct IdentityInfo {
                    pub additional:
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<(
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum Judgement<_0> {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    FeePaid(_0),
                    #[codec(index = 2)]
                    Reasonable,
                    #[codec(index = 3)]
                    KnownGood,
                    #[codec(index = 4)]
                    OutOfDate,
                    #[codec(index = 5)]
                    LowQuality,
                    #[codec(index = 6)]
                    Erroneous,
                }
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "## Complexity:"] # [doc = "- `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is length of"] # [doc = "  `heartbeat.network_state.external_address`"] # [doc = "  - `O(K)`: decoding of length `K`"] # [doc = "  - `O(E)`: decoding/encoding of length `E`"] heartbeat { heartbeat : runtime_types :: pallet_im_online :: Heartbeat < :: core :: primitive :: u32 > , signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature , } , }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Assign an previously unassigned index."]
                    #[doc = ""]
                    #[doc = "Payment: `Deposit` is reserved from the sender account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be claimed. This must not be in use."]
                    #[doc = ""]
                    #[doc = "Emits `IndexAssigned` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    claim { index: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
                    #[doc = "is effectively transferred to the new account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
                    #[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
                    #[doc = ""]
                    #[doc = "Emits `IndexAssigned` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    transfer {
                        new: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Free up an index owned by the sender."]
                    #[doc = ""]
                    #[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be freed. This must be owned by the sender."]
                    #[doc = ""]
                    #[doc = "Emits `IndexFreed` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    free { index: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    #[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
                    #[doc = "held, then any deposit is reimbursed to its current owner."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be (re-)assigned."]
                    #[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
                    #[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
                    #[doc = ""]
                    #[doc = "Emits `IndexAssigned` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    force_transfer {
                        new: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        index: ::core::primitive::u32,
                        freeze: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    #[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
                    #[doc = "deposit."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
                    #[doc = "non-frozen account `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be frozen in place."]
                    #[doc = ""]
                    #[doc = "Emits `IndexFrozen` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    freeze { index: ::core::primitive::u32 },
                }
            }
        }
        pub mod pallet_lottery {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Buy a ticket to enter the lottery."]
                    #[doc = ""]
                    #[doc = "This extrinsic acts as a passthrough function for `call`. In all"]
                    #[doc = "situations where `call` alone would succeed, this extrinsic should"]
                    #[doc = "succeed."]
                    #[doc = ""]
                    #[doc = "If `call` is successful, then we will attempt to purchase a ticket,"]
                    #[doc = "which may fail silently. To detect success of a ticket purchase, you"]
                    #[doc = "should listen for the `TicketBought` event."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called by a signed origin."]
                    buy_ticket {
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set calls in storage which can be used to purchase a lottery ticket."]
                    #[doc = ""]
                    #[doc = "This function only matters if you use the `ValidateCall` implementation"]
                    #[doc = "provided by this pallet, which uses storage to determine the valid calls."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called by the Manager origin."]
                    set_calls {
                        calls: ::std::vec::Vec<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Start a lottery using the provided configuration."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called by the `ManagerOrigin`."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = ""]
                    #[doc = "* `price`: The cost of a single ticket."]
                    #[doc = "* `length`: How long the lottery should run for starting at the current block."]
                    #[doc = "* `delay`: How long after the lottery end we should wait before picking a winner."]
                    #[doc = "* `repeat`: If the lottery should repeat when completed."]
                    start_lottery {
                        price: ::core::primitive::u128,
                        length: ::core::primitive::u32,
                        delay: ::core::primitive::u32,
                        repeat: ::core::primitive::bool,
                    },
                    #[codec(index = 3)]
                    #[doc = "If a lottery is repeating, you can use this to stop the repeat."]
                    #[doc = "The lottery will continue to run to completion."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called by the `ManagerOrigin`."]
                    stop_repeat,
                }
            }
        }
        pub mod pallet_membership {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a member `who` to the set."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::AddOrigin`."]
                    add_member {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Remove a member `who` from the set."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::RemoveOrigin`."]
                    remove_member {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Swap out one member `remove` for another `add`."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::SwapOrigin`."]
                    #[doc = ""]
                    #[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
                    swap_member {
                        remove: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        add: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
                    #[doc = "pass `members` pre-sorted."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::ResetOrigin`."]
                    reset_members {
                        members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Swap out the sending member for some other key `new`."]
                    #[doc = ""]
                    #[doc = "May only be called from `Signed` origin of a current member."]
                    #[doc = ""]
                    #[doc = "Prime membership is passed from the origin account to `new`, if extant."]
                    change_key {
                        new: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set the prime member. Must be a current member."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::PrimeOrigin`."]
                    set_prime {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Remove the prime member if it exists."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::PrimeOrigin`."]
                    clear_prime,
                }
            }
        }
        pub mod pallet_message_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Remove a page which has no more messages remaining to be processed or is stale."]
                    reap_page {
                        message_origin: ::core::primitive::u32,
                        page_index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Execute an overweight message."]
                    #[doc = ""]
                    #[doc = "Temporary processing errors will be propagated whereas permanent errors are treated"]
                    #[doc = "as success condition."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `message_origin`: The origin from which the message to be executed arrived."]
                    #[doc = "- `page`: The page in the queue in which the message to be executed is sitting."]
                    #[doc = "- `index`: The index into the queue of the message to be executed."]
                    #[doc = "- `weight_limit`: The maximum amount of weight allowed to be consumed in the execution"]
                    #[doc = "  of the message."]
                    #[doc = ""]
                    #[doc = "Benchmark complexity considerations: O(index + weight_limit)."]
                    execute_overweight {
                        message_origin: ::core::primitive::u32,
                        page: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                        weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                    },
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
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
                    #[doc = "multi-signature, but do not participate in the approval process."]
                    #[doc = "- `call`: The call to be executed."]
                    #[doc = ""]
                    #[doc = "Result is equivalent to the dispatched result."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                    #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                    #[doc = ""]
                    #[doc = "If there are enough, then dispatch the call."]
                    #[doc = ""]
                    #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                    #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                    #[doc = "is cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                    #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                    #[doc = "transaction index) of the first approval transaction."]
                    #[doc = "- `call`: The call to be executed."]
                    #[doc = ""]
                    #[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
                    #[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
                    #[doc = ""]
                    #[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
                    #[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
                    #[doc = "may be found in the deposited `MultisigExecuted` event."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S + Z + Call)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                    #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                    #[doc = "- One event."]
                    #[doc = "- The weight of the `call`."]
                    #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                    #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<
                                ::core::primitive::u32,
                            >,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                    #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                    #[doc = ""]
                    #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                    #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                    #[doc = "is cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                    #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                    #[doc = "transaction index) of the first approval transaction."]
                    #[doc = "- `call_hash`: The hash of the call to be executed."]
                    #[doc = ""]
                    #[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                    #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                    #[doc = "- One event."]
                    #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                    #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<
                                ::core::primitive::u32,
                            >,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 3)]
                    #[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
                    #[doc = "for this operation will be unreserved on success."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
                    #[doc = "transaction for this dispatch."]
                    #[doc = "- `call_hash`: The hash of the call to be executed."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- One event."]
                    #[doc = "- I/O: 1 read `O(S)`, one remove."]
                    #[doc = "- Storage: removes one item."]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        timepoint: runtime_types::pallet_multisig::Timepoint<
                            ::core::primitive::u32,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_nfts {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Issue a new collection of non-fungible items from a public origin."] # [doc = ""] # [doc = "This new collection has no items initially and its owner is the origin."] # [doc = ""] # [doc = "The origin must be Signed and the sender must have sufficient funds free."] # [doc = ""] # [doc = "`ItemDeposit` funds of sender are reserved."] # [doc = ""] # [doc = "Parameters:"] # [doc = "- `admin`: The admin of this collection. The admin is the initial address of each"] # [doc = "member of the collection's admin team."] # [doc = ""] # [doc = "Emits `Created` event when successful."] # [doc = ""] # [doc = "Weight: `O(1)`"] create { admin : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , config : runtime_types :: pallet_nfts :: types :: CollectionConfig < :: core :: primitive :: u128 , :: core :: primitive :: u32 , :: core :: primitive :: u32 > , } , # [codec (index = 1)] # [doc = "Issue a new collection of non-fungible items from a privileged origin."] # [doc = ""] # [doc = "This new collection has no items initially."] # [doc = ""] # [doc = "The origin must conform to `ForceOrigin`."] # [doc = ""] # [doc = "Unlike `create`, no funds are reserved."] # [doc = ""] # [doc = "- `owner`: The owner of this collection of items. The owner has full superuser"] # [doc = "  permissions over this item, but may later change and configure the permissions using"] # [doc = "  `transfer_ownership` and `set_team`."] # [doc = ""] # [doc = "Emits `ForceCreated` event when successful."] # [doc = ""] # [doc = "Weight: `O(1)`"] force_create { owner : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , config : runtime_types :: pallet_nfts :: types :: CollectionConfig < :: core :: primitive :: u128 , :: core :: primitive :: u32 , :: core :: primitive :: u32 > , } , # [codec (index = 2)] # [doc = "Destroy a collection of fungible items."] # [doc = ""] # [doc = "The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the"] # [doc = "owner of the `collection`."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection to be destroyed."] # [doc = "- `witness`: Information on the items minted in the collection. This must be"] # [doc = "correct."] # [doc = ""] # [doc = "Emits `Destroyed` event when successful."] # [doc = ""] # [doc = "Weight: `O(n + m)` where:"] # [doc = "- `n = witness.items`"] # [doc = "- `m = witness.item_metadatas`"] # [doc = "- `a = witness.attributes`"] destroy { collection : :: core :: primitive :: u32 , witness : runtime_types :: pallet_nfts :: types :: DestroyWitness , } , # [codec (index = 3)] # [doc = "Mint an item of a particular collection."] # [doc = ""] # [doc = "The origin must be Signed and the sender must be the Issuer of the `collection`."] # [doc = ""] # [doc = "- `collection`: The collection of the item to be minted."] # [doc = "- `item`: An identifier of the new item."] # [doc = "- `mint_to`: Account into which the item will be minted."] # [doc = "- `witness_data`: When the mint type is `HolderOf(collection_id)`, then the owned"] # [doc = "  item_id from that collection needs to be provided within the witness data object."] # [doc = ""] # [doc = "Note: the deposit will be taken from the `origin` and not the `owner` of the `item`."] # [doc = ""] # [doc = "Emits `Issued` event when successful."] # [doc = ""] # [doc = "Weight: `O(1)`"] mint { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , mint_to : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , witness_data : :: core :: option :: Option < runtime_types :: pallet_nfts :: types :: MintWitness < :: core :: primitive :: u32 > > , } , # [codec (index = 4)] # [doc = "Mint an item of a particular collection from a privileged origin."] # [doc = ""] # [doc = "The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the"] # [doc = "Issuer of the `collection`."] # [doc = ""] # [doc = "- `collection`: The collection of the item to be minted."] # [doc = "- `item`: An identifier of the new item."] # [doc = "- `mint_to`: Account into which the item will be minted."] # [doc = "- `item_config`: A config of the new item."] # [doc = ""] # [doc = "Emits `Issued` event when successful."] # [doc = ""] # [doc = "Weight: `O(1)`"] force_mint { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , mint_to : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , item_config : runtime_types :: pallet_nfts :: types :: ItemConfig , } , # [codec (index = 5)] # [doc = "Destroy a single item."] # [doc = ""] # [doc = "Origin must be Signed and the signing account must be either:"] # [doc = "- the Admin of the `collection`;"] # [doc = "- the Owner of the `item`;"] # [doc = ""] # [doc = "- `collection`: The collection of the item to be burned."] # [doc = "- `item`: The item to be burned."] # [doc = "- `check_owner`: If `Some` then the operation will fail with `WrongOwner` unless the"] # [doc = "  item is owned by this value."] # [doc = ""] # [doc = "Emits `Burned` with the actual amount burned."] # [doc = ""] # [doc = "Weight: `O(1)`"] # [doc = "Modes: `check_owner.is_some()`."] burn { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , check_owner : :: core :: option :: Option < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 6)] # [doc = "Move an item from the sender account to another."] # [doc = ""] # [doc = "Origin must be Signed and the signing account must be either:"] # [doc = "- the Admin of the `collection`;"] # [doc = "- the Owner of the `item`;"] # [doc = "- the approved delegate for the `item` (in this case, the approval is reset)."] # [doc = ""] # [doc = "Arguments:"] # [doc = "- `collection`: The collection of the item to be transferred."] # [doc = "- `item`: The item to be transferred."] # [doc = "- `dest`: The account to receive ownership of the item."] # [doc = ""] # [doc = "Emits `Transferred`."] # [doc = ""] # [doc = "Weight: `O(1)`"] transfer { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , dest : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 7)] # [doc = "Re-evaluate the deposits on some items."] # [doc = ""] # [doc = "Origin must be Signed and the sender should be the Owner of the `collection`."] # [doc = ""] # [doc = "- `collection`: The collection of the items to be reevaluated."] # [doc = "- `items`: The items of the collection whose deposits will be reevaluated."] # [doc = ""] # [doc = "NOTE: This exists as a best-effort function. Any items which are unknown or"] # [doc = "in the case that the owner account does not have reservable funds to pay for a"] # [doc = "deposit increase are ignored. Generally the owner isn't going to call this on items"] # [doc = "whose existing deposit is less than the refreshed deposit as it would only cost them,"] # [doc = "so it's of little consequence."] # [doc = ""] # [doc = "It will still return an error in the case that the collection is unknown or the signer"] # [doc = "is not permitted to call it."] # [doc = ""] # [doc = "Weight: `O(items.len())`"] redeposit { collection : :: core :: primitive :: u32 , items : :: std :: vec :: Vec < :: core :: primitive :: u32 > , } , # [codec (index = 8)] # [doc = "Disallow further unprivileged transfer of an item."] # [doc = ""] # [doc = "Origin must be Signed and the sender should be the Freezer of the `collection`."] # [doc = ""] # [doc = "- `collection`: The collection of the item to be changed."] # [doc = "- `item`: The item to become non-transferable."] # [doc = ""] # [doc = "Emits `ItemTransferLocked`."] # [doc = ""] # [doc = "Weight: `O(1)`"] lock_item_transfer { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , } , # [codec (index = 9)] # [doc = "Re-allow unprivileged transfer of an item."] # [doc = ""] # [doc = "Origin must be Signed and the sender should be the Freezer of the `collection`."] # [doc = ""] # [doc = "- `collection`: The collection of the item to be changed."] # [doc = "- `item`: The item to become transferable."] # [doc = ""] # [doc = "Emits `ItemTransferUnlocked`."] # [doc = ""] # [doc = "Weight: `O(1)`"] unlock_item_transfer { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , } , # [codec (index = 10)] # [doc = "Disallows specified settings for the whole collection."] # [doc = ""] # [doc = "Origin must be Signed and the sender should be the Freezer of the `collection`."] # [doc = ""] # [doc = "- `collection`: The collection to be locked."] # [doc = "- `lock_settings`: The settings to be locked."] # [doc = ""] # [doc = "Note: it's possible to only lock(set) the setting, but not to unset it."] # [doc = "Emits `CollectionLocked`."] # [doc = ""] # [doc = "Weight: `O(1)`"] lock_collection { collection : :: core :: primitive :: u32 , lock_settings : runtime_types :: pallet_nfts :: types :: BitFlags < runtime_types :: pallet_nfts :: types :: CollectionSetting > , } , # [codec (index = 11)] # [doc = "Change the Owner of a collection."] # [doc = ""] # [doc = "Origin must be Signed and the sender should be the Owner of the `collection`."] # [doc = ""] # [doc = "- `collection`: The collection whose owner should be changed."] # [doc = "- `owner`: The new Owner of this collection. They must have called"] # [doc = "  `set_accept_ownership` with `collection` in order for this operation to succeed."] # [doc = ""] # [doc = "Emits `OwnerChanged`."] # [doc = ""] # [doc = "Weight: `O(1)`"] transfer_ownership { collection : :: core :: primitive :: u32 , owner : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 12)] # [doc = "Change the Issuer, Admin and Freezer of a collection."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"] # [doc = "`collection`."] # [doc = ""] # [doc = "- `collection`: The collection whose team should be changed."] # [doc = "- `issuer`: The new Issuer of this collection."] # [doc = "- `admin`: The new Admin of this collection."] # [doc = "- `freezer`: The new Freezer of this collection."] # [doc = ""] # [doc = "Emits `TeamChanged`."] # [doc = ""] # [doc = "Weight: `O(1)`"] set_team { collection : :: core :: primitive :: u32 , issuer : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , admin : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , freezer : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 13)] # [doc = "Change the Owner of a collection."] # [doc = ""] # [doc = "Origin must be `ForceOrigin`."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection."] # [doc = "- `owner`: The new Owner of this collection."] # [doc = ""] # [doc = "Emits `OwnerChanged`."] # [doc = ""] # [doc = "Weight: `O(1)`"] force_collection_owner { collection : :: core :: primitive :: u32 , owner : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 14)] # [doc = "Change the config of a collection."] # [doc = ""] # [doc = "Origin must be `ForceOrigin`."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection."] # [doc = "- `config`: The new config of this collection."] # [doc = ""] # [doc = "Emits `CollectionConfigChanged`."] # [doc = ""] # [doc = "Weight: `O(1)`"] force_collection_config { collection : :: core :: primitive :: u32 , config : runtime_types :: pallet_nfts :: types :: CollectionConfig < :: core :: primitive :: u128 , :: core :: primitive :: u32 , :: core :: primitive :: u32 > , } , # [codec (index = 15)] # [doc = "Approve an item to be transferred by a delegated third-party account."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"] # [doc = "`item`."] # [doc = ""] # [doc = "- `collection`: The collection of the item to be approved for delegated transfer."] # [doc = "- `item`: The item to be approved for delegated transfer."] # [doc = "- `delegate`: The account to delegate permission to transfer the item."] # [doc = "- `maybe_deadline`: Optional deadline for the approval. Specified by providing the"] # [doc = "\tnumber of blocks after which the approval will expire"] # [doc = ""] # [doc = "Emits `TransferApproved` on success."] # [doc = ""] # [doc = "Weight: `O(1)`"] approve_transfer { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , delegate : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , maybe_deadline : :: core :: option :: Option < :: core :: primitive :: u32 > , } , # [codec (index = 16)] # [doc = "Cancel one of the transfer approvals for a specific item."] # [doc = ""] # [doc = "Origin must be either:"] # [doc = "- the `Force` origin;"] # [doc = "- `Signed` with the signer being the Admin of the `collection`;"] # [doc = "- `Signed` with the signer being the Owner of the `item`;"] # [doc = ""] # [doc = "Arguments:"] # [doc = "- `collection`: The collection of the item of whose approval will be cancelled."] # [doc = "- `item`: The item of the collection of whose approval will be cancelled."] # [doc = "- `delegate`: The account that is going to loose their approval."] # [doc = ""] # [doc = "Emits `ApprovalCancelled` on success."] # [doc = ""] # [doc = "Weight: `O(1)`"] cancel_approval { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , delegate : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 17)] # [doc = "Cancel all the approvals of a specific item."] # [doc = ""] # [doc = "Origin must be either:"] # [doc = "- the `Force` origin;"] # [doc = "- `Signed` with the signer being the Admin of the `collection`;"] # [doc = "- `Signed` with the signer being the Owner of the `item`;"] # [doc = ""] # [doc = "Arguments:"] # [doc = "- `collection`: The collection of the item of whose approvals will be cleared."] # [doc = "- `item`: The item of the collection of whose approvals will be cleared."] # [doc = ""] # [doc = "Emits `AllApprovalsCancelled` on success."] # [doc = ""] # [doc = "Weight: `O(1)`"] clear_all_transfer_approvals { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , } , # [codec (index = 18)] # [doc = "Disallows changing the metadata or attributes of the item."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"] # [doc = "`collection`."] # [doc = ""] # [doc = "- `collection`: The collection if the `item`."] # [doc = "- `item`: An item to be locked."] # [doc = "- `lock_metadata`: Specifies whether the metadata should be locked."] # [doc = "- `lock_attributes`: Specifies whether the attributes in the `CollectionOwner` namespace"] # [doc = "  should be locked."] # [doc = ""] # [doc = "Note: `lock_attributes` affects the attributes in the `CollectionOwner` namespace"] # [doc = "only. When the metadata or attributes are locked, it won't be possible the unlock them."] # [doc = ""] # [doc = "Emits `ItemPropertiesLocked`."] # [doc = ""] # [doc = "Weight: `O(1)`"] lock_item_properties { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , lock_metadata : :: core :: primitive :: bool , lock_attributes : :: core :: primitive :: bool , } , # [codec (index = 19)] # [doc = "Set an attribute for a collection or item."] # [doc = ""] # [doc = "Origin must be Signed and must conform to the namespace ruleset:"] # [doc = "- `CollectionOwner` namespace could be modified by the `collection` owner only;"] # [doc = "- `ItemOwner` namespace could be modified by the `maybe_item` owner only. `maybe_item`"] # [doc = "  should be set in that case;"] # [doc = "- `Account(AccountId)` namespace could be modified only when the `origin` was given a"] # [doc = "  permission to do so;"] # [doc = ""] # [doc = "The funds of `origin` are reserved according to the formula:"] # [doc = "`AttributeDepositBase + DepositPerByte * (key.len + value.len)` taking into"] # [doc = "account any already reserved funds."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection whose item's metadata to set."] # [doc = "- `maybe_item`: The identifier of the item whose metadata to set."] # [doc = "- `namespace`: Attribute's namespace."] # [doc = "- `key`: The key of the attribute."] # [doc = "- `value`: The value to which to set the attribute."] # [doc = ""] # [doc = "Emits `AttributeSet`."] # [doc = ""] # [doc = "Weight: `O(1)`"] set_attribute { collection : :: core :: primitive :: u32 , maybe_item : :: core :: option :: Option < :: core :: primitive :: u32 > , namespace : runtime_types :: frame_support :: traits :: tokens :: misc :: AttributeNamespace < :: subxt :: utils :: AccountId32 > , key : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , value : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , } , # [codec (index = 20)] # [doc = "Force-set an attribute for a collection or item."] # [doc = ""] # [doc = "Origin must be `ForceOrigin`."] # [doc = ""] # [doc = "If the attribute already exists and it was set by another account, the deposit"] # [doc = "will be returned to the previous owner."] # [doc = ""] # [doc = "- `set_as`: An optional owner of the attribute."] # [doc = "- `collection`: The identifier of the collection whose item's metadata to set."] # [doc = "- `maybe_item`: The identifier of the item whose metadata to set."] # [doc = "- `namespace`: Attribute's namespace."] # [doc = "- `key`: The key of the attribute."] # [doc = "- `value`: The value to which to set the attribute."] # [doc = ""] # [doc = "Emits `AttributeSet`."] # [doc = ""] # [doc = "Weight: `O(1)`"] force_set_attribute { set_as : :: core :: option :: Option < :: subxt :: utils :: AccountId32 > , collection : :: core :: primitive :: u32 , maybe_item : :: core :: option :: Option < :: core :: primitive :: u32 > , namespace : runtime_types :: frame_support :: traits :: tokens :: misc :: AttributeNamespace < :: subxt :: utils :: AccountId32 > , key : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , value : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , } , # [codec (index = 21)] # [doc = "Clear an attribute for a collection or item."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"] # [doc = "attribute."] # [doc = ""] # [doc = "Any deposit is freed for the collection's owner."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection whose item's metadata to clear."] # [doc = "- `maybe_item`: The identifier of the item whose metadata to clear."] # [doc = "- `namespace`: Attribute's namespace."] # [doc = "- `key`: The key of the attribute."] # [doc = ""] # [doc = "Emits `AttributeCleared`."] # [doc = ""] # [doc = "Weight: `O(1)`"] clear_attribute { collection : :: core :: primitive :: u32 , maybe_item : :: core :: option :: Option < :: core :: primitive :: u32 > , namespace : runtime_types :: frame_support :: traits :: tokens :: misc :: AttributeNamespace < :: subxt :: utils :: AccountId32 > , key : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , } , # [codec (index = 22)] # [doc = "Approve item's attributes to be changed by a delegated third-party account."] # [doc = ""] # [doc = "Origin must be Signed and must be an owner of the `item`."] # [doc = ""] # [doc = "- `collection`: A collection of the item."] # [doc = "- `item`: The item that holds attributes."] # [doc = "- `delegate`: The account to delegate permission to change attributes of the item."] # [doc = ""] # [doc = "Emits `ItemAttributesApprovalAdded` on success."] approve_item_attributes { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , delegate : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 23)] # [doc = "Cancel the previously provided approval to change item's attributes."] # [doc = "All the previously set attributes by the `delegate` will be removed."] # [doc = ""] # [doc = "Origin must be Signed and must be an owner of the `item`."] # [doc = ""] # [doc = "- `collection`: Collection that the item is contained within."] # [doc = "- `item`: The item that holds attributes."] # [doc = "- `delegate`: The previously approved account to remove."] # [doc = ""] # [doc = "Emits `ItemAttributesApprovalRemoved` on success."] cancel_item_attributes_approval { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , delegate : :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , witness : runtime_types :: pallet_nfts :: types :: CancelAttributesApprovalWitness , } , # [codec (index = 24)] # [doc = "Set the metadata for an item."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"] # [doc = "`collection`."] # [doc = ""] # [doc = "If the origin is Signed, then funds of signer are reserved according to the formula:"] # [doc = "`MetadataDepositBase + DepositPerByte * data.len` taking into"] # [doc = "account any already reserved funds."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection whose item's metadata to set."] # [doc = "- `item`: The identifier of the item whose metadata to set."] # [doc = "- `data`: The general information of this item. Limited in length by `StringLimit`."] # [doc = ""] # [doc = "Emits `ItemMetadataSet`."] # [doc = ""] # [doc = "Weight: `O(1)`"] set_metadata { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , data : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , } , # [codec (index = 25)] # [doc = "Clear the metadata for an item."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"] # [doc = "`collection`."] # [doc = ""] # [doc = "Any deposit is freed for the collection's owner."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection whose item's metadata to clear."] # [doc = "- `item`: The identifier of the item whose metadata to clear."] # [doc = ""] # [doc = "Emits `ItemMetadataCleared`."] # [doc = ""] # [doc = "Weight: `O(1)`"] clear_metadata { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , } , # [codec (index = 26)] # [doc = "Set the metadata for a collection."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"] # [doc = "the `collection`."] # [doc = ""] # [doc = "If the origin is `Signed`, then funds of signer are reserved according to the formula:"] # [doc = "`MetadataDepositBase + DepositPerByte * data.len` taking into"] # [doc = "account any already reserved funds."] # [doc = ""] # [doc = "- `collection`: The identifier of the item whose metadata to update."] # [doc = "- `data`: The general information of this item. Limited in length by `StringLimit`."] # [doc = ""] # [doc = "Emits `CollectionMetadataSet`."] # [doc = ""] # [doc = "Weight: `O(1)`"] set_collection_metadata { collection : :: core :: primitive :: u32 , data : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > , } , # [codec (index = 27)] # [doc = "Clear the metadata for a collection."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"] # [doc = "the `collection`."] # [doc = ""] # [doc = "Any deposit is freed for the collection's owner."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection whose metadata to clear."] # [doc = ""] # [doc = "Emits `CollectionMetadataCleared`."] # [doc = ""] # [doc = "Weight: `O(1)`"] clear_collection_metadata { collection : :: core :: primitive :: u32 , } , # [codec (index = 28)] # [doc = "Set (or reset) the acceptance of ownership for a particular account."] # [doc = ""] # [doc = "Origin must be `Signed` and if `maybe_collection` is `Some`, then the signer must have a"] # [doc = "provider reference."] # [doc = ""] # [doc = "- `maybe_collection`: The identifier of the collection whose ownership the signer is"] # [doc = "  willing to accept, or if `None`, an indication that the signer is willing to accept no"] # [doc = "  ownership transferal."] # [doc = ""] # [doc = "Emits `OwnershipAcceptanceChanged`."] set_accept_ownership { maybe_collection : :: core :: option :: Option < :: core :: primitive :: u32 > , } , # [codec (index = 29)] # [doc = "Set the maximum number of items a collection could have."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"] # [doc = "the `collection`."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection to change."] # [doc = "- `max_supply`: The maximum number of items a collection could have."] # [doc = ""] # [doc = "Emits `CollectionMaxSupplySet` event when successful."] set_collection_max_supply { collection : :: core :: primitive :: u32 , max_supply : :: core :: primitive :: u32 , } , # [codec (index = 30)] # [doc = "Update mint settings."] # [doc = ""] # [doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"] # [doc = "the `collection`."] # [doc = ""] # [doc = "- `collection`: The identifier of the collection to change."] # [doc = "- `mint_settings`: The new mint settings."] # [doc = ""] # [doc = "Emits `CollectionMintSettingsUpdated` event when successful."] update_mint_settings { collection : :: core :: primitive :: u32 , mint_settings : runtime_types :: pallet_nfts :: types :: MintSettings < :: core :: primitive :: u128 , :: core :: primitive :: u32 , :: core :: primitive :: u32 > , } , # [codec (index = 31)] # [doc = "Set (or reset) the price for an item."] # [doc = ""] # [doc = "Origin must be Signed and must be the owner of the asset `item`."] # [doc = ""] # [doc = "- `collection`: The collection of the item."] # [doc = "- `item`: The item to set the price for."] # [doc = "- `price`: The price for the item. Pass `None`, to reset the price."] # [doc = "- `buyer`: Restricts the buy operation to a specific account."] # [doc = ""] # [doc = "Emits `ItemPriceSet` on success if the price is not `None`."] # [doc = "Emits `ItemPriceRemoved` on success if the price is `None`."] set_price { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , price : :: core :: option :: Option < :: core :: primitive :: u128 > , whitelisted_buyer : :: core :: option :: Option < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 32)] # [doc = "Allows to buy an item if it's up for sale."] # [doc = ""] # [doc = "Origin must be Signed and must not be the owner of the `item`."] # [doc = ""] # [doc = "- `collection`: The collection of the item."] # [doc = "- `item`: The item the sender wants to buy."] # [doc = "- `bid_price`: The price the sender is willing to pay."] # [doc = ""] # [doc = "Emits `ItemBought` on success."] buy_item { collection : :: core :: primitive :: u32 , item : :: core :: primitive :: u32 , bid_price : :: core :: primitive :: u128 , } , # [codec (index = 33)] # [doc = "Allows to pay the tips."] # [doc = ""] # [doc = "Origin must be Signed."] # [doc = ""] # [doc = "- `tips`: Tips array."] # [doc = ""] # [doc = "Emits `TipSent` on every tip transfer."] pay_tips { tips : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < runtime_types :: pallet_nfts :: types :: ItemTip < :: core :: primitive :: u32 , :: core :: primitive :: u32 , :: subxt :: utils :: AccountId32 , :: core :: primitive :: u128 > > , } , # [codec (index = 34)] # [doc = "Register a new atomic swap, declaring an intention to send an `item` in exchange for"] # [doc = "`desired_item` from origin to target on the current blockchain."] # [doc = "The target can execute the swap during the specified `duration` of blocks (if set)."] # [doc = "Additionally, the price could be set for the desired `item`."] # [doc = ""] # [doc = "Origin must be Signed and must be an owner of the `item`."] # [doc = ""] # [doc = "- `collection`: The collection of the item."] # [doc = "- `item`: The item an owner wants to give."] # [doc = "- `desired_collection`: The collection of the desired item."] # [doc = "- `desired_item`: The desired item an owner wants to receive."] # [doc = "- `maybe_price`: The price an owner is willing to pay or receive for the desired `item`."] # [doc = "- `duration`: A deadline for the swap. Specified by providing the number of blocks"] # [doc = "\tafter which the swap will expire."] # [doc = ""] # [doc = "Emits `SwapCreated` on success."] create_swap { offered_collection : :: core :: primitive :: u32 , offered_item : :: core :: primitive :: u32 , desired_collection : :: core :: primitive :: u32 , maybe_desired_item : :: core :: option :: Option < :: core :: primitive :: u32 > , maybe_price : :: core :: option :: Option < runtime_types :: pallet_nfts :: types :: PriceWithDirection < :: core :: primitive :: u128 > > , duration : :: core :: primitive :: u32 , } , # [codec (index = 35)] # [doc = "Cancel an atomic swap."] # [doc = ""] # [doc = "Origin must be Signed."] # [doc = "Origin must be an owner of the `item` if the deadline hasn't expired."] # [doc = ""] # [doc = "- `collection`: The collection of the item."] # [doc = "- `item`: The item an owner wants to give."] # [doc = ""] # [doc = "Emits `SwapCancelled` on success."] cancel_swap { offered_collection : :: core :: primitive :: u32 , offered_item : :: core :: primitive :: u32 , } , # [codec (index = 36)] # [doc = "Claim an atomic swap."] # [doc = "This method executes a pending swap, that was created by a counterpart before."] # [doc = ""] # [doc = "Origin must be Signed and must be an owner of the `item`."] # [doc = ""] # [doc = "- `send_collection`: The collection of the item to be sent."] # [doc = "- `send_item`: The item to be sent."] # [doc = "- `receive_collection`: The collection of the item to be received."] # [doc = "- `receive_item`: The item to be received."] # [doc = "- `witness_price`: A price that was previously agreed on."] # [doc = ""] # [doc = "Emits `SwapClaimed` on success."] claim_swap { send_collection : :: core :: primitive :: u32 , send_item : :: core :: primitive :: u32 , receive_collection : :: core :: primitive :: u32 , receive_item : :: core :: primitive :: u32 , witness_price : :: core :: option :: Option < runtime_types :: pallet_nfts :: types :: PriceWithDirection < :: core :: primitive :: u128 > > , } , # [codec (index = 37)] # [doc = "Mint an item by providing the pre-signed approval."] # [doc = ""] # [doc = "Origin must be Signed."] # [doc = ""] # [doc = "- `mint_data`: The pre-signed approval that consists of the information about the item,"] # [doc = "  its metadata, attributes, who can mint it (`None` for anyone) and until what block"] # [doc = "  number."] # [doc = "- `signature`: The signature of the `data` object."] # [doc = "- `signer`: The `data` object's signer. Should be an owner of the collection."] # [doc = ""] # [doc = "Emits `Issued` on success."] # [doc = "Emits `AttributeSet` if the attributes were provided."] # [doc = "Emits `ItemMetadataSet` if the metadata was not empty."] mint_pre_signed { mint_data : runtime_types :: pallet_nfts :: types :: PreSignedMint < :: core :: primitive :: u32 , :: core :: primitive :: u32 , :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , signature : runtime_types :: sp_runtime :: MultiSignature , signer : :: subxt :: utils :: AccountId32 , } , # [codec (index = 38)] # [doc = "Set attributes for an item by providing the pre-signed approval."] # [doc = ""] # [doc = "Origin must be Signed and must be an owner of the `data.item`."] # [doc = ""] # [doc = "- `data`: The pre-signed approval that consists of the information about the item,"] # [doc = "  attributes to update and until what block number."] # [doc = "- `signature`: The signature of the `data` object."] # [doc = "- `signer`: The `data` object's signer. Should be an owner of the collection for the"] # [doc = "  `CollectionOwner` namespace."] # [doc = ""] # [doc = "Emits `AttributeSet` for each provided attribute."] # [doc = "Emits `ItemAttributesApprovalAdded` if the approval wasn't set before."] # [doc = "Emits `PreSignedAttributesSet` on success."] set_attributes_pre_signed { data : runtime_types :: pallet_nfts :: types :: PreSignedAttributes < :: core :: primitive :: u32 , :: core :: primitive :: u32 , :: subxt :: utils :: AccountId32 , :: core :: primitive :: u32 > , signature : runtime_types :: sp_runtime :: MultiSignature , signer : :: subxt :: utils :: AccountId32 , } , }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct BitFlags<_0>(
                    pub ::core::primitive::u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct CancelAttributesApprovalWitness {
                    pub account_attributes: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct CollectionConfig<_0, _1, _2> {
                    pub settings: runtime_types::pallet_nfts::types::BitFlags<
                        runtime_types::pallet_nfts::types::CollectionSetting,
                    >,
                    pub max_supply: ::core::option::Option<_1>,
                    pub mint_settings:
                        runtime_types::pallet_nfts::types::MintSettings<_0, _1, _1>,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum CollectionSetting {
                    #[codec(index = 1)]
                    TransferableItems,
                    #[codec(index = 2)]
                    UnlockedMetadata,
                    #[codec(index = 4)]
                    UnlockedAttributes,
                    #[codec(index = 8)]
                    UnlockedMaxSupply,
                    #[codec(index = 16)]
                    DepositRequired,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub items: ::core::primitive::u32,
                    #[codec(compact)]
                    pub item_metadatas: ::core::primitive::u32,
                    #[codec(compact)]
                    pub attributes: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct ItemConfig {
                    pub settings: runtime_types::pallet_nfts::types::BitFlags<
                        runtime_types::pallet_nfts::types::ItemSetting,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum ItemSetting {
                    #[codec(index = 1)]
                    Transferable,
                    #[codec(index = 2)]
                    UnlockedMetadata,
                    #[codec(index = 4)]
                    UnlockedAttributes,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct ItemTip<_0, _1, _2, _3> {
                    pub collection: _0,
                    pub item: _0,
                    pub receiver: _2,
                    pub amount: _3,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct MintSettings<_0, _1, _2> {
                    pub mint_type: runtime_types::pallet_nfts::types::MintType<_1>,
                    pub price: ::core::option::Option<_0>,
                    pub start_block: ::core::option::Option<_1>,
                    pub end_block: ::core::option::Option<_1>,
                    pub default_item_settings:
                        runtime_types::pallet_nfts::types::BitFlags<
                            runtime_types::pallet_nfts::types::ItemSetting,
                        >,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum MintType<_0> {
                    #[codec(index = 0)]
                    Issuer,
                    #[codec(index = 1)]
                    Public,
                    #[codec(index = 2)]
                    HolderOf(_0),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct MintWitness<_0> {
                    pub owner_of_item: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct PreSignedAttributes < _0 , _1 , _2 , _3 > { pub collection : _0 , pub item : _0 , pub attributes : :: std :: vec :: Vec < (:: std :: vec :: Vec < :: core :: primitive :: u8 > , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) > , pub namespace : runtime_types :: frame_support :: traits :: tokens :: misc :: AttributeNamespace < _2 > , pub deadline : _0 , # [codec (skip)] pub __subxt_unused_type_params : :: core :: marker :: PhantomData < (_3 , _1) > }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct PreSignedMint<_0, _1, _2, _3> {
                    pub collection: _0,
                    pub item: _0,
                    pub attributes: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    pub metadata: ::std::vec::Vec<::core::primitive::u8>,
                    pub only_account: ::core::option::Option<_2>,
                    pub deadline: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<(_3, _1)>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum PriceDirection {
                    #[codec(index = 0)]
                    Send,
                    #[codec(index = 1)]
                    Receive,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct PriceWithDirection<_0> {
                    pub amount: _0,
                    pub direction: runtime_types::pallet_nfts::types::PriceDirection,
                }
            }
        }
        pub mod pallet_nis {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Place a bid."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed, and account must have at least `amount` in free balance."]
                    #[doc = ""]
                    #[doc = "- `amount`: The amount of the bid; these funds will be reserved, and if/when"]
                    #[doc = "  consolidated, removed. Must be at least `MinBid`."]
                    #[doc = "- `duration`: The number of periods before which the newly consolidated bid may be"]
                    #[doc = "  thawed. Must be greater than 1 and no more than `QueueCount`."]
                    #[doc = ""]
                    #[doc = "Complexities:"]
                    #[doc = "- `Queues[duration].len()` (just take max)."]
                    place_bid {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        duration: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Retract a previously placed bid."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed, and the account should have previously issued a still-active bid"]
                    #[doc = "of `amount` for `duration`."]
                    #[doc = ""]
                    #[doc = "- `amount`: The amount of the previous bid."]
                    #[doc = "- `duration`: The duration of the previous bid."]
                    retract_bid {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        duration: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Ensure we have sufficient funding for all potential payouts."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be accepted by `FundOrigin`."]
                    fund_deficit,
                    #[codec(index = 3)]
                    #[doc = "Reduce or remove an outstanding receipt, placing the according proportion of funds into"]
                    #[doc = "the account of the owner."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Signed and the account must be the owner of the receipt `index` as"]
                    #[doc = "  well as any fungible counterpart."]
                    #[doc = "- `index`: The index of the receipt."]
                    #[doc = "- `portion`: If `Some`, then only the given portion of the receipt should be thawed. If"]
                    #[doc = "  `None`, then all of it should be."]
                    thaw_private {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        maybe_proportion: ::core::option::Option<
                            runtime_types::sp_arithmetic::per_things::Perquintill,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Reduce or remove an outstanding receipt, placing the according proportion of funds into"]
                    #[doc = "the account of the owner."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Signed and the account must be the owner of the fungible counterpart"]
                    #[doc = "  for receipt `index`."]
                    #[doc = "- `index`: The index of the receipt."]
                    thaw_communal {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Make a private receipt communal and create fungible counterparts for its owner."]
                    communify {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Make a communal receipt private and burn fungible counterparts from its owner."]
                    privatize {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_nomination_pools {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Stake funds with a pool. The amount to bond is transferred from the member to the"]
                    #[doc = "pools account and immediately increases the pools bond."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "* An account can only be a member of a single pool."]
                    #[doc = "* An account cannot join the same pool multiple times."]
                    #[doc = "* This call will *not* dust the member account, so the member must have at least"]
                    #[doc = "  `existential deposit + amount` in their account."]
                    #[doc = "* Only a pool with [`PoolState::Open`] can be joined"]
                    join {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        pool_id: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Bond `extra` more funds from `origin` into the pool to which they already belong."]
                    #[doc = ""]
                    #[doc = "Additional funds can come from either the free balance of the account, of from the"]
                    #[doc = "accumulated rewards, see [`BondExtra`]."]
                    #[doc = ""]
                    #[doc = "Bonding extra funds implies an automatic payout of all pending rewards as well."]
                    #[doc = "See `bond_extra_other` to bond pending rewards of `other` members."]
                    bond_extra {
                        extra: runtime_types::pallet_nomination_pools::BondExtra<
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "A bonded member can use this to claim their payout based on the rewards that the pool"]
                    #[doc = "has accumulated since their last claimed payout (OR since joining if this is there first"]
                    #[doc = "time claiming rewards). The payout will be transferred to the member's account."]
                    #[doc = ""]
                    #[doc = "The member will earn rewards pro rata based on the members stake vs the sum of the"]
                    #[doc = "members in the pools stake. Rewards do not \"expire\"."]
                    #[doc = ""]
                    #[doc = "See `claim_payout_other` to caim rewards on bahalf of some `other` pool member."]
                    claim_payout,
                    #[codec(index = 3)]
                    #[doc = "Unbond up to `unbonding_points` of the `member_account`'s funds from the pool. It"]
                    #[doc = "implicitly collects the rewards one last time, since not doing so would mean some"]
                    #[doc = "rewards would be forfeited."]
                    #[doc = ""]
                    #[doc = "Under certain conditions, this call can be dispatched permissionlessly (i.e. by any"]
                    #[doc = "account)."]
                    #[doc = ""]
                    #[doc = "# Conditions for a permissionless dispatch."]
                    #[doc = ""]
                    #[doc = "* The pool is blocked and the caller is either the root or bouncer. This is refereed to"]
                    #[doc = "  as a kick."]
                    #[doc = "* The pool is destroying and the member is not the depositor."]
                    #[doc = "* The pool is destroying, the member is the depositor and no other members are in the"]
                    #[doc = "  pool."]
                    #[doc = ""]
                    #[doc = "## Conditions for permissioned dispatch (i.e. the caller is also the"]
                    #[doc = "`member_account`):"]
                    #[doc = ""]
                    #[doc = "* The caller is not the depositor."]
                    #[doc = "* The caller is the depositor, the pool is destroying and no other members are in the"]
                    #[doc = "  pool."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "If there are too many unlocking chunks to unbond with the pool account,"]
                    #[doc = "[`Call::pool_withdraw_unbonded`] can be called to try and minimize unlocking chunks."]
                    #[doc = "The [`StakingInterface::unbond`] will implicitly call [`Call::pool_withdraw_unbonded`]"]
                    #[doc = "to try to free chunks if necessary (ie. if unbound was called and no unlocking chunks"]
                    #[doc = "are available). However, it may not be possible to release the current unlocking chunks,"]
                    #[doc = "in which case, the result of this call will likely be the `NoMoreChunks` error from the"]
                    #[doc = "staking system."]
                    unbond {
                        member_account: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        unbonding_points: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Call `withdraw_unbonded` for the pools account. This call can be made by any account."]
                    #[doc = ""]
                    #[doc = "This is useful if their are too many unlocking chunks to call `unbond`, and some"]
                    #[doc = "can be cleared by withdrawing. In the case there are too many unlocking chunks, the user"]
                    #[doc = "would probably see an error like `NoMoreChunks` emitted from the staking system when"]
                    #[doc = "they attempt to unbond."]
                    pool_withdraw_unbonded {
                        pool_id: ::core::primitive::u32,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Withdraw unbonded funds from `member_account`. If no bonded funds can be unbonded, an"]
                    #[doc = "error is returned."]
                    #[doc = ""]
                    #[doc = "Under certain conditions, this call can be dispatched permissionlessly (i.e. by any"]
                    #[doc = "account)."]
                    #[doc = ""]
                    #[doc = "# Conditions for a permissionless dispatch"]
                    #[doc = ""]
                    #[doc = "* The pool is in destroy mode and the target is not the depositor."]
                    #[doc = "* The target is the depositor and they are the only member in the sub pools."]
                    #[doc = "* The pool is blocked and the caller is either the root or bouncer."]
                    #[doc = ""]
                    #[doc = "# Conditions for permissioned dispatch"]
                    #[doc = ""]
                    #[doc = "* The caller is the target and they are not the depositor."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "If the target is the depositor, the pool will be destroyed."]
                    withdraw_unbonded {
                        member_account: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Create a new delegation pool."]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = "* `amount` - The amount of funds to delegate to the pool. This also acts of a sort of"]
                    #[doc = "  deposit since the pools creator cannot fully unbond funds until the pool is being"]
                    #[doc = "  destroyed."]
                    #[doc = "* `index` - A disambiguation index for creating the account. Likely only useful when"]
                    #[doc = "  creating multiple pools in the same extrinsic."]
                    #[doc = "* `root` - The account to set as [`PoolRoles::root`]."]
                    #[doc = "* `nominator` - The account to set as the [`PoolRoles::nominator`]."]
                    #[doc = "* `bouncer` - The account to set as the [`PoolRoles::bouncer`]."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "In addition to `amount`, the caller will transfer the existential deposit; so the caller"]
                    #[doc = "needs at have at least `amount + existential_deposit` transferrable."]
                    create {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        root: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        nominator: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        bouncer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 7)]
                    #[doc = "Create a new delegation pool with a previously used pool id"]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = "same as `create` with the inclusion of"]
                    #[doc = "* `pool_id` - `A valid PoolId."]
                    create_with_pool_id {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        root: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        nominator: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        bouncer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        pool_id: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Nominate on behalf of the pool."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed by the pool nominator or the pool"]
                    #[doc = "root role."]
                    #[doc = ""]
                    #[doc = "This directly forward the call to the staking pallet, on behalf of the pool bonded"]
                    #[doc = "account."]
                    nominate {
                        pool_id: ::core::primitive::u32,
                        validators: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 9)]
                    #[doc = "Set a new state for the pool."]
                    #[doc = ""]
                    #[doc = "If a pool is already in the `Destroying` state, then under no condition can its state"]
                    #[doc = "change again."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be either:"]
                    #[doc = ""]
                    #[doc = "1. signed by the bouncer, or the root role of the pool,"]
                    #[doc = "2. if the pool conditions to be open are NOT met (as described by `ok_to_be_open`), and"]
                    #[doc = "   then the state of the pool can be permissionlessly changed to `Destroying`."]
                    set_state {
                        pool_id: ::core::primitive::u32,
                        state: runtime_types::pallet_nomination_pools::PoolState,
                    },
                    #[codec(index = 10)]
                    #[doc = "Set a new metadata for the pool."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed by the bouncer, or the root role"]
                    #[doc = "of the pool."]
                    set_metadata {
                        pool_id: ::core::primitive::u32,
                        metadata: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 11)]
                    #[doc = "Update configurations for the nomination pools. The origin for this call must be"]
                    #[doc = "Root."]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = "* `min_join_bond` - Set [`MinJoinBond`]."]
                    #[doc = "* `min_create_bond` - Set [`MinCreateBond`]."]
                    #[doc = "* `max_pools` - Set [`MaxPools`]."]
                    #[doc = "* `max_members` - Set [`MaxPoolMembers`]."]
                    #[doc = "* `max_members_per_pool` - Set [`MaxPoolMembersPerPool`]."]
                    set_configs {
                        min_join_bond: runtime_types::pallet_nomination_pools::ConfigOp<
                            ::core::primitive::u128,
                        >,
                        min_create_bond: runtime_types::pallet_nomination_pools::ConfigOp<
                            ::core::primitive::u128,
                        >,
                        max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
                            ::core::primitive::u32,
                        >,
                        max_members: runtime_types::pallet_nomination_pools::ConfigOp<
                            ::core::primitive::u32,
                        >,
                        max_members_per_pool:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::core::primitive::u32,
                            >,
                    },
                    #[codec(index = 12)]
                    #[doc = "Update the roles of the pool."]
                    #[doc = ""]
                    #[doc = "The root is the only entity that can change any of the roles, including itself,"]
                    #[doc = "excluding the depositor, who can never change."]
                    #[doc = ""]
                    #[doc = "It emits an event, notifying UIs of the role change. This event is quite relevant to"]
                    #[doc = "most pool members and they should be informed of changes to pool roles."]
                    update_roles {
                        pool_id: ::core::primitive::u32,
                        new_root: runtime_types::pallet_nomination_pools::ConfigOp<
                            ::subxt::utils::AccountId32,
                        >,
                        new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
                            ::subxt::utils::AccountId32,
                        >,
                        new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
                            ::subxt::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 13)]
                    #[doc = "Chill on behalf of the pool."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed by the pool nominator or the pool"]
                    #[doc = "root role, same as [`Pallet::nominate`]."]
                    #[doc = ""]
                    #[doc = "This directly forward the call to the staking pallet, on behalf of the pool bonded"]
                    #[doc = "account."]
                    chill { pool_id: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    #[doc = "`origin` bonds funds from `extra` for some pool member `member` into their respective"]
                    #[doc = "pools."]
                    #[doc = ""]
                    #[doc = "`origin` can bond extra funds from free balance or pending rewards when `origin =="]
                    #[doc = "other`."]
                    #[doc = ""]
                    #[doc = "In the case of `origin != other`, `origin` can only bond extra pending rewards of"]
                    #[doc = "`other` members assuming set_claim_permission for the given member is"]
                    #[doc = "`PermissionlessAll` or `PermissionlessCompound`."]
                    bond_extra_other {
                        member: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        extra: runtime_types::pallet_nomination_pools::BondExtra<
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 15)]
                    #[doc = "Allows a pool member to set a claim permission to allow or disallow permissionless"]
                    #[doc = "bonding and withdrawing."]
                    #[doc = ""]
                    #[doc = "By default, this is `Permissioned`, which implies only the pool member themselves can"]
                    #[doc = "claim their pending rewards. If a pool member wishes so, they can set this to"]
                    #[doc = "`PermissionlessAll` to allow any account to claim their rewards and bond extra to the"]
                    #[doc = "pool."]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = "* `origin` - Member of a pool."]
                    #[doc = "* `actor` - Account to claim reward. // improve this"]
                    set_claim_permission {
                        permission:
                            runtime_types::pallet_nomination_pools::ClaimPermission,
                    },
                    #[codec(index = 16)]
                    #[doc = "`origin` can claim payouts on some pool member `other`'s behalf."]
                    #[doc = ""]
                    #[doc = "Pool member `other` must have a `PermissionlessAll` or `PermissionlessWithdraw` in order"]
                    #[doc = "for this call to be successful."]
                    claim_payout_other { other: ::subxt::utils::AccountId32 },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum BondExtra<_0> {
                #[codec(index = 0)]
                FreeBalance(_0),
                #[codec(index = 1)]
                Rewards,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum ClaimPermission {
                #[codec(index = 0)]
                Permissioned,
                #[codec(index = 1)]
                PermissionlessCompound,
                #[codec(index = 2)]
                PermissionlessWithdraw,
                #[codec(index = 3)]
                PermissionlessAll,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                Debug,
            )]
            pub enum PoolState {
                #[codec(index = 0)]
                Open,
                #[codec(index = 1)]
                Blocked,
                #[codec(index = 2)]
                Destroying,
            }
        }
        pub mod pallet_preimage {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Register a preimage on-chain."]
                    #[doc = ""]
                    #[doc = "If the preimage was previously requested, no fees or deposits are taken for providing"]
                    #[doc = "the preimage. Otherwise, a deposit is taken proportional to the size of the preimage."]
                    note_preimage {
                        bytes: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Clear an unrequested preimage from the runtime storage."]
                    #[doc = ""]
                    #[doc = "If `len` is provided, then it will be a much cheaper operation."]
                    #[doc = ""]
                    #[doc = "- `hash`: The hash of the preimage to be removed from the store."]
                    #[doc = "- `len`: The length of the preimage of `hash`."]
                    unnote_preimage { hash: ::subxt::utils::H256 },
                    #[codec(index = 2)]
                    #[doc = "Request a preimage be uploaded to the chain without paying any fees or deposits."]
                    #[doc = ""]
                    #[doc = "If the preimage requests has already been provided on-chain, we unreserve any deposit"]
                    #[doc = "a user may have paid, and take the control of the preimage out of their hands."]
                    request_preimage { hash: ::subxt::utils::H256 },
                    #[codec(index = 3)]
                    #[doc = "Clear a previously made request for a preimage."]
                    #[doc = ""]
                    #[doc = "NOTE: THIS MUST NOT BE CALLED ON `hash` MORE TIMES THAN `request_preimage`."]
                    unrequest_preimage { hash: ::subxt::utils::H256 },
                }
            }
        }
        pub mod pallet_proxy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Dispatch the given `call` from an account that the sender is authorised for through"]
                    #[doc = "`add_proxy`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                    #[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
                    #[doc = "- `call`: The call to be made by the `real` account."]
                    proxy {
                        real: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        force_proxy_type: ::core::option::Option<
                            runtime_types::kitchensink_runtime::ProxyType,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Register a proxy account for the sender that is able to make calls on its behalf."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `proxy`: The account that the `caller` would like to make a proxy."]
                    #[doc = "- `proxy_type`: The permissions allowed for this proxy account."]
                    #[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
                    #[doc = "zero."]
                    add_proxy {
                        delegate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        proxy_type: runtime_types::kitchensink_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Unregister a proxy account for the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `proxy`: The account that the `caller` would like to remove as a proxy."]
                    #[doc = "- `proxy_type`: The permissions currently enabled for the removed proxy account."]
                    remove_proxy {
                        delegate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        proxy_type: runtime_types::kitchensink_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Unregister all proxy accounts for the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "WARNING: This may be called on accounts created by `pure`, however if done, then"]
                    #[doc = "the unreserved fees will be inaccessible. **All access to this account will be lost.**"]
                    remove_proxies,
                    #[codec(index = 4)]
                    #[doc = "Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and"]
                    #[doc = "initialize it with a proxy of `proxy_type` for `origin` sender."]
                    #[doc = ""]
                    #[doc = "Requires a `Signed` origin."]
                    #[doc = ""]
                    #[doc = "- `proxy_type`: The type of the proxy that the sender will be registered as over the"]
                    #[doc = "new account. This will almost always be the most permissive `ProxyType` possible to"]
                    #[doc = "allow for maximum flexibility."]
                    #[doc = "- `index`: A disambiguation index, in case this is called multiple times in the same"]
                    #[doc = "transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just"]
                    #[doc = "want to use `0`."]
                    #[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
                    #[doc = "zero."]
                    #[doc = ""]
                    #[doc = "Fails with `Duplicate` if this has already been called in this transaction, from the"]
                    #[doc = "same sender, with the same parameters."]
                    #[doc = ""]
                    #[doc = "Fails if there are insufficient funds to pay for deposit."]
                    create_pure {
                        proxy_type: runtime_types::kitchensink_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                        index: ::core::primitive::u16,
                    },
                    #[codec(index = 5)]
                    #[doc = "Removes a previously spawned pure proxy."]
                    #[doc = ""]
                    #[doc = "WARNING: **All access to this account will be lost.** Any funds held in it will be"]
                    #[doc = "inaccessible."]
                    #[doc = ""]
                    #[doc = "Requires a `Signed` origin, and the sender account must have been created by a call to"]
                    #[doc = "`pure` with corresponding parameters."]
                    #[doc = ""]
                    #[doc = "- `spawner`: The account that originally called `pure` to create this account."]
                    #[doc = "- `index`: The disambiguation index originally passed to `pure`. Probably `0`."]
                    #[doc = "- `proxy_type`: The proxy type originally passed to `pure`."]
                    #[doc = "- `height`: The height of the chain when the call to `pure` was processed."]
                    #[doc = "- `ext_index`: The extrinsic index in which the call to `pure` was processed."]
                    #[doc = ""]
                    #[doc = "Fails with `NoPermission` in case the caller is not a previously created pure"]
                    #[doc = "account whose `pure` call has corresponding parameters."]
                    kill_pure {
                        spawner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        proxy_type: runtime_types::kitchensink_runtime::ProxyType,
                        index: ::core::primitive::u16,
                        #[codec(compact)]
                        height: ::core::primitive::u32,
                        #[codec(compact)]
                        ext_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Publish the hash of a proxy-call that will be made in the future."]
                    #[doc = ""]
                    #[doc = "This must be called some number of blocks before the corresponding `proxy` is attempted"]
                    #[doc = "if the delay associated with the proxy relationship is greater than zero."]
                    #[doc = ""]
                    #[doc = "No more than `MaxPending` announcements may be made at any one time."]
                    #[doc = ""]
                    #[doc = "This will take a deposit of `AnnouncementDepositFactor` as well as"]
                    #[doc = "`AnnouncementDepositBase` if there are no other pending announcements."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and a proxy of `real`."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                    #[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
                    announce {
                        real: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 7)]
                    #[doc = "Remove a given announcement."]
                    #[doc = ""]
                    #[doc = "May be called by a proxy account to remove a call they previously announced and return"]
                    #[doc = "the deposit."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                    #[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
                    remove_announcement {
                        real: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 8)]
                    #[doc = "Remove the given announcement of a delegate."]
                    #[doc = ""]
                    #[doc = "May be called by a target (proxied) account to remove a call that one of their delegates"]
                    #[doc = "(`delegate`) has announced they want to execute. The deposit is returned."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `delegate`: The account that previously announced the call."]
                    #[doc = "- `call_hash`: The hash of the call to be made."]
                    reject_announcement {
                        delegate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 9)]
                    #[doc = "Dispatch the given `call` from an account that the sender is authorized for through"]
                    #[doc = "`add_proxy`."]
                    #[doc = ""]
                    #[doc = "Removes any corresponding announcement(s)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                    #[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
                    #[doc = "- `call`: The call to be made by the `real` account."]
                    proxy_announced {
                        delegate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        real: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        force_proxy_type: ::core::option::Option<
                            runtime_types::kitchensink_runtime::ProxyType,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                }
            }
        }
        pub mod pallet_ranked_collective {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Introduce a new member."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be the `AdminOrigin`."]
                    #[doc = "- `who`: Account of non-member which will become a member."]
                    #[doc = "- `rank`: The rank to give the new member."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    add_member {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Increment the rank of an existing member by one."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be the `AdminOrigin`."]
                    #[doc = "- `who`: Account of existing member."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    promote_member {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Decrement the rank of an existing member by one. If the member is already at rank zero,"]
                    #[doc = "then they are removed entirely."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be the `AdminOrigin`."]
                    #[doc = "- `who`: Account of existing member of rank greater than zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`, less if the member's index is highest in its rank."]
                    demote_member {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Remove the member entirely."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be the `AdminOrigin`."]
                    #[doc = "- `who`: Account of existing member of rank greater than zero."]
                    #[doc = "- `min_rank`: The rank of the member or greater."]
                    #[doc = ""]
                    #[doc = "Weight: `O(min_rank)`."]
                    remove_member {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        min_rank: ::core::primitive::u16,
                    },
                    #[codec(index = 4)]
                    #[doc = "Add an aye or nay vote for the sender to the given proposal."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed` by a member account."]
                    #[doc = "- `poll`: Index of a poll which is ongoing."]
                    #[doc = "- `aye`: `true` if the vote is to approve the proposal, `false` otherwise."]
                    #[doc = ""]
                    #[doc = "Transaction fees are be waived if the member is voting on any particular proposal"]
                    #[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
                    #[doc = "fee."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`, less if there was no previous vote on the poll by the member."]
                    vote {
                        poll: ::core::primitive::u32,
                        aye: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Remove votes from the given poll. It must have ended."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed` by any account."]
                    #[doc = "- `poll_index`: Index of a poll which is completed and for which votes continue to"]
                    #[doc = "  exist."]
                    #[doc = "- `max`: Maximum number of vote items from remove in this call."]
                    #[doc = ""]
                    #[doc = "Transaction fees are waived if the operation is successful."]
                    #[doc = ""]
                    #[doc = "Weight `O(max)` (less if there are fewer items to remove than `max`)."]
                    cleanup_poll {
                        poll_index: ::core::primitive::u32,
                        max: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_recovery {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Send a call through a recovered account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and registered to"]
                    #[doc = "be able to make calls on behalf of the recovered account."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `account`: The recovered account you want to make a call on-behalf-of."]
                    #[doc = "- `call`: The call you want to make with the recovered account."]
                    as_recovered {
                        account: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Allow ROOT to bypass the recovery process and set an a rescuer account"]
                    #[doc = "for a lost account directly."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _ROOT_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `lost`: The \"lost account\" to be recovered."]
                    #[doc = "- `rescuer`: The \"rescuer account\" which can call as the lost account."]
                    set_recovered {
                        lost: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        rescuer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Create a recovery configuration for your account. This makes your account recoverable."]
                    #[doc = ""]
                    #[doc = "Payment: `ConfigDepositBase` + `FriendDepositFactor` * #_of_friends balance"]
                    #[doc = "will be reserved for storing the recovery configuration. This deposit is returned"]
                    #[doc = "in full when the user calls `remove_recovery`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `friends`: A list of friends you trust to vouch for recovery attempts. Should be"]
                    #[doc = "  ordered and contain no duplicate values."]
                    #[doc = "- `threshold`: The number of friends that must vouch for a recovery attempt before the"]
                    #[doc = "  account can be recovered. Should be less than or equal to the length of the list of"]
                    #[doc = "  friends."]
                    #[doc = "- `delay_period`: The number of blocks after a recovery attempt is initialized that"]
                    #[doc = "  needs to pass before the account can be recovered."]
                    create_recovery {
                        friends: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        threshold: ::core::primitive::u16,
                        delay_period: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Initiate the process for recovering a recoverable account."]
                    #[doc = ""]
                    #[doc = "Payment: `RecoveryDeposit` balance will be reserved for initiating the"]
                    #[doc = "recovery process. This deposit will always be repatriated to the account"]
                    #[doc = "trying to be recovered. See `close_recovery`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `account`: The lost account that you want to recover. This account needs to be"]
                    #[doc = "  recoverable (i.e. have a recovery configuration)."]
                    initiate_recovery {
                        account: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Allow a \"friend\" of a recoverable account to vouch for an active recovery"]
                    #[doc = "process for that account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and must be a \"friend\""]
                    #[doc = "for the recoverable account."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `lost`: The lost account that you want to recover."]
                    #[doc = "- `rescuer`: The account trying to rescue the lost account that you want to vouch for."]
                    #[doc = ""]
                    #[doc = "The combination of these two parameters must point to an active recovery"]
                    #[doc = "process."]
                    vouch_recovery {
                        lost: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        rescuer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Allow a successful rescuer to claim their recovered account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and must be a \"rescuer\""]
                    #[doc = "who has successfully completed the account recovery process: collected"]
                    #[doc = "`threshold` or more vouches, waited `delay_period` blocks since initiation."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `account`: The lost account that you want to claim has been successfully recovered by"]
                    #[doc = "  you."]
                    claim_recovery {
                        account: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "As the controller of a recoverable account, close an active recovery"]
                    #[doc = "process for your account."]
                    #[doc = ""]
                    #[doc = "Payment: By calling this function, the recoverable account will receive"]
                    #[doc = "the recovery deposit `RecoveryDeposit` placed by the rescuer."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and must be a"]
                    #[doc = "recoverable account with an active recovery process for it."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `rescuer`: The account trying to rescue this recoverable account."]
                    close_recovery {
                        rescuer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 7)]
                    #[doc = "Remove the recovery process for your account. Recovered accounts are still accessible."]
                    #[doc = ""]
                    #[doc = "NOTE: The user must make sure to call `close_recovery` on all active"]
                    #[doc = "recovery attempts before calling this function else it will fail."]
                    #[doc = ""]
                    #[doc = "Payment: By calling this function the recoverable account will unreserve"]
                    #[doc = "their recovery configuration deposit."]
                    #[doc = "(`ConfigDepositBase` + `FriendDepositFactor` * #_of_friends)"]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and must be a"]
                    #[doc = "recoverable account (i.e. has a recovery configuration)."]
                    remove_recovery,
                    #[codec(index = 8)]
                    #[doc = "Cancel the ability to use `as_recovered` for `account`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and registered to"]
                    #[doc = "be able to make calls on behalf of the recovered account."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `account`: The recovered account you are able to call on-behalf-of."]
                    cancel_recovered {
                        account: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                }
            }
        }
        pub mod pallet_referenda {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Propose a referendum on a privileged action."]
                    #[doc = ""]
                    #[doc = "- `origin`: must be `SubmitOrigin` and the account must have `SubmissionDeposit` funds"]
                    #[doc = "  available."]
                    #[doc = "- `proposal_origin`: The origin from which the proposal should be executed."]
                    #[doc = "- `proposal`: The proposal."]
                    #[doc = "- `enactment_moment`: The moment that the proposal should be enacted."]
                    #[doc = ""]
                    #[doc = "Emits `Submitted`."]
                    submit {
                        proposal_origin: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::OriginCaller,
                        >,
                        proposal:
                            runtime_types::frame_support::traits::preimages::Bounded<
                                runtime_types::kitchensink_runtime::RuntimeCall,
                            >,
                        enactment_moment:
                            runtime_types::frame_support::traits::schedule::DispatchTime<
                                ::core::primitive::u32,
                            >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Post the Decision Deposit for a referendum."]
                    #[doc = ""]
                    #[doc = "- `origin`: must be `Signed` and the account must have funds available for the"]
                    #[doc = "  referendum's track's Decision Deposit."]
                    #[doc = "- `index`: The index of the submitted referendum whose Decision Deposit is yet to be"]
                    #[doc = "  posted."]
                    #[doc = ""]
                    #[doc = "Emits `DecisionDepositPlaced`."]
                    place_decision_deposit { index: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    #[doc = "Refund the Decision Deposit for a closed referendum back to the depositor."]
                    #[doc = ""]
                    #[doc = "- `origin`: must be `Signed` or `Root`."]
                    #[doc = "- `index`: The index of a closed referendum whose Decision Deposit has not yet been"]
                    #[doc = "  refunded."]
                    #[doc = ""]
                    #[doc = "Emits `DecisionDepositRefunded`."]
                    refund_decision_deposit { index: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    #[doc = "Cancel an ongoing referendum."]
                    #[doc = ""]
                    #[doc = "- `origin`: must be the `CancelOrigin`."]
                    #[doc = "- `index`: The index of the referendum to be cancelled."]
                    #[doc = ""]
                    #[doc = "Emits `Cancelled`."]
                    cancel { index: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "Cancel an ongoing referendum and slash the deposits."]
                    #[doc = ""]
                    #[doc = "- `origin`: must be the `KillOrigin`."]
                    #[doc = "- `index`: The index of the referendum to be cancelled."]
                    #[doc = ""]
                    #[doc = "Emits `Killed` and `DepositSlashed`."]
                    kill { index: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "Advance a referendum onto its next logical state. Only used internally."]
                    #[doc = ""]
                    #[doc = "- `origin`: must be `Root`."]
                    #[doc = "- `index`: the referendum to be advanced."]
                    nudge_referendum { index: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "Advance a track onto its next logical state. Only used internally."]
                    #[doc = ""]
                    #[doc = "- `origin`: must be `Root`."]
                    #[doc = "- `track`: the track to be advanced."]
                    #[doc = ""]
                    #[doc = "Action item for when there is now one fewer referendum in the deciding phase and the"]
                    #[doc = "`DecidingCount` is not yet updated. This means that we should either:"]
                    #[doc = "- begin deciding another referendum (and leave `DecidingCount` alone); or"]
                    #[doc = "- decrement `DecidingCount`."]
                    one_fewer_deciding { track: ::core::primitive::u16 },
                    #[codec(index = 7)]
                    #[doc = "Refund the Submission Deposit for a closed referendum back to the depositor."]
                    #[doc = ""]
                    #[doc = "- `origin`: must be `Signed` or `Root`."]
                    #[doc = "- `index`: The index of a closed referendum whose Submission Deposit has not yet been"]
                    #[doc = "  refunded."]
                    #[doc = ""]
                    #[doc = "Emits `SubmissionDepositRefunded`."]
                    refund_submission_deposit { index: ::core::primitive::u32 },
                    #[codec(index = 8)]
                    #[doc = "Set or clear metadata of a referendum."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `origin`: Must be `Signed` by a creator of a referendum or by anyone to clear a"]
                    #[doc = "  metadata of a finished referendum."]
                    #[doc = "- `index`:  The index of a referendum to set or clear metadata for."]
                    #[doc = "- `maybe_hash`: The hash of an on-chain stored preimage. `None` to clear a metadata."]
                    set_metadata {
                        index: ::core::primitive::u32,
                        maybe_hash: ::core::option::Option<::subxt::utils::H256>,
                    },
                }
            }
        }
        pub mod pallet_remark {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Index and store data off chain."]
                    store {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
        }
        pub mod pallet_root_testing {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                }
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Anonymously schedule a task."]
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Cancel an anonymously scheduled task."]
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Schedule a named task."]
                    schedule_named {
                        id: [::core::primitive::u8; 32usize],
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Cancel a named scheduled task."]
                    cancel_named {
                        id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 4)]
                    #[doc = "Anonymously schedule a task after a delay."]
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Schedule a named task after a delay."]
                    schedule_named_after {
                        id: [::core::primitive::u8; 32usize],
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                }
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Sets the session key(s) of the function caller to `keys`."]
                    #[doc = "Allows an account to set its session key prior to becoming a validator."]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be signed."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
                    #[doc = "  fixed."]
                    set_keys {
                        keys: runtime_types::kitchensink_runtime::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Removes any session key(s) of the function caller."]
                    #[doc = ""]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                    #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                    #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                    #[doc = "usually means being a stash account)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
                    #[doc = "  `T::Keys::key_ids()` which is fixed."]
                    purge_keys,
                }
            }
        }
        pub mod pallet_society {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "A user outside of the society can make a bid for entry."]
                    #[doc = ""]
                    #[doc = "Payment: `CandidateDeposit` will be reserved for making a bid. It is returned"]
                    #[doc = "when the bid becomes a member, or if the bid calls `unbid`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `value`: A one time payment the bid would like to receive when joining the society."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(M + B + C + logM + logB + X)"]
                    #[doc = "\t  - B (len of bids)"]
                    #[doc = "  - C (len of candidates)"]
                    #[doc = "  - M (len of members)"]
                    #[doc = "  - X (balance reserve)"]
                    bid { value: ::core::primitive::u128 },
                    #[codec(index = 1)]
                    #[doc = "A bidder can remove their bid for entry into society."]
                    #[doc = "By doing so, they will have their candidate deposit returned or"]
                    #[doc = "they will unvouch their voucher."]
                    #[doc = ""]
                    #[doc = "Payment: The bid deposit is unreserved if the user made a bid."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and a bidder."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `pos`: Position in the `Bids` vector of the bid who wants to unbid."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(B + X)"]
                    #[doc = "  - B (len of bids)"]
                    #[doc = "  - X (balance unreserve)"]
                    unbid { pos: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    #[doc = "As a member, vouch for someone to join society by placing a bid on their behalf."]
                    #[doc = ""]
                    #[doc = "There is no deposit required to vouch for a new bid, but a member can only vouch for"]
                    #[doc = "one bid at a time. If the bid becomes a suspended candidate and ultimately rejected by"]
                    #[doc = "the suspension judgement origin, the member will be banned from vouching again."]
                    #[doc = ""]
                    #[doc = "As a vouching member, you can claim a tip if the candidate is accepted. This tip will"]
                    #[doc = "be paid as a portion of the reward the member will receive for joining the society."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and a member."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `who`: The user who you would like to vouch for."]
                    #[doc = "- `value`: The total reward to be paid between you and the candidate if they become"]
                    #[doc = "a member in the society."]
                    #[doc = "- `tip`: Your cut of the total `value` payout when the candidate is inducted into"]
                    #[doc = "the society. Tips larger than `value` will be saturated upon payout."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(M + B + C + logM + logB + X)"]
                    #[doc = "  - B (len of bids)"]
                    #[doc = "  - C (len of candidates)"]
                    #[doc = "  - M (len of members)"]
                    #[doc = "  - X (balance reserve)"]
                    vouch {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        value: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "As a vouching member, unvouch a bid. This only works while vouched user is"]
                    #[doc = "only a bidder (and not a candidate)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and a vouching member."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `pos`: Position in the `Bids` vector of the bid who should be unvouched."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(B)"]
                    #[doc = "  - B (len of bids)"]
                    unvouch { pos: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "As a member, vote on a candidate."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and a member."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `candidate`: The candidate that the member would like to bid on."]
                    #[doc = "- `approve`: A boolean which says if the candidate should be approved (`true`) or"]
                    #[doc = "  rejected (`false`)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(M + logM + C)"]
                    #[doc = "  - C (len of candidates)"]
                    #[doc = "  - M (len of members)"]
                    vote {
                        candidate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        approve: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "As a member, vote on the defender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and a member."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `approve`: A boolean which says if the candidate should be"]
                    #[doc = "approved (`true`) or rejected (`false`)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(M + logM)"]
                    #[doc = "  - M (len of members)"]
                    defender_vote { approve: ::core::primitive::bool },
                    #[codec(index = 6)]
                    #[doc = "Transfer the first matured payout for the sender and remove it from the records."]
                    #[doc = ""]
                    #[doc = "NOTE: This extrinsic needs to be called multiple times to claim multiple matured"]
                    #[doc = "payouts."]
                    #[doc = ""]
                    #[doc = "Payment: The member will receive a payment equal to their first matured"]
                    #[doc = "payout to their free balance."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and a member with"]
                    #[doc = "payouts remaining."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(M + logM + P + X)"]
                    #[doc = "  - M (len of members)"]
                    #[doc = "  - P (number of payouts for a particular member)"]
                    #[doc = "  - X (currency transfer call)"]
                    payout,
                    #[codec(index = 7)]
                    #[doc = "Found the society."]
                    #[doc = ""]
                    #[doc = "This is done as a discrete action in order to allow for the"]
                    #[doc = "pallet to be included into a running chain and can only be done once."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be from the _FounderSetOrigin_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `founder` - The first member and head of the newly founded society."]
                    #[doc = "- `max_members` - The initial max number of members for the society."]
                    #[doc = "- `rules` - The rules of this society concerning membership."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)"]
                    found {
                        founder: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        max_members: ::core::primitive::u32,
                        rules: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    #[doc = "Annul the founding of the society."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be Signed, and the signing account must be both"]
                    #[doc = "the `Founder` and the `Head`. This implies that it may only be done when there is one"]
                    #[doc = "member."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)"]
                    unfound,
                    #[codec(index = 9)]
                    #[doc = "Allow suspension judgement origin to make judgement on a suspended member."]
                    #[doc = ""]
                    #[doc = "If a suspended member is forgiven, we simply add them back as a member, not affecting"]
                    #[doc = "any of the existing storage items for that member."]
                    #[doc = ""]
                    #[doc = "If a suspended member is rejected, remove all associated storage items, including"]
                    #[doc = "their payouts, and remove any vouched bids they currently have."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be from the _SuspensionJudgementOrigin_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `who` - The suspended member to be judged."]
                    #[doc = "- `forgive` - A boolean representing whether the suspension judgement origin forgives"]
                    #[doc = "  (`true`) or rejects (`false`) a suspended member."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(M + logM + B)"]
                    #[doc = "  - B (len of bids)"]
                    #[doc = "  - M (len of members)"]
                    judge_suspended_member {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        forgive: ::core::primitive::bool,
                    },
                    #[codec(index = 10)]
                    #[doc = "Allow suspended judgement origin to make judgement on a suspended candidate."]
                    #[doc = ""]
                    #[doc = "If the judgement is `Approve`, we add them to society as a member with the appropriate"]
                    #[doc = "payment for joining society."]
                    #[doc = ""]
                    #[doc = "If the judgement is `Reject`, we either slash the deposit of the bid, giving it back"]
                    #[doc = "to the society treasury, or we ban the voucher from vouching again."]
                    #[doc = ""]
                    #[doc = "If the judgement is `Rebid`, we put the candidate back in the bid pool and let them go"]
                    #[doc = "through the induction process again."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be from the _SuspensionJudgementOrigin_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `who` - The suspended candidate to be judged."]
                    #[doc = "- `judgement` - `Approve`, `Reject`, or `Rebid`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(M + logM + B + X)"]
                    #[doc = "  - B (len of bids)"]
                    #[doc = "  - M (len of members)"]
                    #[doc = "  - X (balance action)"]
                    judge_suspended_candidate {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        judgement: runtime_types::pallet_society::Judgement,
                    },
                    #[codec(index = 11)]
                    #[doc = "Allows root origin to change the maximum number of members in society."]
                    #[doc = "Max membership count must be greater than 1."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be from _ROOT_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `max` - The maximum number of members for the society."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)"]
                    set_max_members { max: ::core::primitive::u32 },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum Judgement {
                #[codec(index = 0)]
                Rebid,
                #[codec(index = 1)]
                Reject,
                #[codec(index = 2)]
                Approve,
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
                        Debug,
                    )]
                    #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                    pub enum Call {
                        #[codec(index = 0)]
                        #[doc = "Take the origin account as a stash and lock up `value` of its balance. `controller` will"]
                        #[doc = "be the account that controls it."]
                        #[doc = ""]
                        #[doc = "`value` must be more than the `minimum_balance` specified by `T::Currency`."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the stash account."]
                        #[doc = ""]
                        #[doc = "Emits `Bonded`."]
                        #[doc = "## Complexity"]
                        #[doc = "- Independent of the arguments. Moderate complexity."]
                        #[doc = "- O(1)."]
                        #[doc = "- Three extra DB entries."]
                        #[doc = ""]
                        #[doc = "NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned"]
                        #[doc = "unless the `origin` falls below _existential deposit_ and gets removed as dust."]
                        bond {
                            controller: ::subxt::utils::MultiAddress<
                                ::subxt::utils::AccountId32,
                                ::core::primitive::u32,
                            >,
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::utils::AccountId32,
                            >,
                        },
                        #[codec(index = 1)]
                        #[doc = "Add some extra amount that have appeared in the stash `free_balance` into the balance up"]
                        #[doc = "for staking."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
                        #[doc = ""]
                        #[doc = "Use this if there are additional funds in your stash account that you wish to bond."]
                        #[doc = "Unlike [`bond`](Self::bond) or [`unbond`](Self::unbond) this function does not impose"]
                        #[doc = "any limitation on the amount that can be added."]
                        #[doc = ""]
                        #[doc = "Emits `Bonded`."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "- Independent of the arguments. Insignificant complexity."]
                        #[doc = "- O(1)."]
                        bond_extra {
                            #[codec(compact)]
                            max_additional: ::core::primitive::u128,
                        },
                        #[codec(index = 2)]
                        #[doc = "Schedule a portion of the stash to be unlocked ready for transfer out after the bond"]
                        #[doc = "period ends. If this leaves an amount actively bonded less than"]
                        #[doc = "T::Currency::minimum_balance(), then it is increased to the full amount."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                        #[doc = ""]
                        #[doc = "Once the unlock period is done, you can call `withdraw_unbonded` to actually move"]
                        #[doc = "the funds out of management ready for transfer."]
                        #[doc = ""]
                        #[doc = "No more than a limited number of unlocking chunks (see `MaxUnlockingChunks`)"]
                        #[doc = "can co-exists at the same time. If there are no unlocking chunks slots available"]
                        #[doc = "[`Call::withdraw_unbonded`] is called to remove some of the chunks (if possible)."]
                        #[doc = ""]
                        #[doc = "If a user encounters the `InsufficientBond` error when calling this extrinsic,"]
                        #[doc = "they should call `chill` first in order to free up their bonded funds."]
                        #[doc = ""]
                        #[doc = "Emits `Unbonded`."]
                        #[doc = ""]
                        #[doc = "See also [`Call::withdraw_unbonded`]."]
                        unbond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        #[codec(index = 3)]
                        #[doc = "Remove any unlocked chunks from the `unlocking` queue from our management."]
                        #[doc = ""]
                        #[doc = "This essentially frees up that balance to be used by the stash account to do"]
                        #[doc = "whatever it wants."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the controller."]
                        #[doc = ""]
                        #[doc = "Emits `Withdrawn`."]
                        #[doc = ""]
                        #[doc = "See also [`Call::unbond`]."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "O(S) where S is the number of slashing spans to remove"]
                        #[doc = "NOTE: Weight annotation is the kill scenario, we refund otherwise."]
                        withdraw_unbonded {
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        #[doc = "Declare the desire to validate for the origin controller."]
                        #[doc = ""]
                        #[doc = "Effects will be felt at the beginning of the next era."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                        validate {
                            prefs: runtime_types::pallet_staking::ValidatorPrefs,
                        },
                        #[codec(index = 5)]
                        #[doc = "Declare the desire to nominate `targets` for the origin controller."]
                        #[doc = ""]
                        #[doc = "Effects will be felt at the beginning of the next era."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "- The transaction's complexity is proportional to the size of `targets` (N)"]
                        #[doc = "which is capped at CompactAssignments::LIMIT (T::MaxNominations)."]
                        #[doc = "- Both the reads and writes follow a similar pattern."]
                        nominate {
                            targets: ::std::vec::Vec<
                                ::subxt::utils::MultiAddress<
                                    ::subxt::utils::AccountId32,
                                    ::core::primitive::u32,
                                >,
                            >,
                        },
                        #[codec(index = 6)]
                        #[doc = "Declare no desire to either validate or nominate."]
                        #[doc = ""]
                        #[doc = "Effects will be felt at the beginning of the next era."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "- Independent of the arguments. Insignificant complexity."]
                        #[doc = "- Contains one read."]
                        #[doc = "- Writes are limited to the `origin` account key."]
                        chill,
                        #[codec(index = 7)]
                        #[doc = "(Re-)set the payment target for a controller."]
                        #[doc = ""]
                        #[doc = "Effects will be felt instantly (as soon as this function is completed successfully)."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "- O(1)"]
                        #[doc = "- Independent of the arguments. Insignificant complexity."]
                        #[doc = "- Contains a limited number of reads."]
                        #[doc = "- Writes are limited to the `origin` account key."]
                        #[doc = "---------"]
                        set_payee {
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::utils::AccountId32,
                            >,
                        },
                        #[codec(index = 8)]
                        #[doc = "(Re-)set the controller of a stash."]
                        #[doc = ""]
                        #[doc = "Effects will be felt instantly (as soon as this function is completed successfully)."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "O(1)"]
                        #[doc = "- Independent of the arguments. Insignificant complexity."]
                        #[doc = "- Contains a limited number of reads."]
                        #[doc = "- Writes are limited to the `origin` account key."]
                        set_controller {
                            controller: ::subxt::utils::MultiAddress<
                                ::subxt::utils::AccountId32,
                                ::core::primitive::u32,
                            >,
                        },
                        #[codec(index = 9)]
                        #[doc = "Sets the ideal number of validators."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be Root."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "O(1)"]
                        set_validator_count {
                            #[codec(compact)]
                            new: ::core::primitive::u32,
                        },
                        #[codec(index = 10)]
                        #[doc = "Increments the ideal number of validators upto maximum of"]
                        #[doc = "`ElectionProviderBase::MaxWinners`."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be Root."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "Same as [`Self::set_validator_count`]."]
                        increase_validator_count {
                            #[codec(compact)]
                            additional: ::core::primitive::u32,
                        },
                        #[codec(index = 11)]
                        #[doc = "Scale up the ideal number of validators by a factor upto maximum of"]
                        #[doc = "`ElectionProviderBase::MaxWinners`."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be Root."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "Same as [`Self::set_validator_count`]."]
                        scale_validator_count {
                            factor: runtime_types::sp_arithmetic::per_things::Percent,
                        },
                        #[codec(index = 12)]
                        #[doc = "Force there to be no new eras indefinitely."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be Root."]
                        #[doc = ""]
                        #[doc = "# Warning"]
                        #[doc = ""]
                        #[doc = "The election process starts multiple blocks before the end of the era."]
                        #[doc = "Thus the election process may be ongoing when this is called. In this case the"]
                        #[doc = "election will continue until the next era is triggered."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "- No arguments."]
                        #[doc = "- Weight: O(1)"]
                        force_no_eras,
                        #[codec(index = 13)]
                        #[doc = "Force there to be a new era at the end of the next session. After this, it will be"]
                        #[doc = "reset to normal (non-forced) behaviour."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be Root."]
                        #[doc = ""]
                        #[doc = "# Warning"]
                        #[doc = ""]
                        #[doc = "The election process starts multiple blocks before the end of the era."]
                        #[doc = "If this is called just before a new era is triggered, the election process may not"]
                        #[doc = "have enough blocks to get a result."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "- No arguments."]
                        #[doc = "- Weight: O(1)"]
                        force_new_era,
                        #[codec(index = 14)]
                        #[doc = "Set the validators who cannot be slashed (if any)."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be Root."]
                        set_invulnerables {
                            invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        },
                        #[codec(index = 15)]
                        #[doc = "Force a current staker to become completely unstaked, immediately."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be Root."]
                        force_unstake {
                            stash: ::subxt::utils::AccountId32,
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 16)]
                        #[doc = "Force there to be a new era at the end of sessions indefinitely."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be Root."]
                        #[doc = ""]
                        #[doc = "# Warning"]
                        #[doc = ""]
                        #[doc = "The election process starts multiple blocks before the end of the era."]
                        #[doc = "If this is called just before a new era is triggered, the election process may not"]
                        #[doc = "have enough blocks to get a result."]
                        force_new_era_always,
                        #[codec(index = 17)]
                        #[doc = "Cancel enactment of a deferred slash."]
                        #[doc = ""]
                        #[doc = "Can be called by the `T::AdminOrigin`."]
                        #[doc = ""]
                        #[doc = "Parameters: era and indices of the slashes for that era to kill."]
                        cancel_deferred_slash {
                            era: ::core::primitive::u32,
                            slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                        },
                        #[codec(index = 18)]
                        #[doc = "Pay out all the stakers behind a single validator for a single era."]
                        #[doc = ""]
                        #[doc = "- `validator_stash` is the stash account of the validator. Their nominators, up to"]
                        #[doc = "  `T::MaxNominatorRewardedPerValidator`, will also receive their rewards."]
                        #[doc = "- `era` may be any era between `[current_era - history_depth; current_era]`."]
                        #[doc = ""]
                        #[doc = "The origin of this call must be _Signed_. Any account can call this function, even if"]
                        #[doc = "it is not one of the stakers."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "- At most O(MaxNominatorRewardedPerValidator)."]
                        payout_stakers {
                            validator_stash: ::subxt::utils::AccountId32,
                            era: ::core::primitive::u32,
                        },
                        #[codec(index = 19)]
                        #[doc = "Rebond a portion of the stash scheduled to be unlocked."]
                        #[doc = ""]
                        #[doc = "The dispatch origin must be signed by the controller."]
                        #[doc = ""]
                        #[doc = "## Complexity"]
                        #[doc = "- Time complexity: O(L), where L is unlocking chunks"]
                        #[doc = "- Bounded by `MaxUnlockingChunks`."]
                        rebond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        #[codec(index = 20)]
                        #[doc = "Remove all data structures concerning a staker/stash once it is at a state where it can"]
                        #[doc = "be considered `dust` in the staking system. The requirements are:"]
                        #[doc = ""]
                        #[doc = "1. the `total_balance` of the stash is below existential deposit."]
                        #[doc = "2. or, the `ledger.total` of the stash is below existential deposit."]
                        #[doc = ""]
                        #[doc = "The former can happen in cases like a slash; the latter when a fully unbonded account"]
                        #[doc = "is still receiving staking rewards in `RewardDestination::Staked`."]
                        #[doc = ""]
                        #[doc = "It can be called by anyone, as long as `stash` meets the above requirements."]
                        #[doc = ""]
                        #[doc = "Refunds the transaction fees upon successful execution."]
                        reap_stash {
                            stash: ::subxt::utils::AccountId32,
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 21)]
                        #[doc = "Remove the given nominations from the calling validator."]
                        #[doc = ""]
                        #[doc = "Effects will be felt at the beginning of the next era."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                        #[doc = ""]
                        #[doc = "- `who`: A list of nominator stash accounts who are nominating this validator which"]
                        #[doc = "  should no longer be nominating this validator."]
                        #[doc = ""]
                        #[doc = "Note: Making this call only makes sense if you first set the validator preferences to"]
                        #[doc = "block any further nominations."]
                        kick {
                            who: ::std::vec::Vec<
                                ::subxt::utils::MultiAddress<
                                    ::subxt::utils::AccountId32,
                                    ::core::primitive::u32,
                                >,
                            >,
                        },
                        #[codec(index = 22)]
                        #[doc = "Update the various staking configurations ."]
                        #[doc = ""]
                        #[doc = "* `min_nominator_bond`: The minimum active bond needed to be a nominator."]
                        #[doc = "* `min_validator_bond`: The minimum active bond needed to be a validator."]
                        #[doc = "* `max_nominator_count`: The max number of users who can be a nominator at once. When"]
                        #[doc = "  set to `None`, no limit is enforced."]
                        #[doc = "* `max_validator_count`: The max number of users who can be a validator at once. When"]
                        #[doc = "  set to `None`, no limit is enforced."]
                        #[doc = "* `chill_threshold`: The ratio of `max_nominator_count` or `max_validator_count` which"]
                        #[doc = "  should be filled in order for the `chill_other` transaction to work."]
                        #[doc = "* `min_commission`: The minimum amount of commission that each validators must maintain."]
                        #[doc = "  This is checked only upon calling `validate`. Existing validators are not affected."]
                        #[doc = ""]
                        #[doc = "RuntimeOrigin must be Root to call this function."]
                        #[doc = ""]
                        #[doc = "NOTE: Existing nominators and validators will not be affected by this update."]
                        #[doc = "to kick people under the new limits, `chill_other` should be called."]
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
                            min_commission:
                                runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                    runtime_types::sp_arithmetic::per_things::Perbill,
                                >,
                        },
                        #[codec(index = 23)]
                        #[doc = "Declare a `controller` to stop participating as either a validator or nominator."]
                        #[doc = ""]
                        #[doc = "Effects will be felt at the beginning of the next era."]
                        #[doc = ""]
                        #[doc = "The dispatch origin for this call must be _Signed_, but can be called by anyone."]
                        #[doc = ""]
                        #[doc = "If the caller is the same as the controller being targeted, then no further checks are"]
                        #[doc = "enforced, and this function behaves just like `chill`."]
                        #[doc = ""]
                        #[doc = "If the caller is different than the controller being targeted, the following conditions"]
                        #[doc = "must be met:"]
                        #[doc = ""]
                        #[doc = "* `controller` must belong to a nominator who has become non-decodable,"]
                        #[doc = ""]
                        #[doc = "Or:"]
                        #[doc = ""]
                        #[doc = "* A `ChillThreshold` must be set and checked which defines how close to the max"]
                        #[doc = "  nominators or validators we must reach before users can start chilling one-another."]
                        #[doc = "* A `MaxNominatorCount` and `MaxValidatorCount` must be set which is used to determine"]
                        #[doc = "  how close we are to the threshold."]
                        #[doc = "* A `MinNominatorBond` and `MinValidatorBond` must be set and checked, which determines"]
                        #[doc = "  if this is a person that should be chilled because they have not met the threshold"]
                        #[doc = "  bond required."]
                        #[doc = ""]
                        #[doc = "This can be helpful if bond requirements are updated, and we need to remove old users"]
                        #[doc = "who do not satisfy these requirements."]
                        chill_other {
                            controller: ::subxt::utils::AccountId32,
                        },
                        #[codec(index = 24)]
                        #[doc = "Force a validator to have at least the minimum commission. This will not affect a"]
                        #[doc = "validator who already has a commission greater than or equal to the minimum. Any account"]
                        #[doc = "can call this."]
                        force_apply_min_commission {
                            validator_stash: ::subxt::utils::AccountId32,
                        },
                        #[codec(index = 25)]
                        #[doc = "Sets the minimum amount of commission that each validators must maintain."]
                        #[doc = ""]
                        #[doc = "This call has lower privilege requirements than `set_staking_config` and can be called"]
                        #[doc = "by the `T::AdminOrigin`. Root can always call this."]
                        set_min_commission {
                            new: runtime_types::sp_arithmetic::per_things::Perbill,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum ConfigOp<_0> {
                        #[codec(index = 0)]
                        Noop,
                        #[codec(index = 1)]
                        Set(_0),
                        #[codec(index = 2)]
                        Remove,
                    }
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                Debug,
            )]
            pub struct ValidatorPrefs {
                #[codec(compact)]
                pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
                pub blocked: ::core::primitive::bool,
            }
        }
        pub mod pallet_state_trie_migration {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Control the automatic migration."] # [doc = ""] # [doc = "The dispatch origin of this call must be [`Config::ControlOrigin`]."] control_auto_migration { maybe_config : :: core :: option :: Option < runtime_types :: pallet_state_trie_migration :: pallet :: MigrationLimits > , } , # [codec (index = 1)] # [doc = "Continue the migration for the given `limits`."] # [doc = ""] # [doc = "The dispatch origin of this call can be any signed account."] # [doc = ""] # [doc = "This transaction has NO MONETARY INCENTIVES. calling it will not reward anyone. Albeit,"] # [doc = "Upon successful execution, the transaction fee is returned."] # [doc = ""] # [doc = "The (potentially over-estimated) of the byte length of all the data read must be"] # [doc = "provided for up-front fee-payment and weighing. In essence, the caller is guaranteeing"] # [doc = "that executing the current `MigrationTask` with the given `limits` will not exceed"] # [doc = "`real_size_upper` bytes of read data."] # [doc = ""] # [doc = "The `witness_task` is merely a helper to prevent the caller from being slashed or"] # [doc = "generally trigger a migration that they do not intend. This parameter is just a message"] # [doc = "from caller, saying that they believed `witness_task` was the last state of the"] # [doc = "migration, and they only wish for their transaction to do anything, if this assumption"] # [doc = "holds. In case `witness_task` does not match, the transaction fails."] # [doc = ""] # [doc = "Based on the documentation of [`MigrationTask::migrate_until_exhaustion`], the"] # [doc = "recommended way of doing this is to pass a `limit` that only bounds `count`, as the"] # [doc = "`size` limit can always be overwritten."] continue_migrate { limits : runtime_types :: pallet_state_trie_migration :: pallet :: MigrationLimits , real_size_upper : :: core :: primitive :: u32 , witness_task : runtime_types :: pallet_state_trie_migration :: pallet :: MigrationTask , } , # [codec (index = 2)] # [doc = "Migrate the list of top keys by iterating each of them one by one."] # [doc = ""] # [doc = "This does not affect the global migration process tracker ([`MigrationProcess`]), and"] # [doc = "should only be used in case any keys are leftover due to a bug."] migrate_custom_top { keys : :: std :: vec :: Vec < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , witness_size : :: core :: primitive :: u32 , } , # [codec (index = 3)] # [doc = "Migrate the list of child keys by iterating each of them one by one."] # [doc = ""] # [doc = "All of the given child keys must be present under one `child_root`."] # [doc = ""] # [doc = "This does not affect the global migration process tracker ([`MigrationProcess`]), and"] # [doc = "should only be used in case any keys are leftover due to a bug."] migrate_custom_child { root : :: std :: vec :: Vec < :: core :: primitive :: u8 > , child_keys : :: std :: vec :: Vec < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , total_size : :: core :: primitive :: u32 , } , # [codec (index = 4)] # [doc = "Set the maximum limit of the signed migration."] set_signed_max_limits { limits : runtime_types :: pallet_state_trie_migration :: pallet :: MigrationLimits , } , # [codec (index = 5)] # [doc = "Forcefully set the progress the running migration."] # [doc = ""] # [doc = "This is only useful in one case: the next key to migrate is too big to be migrated with"] # [doc = "a signed account, in a parachain context, and we simply want to skip it. A reasonable"] # [doc = "example of this would be `:code:`, which is both very expensive to migrate, and commonly"] # [doc = "used, so probably it is already migrated."] # [doc = ""] # [doc = "In case you mess things up, you can also, in principle, use this to reset the migration"] # [doc = "process."] force_set_progress { progress_top : runtime_types :: pallet_state_trie_migration :: pallet :: Progress , progress_child : runtime_types :: pallet_state_trie_migration :: pallet :: Progress , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct MigrationLimits {
                    pub size: ::core::primitive::u32,
                    pub item: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct MigrationTask {
                    pub progress_top:
                        runtime_types::pallet_state_trie_migration::pallet::Progress,
                    pub progress_child:
                        runtime_types::pallet_state_trie_migration::pallet::Progress,
                    pub size: ::core::primitive::u32,
                    pub top_items: ::core::primitive::u32,
                    pub child_items: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum Progress {
                    #[codec(index = 0)]
                    ToStart,
                    #[codec(index = 1)]
                    LastKey(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    #[codec(index = 2)]
                    Complete,
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo {
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    set_key {
                        new: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo_as {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
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
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report something `reason` that deserves a tip and claim any eventual the finder's fee."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Payment: `TipReportDepositBase` will be reserved from the origin account, as well as"]
                    #[doc = "`DataDepositPerByte` for each byte in `reason`."]
                    #[doc = ""]
                    #[doc = "- `reason`: The reason for, or the thing that deserves, the tip; generally this will be"]
                    #[doc = "  a UTF-8-encoded URL."]
                    #[doc = "- `who`: The account which should be credited for the tip."]
                    #[doc = ""]
                    #[doc = "Emits `NewTip` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)` where `R` length of `reason`."]
                    #[doc = "  - encoding and hashing of 'reason'"]
                    report_awesome {
                        reason: ::std::vec::Vec<::core::primitive::u8>,
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Retract a prior tip-report from `report_awesome`, and cancel the process of tipping."]
                    #[doc = ""]
                    #[doc = "If successful, the original deposit will be unreserved."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the tip identified by `hash`"]
                    #[doc = "must have been reported by the signing account through `report_awesome` (and not"]
                    #[doc = "through `tip_new`)."]
                    #[doc = ""]
                    #[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
                    #[doc = "  as the hash of the tuple of the original tip `reason` and the beneficiary account ID."]
                    #[doc = ""]
                    #[doc = "Emits `TipRetracted` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`"]
                    #[doc = "  - Depends on the length of `T::Hash` which is fixed."]
                    retract_tip { hash: ::subxt::utils::H256 },
                    #[codec(index = 2)]
                    #[doc = "Give a tip for something new; no finder's fee will be taken."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the signing account must be a"]
                    #[doc = "member of the `Tippers` set."]
                    #[doc = ""]
                    #[doc = "- `reason`: The reason for, or the thing that deserves, the tip; generally this will be"]
                    #[doc = "  a UTF-8-encoded URL."]
                    #[doc = "- `who`: The account which should be credited for the tip."]
                    #[doc = "- `tip_value`: The amount of tip that the sender would like to give. The median tip"]
                    #[doc = "  value of active tippers will be given to the `who`."]
                    #[doc = ""]
                    #[doc = "Emits `NewTip` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + T)` where `R` length of `reason`, `T` is the number of tippers."]
                    #[doc = "  - `O(T)`: decoding `Tipper` vec of length `T`. `T` is charged as upper bound given by"]
                    #[doc = "    `ContainsLengthBound`. The actual cost depends on the implementation of"]
                    #[doc = "    `T::Tippers`."]
                    #[doc = "  - `O(R)`: hashing and encoding of reason of length `R`"]
                    tip_new {
                        reason: ::std::vec::Vec<::core::primitive::u8>,
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        tip_value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Declare a tip value for an already-open tip."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the signing account must be a"]
                    #[doc = "member of the `Tippers` set."]
                    #[doc = ""]
                    #[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
                    #[doc = "  as the hash of the tuple of the hash of the original tip `reason` and the beneficiary"]
                    #[doc = "  account ID."]
                    #[doc = "- `tip_value`: The amount of tip that the sender would like to give. The median tip"]
                    #[doc = "  value of active tippers will be given to the `who`."]
                    #[doc = ""]
                    #[doc = "Emits `TipClosing` if the threshold of tippers has been reached and the countdown period"]
                    #[doc = "has started."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(T)` where `T` is the number of tippers. decoding `Tipper` vec of length `T`, insert"]
                    #[doc = "  tip and check closing, `T` is charged as upper bound given by `ContainsLengthBound`."]
                    #[doc = "  The actual cost depends on the implementation of `T::Tippers`."]
                    #[doc = ""]
                    #[doc = "  Actually weight could be lower as it depends on how many tips are in `OpenTip` but it"]
                    #[doc = "  is weighted as if almost full i.e of length `T-1`."]
                    tip {
                        hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        tip_value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Close and payout a tip."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "The tip identified by `hash` must have finished its countdown period."]
                    #[doc = ""]
                    #[doc = "- `hash`: The identity of the open tip for which a tip value is declared. This is formed"]
                    #[doc = "  as the hash of the tuple of the original tip `reason` and the beneficiary account ID."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- : `O(T)` where `T` is the number of tippers. decoding `Tipper` vec of length `T`. `T`"]
                    #[doc = "  is charged as upper bound given by `ContainsLengthBound`. The actual cost depends on"]
                    #[doc = "  the implementation of `T::Tippers`."]
                    close_tip { hash: ::subxt::utils::H256 },
                    #[codec(index = 5)]
                    #[doc = "Remove and slash an already-open tip."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::RejectOrigin`."]
                    #[doc = ""]
                    #[doc = "As a result, the finder is slashed and the deposits are lost."]
                    #[doc = ""]
                    #[doc = "Emits `TipSlashed` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    slash_tip { hash: ::subxt::utils::H256 },
                }
            }
        }
        pub mod pallet_transaction_storage {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Index and store data off chain. Minimum data size is 1 bytes, maximum is"] # [doc = "`MaxTransactionSize`. Data will be removed after `STORAGE_PERIOD` blocks, unless `renew`"] # [doc = "is called."] # [doc = "## Complexity"] # [doc = "- O(n*log(n)) of data size, as all data is pushed to an in-memory trie."] store { data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] # [doc = "Renew previously stored data. Parameters are the block number that contains"] # [doc = "previous `store` or `renew` call and transaction index within that block."] # [doc = "Transaction index is emitted in the `Stored` or `Renewed` event."] # [doc = "Applies same fees as `store`."] # [doc = "## Complexity"] # [doc = "- O(1)."] renew { block : :: core :: primitive :: u32 , index : :: core :: primitive :: u32 , } , # [codec (index = 2)] # [doc = "Check storage proof for block number `block_number() - StoragePeriod`."] # [doc = "If such block does not exist the proof is expected to be `None`."] # [doc = "## Complexity"] # [doc = "- Linear w.r.t the number of indexed transactions in the proved block for random"] # [doc = "  probing."] # [doc = "There's a DB read for each transaction."] check_proof { proof : runtime_types :: sp_transaction_storage_proof :: TransactionStorageProof , } , }
            }
        }
        pub mod pallet_treasury {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
                    #[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
                    #[doc = "proposal is awarded."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)"]
                    propose_spend {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        beneficiary: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Reject a proposed spend. The original deposit will be slashed."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::RejectOrigin`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)"]
                    reject_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
                    #[doc = "and the original deposit will be returned."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::ApproveOrigin`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = " - O(1)."]
                    approve_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Propose and approve a spend of treasury funds."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `SpendOrigin` with the `Success` value being at least `amount`."]
                    #[doc = "- `amount`: The amount to be transferred from the treasury to the `beneficiary`."]
                    #[doc = "- `beneficiary`: The destination account for the transfer."]
                    #[doc = ""]
                    #[doc = "NOTE: For record-keeping purposes, the proposer is deemed to be equivalent to the"]
                    #[doc = "beneficiary."]
                    spend {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        beneficiary: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Force a previously approved proposal to be removed from the approval queue."]
                    #[doc = "The original deposit will no longer be returned."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::RejectOrigin`."]
                    #[doc = "- `proposal_id`: The index of a proposal"]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(A) where `A` is the number of approvals"]
                    #[doc = ""]
                    #[doc = "Errors:"]
                    #[doc = "- `ProposalNotApproved`: The `proposal_id` supplied was not found in the approval queue,"]
                    #[doc = "i.e., the proposal has not been approved. This could also mean the proposal does not"]
                    #[doc = "exist altogether, thus there is no way it would have been approved in the first place."]
                    remove_approval {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_uniques {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Issue a new collection of non-fungible items from a public origin."]
                    #[doc = ""]
                    #[doc = "This new collection has no items initially and its owner is the origin."]
                    #[doc = ""]
                    #[doc = "The origin must conform to the configured `CreateOrigin` and have sufficient funds free."]
                    #[doc = ""]
                    #[doc = "`ItemDeposit` funds of sender are reserved."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `collection`: The identifier of the new collection. This must not be currently in use."]
                    #[doc = "- `admin`: The admin of this collection. The admin is the initial address of each"]
                    #[doc = "member of the collection's admin team."]
                    #[doc = ""]
                    #[doc = "Emits `Created` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    create {
                        collection: ::core::primitive::u32,
                        admin: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Issue a new collection of non-fungible items from a privileged origin."]
                    #[doc = ""]
                    #[doc = "This new collection has no items initially."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "Unlike `create`, no funds are reserved."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the new item. This must not be currently in use."]
                    #[doc = "- `owner`: The owner of this collection of items. The owner has full superuser"]
                    #[doc = "  permissions"]
                    #[doc = "over this item, but may later change and configure the permissions using"]
                    #[doc = "`transfer_ownership` and `set_team`."]
                    #[doc = ""]
                    #[doc = "Emits `ForceCreated` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_create {
                        collection: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        free_holding: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    #[doc = "Destroy a collection of fungible items."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be the"]
                    #[doc = "owner of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the collection to be destroyed."]
                    #[doc = "- `witness`: Information on the items minted in the collection. This must be"]
                    #[doc = "correct."]
                    #[doc = ""]
                    #[doc = "Emits `Destroyed` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(n + m)` where:"]
                    #[doc = "- `n = witness.items`"]
                    #[doc = "- `m = witness.item_metadatas`"]
                    #[doc = "- `a = witness.attributes`"]
                    destroy {
                        collection: ::core::primitive::u32,
                        witness: runtime_types::pallet_uniques::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    #[doc = "Mint an item of a particular collection."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed and the sender must be the Issuer of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection of the item to be minted."]
                    #[doc = "- `item`: The item value of the item to be minted."]
                    #[doc = "- `beneficiary`: The initial owner of the minted item."]
                    #[doc = ""]
                    #[doc = "Emits `Issued` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    mint {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Destroy a single item."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the signing account must be either:"]
                    #[doc = "- the Admin of the `collection`;"]
                    #[doc = "- the Owner of the `item`;"]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection of the item to be burned."]
                    #[doc = "- `item`: The item of the item to be burned."]
                    #[doc = "- `check_owner`: If `Some` then the operation will fail with `WrongOwner` unless the"]
                    #[doc = "  item is owned by this value."]
                    #[doc = ""]
                    #[doc = "Emits `Burned` with the actual amount burned."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: `check_owner.is_some()`."]
                    burn {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                        check_owner: ::core::option::Option<
                            ::subxt::utils::MultiAddress<
                                ::subxt::utils::AccountId32,
                                ::core::primitive::u32,
                            >,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Move an item from the sender account to another."]
                    #[doc = ""]
                    #[doc = "This resets the approved account of the item."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the signing account must be either:"]
                    #[doc = "- the Admin of the `collection`;"]
                    #[doc = "- the Owner of the `item`;"]
                    #[doc = "- the approved delegate for the `item` (in this case, the approval is reset)."]
                    #[doc = ""]
                    #[doc = "Arguments:"]
                    #[doc = "- `collection`: The collection of the item to be transferred."]
                    #[doc = "- `item`: The item of the item to be transferred."]
                    #[doc = "- `dest`: The account to receive ownership of the item."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Reevaluate the deposits on some items."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection to be frozen."]
                    #[doc = "- `items`: The items of the collection whose deposits will be reevaluated."]
                    #[doc = ""]
                    #[doc = "NOTE: This exists as a best-effort function. Any items which are unknown or"]
                    #[doc = "in the case that the owner account does not have reservable funds to pay for a"]
                    #[doc = "deposit increase are ignored. Generally the owner isn't going to call this on items"]
                    #[doc = "whose existing deposit is less than the refreshed deposit as it would only cost them,"]
                    #[doc = "so it's of little consequence."]
                    #[doc = ""]
                    #[doc = "It will still return an error in the case that the collection is unknown of the signer"]
                    #[doc = "is not permitted to call it."]
                    #[doc = ""]
                    #[doc = "Weight: `O(items.len())`"]
                    redeposit {
                        collection: ::core::primitive::u32,
                        items: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Disallow further unprivileged transfer of an item."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection of the item to be frozen."]
                    #[doc = "- `item`: The item of the item to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `Frozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Re-allow unprivileged transfer of an item."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection of the item to be thawed."]
                    #[doc = "- `item`: The item of the item to be thawed."]
                    #[doc = ""]
                    #[doc = "Emits `Thawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    #[doc = "Disallow further unprivileged transfers for a whole collection."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `CollectionFrozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze_collection { collection: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    #[doc = "Re-allow unprivileged transfers for a whole collection."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection to be thawed."]
                    #[doc = ""]
                    #[doc = "Emits `CollectionThawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw_collection { collection: ::core::primitive::u32 },
                    #[codec(index = 11)]
                    #[doc = "Change the Owner of a collection."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection whose owner should be changed."]
                    #[doc = "- `owner`: The new Owner of this collection. They must have called"]
                    #[doc = "  `set_accept_ownership` with `collection` in order for this operation to succeed."]
                    #[doc = ""]
                    #[doc = "Emits `OwnerChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer_ownership {
                        collection: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 12)]
                    #[doc = "Change the Issuer, Admin and Freezer of a collection."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the `collection`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection whose team should be changed."]
                    #[doc = "- `issuer`: The new Issuer of this collection."]
                    #[doc = "- `admin`: The new Admin of this collection."]
                    #[doc = "- `freezer`: The new Freezer of this collection."]
                    #[doc = ""]
                    #[doc = "Emits `TeamChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_team {
                        collection: ::core::primitive::u32,
                        issuer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        admin: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        freezer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 13)]
                    #[doc = "Approve an item to be transferred by a delegated third-party account."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin` or must be `Signed` and the sender must be"]
                    #[doc = "either the owner of the `item` or the admin of the collection."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection of the item to be approved for delegated transfer."]
                    #[doc = "- `item`: The item of the item to be approved for delegated transfer."]
                    #[doc = "- `delegate`: The account to delegate permission to transfer the item."]
                    #[doc = ""]
                    #[doc = "Important NOTE: The `approved` account gets reset after each transfer."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovedTransfer` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    approve_transfer {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                        delegate: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 14)]
                    #[doc = "Cancel the prior approval for the transfer of an item by a delegate."]
                    #[doc = ""]
                    #[doc = "Origin must be either:"]
                    #[doc = "- the `Force` origin;"]
                    #[doc = "- `Signed` with the signer being the Admin of the `collection`;"]
                    #[doc = "- `Signed` with the signer being the Owner of the `item`;"]
                    #[doc = ""]
                    #[doc = "Arguments:"]
                    #[doc = "- `collection`: The collection of the item of whose approval will be cancelled."]
                    #[doc = "- `item`: The item of the item of whose approval will be cancelled."]
                    #[doc = "- `maybe_check_delegate`: If `Some` will ensure that the given account is the one to"]
                    #[doc = "  which permission of transfer is delegated."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovalCancelled` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    cancel_approval {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                        maybe_check_delegate: ::core::option::Option<
                            ::subxt::utils::MultiAddress<
                                ::subxt::utils::AccountId32,
                                ::core::primitive::u32,
                            >,
                        >,
                    },
                    #[codec(index = 15)]
                    #[doc = "Alter the attributes of a given item."]
                    #[doc = ""]
                    #[doc = "Origin must be `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the item."]
                    #[doc = "- `owner`: The new Owner of this item."]
                    #[doc = "- `issuer`: The new Issuer of this item."]
                    #[doc = "- `admin`: The new Admin of this item."]
                    #[doc = "- `freezer`: The new Freezer of this item."]
                    #[doc = "- `free_holding`: Whether a deposit is taken for holding an item of this collection."]
                    #[doc = "- `is_frozen`: Whether this collection is frozen except for permissioned/admin"]
                    #[doc = "instructions."]
                    #[doc = ""]
                    #[doc = "Emits `ItemStatusChanged` with the identity of the item."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_item_status {
                        collection: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        issuer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        admin: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        freezer: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        free_holding: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    #[doc = "Set an attribute for a collection or item."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                    #[doc = "`collection`."]
                    #[doc = ""]
                    #[doc = "If the origin is Signed, then funds of signer are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + DepositPerByte * (key.len + value.len)` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the collection whose item's metadata to set."]
                    #[doc = "- `maybe_item`: The identifier of the item whose metadata to set."]
                    #[doc = "- `key`: The key of the attribute."]
                    #[doc = "- `value`: The value to which to set the attribute."]
                    #[doc = ""]
                    #[doc = "Emits `AttributeSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_attribute {
                        collection: ::core::primitive::u32,
                        maybe_item: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        value:
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                    },
                    #[codec(index = 17)]
                    #[doc = "Clear an attribute for a collection or item."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                    #[doc = "`collection`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the collection's owner."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the collection whose item's metadata to clear."]
                    #[doc = "- `maybe_item`: The identifier of the item whose metadata to clear."]
                    #[doc = "- `key`: The key of the attribute."]
                    #[doc = ""]
                    #[doc = "Emits `AttributeCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_attribute {
                        collection: ::core::primitive::u32,
                        maybe_item: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 18)]
                    #[doc = "Set the metadata for an item."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                    #[doc = "`collection`."]
                    #[doc = ""]
                    #[doc = "If the origin is Signed, then funds of signer are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + DepositPerByte * data.len` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the collection whose item's metadata to set."]
                    #[doc = "- `item`: The identifier of the item whose metadata to set."]
                    #[doc = "- `data`: The general information of this item. Limited in length by `StringLimit`."]
                    #[doc = "- `is_frozen`: Whether the metadata should be frozen against further changes."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_metadata {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                        data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 19)]
                    #[doc = "Clear the metadata for an item."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or Signed and the sender should be the Owner of the"]
                    #[doc = "`item`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the collection's owner."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the collection whose item's metadata to clear."]
                    #[doc = "- `item`: The identifier of the item whose metadata to clear."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_metadata {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                    },
                    #[codec(index = 20)]
                    #[doc = "Set the metadata for a collection."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"]
                    #[doc = "the `collection`."]
                    #[doc = ""]
                    #[doc = "If the origin is `Signed`, then funds of signer are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + DepositPerByte * data.len` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the item whose metadata to update."]
                    #[doc = "- `data`: The general information of this item. Limited in length by `StringLimit`."]
                    #[doc = "- `is_frozen`: Whether the metadata should be frozen against further changes."]
                    #[doc = ""]
                    #[doc = "Emits `CollectionMetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_collection_metadata {
                        collection: ::core::primitive::u32,
                        data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 21)]
                    #[doc = "Clear the metadata for a collection."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"]
                    #[doc = "the `collection`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the collection's owner."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the collection whose metadata to clear."]
                    #[doc = ""]
                    #[doc = "Emits `CollectionMetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_collection_metadata { collection: ::core::primitive::u32 },
                    #[codec(index = 22)]
                    #[doc = "Set (or reset) the acceptance of ownership for a particular account."]
                    #[doc = ""]
                    #[doc = "Origin must be `Signed` and if `maybe_collection` is `Some`, then the signer must have a"]
                    #[doc = "provider reference."]
                    #[doc = ""]
                    #[doc = "- `maybe_collection`: The identifier of the collection whose ownership the signer is"]
                    #[doc = "  willing to accept, or if `None`, an indication that the signer is willing to accept no"]
                    #[doc = "  ownership transferal."]
                    #[doc = ""]
                    #[doc = "Emits `OwnershipAcceptanceChanged`."]
                    set_accept_ownership {
                        maybe_collection: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 23)]
                    #[doc = "Set the maximum amount of items a collection could have."]
                    #[doc = ""]
                    #[doc = "Origin must be either `ForceOrigin` or `Signed` and the sender should be the Owner of"]
                    #[doc = "the `collection`."]
                    #[doc = ""]
                    #[doc = "Note: This function can only succeed once per collection."]
                    #[doc = ""]
                    #[doc = "- `collection`: The identifier of the collection to change."]
                    #[doc = "- `max_supply`: The maximum amount of items a collection could have."]
                    #[doc = ""]
                    #[doc = "Emits `CollectionMaxSupplySet` event when successful."]
                    set_collection_max_supply {
                        collection: ::core::primitive::u32,
                        max_supply: ::core::primitive::u32,
                    },
                    #[codec(index = 24)]
                    #[doc = "Set (or reset) the price for an item."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and must be the owner of the asset `item`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection of the item."]
                    #[doc = "- `item`: The item to set the price for."]
                    #[doc = "- `price`: The price for the item. Pass `None`, to reset the price."]
                    #[doc = "- `buyer`: Restricts the buy operation to a specific account."]
                    #[doc = ""]
                    #[doc = "Emits `ItemPriceSet` on success if the price is not `None`."]
                    #[doc = "Emits `ItemPriceRemoved` on success if the price is `None`."]
                    set_price {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                        price: ::core::option::Option<::core::primitive::u128>,
                        whitelisted_buyer: ::core::option::Option<
                            ::subxt::utils::MultiAddress<
                                ::subxt::utils::AccountId32,
                                ::core::primitive::u32,
                            >,
                        >,
                    },
                    #[codec(index = 25)]
                    #[doc = "Allows to buy an item if it's up for sale."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and must not be the owner of the `item`."]
                    #[doc = ""]
                    #[doc = "- `collection`: The collection of the item."]
                    #[doc = "- `item`: The item the sender wants to buy."]
                    #[doc = "- `bid_price`: The price the sender is willing to pay."]
                    #[doc = ""]
                    #[doc = "Emits `ItemBought` on success."]
                    buy_item {
                        collection: ::core::primitive::u32,
                        item: ::core::primitive::u32,
                        bid_price: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub items: ::core::primitive::u32,
                    #[codec(compact)]
                    pub item_metadatas: ::core::primitive::u32,
                    #[codec(compact)]
                    pub attributes: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Send a batch of dispatch calls."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(C) where C is the number of calls to be batched."]
                    #[doc = ""]
                    #[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
                    #[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
                    #[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
                    #[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
                    #[doc = "event is deposited."]
                    batch {
                        calls: ::std::vec::Vec<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Send a call through an indexed pseudonym of the sender."]
                    #[doc = ""]
                    #[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
                    #[doc = "use the same filter as the origin of this call."]
                    #[doc = ""]
                    #[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
                    #[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
                    #[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
                    #[doc = "in the Multisig pallet instead."]
                    #[doc = ""]
                    #[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Send a batch of dispatch calls and atomically execute them."]
                    #[doc = "The whole transaction will rollback and fail if any of the calls failed."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(C) where C is the number of calls to be batched."]
                    batch_all {
                        calls: ::std::vec::Vec<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Dispatches a function call with a provided origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    dispatch_as {
                        as_origin: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::OriginCaller,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Send a batch of dispatch calls."]
                    #[doc = "Unlike `batch`, it allows errors and won't interrupt."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatch without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(C) where C is the number of calls to be batched."]
                    force_batch {
                        calls: ::std::vec::Vec<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Dispatch a function call with a specified weight."]
                    #[doc = ""]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Root origin to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    with_weight {
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
            }
        }
        pub mod pallet_vesting {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Unlock any vested funds of the sender account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have funds still"]
                    #[doc = "locked under this pallet."]
                    #[doc = ""]
                    #[doc = "Emits either `VestingCompleted` or `VestingUpdated`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    vest,
                    #[codec(index = 1)]
                    #[doc = "Unlock any vested funds of a `target` account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `target`: The account whose vested funds should be unlocked. Must have funds still"]
                    #[doc = "locked under this pallet."]
                    #[doc = ""]
                    #[doc = "Emits either `VestingCompleted` or `VestingUpdated`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    vest_other {
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Create a vested transfer."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `target`: The account receiving the vested funds."]
                    #[doc = "- `schedule`: The vesting schedule attached to the transfer."]
                    #[doc = ""]
                    #[doc = "Emits `VestingCreated`."]
                    #[doc = ""]
                    #[doc = "NOTE: This will unlock all schedules through the current block."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    vested_transfer {
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        schedule:
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Force a vested transfer."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "- `source`: The account whose funds should be transferred."]
                    #[doc = "- `target`: The account that should be transferred the vested funds."]
                    #[doc = "- `schedule`: The vesting schedule attached to the transfer."]
                    #[doc = ""]
                    #[doc = "Emits `VestingCreated`."]
                    #[doc = ""]
                    #[doc = "NOTE: This will unlock all schedules through the current block."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`."]
                    force_vested_transfer {
                        source: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        target: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        schedule:
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Merge two vesting schedules together, creating a new vesting schedule that unlocks over"]
                    #[doc = "the highest possible start and end blocks. If both schedules have already started the"]
                    #[doc = "current block will be used as the schedule start; with the caveat that if one schedule"]
                    #[doc = "is finished by the current block, the other will be treated as the new merged schedule,"]
                    #[doc = "unmodified."]
                    #[doc = ""]
                    #[doc = "NOTE: If `schedule1_index == schedule2_index` this is a no-op."]
                    #[doc = "NOTE: This will unlock all schedules through the current block prior to merging."]
                    #[doc = "NOTE: If both schedules have ended by the current block, no new schedule will be created"]
                    #[doc = "and both will be removed."]
                    #[doc = ""]
                    #[doc = "Merged schedule attributes:"]
                    #[doc = "- `starting_block`: `MAX(schedule1.starting_block, scheduled2.starting_block,"]
                    #[doc = "  current_block)`."]
                    #[doc = "- `ending_block`: `MAX(schedule1.ending_block, schedule2.ending_block)`."]
                    #[doc = "- `locked`: `schedule1.locked_at(current_block) + schedule2.locked_at(current_block)`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `schedule1_index`: index of the first schedule to merge."]
                    #[doc = "- `schedule2_index`: index of the second schedule to merge."]
                    merge_schedules {
                        schedule1_index: ::core::primitive::u32,
                        schedule2_index: ::core::primitive::u32,
                    },
                }
            }
            pub mod vesting_info {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct VestingInfo<_0, _1> {
                    pub locked: _0,
                    pub per_block: _0,
                    pub starting_block: _1,
                }
            }
        }
        pub mod pallet_whitelist {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    whitelist_call { call_hash: ::subxt::utils::H256 },
                    #[codec(index = 1)]
                    remove_whitelisted_call { call_hash: ::subxt::utils::H256 },
                    #[codec(index = 2)]
                    dispatch_whitelisted_call {
                        call_hash: ::subxt::utils::H256,
                        call_encoded_len: ::core::primitive::u32,
                        call_weight_witness: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 3)]
                    dispatch_whitelisted_call_with_preimage {
                        call: ::std::boxed::Box<
                            runtime_types::kitchensink_runtime::RuntimeCall,
                        >,
                    },
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
                    Debug,
                )]
                pub struct PerU16(pub ::core::primitive::u16);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Percent(pub ::core::primitive::u8);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Perquintill(pub ::core::primitive::u64);
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
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
                    Debug,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                Debug,
            )]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation:
                    runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                Debug,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses: ::std::vec::Vec<
                        runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    >,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum Void {}
        }
        pub mod sp_npos_elections {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ElectionScore {
                pub minimal_stake: ::core::primitive::u128,
                pub sum_stake: ::core::primitive::u128,
                pub sum_stake_squared: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
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
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Digest {
                        pub logs: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem,
                        >,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
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
                        Debug,
                    )]
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
                        Debug,
                    )]
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
                        Debug,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                    );
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct BlakeTwo256;
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_transaction_storage_proof {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct TransactionStorageProof {
                pub chunk: ::std::vec::Vec<::core::primitive::u8>,
                pub proof: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct OldWeight(pub ::core::primitive::u64);
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue,"]
    #[doc = r" exposed here for ease of use."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
    }
    #[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
    pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
        client: &C,
    ) -> Result<(), ::subxt::error::MetadataError> {
        let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
        if runtime_metadata_hash
            != [
                83u8, 136u8, 130u8, 96u8, 143u8, 113u8, 229u8, 107u8, 41u8, 31u8, 166u8,
                254u8, 194u8, 33u8, 137u8, 153u8, 215u8, 35u8, 129u8, 80u8, 147u8, 52u8,
                36u8, 136u8, 200u8, 65u8, 108u8, 230u8, 160u8, 3u8, 219u8, 139u8,
            ]
        {
            Err(::subxt::error::MetadataError::IncompatibleMetadata)
        } else {
            Ok(())
        }
    }
}
