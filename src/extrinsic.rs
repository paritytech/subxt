use codec::Decode;
use futures::future;
use jsonrpsee::client::Subscription;
use sc_rpc_api::state::ReadProof;
use sp_core::storage::{
    StorageChangeSet,
    StorageKey,
};
pub use sp_runtime::traits::SignedExtension;
use sp_version::RuntimeVersion;
use std::marker::PhantomData;

use crate::{
    frame::system::{
        AccountStoreExt,
        Phase,
        System,
    },
    rpc::{
        ChainBlock,
        Rpc,
    },
};

/// Creates an payload for an extrinsic.
pub fn create_payload<C: Call<T>>(
    runtime_version: &RuntimeVersion,
    genesis_hash: T::Hash,
    nonce: T::Index,
    call: &Encoded,
) -> Result<SignedPayload<T>, Error>
where
    T: Runtime,
    <<T::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
    Send + Sync,
{
    let spec_version = runtime_version.spec_version;
    let tx_version = runtime_version.transaction_version;
    let extra: T::Extra =
        T::Extra::new(spec_version, tx_version, nonce, genesis_hash);
    let raw_payload = SignedPayload::<T>::new(call, extra.extra())?;
    Ok(raw_payload)
}
