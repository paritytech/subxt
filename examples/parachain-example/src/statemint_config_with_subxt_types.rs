use codec::Encode;
use subxt::config::ExtrinsicParams;
use subxt::{Config};

pub enum StatemintConfig {}

impl Config for StatemintConfig {
    type Index = u32;
    type Hash = subxt::utils::H256;
    type AccountId = subxt::utils::AccountId32;
    type Address = subxt::utils::MultiAddress<Self::AccountId, ()>;
    type Signature = subxt::utils::MultiSignature;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type Header = subxt::config::substrate::SubstrateHeader<u32, Self::Hasher>;
    type ExtrinsicParams = StatemintExtrinsicParams;
}

#[derive(Encode, Debug, Clone)]
pub struct StatemintExtrinsicParams {
    extra_params: StatemintExtraParams,
    additional_params: StatemintAdditionalParams,
}

#[derive(Encode, Debug, Clone)]
pub struct StatemintExtraParams {
    era: subxt::config::extrinsic_params::Era,
    nonce: u32,
    charge: subxt::config::substrate::AssetTip,
}

#[derive(Encode, Debug, Clone)]
pub struct StatemintAdditionalParams {
    spec_version: u32,
    tx_version: u32,
    genesis_hash: subxt::utils::H256,
    mortality_hash: subxt::utils::H256,
}

impl ExtrinsicParams<<StatemintConfig as Config>::Index, <StatemintConfig as Config>::Hash>
for StatemintExtrinsicParams
{
    /// mortality hash, era, charge
    type OtherParams = (
        subxt::utils::H256,
        subxt::config::extrinsic_params::Era,
        subxt::config::substrate::AssetTip,
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
