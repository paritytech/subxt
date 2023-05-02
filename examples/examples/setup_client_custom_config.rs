use subxt::{
    config::{substrate::SubstrateExtrinsicParams, Config, SubstrateConfig},
    OnlineClient,
};

/// Define a custom config type (see the `subxt::config::Config` docs for
/// more information about each type):
enum MyConfig {}
impl Config for MyConfig {
    // This is different from the default `u32`:
    type Index = u64;
    // We can point to the default types if we don't need to change things:
    type Hash = <SubstrateConfig as Config>::Hash;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    // ExtrinsicParams makes use of the index type, so we need to tweak it
    // too to align with our modified index type, above:
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client which uses the custom config:
    let _api = OnlineClient::<MyConfig>::new().await?;

    Ok(())
}
