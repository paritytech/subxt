use subxt::ext::codec::Decode;
use subxt::metadata::Metadata;
use subxt::utils::H256;
use subxt::{config::PolkadotConfig, OfflineClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We need to obtain the following details for an OfflineClient to be instantiated:

    // 1. Genesis hash (RPC call: chain_getBlockHash(0)):
    let genesis_hash = {
        let h = "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";
        let bytes = hex::decode(h).unwrap();
        H256::from_slice(&bytes)
    };

    // 2. A runtime version (system_version constant on a Substrate node has these):
    let runtime_version = subxt::rpc::types::RuntimeVersion {
        spec_version: 9370,
        transaction_version: 20,
        other: Default::default(),
    };

    // 3. Metadata (I'll load it from the downloaded metadata, but you can use
    //    `subxt metadata > file.scale` to download it):
    let metadata = {
        let bytes = std::fs::read("./artifacts/polkadot_metadata_small.scale").unwrap();
        Metadata::decode(&mut &*bytes).unwrap()
    };

    // Create an offline client using the details obtained above:
    let _api = OfflineClient::<PolkadotConfig>::new(genesis_hash, runtime_version, metadata);

    Ok(())
}
