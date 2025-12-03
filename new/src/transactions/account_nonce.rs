use crate::client::OnlineClientAtBlockT;
use crate::config::Config;
use crate::error::AccountNonceError;
use codec::{Decode, Encode};

/// Return the account nonce at some block hash for an account ID.
pub async fn get_account_nonce<T, C>(
    client: &C,
    account_id: &T::AccountId,
) -> Result<u64, AccountNonceError>
where
    T: Config,
    C: OnlineClientAtBlockT<T>,
{
    let block_hash = client.block_hash();
    let account_nonce_bytes = client
        .backend()
        .call(
            "AccountNonceApi_account_nonce",
            Some(&account_id.encode()),
            block_hash,
        )
        .await?;

    // custom decoding from a u16/u32/u64 into a u64, based on the number of bytes we got back.
    let cursor = &mut &account_nonce_bytes[..];
    let account_nonce: u64 = match account_nonce_bytes.len() {
        2 => u16::decode(cursor)?.into(),
        4 => u32::decode(cursor)?.into(),
        8 => u64::decode(cursor)?,
        _ => {
            return Err(AccountNonceError::WrongNumberOfBytes(
                account_nonce_bytes.len(),
            ));
        }
    };
    Ok(account_nonce)
}
