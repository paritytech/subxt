// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{Decode, Encode};
use std::io::Read;
use subxt_metadata::Metadata;

static TEST_DIR_PREFIX: &str = "subxt_generated_pallets_ui_tests_";
static METADATA_FILE: &str = "../../artifacts/polkadot_metadata_full.scale";

pub struct PalletMetadataTestRunner {
    metadata: Metadata,
    index: usize,
    pallet_names: Option<Vec<String>>,
}

impl PalletMetadataTestRunner {
    /// if pallet_names is Some(..) only the provided pallets will be tested.
    pub fn new(pallet_names: Option<&[&str]>) -> PalletMetadataTestRunner {
        let mut file =
            std::fs::File::open(METADATA_FILE).expect("Cannot open metadata.scale artifact");

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .expect("Failed to read metadata.scale file");

        let metadata = Metadata::decode(&mut &*bytes).expect("Cannot decode metadata bytes");
        let pallet_names = pallet_names.map(|v| v.iter().map(|e| e.to_string()).collect());

        PalletMetadataTestRunner {
            metadata,
            index: 0,
            pallet_names,
        }
    }

    pub fn path_to_next_ui_test(&mut self) -> Option<String> {
        let pallet = match self.pallet_names.as_ref() {
            Some(names) => {
                self.metadata.pallet_by_name(&names.get(self.index)?)?
            },
            None => {
                self.metadata.pallets().nth(self.index)?
            }
        };

        let test_name = pallet.name();

        // Increment test index to point at the next pallet.
        let index = self.index;
        self.index += 1;

        // Build custom metadata containing only this pallet.
        let mut metadata = self.metadata.clone();
        metadata.retain(
            |pallet_filter| pallet_filter == pallet.name(),
            |_| true,
        );

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

        let encoded_metadata = metadata.encode();
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
        for idx in 0..self.index {
            let mut tmp_dir = std::env::temp_dir();
            tmp_dir.push(format!("{TEST_DIR_PREFIX}{idx}"));
            let _ = std::fs::remove_dir_all(tmp_dir);
        }
    }
}
