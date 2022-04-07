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

use codec::Decode;
use criterion::*;
use frame_metadata::{
    RuntimeMetadata::V14,
    RuntimeMetadataLastVersion,
    RuntimeMetadataPrefixed,
};
use subxt_metadata::{
    get_metadata_hash,
    get_pallet_hash,
};

fn load_metadata() -> RuntimeMetadataLastVersion {
    let bytes = test_runtime::METADATA;
    let meta: RuntimeMetadataPrefixed =
        Decode::decode(&mut &bytes[..]).expect("Cannot decode scale metadata");

    match meta.1 {
        V14(v14) => v14,
        _ => panic!("Unsupported metadata version {:?}", meta.1),
    }
}

fn bench(c: &mut Criterion) {
    let metadata = load_metadata();

    c.bench_function("full_metadata_validation", |b| {
        b.iter(|| get_metadata_hash(&metadata))
    });

    for pallet in metadata.pallets.iter() {
        let bench_name = format!("pallet_validation/{}", pallet.name);
        c.bench_function(&bench_name, |b| {
            b.iter(|| get_pallet_hash(&metadata.types, &pallet))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
