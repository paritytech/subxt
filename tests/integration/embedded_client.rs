pub use crate::node_runtime::{
    self,
    system,
    DefaultConfig,
};
use node_cli::service::NewFullBase;
use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    PairSigner,
};
use subxt_client::{
    DatabaseSource,
    KeystoreConfig,
    Role,
    SubxtClient,
    SubxtClientConfig,
    WasmExecutionMethod,
};
use tempdir::TempDir;

#[async_std::test]
pub async fn test_embedded_client() {
    let tmp = TempDir::new("subxt-").expect("failed to create tempdir");
    let config = SubxtClientConfig {
        impl_name: "full-client",
        impl_version: "0.0.1",
        author: "",
        copyright_start_year: 2020,
        db: DatabaseSource::RocksDb {
            path: tmp.path().join("db"),
            cache_size: 16,
        },
        keystore: KeystoreConfig::Path {
            path: tmp.path().join("keystore"),
            password: None,
        },
        chain_spec: node_cli::chain_spec::development_config(),
        role: Role::Authority(AccountKeyring::Alice),
        telemetry: None,
        wasm_method: WasmExecutionMethod::Compiled,
        tokio_handle: tokio::runtime::Handle::current(),
    };

    let NewFullBase {
        task_manager,
        rpc_handlers,
        ..
    } = node_cli::service::new_full_base(config.into_service_config(), |_, _| ())
        .unwrap();

    let client = SubxtClient::new(task_manager, rpc_handlers);

    let ext_client = ClientBuilder::new()
        .set_client(client)
        .build::<DefaultConfig>()
        .await
        .unwrap();
    let api: node_runtime::RuntimeApi<DefaultConfig> =
        ext_client.clone().to_runtime_api();

    // verify that we can read storage
    api.storage()
        .system()
        .account(AccountKeyring::Alice.to_account_id().into(), None)
        .await
        .unwrap();

    let alice = PairSigner::<DefaultConfig, _>::new(AccountKeyring::Alice.pair());
    let bob_address = AccountKeyring::Bob.to_account_id().into();

    // verify that we can call dispatchable functions
    let result = api
        .tx()
        .balances()
        .transfer(bob_address, 100_000_000_000_000_000)
        .sign_and_submit_then_watch(&alice)
        .await
        .unwrap();

    // verify that we receive events
    let success = result
        .find_event::<system::events::ExtrinsicSuccess>()
        .unwrap();
    assert!(success.is_some());
}
