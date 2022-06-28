// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use regex::Regex;
use subxt_codegen::{
    DerivesRegistry,
    RuntimeGenerator,
};

fn metadata_docs() -> Vec<String> {
    // Load the runtime metadata downloaded from a node via `test-runtime`.
    let bytes = test_runtime::METADATA;
    let meta: frame_metadata::RuntimeMetadataPrefixed =
        codec::Decode::decode(&mut &*bytes).expect("Cannot decode scale metadata");
    let metadata = match meta.1 {
        frame_metadata::RuntimeMetadata::V14(v14) => v14,
        _ => panic!("Unsupported metadata version {:?}", meta.1),
    };

    // Inspect the metadata types and collect the documentation.
    let mut docs = Vec::new();
    for ty in metadata.types.types() {
        docs.extend_from_slice(ty.ty().docs());
    }

    for pallet in metadata.pallets {
        if let Some(storage) = pallet.storage {
            for entry in storage.entries {
                docs.extend(entry.docs);
            }
        }
        // Note: Calls, Events and Errors are deduced directly to
        // PortableTypes which are handled above.
        for constant in pallet.constants {
            docs.extend(constant.docs);
        }
    }
    // Note: Extrinsics do not have associated documentation, but is implied by
    // associated Type.

    docs
}

fn generate_runtime_interface() -> String {
    // Load the runtime metadata downloaded from a node via `test-runtime`.
    let bytes = test_runtime::METADATA;
    let metadata: frame_metadata::RuntimeMetadataPrefixed =
        codec::Decode::decode(&mut &*bytes).expect("Cannot decode scale metadata");

    // Generate a runtime interface from the provided metadata.
    let generator = RuntimeGenerator::new(metadata);
    let item_mod = syn::parse_quote!(
        pub mod api {}
    );
    let derives = DerivesRegistry::default();
    generator.generate_runtime(item_mod, derives).to_string()
}

fn interface_docs() -> Vec<String> {
    // Generate the runtime interface from the node's metadata.
    // Note: the API is generated on a single line.
    let runtime_api = generate_runtime_interface();

    // Documentation lines have the following format:
    //    # [ doc = "Upward message is invalid XCM."]
    // Given the API is generated on a single line, the regex matching
    // must be lazy hence the `?` in the matched group `(.*?)`.
    //
    // The greedy `non-?` matching would lead to one single match
    // from the beginning of the first documentation tag, containing everything up to
    // the last documentation tag
    // `# [ doc = "msg"] # [ doc = "msg2"] ... api ... # [ doc = "msgN" ]`
    //
    // The `(.*?)` stands for match any character zero or more times lazily.
    let re = Regex::new(r#"\# \[doc = "(.*?)"\]"#).unwrap();
    re.captures_iter(&runtime_api)
        .filter_map(|capture| {
            // Get the matched group (ie index 1).
            capture.get(1).as_ref().map(|doc| {
                // Generated documentation will escape special characters.
                // Replace escaped characters with unescaped variants for
                // exact matching on the raw metadata documentation.
                doc.as_str()
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\\"", "\"")
            })
        })
        .collect()
}

#[test]
fn check_documentation() {
    // Inspect metadata recursively and obtain all associated documentation.
    let raw_docs = metadata_docs();
    // Obtain documentation from the generated API.
    let runtime_docs = interface_docs();

    for raw in raw_docs.iter() {
        assert!(
            runtime_docs.contains(raw),
            "Documentation not present in runtime API: {}",
            raw
        );
    }
}
