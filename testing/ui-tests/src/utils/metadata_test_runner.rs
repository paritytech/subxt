// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::Encode;
use frame_metadata::RuntimeMetadataPrefixed;

static TEST_DIR_PREFIX: &str = "subxt_generated_ui_tests_";

#[derive(Default)]
pub struct MetadataTestRunner {
    index: usize,
}

impl MetadataTestRunner {
    pub fn path_to_ui_test_for_metadata(
        &mut self,
        name: impl AsRef<str>,
        metadata: RuntimeMetadataPrefixed,
    ) -> String {
        let test_name = name.as_ref();

        // increment test index to avoid overlaps.
        let index = self.index;
        self.index += 1;

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
        std::fs::write(&tmp_metadata_path, &encoded_metadata).unwrap();
        // Write test file to tmp folder (it'll be moved by trybuild):
        std::fs::write(&tmp_rust_path, &rust_file).unwrap();

        tmp_rust_path
    }
}

// `trybuild` runs all tests once it's dropped. So, we defer all cleanup until we
// are dropped too, to make sure that cleanup happens after tests are ran.
impl Drop for MetadataTestRunner {
    fn drop(&mut self) {
        for i in 0..self.index {
            let mut tmp_dir = std::env::temp_dir();
            tmp_dir.push(format!("{TEST_DIR_PREFIX}{i}"));
            std::fs::remove_dir_all(tmp_dir).expect("cannot cleanup temp files");
        }
    }
}
