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

//! Create signed or unsigned extrinsics.

mod extra;
mod signer;

pub use self::{
    extra::{
        DefaultExtra,
        Extra,
        SignedExtra,
    },
    signer::{
        PairSigner,
        Signer,
    },
};

use sp_runtime::{
    traits::SignedExtension,
    generic::Era
};
use sp_version::RuntimeVersion;

use crate::{
    frame::system::System,
    runtimes::Runtime,
    Encoded,
    Error,
};

/// A reasonable default for `era_period`
pub const DEFAULT_ERA_PERIOD: u64 = 64;

/// UncheckedExtrinsic type.
pub type UncheckedExtrinsic<T> = sp_runtime::generic::UncheckedExtrinsic<
    <T as System>::Address,
    Encoded,
    <T as Runtime>::Signature,
    Extra<T>,
>;

/// SignedPayload type.
pub type SignedPayload<T> = sp_runtime::generic::SignedPayload<Encoded, Extra<T>>;

/// Creates a signed extrinsic
pub async fn create_signed<T>(
    runtime_version: &RuntimeVersion,
    genesis_hash: T::Hash,
    nonce: T::Index,
    call: Encoded,
    signer: &(dyn Signer<T> + Send + Sync),
    era_opts: Option<(u64, u64, T::Hash)>,
) -> Result<UncheckedExtrinsic<T>, Error>
where
    T: Runtime,
    <<T::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
        Send + Sync,
{
    let spec_version = runtime_version.spec_version;
    let tx_version = runtime_version.transaction_version;
    let era_info = match era_opts {
        Some((period, cur_num, cur_hash)) => {
            (Era::mortal(period, cur_num), cur_hash)
        },
        None => (Era::Immortal, genesis_hash)
    };
    let extra = T::Extra::new(spec_version, tx_version, nonce, genesis_hash, era_info);
    let payload = SignedPayload::<T>::new(call, extra.extra())?;
    let signed = signer.sign(payload).await?;
    Ok(signed)
}

/// Creates an unsigned extrinsic
pub fn create_unsigned<T>(call: Encoded) -> UncheckedExtrinsic<T>
where
    T: Runtime,
{
    UncheckedExtrinsic::<T>::new_unsigned(call)
}
