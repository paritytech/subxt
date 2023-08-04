use codec::Encode;
use subxt::client::OfflineClientT;
use subxt::config::signed_extensions;
use subxt::config::{
    Config, DefaultExtrinsicParamsBuilder, ExtrinsicParams, ExtrinsicParamsEncoder,
};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
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
    type ExtrinsicParams = signed_extensions::AnyOf<
        Self,
        (
            // Load in all of the existing signed extensions:
            signed_extensions::CheckSpecVersion,
            signed_extensions::CheckTxVersion,
            signed_extensions::CheckNonce,
            signed_extensions::CheckGenesis<Self>,
            signed_extensions::CheckMortality<Self>,
            signed_extensions::ChargeAssetTxPayment,
            signed_extensions::ChargeTransactionPayment,
            // And add a new one of our own:
            CustomSignedExtension,
        ),
    >;
}

// Our custom signed extension doesn't do much:
pub struct CustomSignedExtension;

// Give the extension a name; this allows [`AnyOf`] to look it
// up in the chain metadata in order to know when and if to use it.
impl<T: Config> signed_extensions::SignedExtension<T> for CustomSignedExtension {
    const NAME: &'static str = "CustomSignedExtension";
}

// Gather together any params we need for our signed extension, here none.
impl<T: Config> ExtrinsicParams<T> for CustomSignedExtension {
    type OtherParams = ();
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        _client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
        Ok(CustomSignedExtension)
    }
}

// Encode whatever the extension needs to provide when asked:
impl ExtrinsicParamsEncoder for CustomSignedExtension {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        "Hello".encode_to(v);
    }
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        true.encode_to(v)
    }
}

// When composing a tuple of signed extensions, the user parameters we need must
// be able to convert `Into` a tuple of corresponding `OtherParams`. Here, we just
// "hijack" the default param builder, but add the `OtherParams` (`()`) for our
// new signed extension at the end, to make the types line up. IN reality you may wish
// to construct an entirely new interface to provide the relevant `OtherParams`.
pub fn custom(
    params: DefaultExtrinsicParamsBuilder<CustomConfig>,
) -> <<CustomConfig as Config>::ExtrinsicParams as ExtrinsicParams<CustomConfig>>::OtherParams {
    let (a, b, c, d, e, f, g) = params.raw();
    (a, b, c, d, e, f, g, ())
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
        .sign_and_submit_then_watch(&tx_payload, &dev::alice(), custom(tx_config));
}
