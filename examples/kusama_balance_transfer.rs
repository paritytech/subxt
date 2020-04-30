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

use sp_keyring::AccountKeyring;
use substrate_subxt::{
    balances,
    Error,
    KusamaRuntime,
};

fn main() {
    async_std::task::block_on(async move {
        env_logger::init();

        let xt_result = transfer_balance().await;
        match xt_result {
            Ok(hash) => println!("Balance transfer extrinsic submitted: {}", hash),
            Err(_) => eprintln!("Balance transfer extrinisic failed"),
        }
    });
}

async fn transfer_balance() -> Result<sp_core::H256, Error> {
    let signer = AccountKeyring::Alice.pair();
    let dest = AccountKeyring::Bob.to_account_id().into();

    // note use of `KusamaRuntime`
    substrate_subxt::ClientBuilder::<KusamaRuntime>::new()
        .build()
        .await?
        .xt(signer, None)
        .await?
        .submit(balances::TransferCall {
            to: &dest,
            amount: 10_000,
        })
        .await
}
