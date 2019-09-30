// Copyright 2019 Parity Technologies (UK) Ltd.
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

use std::marker::PhantomData;
use crate::srml::{System, Balances};

/// SignedExtra checks copied from substrate, in order to remove requirement to implement
/// substrate's `srml_system::Trait`, and allow additional signed data to be pass
mod extras {
    /// Ensure the runtime version registered in the transaction is the same as at present.
    #[derive(Encode, Decode, Clone, Eq, PartialEq)]
    pub struct CheckVersion<T: System>(PhantomData<T>);

    /// Nonce check and increment to give replay protection for transactions.
    #[derive(Encode, Decode, Clone, Eq, PartialEq)]
    pub struct CheckGenesis<T: System>(PhantomData<T>);

    /// Check for transaction mortality.
    #[derive(Encode, Decode, Clone, Eq, PartialEq)]
    pub struct CheckEra<T: System>((Era, PhantomData<T>));

    /// Nonce check and increment to give replay protection for transactions.
    #[derive(Encode, Decode, Clone, Eq, PartialEq)]
    pub struct CheckNonce<T: System>(#[codec(compact)] T::Index);

    /// Resource limit check.
    #[derive(Encode, Decode, Clone, Eq, PartialEq)]
    pub struct CheckWeight<T: Trait>(PhantomData<T>);

    /// Require the transactor pay for themselves and maybe include a tip to gain additional priority
    /// in the queue.
    #[derive(Encode, Decode, Clone, Eq, PartialEq)]
    pub struct TakeFees<T: Balances>(#[codec(compact)] T::Balance);

    pub trait SignedExtra<T> {
        type Extra: SignedExtension;
        type AdditionalSigned;

        fn extra(&self) -> Self::Extra;
    }

    pub struct DefaultExtra<T: System> {
        version: u32,
        nonce: T::Index,
        genesis_hash: T::Hash,
//        marker:PhantomData<fn() -> T>
    }

    impl<T: System + Balances> DefaultExtra<T> {
        pub fn new(version: u32, nonce: T::Index, genesis_hash: T::Hash) -> Self {
            DefaultExtra {
                version,
                nonce,
                genesis_hash,
            }
        }
    }

    impl<T: System + Balances> SignedExtra<T> for DefaultExtra<T> {
        type Extra = (
            CheckVersion<T>,
            CheckGenesis<T>,
            CheckEra<T>,
            CheckNonce<T>,
            CheckWeight<T>,
            TakeFees<T>,
        );

        type AdditionalSigned = (
            u32,        // CheckVersion
            T::Hash,    // CheckGenesis
            T::Hash,    // CheckEra(Era::Immortal)
            (),         // CheckNonce
            (),         // CheckWeight
            (),         // Take Fees
        );

        fn extra(&self) -> Self::Extra {
            (
                CheckVersion(PhantomData),
                CheckGenesis(PhantomData),
                CheckEra((Era::Immortal, PhantomData)),
                CheckNonce(self.nonce),
                CheckWeight(PhantomData),
                TakeFees(<T as Balances>::Balance::default())
            )
        }
    }

    impl<T: System + Balances> SignedExtra
}

/// Creates and signs an Extrinsic for the supplied `Call`
pub fn create_and_sign<T, C, P, E>(
    call: C,
    signer: P,
    extra: E,
) -> Result<
    UncheckedExtrinsic<
        <T::Lookup as StaticLookup>::Source,
        Encoded,
        P::Signature,
        T::SignedExtra,
    >,
    MetadataError,
>
    where
        C: Encoded,
        P: Pair,
        P::Public: Into<<T::Lookup as StaticLookup>::Source>,
        P::Signature: Codec,
        E: extras::SignedExtra,
{
    let raw_payload = (
        call.clone(),
        extra.clone(),
        version,
        (&genesis_hash, &genesis_hash),
    );
    let signature = raw_payload.using_encoded(|payload| {
        if payload.len() > 256 {
            signer.sign(&blake2_256(payload)[..])
        } else {
            signer.sign(payload)
        }
    });

    Ok(UncheckedExtrinsic::new_signed(
        raw_payload.0,
        signer.public().into(),
        signature.into(),
        extra,
    ))
}
