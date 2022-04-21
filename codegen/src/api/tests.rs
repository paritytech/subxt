// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

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

#[test]
fn check_documentation() {
    // Inspect metadata recursively and obtain all associated documentation.
    let _raw_docs = metadata_docs();
}
