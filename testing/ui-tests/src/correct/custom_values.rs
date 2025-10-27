use codec::{Decode};
use subxt::{config::substrate::H256, OfflineClient, PolkadotConfig};
use subxt_metadata::Metadata;

#[subxt::subxt(runtime_metadata_path = "../../../../artifacts/metadata_with_custom_values.scale", derive_for_all_types = "Eq, PartialEq")]
pub mod node {}
use node::runtime_types::generate_custom_metadata::Foo;

fn main() {
    let api = construct_offline_client();

    let expected_foo = Foo {
        a: 42,
        b: "Have a great day!".into(),
    };

    // static query:
    let foo_address = node::custom().foo();
    let foo = api.custom_values().at(&foo_address).unwrap();
    assert_eq!(foo, expected_foo);

    // dynamic query:
    let foo_address = subxt::dynamic::custom_value::<Foo>("Foo");
    let foo = api.custom_values().at(&foo_address).unwrap();
    assert_eq!(foo, expected_foo);

    // static query for some custom value that has an invalid type id: (we can still access the bytes)
    let custom_bytes = api.custom_values().bytes_at("InvalidTypeId").unwrap();
    assert_eq!(vec![0,1,2,3], custom_bytes);
}

fn construct_offline_client() -> OfflineClient<PolkadotConfig> {
    let genesis_hash = {
        let h = "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";
        let bytes = hex::decode(h).unwrap();
        H256::from_slice(&bytes)
    };
    let runtime_version = subxt::client::RuntimeVersion {
        spec_version: 9370,
        transaction_version: 20,
    };

    let metadata = {
        let bytes = std::fs::read("../../../../artifacts/metadata_with_custom_values.scale").unwrap();
        Metadata::decode(&mut &*bytes).unwrap()
    };
    OfflineClient::<PolkadotConfig>::new(genesis_hash, runtime_version, metadata)
}
