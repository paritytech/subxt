

use subxt::{Config, PolkadotConfig, SubstrateConfig};

pub enum StatemintConfig {}

impl Config for StatemintConfig {
    type Index = <PolkadotConfig as Config>::Index;
    type Hash = <PolkadotConfig as Config>::Hash;
    type AccountId = <PolkadotConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <PolkadotConfig as Config>::Signature;
    type Hasher = <PolkadotConfig as Config>::Hasher;
    type Header = <PolkadotConfig as Config>::Header;
    // this is the only difference to the PolkadotConfig:
    type ExtrinsicParams = <SubstrateConfig as Config>::ExtrinsicParams;
}
