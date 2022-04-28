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
    RuntimeMetadataPrefixed,
    RuntimeMetadataV14,
};
use scale_info::{
    form::PortableForm,
    TypeDef,
    TypeDefVariant,
};
use std::{
    fs,
    path::Path,
};
use subxt_metadata::{
    get_call_hash,
    get_constant_hash,
    get_metadata_hash,
    get_pallet_hash,
    get_storage_hash,
};

fn load_metadata() -> RuntimeMetadataV14 {
    let bytes = fs::read(Path::new("../artifacts/polkadot_metadata.scale"))
        .expect("Cannot read metadata blob");
    let meta: RuntimeMetadataPrefixed =
        Decode::decode(&mut &*bytes).expect("Cannot decode scale metadata");

    match meta.1 {
        V14(v14) => v14,
        _ => panic!("Unsupported metadata version {:?}", meta.1),
    }
}

fn expect_variant(def: &TypeDef<PortableForm>) -> &TypeDefVariant<PortableForm> {
    match def {
        TypeDef::Variant(variant) => variant,
        _ => panic!("Expected a variant type, got {def:?}"),
    }
}

fn bench_get_metadata_hash(c: &mut Criterion) {
    let metadata = load_metadata();

    c.bench_function("get_metadata_hash", |b| {
        b.iter(|| get_metadata_hash(&metadata))
    });
}

fn bench_get_pallet_hash(c: &mut Criterion) {
    let metadata = load_metadata();
    let mut group = c.benchmark_group("get_pallet_hash");

    for pallet in metadata.pallets.iter() {
        let pallet_name = &pallet.name;
        group.bench_function(pallet_name, |b| {
            b.iter(|| get_pallet_hash(&metadata.types, pallet))
        });
    }
}

fn bench_get_call_hash(c: &mut Criterion) {
    let metadata = load_metadata();
    let mut group = c.benchmark_group("get_call_hash");

    for pallet in metadata.pallets.iter() {
        let pallet_name = &pallet.name;
        let call_type_id = match &pallet.calls {
            Some(calls) => calls.ty.id(),
            None => continue,
        };
        let call_type = metadata.types.resolve(call_type_id).unwrap();
        let variants = expect_variant(call_type.type_def());

        for variant in variants.variants() {
            let call_name = variant.name();
            let bench_name = format!("{pallet_name}/{call_name}");
            group.bench_function(&bench_name, |b| {
                b.iter(|| get_call_hash(&metadata, &pallet.name, call_name))
            });
        }
    }
}

fn bench_get_constant_hash(c: &mut Criterion) {
    let metadata = load_metadata();
    let mut group = c.benchmark_group("get_constant_hash");

    for pallet in metadata.pallets.iter() {
        let pallet_name = &pallet.name;
        for constant in &pallet.constants {
            let constant_name = &constant.name;
            let bench_name = format!("{pallet_name}/{constant_name}");
            group.bench_function(&bench_name, |b| {
                b.iter(|| get_constant_hash(&metadata, &pallet.name, constant_name))
            });
        }
    }
}

fn bench_get_storage_hash(c: &mut Criterion) {
    let metadata = load_metadata();
    let mut group = c.benchmark_group("get_storage_hash");

    for pallet in metadata.pallets.iter() {
        let pallet_name = &pallet.name;
        let storage_entries = match &pallet.storage {
            Some(storage) => &storage.entries,
            None => continue,
        };

        for storage in storage_entries {
            let storage_name = &storage.name;
            let bench_name = format!("{pallet_name}/{storage_name}");
            group.bench_function(&bench_name, |b| {
                b.iter(|| get_storage_hash(&metadata, &pallet.name, storage_name))
            });
        }
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets =
        bench_get_metadata_hash,
        bench_get_pallet_hash,
        bench_get_call_hash,
        bench_get_constant_hash,
        bench_get_storage_hash,
);
criterion_main!(benches);
