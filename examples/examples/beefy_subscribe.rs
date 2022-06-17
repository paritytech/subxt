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

//! To run this example, a local polkadot node should be running.
//!
//! E.g.
//! ```bash
//! git clone https://github.com/octopus-network/barnacle.git
//! cargo build
//! ./target/debug/appchain-barnacle --dev --tmp
//! ```

use beefy_light_client::commitment;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    PolkadotExtrinsicParams as BarnacleExtrinsicParams,
};
use sp_core::hexdisplay::HexDisplay;

#[subxt::subxt(runtime_metadata_path = "../artifacts/barnacle_metadata.scale")]
pub mod barnacle {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<barnacle::RuntimeApi<DefaultConfig, BarnacleExtrinsicParams<DefaultConfig>>>();

    let mut sub = api.client.rpc().subscribe_beefy_justifications().await?;

    let raw = sub.next().await.unwrap().unwrap().0;

    let raw_vec = raw.0.clone();
    println!("raw SignedCommitment = {:?}", HexDisplay::from(&raw_vec));

    let data =
        <commitment::SignedCommitment as codec::Decode>::decode(&mut &raw[..]).unwrap();

    let commitment::Commitment {
        payload,
        block_number,
        validator_set_id,
    } = data.commitment;
    println!("signed commitment block_number : {}", block_number);
    println!("signed commitment validator_set_id : {}", validator_set_id);
    println!("signed commitment payload : {:?}", payload);
    println!("signatures :  {:?}", data.signatures);

    Ok(())
}