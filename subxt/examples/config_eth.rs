//! Configuring Subxt to talk to AssetHub.
use subxt::Error;
use subxt::config::{Config, DefaultTransactionExtensions, SubstrateConfig};

/// Our EthConfig wraps SubstrateConfig and configures the
/// account ID / address / signature to be based on 20 byte IDs.
#[derive(Debug, Clone, Default)]
pub struct EthConfig(SubstrateConfig);

impl Config for EthConfig {
    // Eth based chains use 20 byte account IDs
    // and ecdsa based signing:
    type Address = subxt::utils::AccountId20;
    type AccountId = subxt::utils::AccountId20;
    type Signature = subxt_signer::eth::Signature;

    // Just copy the default SubstrateConfig for these:
    type AssetId = <SubstrateConfig as Config>::AssetId;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type TransactionExtensions = DefaultTransactionExtensions<EthConfig>;

    // Forward these methods to the default SubstrateConfig:
    fn genesis_hash(&self) -> Option<subxt::config::HashFor<Self>> {
        self.0.genesis_hash()
    }
    fn legacy_types_for_spec_version<'this>(
        &'this self,
        spec_version: u32,
    ) -> Option<scale_info_legacy::TypeRegistrySet<'this>> {
        self.0.legacy_types_for_spec_version(spec_version)
    }
    fn metadata_for_spec_version(&self, spec_version: u32) -> Option<subxt::ArcMetadata> {
        self.0.metadata_for_spec_version(spec_version)
    }
    fn set_metadata_for_spec_version(&self, spec_version: u32, metadata: subxt::ArcMetadata) {
        self.0.set_metadata_for_spec_version(spec_version, metadata);
    }
    fn spec_and_transaction_version_for_block_number(
        &self,
        block_number: u64,
    ) -> Option<(u32, u32)> {
        self.0
            .spec_and_transaction_version_for_block_number(block_number)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // With the eth config defined, we can create a Subxt client using it:
    let _client = subxt::OnlineClient::<EthConfig>::new().await?;

    Ok(())
}
