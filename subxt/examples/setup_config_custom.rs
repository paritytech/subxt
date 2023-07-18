use codec::Encode;
use primitive_types::H256;
use subxt::config::{Config, ExtrinsicParams};

// We don't need to construct this at runtime,
// so an empty enum is appropriate:
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

#[derive(Encode, Debug, Clone, Eq, PartialEq)]
pub struct StatemintExtrinsicParams {
    extra_params: StatemintExtraParams,
    additional_params: StatemintAdditionalParams,
}

impl ExtrinsicParams<u32, H256> for StatemintExtrinsicParams {
    // We need these additional values that aren't otherwise
    // provided. Calls like api.tx().sign_and_submit_then_watch()
    // allow the user to provide an instance of these, so it's wise
    // to give this a nicer interface in reality:
    type OtherParams = (
        sp_core::H256,
        sp_runtime::generic::Era,
        ChargeAssetTxPayment,
    );

    // Gather together all of the params we will need to encode:
    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: u32,
        genesis_hash: H256,
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

    // Encode the relevant params when asked:
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.extra_params.encode_to(v);
    }
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.additional_params.encode_to(v);
    }
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

#[tokio::main]
async fn main() {
    // With the config defined, it can be handed to Subxt as follows:
    let _client_fut = subxt::OnlineClient::<StatemintConfig>::new();
}
