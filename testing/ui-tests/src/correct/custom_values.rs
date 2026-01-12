use codec::Decode;
use subxt::client::{OfflineClient, OfflineClientAtBlock};
use subxt::config::substrate::{SubstrateConfig, SpecVersionForRange};
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
    let foo_address = node::custom_values().foo();
    let foo = api.custom_values().entry(&foo_address).unwrap();
    assert_eq!(foo, expected_foo);

    // dynamic query:
    let foo_address = subxt::dynamic::custom_value::<Foo>("Foo");
    let foo = api.custom_values().entry(&foo_address).unwrap();
    assert_eq!(foo, expected_foo);

    // static query for some custom value that has an invalid type id: (we can still access the bytes)
    let custom_bytes = api.custom_values().entry_bytes("InvalidTypeId").unwrap();
    assert_eq!(vec![0,1,2,3], custom_bytes);
}

fn construct_offline_client() -> OfflineClientAtBlock<SubstrateConfig> {
    let metadata = {
        let bytes = std::fs::read("../../../../artifacts/metadata_with_custom_values.scale").unwrap();
        Metadata::decode(&mut &*bytes).unwrap()
    };

    let config = SubstrateConfig::builder()
        .set_metadata_for_spec_versions([(0, metadata.arc())])
        .set_spec_version_for_block_ranges([SpecVersionForRange {
            block_range: 0..u64::MAX,
            spec_version: 0,
            transaction_version: 0,
        }])
        .build();

    let offline_client = OfflineClient::new_with_config(config);
    offline_client.at_block(0u64).unwrap()
}
