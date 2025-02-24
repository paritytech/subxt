#![allow(missing_docs)]
use codec::Encode;
use subxt::client::ClientState;
use subxt::config::{
    transaction_extensions::Params, Config, ExtrinsicParams, ExtrinsicParamsEncoder,
    ExtrinsicParamsError,
};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_full.scale")]
pub mod runtime {}

// We don't need to construct this at runtime,
// so an empty enum is appropriate:
pub enum CustomConfig {}

impl Config for CustomConfig {
    type Hash = subxt::utils::H256;
    type AccountId = subxt::utils::AccountId32;
    type Address = subxt::utils::MultiAddress<Self::AccountId, ()>;
    type Signature = subxt::utils::MultiSignature;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type Header = subxt::config::substrate::SubstrateHeader<u32, Self::Hasher>;
    type ExtrinsicParams = CustomExtrinsicParams<Self>;
    type AssetId = u32;
}

// This represents some arbitrary (and nonsensical) custom parameters that
// will be attached to transaction extra and additional payloads:
pub struct CustomExtrinsicParams<T: Config> {
    genesis_hash: T::Hash,
    tip: u128,
    foo: bool,
}

// We can provide a "pretty" interface to allow users to provide these:
#[derive(Default)]
pub struct CustomExtrinsicParamsBuilder {
    tip: u128,
    foo: bool,
}

impl CustomExtrinsicParamsBuilder {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn tip(mut self, value: u128) -> Self {
        self.tip = value;
        self
    }
    pub fn enable_foo(mut self) -> Self {
        self.foo = true;
        self
    }
}

impl<T: Config> Params<T> for CustomExtrinsicParamsBuilder {}

// Describe how to fetch and then encode the params:
impl<T: Config> ExtrinsicParams<T> for CustomExtrinsicParams<T> {
    type Params = CustomExtrinsicParamsBuilder;

    // Gather together all of the params we will need to encode:
    fn new(client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(Self {
            genesis_hash: client.genesis_hash,
            tip: params.tip,
            foo: params.foo,
        })
    }
}

// Encode the relevant params when asked:
impl<T: Config> ExtrinsicParamsEncoder for CustomExtrinsicParams<T> {
    fn encode_value_to(&self, v: &mut Vec<u8>) {
        (self.tip, self.foo).encode_to(v);
    }
    fn encode_implicit_to(&self, v: &mut Vec<u8>) {
        self.genesis_hash.encode_to(v)
    }
}

#[tokio::main]
async fn main() {
    // With the config defined, it can be handed to Subxt as follows:
    let client = subxt::OnlineClient::<CustomConfig>::new().await.unwrap();

    let tx_payload = runtime::tx().system().remark(b"Hello".to_vec());

    // Build your custom "Params":
    let tx_config = CustomExtrinsicParamsBuilder::new().tip(1234).enable_foo();

    // And provide them when submitting a transaction:
    let _ = client
        .tx()
        .sign_and_submit_then_watch(&tx_payload, &dev::alice(), tx_config)
        .await;
}
