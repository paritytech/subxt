//! In this example we will connect to a parachain and subscribe to its blocks.
//! There are many parachains on Polkadot and Kusama. In order to connect to a
//! chain we first need to fetch its metadata. This requires us to know a valid
//! RPC endpoint of the chain.
//! The [Substrate Chains Runtime Api Doc Center](https://substrate.rs/index.html) lists parachains
//! on Polkadot and Kusama and gives us a list of RPC endpoints for each parachain. Alternatively
//! we can also look for the RPC endpoint of a chain via [Polkadot.js](https://polkadot.js.org/apps/).
//! In the side menu, you can select a Polkadot/Kusama parachain. The hit the "Switch" button and
//! let the page load the chains data. The RPC endpoint url can be found as part of the url.
//! You can also open the console, to see the RPC endpoint url printed without URL-encoded special characters.
//!
//! Let's go through an example for the "Ajuna" parachain on Polkadot.
//! Selecting it in the [Polkadot.js](https://polkadot.js.org/apps/) menu, then opening the console
//! lets us observe the following output: `WS endpoint= wss://rpc-parachain.ajuna.network`
//!
//! We can now get the metadata of the chain via the subxt-cli tool.
//! It is important to note that we need to add a `:433` as the port number here:
//! ```txt
//! metadata --url wss://rpc-parachain.ajuna.network:443 > artifacts/ajuna_metadata.scale
//! ```
//!
//! This allows us to construct a module with the ajuna types, calls,
//!
//!

use subxt::{Config, OnlineClient, PolkadotConfig, SubstrateConfig};


/// Default set of commonly used types by Polkadot nodes.
pub enum AjunaConfig {}

impl Config for AjunaConfig {
    type Index = <SubstrateConfig as Config>::Index;
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = <SubstrateConfig as Config>::ExtrinsicParams;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<AjunaConfig>::new().await?;

    Ok(())
}
