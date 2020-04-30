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

use substrate_subxt::{
    system::System,
    Error,
    KusamaRuntime,
};

fn main() {
    async_std::task::block_on(async move {
        env_logger::init();

        let block_hash = fetch_block_hash(1).await;
        match block_hash {
            Ok(Some(hash)) => println!("Block hash for block number 1: {}", hash),
            Ok(None) => println!("Block number 1 not found."),
            Err(_) => eprintln!("Failed to fetch block hash"),
        }
    });
}

async fn fetch_block_hash(
    block_number: u32,
) -> Result<Option<<KusamaRuntime as System>::Hash>, Error> {
    substrate_subxt::ClientBuilder::<KusamaRuntime>::new()
        .set_url("wss://kusama-rpc.polkadot.io")
        .build()
        .await?
        .block_hash(Some(block_number.into()))
        .await
}
