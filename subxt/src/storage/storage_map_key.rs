// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use codec::Encode;
pub use sp_runtime::traits::SignedExtension;

// We use this type a bunch, so export it from here.
pub use frame_metadata::StorageHasher;

/// Storage key for a Map.
#[derive(Clone)]
pub struct StorageMapKey {
    value: Vec<u8>,
    hasher: StorageHasher,
}

impl StorageMapKey {
    /// Create a new [`StorageMapKey`] by pre-encoding static data and pairing it with a hasher.
    pub fn new<Encodable: Encode>(
        value: Encodable,
        hasher: StorageHasher,
    ) -> StorageMapKey {
        Self {
            value: value.encode(),
            hasher,
        }
    }

    /// Convert this [`StorageMapKey`] into bytes and append them to some existing bytes.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) {
        hash_bytes(&self.value, &self.hasher, bytes)
    }
}

/// Take some SCALE encoded bytes and a [`StorageHasher`] and hash the bytes accordingly.
pub(super) fn hash_bytes(input: &[u8], hasher: &StorageHasher, bytes: &mut Vec<u8>) {
    match hasher {
        StorageHasher::Identity => bytes.extend(input),
        StorageHasher::Blake2_128 => bytes.extend(sp_core::blake2_128(input)),
        StorageHasher::Blake2_128Concat => {
            bytes.extend(sp_core::blake2_128(input));
            bytes.extend(input);
        }
        StorageHasher::Blake2_256 => bytes.extend(sp_core::blake2_256(input)),
        StorageHasher::Twox128 => bytes.extend(sp_core::twox_128(input)),
        StorageHasher::Twox256 => bytes.extend(sp_core::twox_256(input)),
        StorageHasher::Twox64Concat => {
            bytes.extend(sp_core::twox_64(input));
            bytes.extend(input);
        }
    }
}
