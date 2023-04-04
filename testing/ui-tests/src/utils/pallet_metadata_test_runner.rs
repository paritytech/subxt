// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{Decode, Encode};
use frame_metadata::{RuntimeMetadataPrefixed, RuntimeMetadataV14};
use std::io::Read;
use subxt_metadata::retain_metadata_pallets;

static TEST_DIR_PREFIX: &str = "subxt_generated_pallets_ui_tests_";
static METADATA_FILE: &str = "../../artifacts/polkadot_metadata.scale";

pub struct PalletMetadataTestRunner {
    metadata: RuntimeMetadataV14,
    index: usize,
}

impl PalletMetadataTestRunner {
    pub fn new() -> PalletMetadataTestRunner {
        let mut file =
            std::fs::File::open(METADATA_FILE).expect("Cannot open metadata.scale artifact");

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .expect("Failed to read metadata.scale file");

        let meta: RuntimeMetadataPrefixed =
            Decode::decode(&mut &*bytes).expect("Cannot decode metadata bytes");

        let metadata = match meta.1 {
            frame_metadata::RuntimeMetadata::V14(v14) => v14,
            _ => panic!("Unsupported metadata version. Tests support only v14"),
        };

        PalletMetadataTestRunner { metadata, index: 0 }
    }

    pub fn path_to_next_ui_test(&mut self) -> Option<String> {
        let Some(pallet) = self.metadata.pallets.get(self.index) else {
            return None
        };
        let test_name = &pallet.name;

        // Increment test index to avoid overlaps.
        let index = self.index;
        self.index += 1;

        // Build custom metadata containing only this pallet.
        let mut metadata = self.metadata.clone();
        retain_metadata_pallets(&mut metadata, |pallet_filter| pallet_filter == pallet.name);

        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push(format!("{TEST_DIR_PREFIX}{index}"));

        let tmp_metadata_path = {
            let mut t = tmp_dir.clone();
            t.push("metadata.scale");
            t.to_string_lossy().into_owned()
        };
        let tmp_rust_path = {
            let mut t = tmp_dir.clone();
            t.push(format!("{test_name}.rs"));
            t.to_string_lossy().into_owned()
        };

        let metadata_prefixed: RuntimeMetadataPrefixed = metadata.into();
        let encoded_metadata = metadata_prefixed.encode();
        let rust_file = format!(
            r#"
            use subxt;

            #[subxt::subxt(runtime_metadata_path = "{tmp_metadata_path}")]
            pub mod polkadot {{}}

            fn main() {{}}
        "#
        );

        std::fs::create_dir_all(&tmp_dir).expect("could not create tmp ui test dir");
        // Write metadata to tmp folder:
        std::fs::write(&tmp_metadata_path, encoded_metadata).unwrap();
        // Write test file to tmp folder (it'll be moved by trybuild):
        std::fs::write(&tmp_rust_path, rust_file).unwrap();

        Some(tmp_rust_path)
    }
}

// `trybuild` runs all tests once it's dropped. So, we defer all cleanup until we
// are dropped too, to make sure that cleanup happens after tests are ran.
impl Drop for PalletMetadataTestRunner {
    fn drop(&mut self) {
        for i in 0..self.index {
            let mut tmp_dir = std::env::temp_dir();
            tmp_dir.push(format!("{TEST_DIR_PREFIX}{i}"));
            std::fs::remove_dir_all(tmp_dir).expect("cannot cleanup temp files");
        }
    }
}
