// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use crate::test_context;
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

#[async_std::test]
async fn storage_get_current_timestamp() {
    let sys_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let cxt = test_context().await;

    // wait until blocks are produced to get the timestamp
    let mut sub = cxt.client().rpc().subscribe_blocks().await.unwrap();
    let block_hash = loop {
        if let Ok(Some(block)) = sub.next().await {
            break block.hash()
        }
    };

    let timestamp = cxt
        .api
        .storage()
        .timestamp()
        .now(Some(block_hash))
        .await
        .unwrap();

    assert!(timestamp > sys_timestamp)
}
