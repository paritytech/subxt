// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
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
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
    Error,
    Metadata,
};

/// 5 min `mortal_period` in milliseconds, to be adjusted based on expected block time
pub const BASELINE_MORTAL_PERIOD: u64 = 5 * 60 * 1000;

/// Fallback `BlockHashCount` of 2,400 blocks
pub const FALLBACK_BLOCK_HASH_COUNT: u64 = 2_400;

/// Fallback expected block time of 6,000 milliseconds
pub const FALLBACK_EXPECTED_BLOCK_TIME: u64 = 6_000;

/// Derive a default mortal period based on a chain's metadata
pub fn derive_mortal_period(metadata: &Metadata) -> Result<u64, Error> {
    let block_hash_count = if let Ok(system_meta) = metadata.module("System") {
        if let Ok(count) = system_meta.constant("BlockHashCount") {
            count.value::<u32>()?.into()
        } else {
            FALLBACK_BLOCK_HASH_COUNT
        }
    } else {
        FALLBACK_BLOCK_HASH_COUNT
    };
    let block_time = if let Ok(babe_meta) = metadata.module("Babe") {
        if let Ok(milliseconds) = babe_meta.constant("ExpectedBlockTime") {
            milliseconds.value::<u64>()?
        } else {
            FALLBACK_EXPECTED_BLOCK_TIME
        }
    } else {
        FALLBACK_EXPECTED_BLOCK_TIME
    };

    Ok((BASELINE_MORTAL_PERIOD / block_time)
        .next_power_of_two()
        .min(block_hash_count.next_power_of_two()))
}
