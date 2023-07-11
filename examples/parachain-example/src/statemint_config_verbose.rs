use codec::Encode;
use subxt::config::ExtrinsicParams;
use subxt::{Config, PolkadotConfig, SubstrateConfig};

pub enum StatemintConfig {}

impl Config for StatemintConfig {
    type Index = u32;
    type Hash = sp_core::H256;
    type AccountId = sp_core::crypto::AccountId32;
    type Address = sp_runtime::MultiAddress<Self::AccountId, ()>;
    type Signature = sp_runtime::MultiSignature;
    type Hasher = sp_runtime::traits::BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, Self::Hasher>;
    type ExtrinsicParams = StatemintExtrinsicParams;
}

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct StatemintExtrinsicParams {
    extra_params: StatemintExtraParams,
    additional_params: StatemintAdditionalParams,
}

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct StatemintExtraParams {
    era: sp_runtime::generic::Era,
    nonce: u32,
    charge: ChargeAssetTxPayment,
}

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct ChargeAssetTxPayment {
    #[codec(compact)]
    tip: u128,
    asset_id: Option<u32>,
}

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct StatemintAdditionalParams {
    spec_version: u32,
    tx_version: u32,
    genesis_hash: sp_core::H256,
    mortality_hash: sp_core::H256,
}

impl ExtrinsicParams<<StatemintConfig as Config>::Index, <StatemintConfig as Config>::Hash>
for StatemintExtrinsicParams
{
    /// mortality hash, era, charge
    type OtherParams = (
        sp_core::H256,
        sp_runtime::generic::Era,
        ChargeAssetTxPayment,
    );

    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: <StatemintConfig as Config>::Index,
        genesis_hash: <StatemintConfig as Config>::Hash,
        other_params: Self::OtherParams,
    ) -> Self {
        let (mortality_hash, era, charge) = other_params;

        let extra_params = StatemintExtraParams { era, nonce, charge };

        let additional_params = StatemintAdditionalParams {
            spec_version,
            tx_version,
            genesis_hash,
            mortality_hash,
        };
        Self {
            extra_params,
            additional_params,
        }
    }

    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.extra_params.encode_to(v);
    }

    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.additional_params.encode_to(v);
    }
}


