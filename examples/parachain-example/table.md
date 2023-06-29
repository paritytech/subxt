| Tuple element                        | encoded type of struct                          | encoded type of `AdditionalSigned` type. |
| ------------------------------------ | ----------------------------------------------- | ---------------------------------------- |
| [frame_system::CheckNonZeroSender]   | ()                                              | ()                                       |
| [frame_system::CheckSpecVersion]     | ()                                              | [u32]                                    |
| [frame_system::CheckTxVersion]       | ()                                              | [u32]                                    |
| [frame_system::CheckGenesis]         | ()                                              | `Config::Hash` =                         |
| [frame_system::CheckMortality]       | [sp_runtime::generic::era::Era]                 | `Config::Hash` = [sp_core::H256]         |
| [frame_system::CheckNonce]           | [Config::Index] = [u32]                         | ()                                       |
| [frame_system::CheckWeight]          | ()                                              | ()                                       |
| [frame_system::ChargeAssetTxPayment] | [pallet_asset_tx_payment::ChargeAssetTxPayment] | ()                                       |
