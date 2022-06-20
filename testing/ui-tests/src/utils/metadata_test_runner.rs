// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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
