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

//! Create signed or unsigned extrinsics.

mod extra;
mod signer;

pub use self::{
    extra::{
        ChargeAssetTxPayment,
        CheckGenesis,
        CheckMortality,
        CheckNonce,
        CheckSpecVersion,
        CheckTxVersion,
        CheckWeight,
        DefaultExtra,
        SignedExtra,
    },
    signer::{
        PairSigner,
        Signer,
    },
};

use sp_runtime::traits::SignedExtension;
use sp_version::RuntimeVersion;

use crate::{
    Config,
    Encoded,
    Error,
    ExtrinsicExtraData,
};

/// UncheckedExtrinsic type.
pub type UncheckedExtrinsic<T> = sp_runtime::generic::UncheckedExtrinsic<
    <T as Config>::Address,
    Encoded,
    <T as Config>::Signature,
    <<T as ExtrinsicExtraData<T>>::Extra as SignedExtra<T>>::Extra,
>;

/// SignedPayload type.
pub type SignedPayload<T> = sp_runtime::generic::SignedPayload<
    Encoded,
    <<T as ExtrinsicExtraData<T>>::Extra as SignedExtra<T>>::Extra,
>;

/// Creates a signed extrinsic
pub async fn create_signed<T>(
    runtime_version: &RuntimeVersion,
    genesis_hash: T::Hash,
    nonce: T::Index,
    call: Encoded,
    signer: &(dyn Signer<T> + Send + Sync),
    additional_params: <T::Extra as SignedExtra<T>>::Parameters,
) -> Result<UncheckedExtrinsic<T>, Error>
where
    T: Config + ExtrinsicExtraData<T>,
    <<<T as ExtrinsicExtraData<T>>::Extra as SignedExtra<T>>::Extra as SignedExtension>::AdditionalSigned:
        Send + Sync,
{
    let spec_version = runtime_version.spec_version;
    let tx_version = runtime_version.transaction_version;
    let extra = <T as ExtrinsicExtraData<T>>::Extra::new(
        spec_version,
        tx_version,
        nonce,
        genesis_hash,
        additional_params,
    );
    let payload = SignedPayload::<T>::new(call, extra.extra())?;
    let signed = signer.sign(payload).await?;
    Ok(signed)
}
