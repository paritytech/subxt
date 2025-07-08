
/*

pub struct OnlineClient<T: Config> {
    rpc: subxt_rpcs::RpcClient<T>,
}

impl <T: Config> OnlineClient<T> {
    #[cfg(feature = "jsonrpsee")]
    pub async fn from_url(url: &str) -> Result<Self, Error> {
        let rpc = subxt_rpcs::RpcClient::new(url).await?;
        Ok(Self { rpc })
    }
}


OnlineClient<T> { RpcClient<T>}
OfflineClient<T> {}

BlocksClient {
    fn at
}

Steps To decode a block:

1. Fetch the block bytes from a node
2. Acquire the relevant metadata (maybe we need to fetch based on spec version, maybe we know we have it)
3. Use types for chain if metadata is <V14 (maybe baked in if known chain, maybe need providing)
4. Provide interface to allow users to decode extrinsic data into types (or maybe scale_value::Value by default)

Steps to decode storage entry are largely same; which block? Do we have/need metadata? Do we have/need types?
Same for Runtime APIs.

trait Config {
    // Return the name of the chain, eg "polkadot", "kusama" etc.
    fn chain_name(&self) -> &str;
}

// Client that can do thigns online...
struct OnlineClient<T, MP, TY> { 
    rpc: RpcClient<T>,
    metadata_provider: MP,
    types_provider: TP,
}
impl <T: Config, MP: OnlineMetadataProvider, TP: TypesProvider> OnlineClientT for OnlineClient<T, MP, TP> {
    // ...
}

// Client that can do things offline...
struct OfflineClient<T, MP> {
    metadata_provider: MP
}
impl <T: Config, MP: OfflineMetadataProvider> OfflineClientT for OfflineClient<T, MP> {
    // ...
}

// Basic impl for this would fetch spec version at the given block number,
// see if we've cached metadata for this spec version and download metadata if not.
// Clever impl would inspect chain (via eg `T::chain_name()`) and if polkadot/kusama,
// use pre-defined mapping from block number to spec version to reduce the effort and only
// fetch new meatdata where needed.
trait OnlineMetadataProvider<T: Config> {
    async fn get_spec_version<Client: OnlineClient<T>>(&self, client: Client, block_number: u64) -> Result<u32, Error>;
    async fn get_metadata<Client: OnlineClient<T>>(&self, client: Client, block_number: u64) -> Result<frame_metadata::RuntimeMetadata, Error>;
}

// An offline client will need a way to provide metadata, probably manually. So maybe need this too?
trait OfflineMetadataProvider<T: Config> {
    async fn get_metadata<Client: OnlineClient<T>>(&self, block_number: u64) -> Result<frame_metadata::RuntimeMetadata, Error>;
}

// We want to be able to fetch historic types for a chain

struct BlocksClient<T, Client> { 
    client: Client
}

impl <T: Config, Client: OnlineClient<T>> BlocksClient<T, Client> {
    /// Get a block at the given block number.
    pub async fn at(&self, block_number: u64) -> Result<Block<T, Client>, Error> {
        // Uses the metadata provider to fetch metadata.
        let md = self.client.get_metadata(block_number).await?;
        // Fetch type info if needed
        let types = self.client.get_types(block_number).await?;

    }
}


fn main() {
    let polkadot_config = PolkadotConfig::default();

    let api = OnlineClient::builder(polkadot_config)
        // The providers have sensible defaults, possibly defined in the config:
        .block_to_spec_version_provider(SomeBlockToSpecVersionProvider)
        .metadata_provider(SomeMetadataProvider)
        .historic_types_provider(SomeChainTypeRegistry)
        // We can connect to a bunch of nodes to round robin requests
        .connect_to_nodes(["https://rpc.polkadot.io"])
        .await
        .unwrap();

    // Signal which block we're working at.
    let api_at_block = api.at(12345);

    // Obtain things at the block. 
    api_at_block.get_extrinsics().await.unwrap();
    api_at_block.get_storage_entry("System", "Account", "5F3sa2...").await.unwrap();

    // Offline client can decode things but not fetch them.
    let exts = api_at_block.decode_extrinsics(bytes);
    let ext = api_at_block.decode_extrinsic(bytes);

    for ext in exts {
        ext.call_data().decode_as::<MyCallType>().unwrap();
        ext.pallet_name()
        ext.variant_name()
    }

}


What configuration do we need for a client?:
- Which historic types to use?
- Which URL(s) to connect to?
- How to resolve block number to spec version?
- How to get metadata for spec version?
- static chain types for RPCs: Header type, Hash type, AccountId type.
  - Header type only needed if we allow block header to be downloaded but may as well
  - AccountId type not necessary I think; only used in legacy RPCs.

*/
#![allow(missing_docs)]

pub mod config;
pub mod client;


