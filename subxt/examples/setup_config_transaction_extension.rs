#![allow(missing_docs)]
use codec::Encode;
use scale_encode::EncodeAsType;
use scale_info::PortableRegistry;
use subxt::client::ClientState;
use subxt::config::transaction_extensions;
use subxt::config::{
    Config, DefaultExtrinsicParamsBuilder, ExtrinsicParams, ExtrinsicParamsEncoder,
    ExtrinsicParamsError,
};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
pub mod runtime {}

// We don't need to construct this at runtime,
// so an empty enum is appropriate:
#[derive(EncodeAsType)]
pub enum CustomConfig {}

impl Config for CustomConfig {
    type AccountId = subxt::utils::AccountId32;
    type Address = subxt::utils::MultiAddress<Self::AccountId, ()>;
    type Signature = subxt::utils::MultiSignature;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type Header = subxt::config::substrate::SubstrateHeader<u32, Self::Hasher>;
    type ExtrinsicParams = transaction_extensions::AnyOf<
        Self,
        (
            // Load in the existing signed extensions we're interested in
            // (if the extension isn't actually needed it'll just be ignored):
            transaction_extensions::VerifySignature<Self>,
            transaction_extensions::CheckSpecVersion,
            transaction_extensions::CheckTxVersion,
            transaction_extensions::CheckNonce,
            transaction_extensions::CheckGenesis<Self>,
            transaction_extensions::CheckMortality<Self>,
            transaction_extensions::ChargeAssetTxPayment<Self>,
            transaction_extensions::ChargeTransactionPayment,
            transaction_extensions::CheckMetadataHash,
            // And add a new one of our own:
            CustomTransactionExtension,
        ),
    >;
    type AssetId = u32;
}

// Our custom signed extension doesn't do much:
pub struct CustomTransactionExtension;

// Give the extension a name; this allows `AnyOf` to look it
// up in the chain metadata in order to know when and if to use it.
impl<T: Config> transaction_extensions::TransactionExtension<T> for CustomTransactionExtension {
    type Decoded = ();
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CustomTransactionExtension"
    }
}

// Gather together any params we need for our signed extension, here none.
impl<T: Config> ExtrinsicParams<T> for CustomTransactionExtension {
    type Params = ();

    fn new(_client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CustomTransactionExtension)
    }
}

// Encode whatever the extension needs to provide when asked:
impl ExtrinsicParamsEncoder for CustomTransactionExtension {
    fn encode_value_to(&self, v: &mut Vec<u8>) {
        "Hello".encode_to(v);
    }
    fn encode_implicit_to(&self, v: &mut Vec<u8>) {
        true.encode_to(v)
    }
}

// When composing a tuple of signed extensions, the user parameters we need must
// be able to convert `Into` a tuple of corresponding `Params`. Here, we just
// "hijack" the default param builder, but add the `Params` (`()`) for our
// new signed extension at the end, to make the types line up. IN reality you may wish
// to construct an entirely new interface to provide the relevant `Params`.
pub fn custom(
    params: DefaultExtrinsicParamsBuilder<CustomConfig>,
) -> <<CustomConfig as Config>::ExtrinsicParams as ExtrinsicParams<CustomConfig>>::Params {
    let (a, b, c, d, e, f, g, h, i) = params.build();
    (a, b, c, d, e, f, g, h, i, ())
}

#[tokio::main]
async fn main() {
    // With the config defined, it can be handed to Subxt as follows:
    let client = subxt::OnlineClient::<CustomConfig>::new().await.unwrap();

    let tx_payload = runtime::tx().system().remark(b"Hello".to_vec());

    // Configure the tx params:
    let tx_config = DefaultExtrinsicParamsBuilder::new().tip(1234);

    // And provide them when submitting a transaction:
    let _ = client
        .tx()
        .sign_and_submit_then_watch(&tx_payload, &dev::alice(), custom(tx_config))
        .await;
}
