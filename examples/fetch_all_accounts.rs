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

use substrate_subxt::{
    system::AccountStoreExt,
    ClientBuilder,
    DefaultNodeRuntime,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = ClientBuilder::<DefaultNodeRuntime>::new().build().await?;
    let mut iter = client.account_iter(None).await?;
    while let Some((key, account)) = iter.next().await? {
        println!("{:?}: {}", key, account.data.free);
    }
    Ok(())
}
