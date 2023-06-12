// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::{Decode, Encode};
use std::io::Read;
use subxt_metadata::Metadata;

static TEST_DIR_PREFIX: &str = "subxt_generated_ui_tests_";
static METADATA_FILE: &str = "../../artifacts/polkadot_metadata_full.scale";

#[derive(Default)]
pub struct MetadataTestRunner {
    index: usize,
}

impl MetadataTestRunner {
    /// Loads metadata that we can use in our tests. Panics if
    /// there is some issue decoding the metadata.
    pub fn load_metadata() -> Metadata {
        let mut file =
            std::fs::File::open(METADATA_FILE).expect("Cannot open metadata.scale artifact");

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .expect("Failed to read metadata.scale file");

        Metadata::decode(&mut &*bytes).expect("Cannot decode metadata bytes")
    }

    /// Create a new test case.
    pub fn new_test_case(&mut self) -> MetadataTestRunnerCaseBuilder {
        let index = self.index;
        // increment index so that each test case gets its own folder path.
        self.index += 1;

        MetadataTestRunnerCaseBuilder::new(index)
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

/// Build a single test case.
pub struct MetadataTestRunnerCaseBuilder {
    index: usize,
    name: String,
    validation_metadata: Option<Metadata>,
    should_be_valid: bool,
}

impl MetadataTestRunnerCaseBuilder {
    fn new(index: usize) -> Self {
        MetadataTestRunnerCaseBuilder {
            index,
            name: format!("Test {index}"),
            validation_metadata: None,
            should_be_valid: true,
        }
    }

    /// Set the test name.
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = name.as_ref().to_owned();
        self
    }

    /// Set metadata to be validated against the generated code.
    /// By default, we'll validate the same metadata used to generate the code.
    pub fn validation_metadata(mut self, md: impl Into<Metadata>) -> Self {
        self.validation_metadata = Some(md.into());
        self
    }

    /// Expect the validation metadata provided to _not_ be valid.
    pub fn expects_invalid(mut self) -> Self {
        self.should_be_valid = false;
        self
    }

    /// At the minimum, takes some metadata and a test name, generates the code
    /// and hands back a path to some generated code that `trybuild` can be pointed at.
    /// validation metadata and expected validity can also be provided.
    ///
    /// The generated code:
    /// - checks that the subxt macro can perform codegen given the
    ///   provided macro_metadata without running into any issues.
    /// - checks that the `runtime::is_codegen_valid_for` function returns
    ///   true or false when compared to the `validation_metadata`, according
    ///   to whether `expects_invalid()` is set or not.
    ///
    /// The generated code will be tidied up when the `MetadataTestRunner` that
    /// this was handed out from is dropped.
    pub fn build<M>(self, macro_metadata: M) -> String
    where
        M: TryInto<Metadata>,
        M::Error: std::fmt::Debug,
    {
        let macro_metadata = macro_metadata.try_into().expect("can into Metadata");
        let validation_metadata = self
            .validation_metadata
            .unwrap_or_else(|| macro_metadata.clone());

        let index = self.index;
        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push(format!("{TEST_DIR_PREFIX}{index}"));

        let tmp_macro_metadata_path = {
            let mut t = tmp_dir.clone();
            t.push("macro_metadata.scale");
            t.to_string_lossy().into_owned()
        };
        let tmp_validation_metadata_path = {
            let mut t = tmp_dir.clone();
            t.push("validation_metadata.scale");
            t.to_string_lossy().into_owned()
        };
        let tmp_rust_path = {
            let mut t = tmp_dir.clone();
            let test_name = &self.name;
            t.push(format!("{test_name}.rs"));
            t.to_string_lossy().into_owned()
        };

        let encoded_macro_metadata = macro_metadata.encode();
        let encoded_validation_metadata = validation_metadata.encode();

        let should_be_valid_str = if self.should_be_valid {
            "true"
        } else {
            "false"
        };

        let rust_file = format!(
            r#"
            use subxt;
            use subxt::ext::codec::Decode;
            use std::io::Read;

            #[subxt::subxt(runtime_metadata_path = "{tmp_macro_metadata_path}")]
            pub mod polkadot {{}}

            fn main() {{
                // load validation metadata:
                let mut file = std::fs::File::open("{tmp_validation_metadata_path}")
                    .expect("validation_metadata exists");

                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)
                    .expect("Failed to read metadata.scale file");

                let metadata = subxt::Metadata::decode(&mut &*bytes)
                    .expect("Cannot decode metadata bytes");

                // validate it:
                let is_valid = polkadot::is_codegen_valid_for(&metadata);
                assert_eq!(is_valid, {should_be_valid_str}, "expected validity to line up");
            }}
        "#
        );

        std::fs::create_dir_all(&tmp_dir).expect("could not create tmp ui test dir");
        // Write metadatas to tmp folder:
        std::fs::write(&tmp_macro_metadata_path, encoded_macro_metadata).unwrap();
        std::fs::write(&tmp_validation_metadata_path, encoded_validation_metadata).unwrap();
        // Write test file to tmp folder (it'll be moved by trybuild):
        std::fs::write(&tmp_rust_path, rust_file).unwrap();

        tmp_rust_path
    }
}
