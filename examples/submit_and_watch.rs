// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use sp_keyring::AccountKeyring;
use substrate_subxt::{
    balances::{
        TransferCallExt,
        TransferEventExt,
    },
    ClientBuilder,
    DefaultNodeRuntime,
    PairSigner,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let client = ClientBuilder::<DefaultNodeRuntime>::new().build().await?;
    let result = client.transfer_and_watch(&signer, &dest, 10_000).await?;

    if let Some(event) = result.transfer()? {
        println!("Balance transfer success: value: {:?}", event.amount);
    } else {
        println!("Failed to find Balances::Transfer Event");
    }
    Ok(())
}
