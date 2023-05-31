// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::Decode;
use criterion::*;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use std::{fs, path::Path};
use subxt_metadata::Metadata;

fn load_metadata() -> Metadata {
    let bytes = fs::read(Path::new("../artifacts/polkadot_metadata_full.scale"))
        .expect("Cannot read metadata blob");
    let meta: RuntimeMetadataPrefixed =
        Decode::decode(&mut &*bytes).expect("Cannot decode scale metadata");

    match meta.1 {
        RuntimeMetadata::V14(v14) => v14.try_into().unwrap(),
        RuntimeMetadata::V15(v15) => v15.try_into().unwrap(),
        _ => panic!("Unsupported metadata version {:?}", meta.1),
    }
}

fn bench_get_metadata_hash(c: &mut Criterion) {
    let metadata = load_metadata();

    c.bench_function("get_metadata_hash", |b| b.iter(|| metadata.hasher().hash()));
}

fn bench_get_call_hash(c: &mut Criterion) {
    let metadata = load_metadata();
    let mut group = c.benchmark_group("get_call_hash");

    for pallet in metadata.pallets() {
        let pallet_name = pallet.name();
        let Some(variants) = pallet.call_variants() else {
            continue;
        };

        for variant in variants {
            let call_name = &variant.name;
            let bench_name = format!("{pallet_name}/{call_name}");
            group.bench_function(&bench_name, |b| b.iter(|| pallet.call_hash(call_name)));
        }
    }
}

fn bench_get_constant_hash(c: &mut Criterion) {
    let metadata = load_metadata();
    let mut group = c.benchmark_group("get_constant_hash");

    for pallet in metadata.pallets() {
        let pallet_name = pallet.name();
        for constant in pallet.constants() {
            let constant_name = constant.name();
            let bench_name = format!("{pallet_name}/{constant_name}");
            group.bench_function(&bench_name, |b| {
                b.iter(|| pallet.constant_hash(constant_name))
            });
        }
    }
}

fn bench_get_storage_hash(c: &mut Criterion) {
    let metadata = load_metadata();
    let mut group = c.benchmark_group("get_storage_hash");

    for pallet in metadata.pallets() {
        let pallet_name = pallet.name();
        let Some(storage_entries) = pallet.storage() else {
            continue;
        };

        for storage in storage_entries.entries() {
            let storage_name = storage.name();
            let bench_name = format!("{pallet_name}/{storage_name}");
            group.bench_function(&bench_name, |b| {
                b.iter(|| pallet.storage_hash(storage_name))
            });
        }
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets =
        bench_get_metadata_hash,
        bench_get_call_hash,
        bench_get_constant_hash,
        bench_get_storage_hash,
);
criterion_main!(benches);
