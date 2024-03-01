// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use subxt::client::OnlineClientT;
use subxt::tx::SubmittableExtrinsic;
use subxt::Config;

pub async fn submit_tx_wait_for_finalized_success<T, C>(
    signed_extrinsic: &SubmittableExtrinsic<T, C>,
) -> Result<subxt::blocks::ExtrinsicEvents<T>, subxt::Error>
where
    T: Config,
    C: OnlineClientT<T>,
{
    let submit = || async {
        signed_extrinsic
            .submit_and_watch()
            .await
            .unwrap()
            .wait_for_finalized_success()
            .await
    };

    #[cfg(lightclient)]
    for _ in 0..2 {
        let result = submit().await;

        match result {
            Ok(tx_in_block) => return Ok(tx_in_block),
            Err(subxt::Error::Transaction(subxt::error::TransactionError::Dropped(_))) => {
                // Retry if the transaction was dropped.
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Err(other) => return Err(other),
        }
    }

    submit().await
}
