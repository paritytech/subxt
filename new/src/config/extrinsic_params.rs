// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains a trait which controls the parameters that must
//! be provided in order to successfully construct an extrinsic.
//! [`crate::config::DefaultExtrinsicParams`] provides a general-purpose
//! implementation of this that will work in many cases.

use crate::{
    config::{Config, HashFor},
    error::ExtrinsicParamsError,
};
use core::any::Any;
use std::sync::Arc;
use subxt_metadata::Metadata;

/// This provides access to some relevant client state in transaction extensions,
/// and is just a combination of some of the available properties.
#[derive(Clone, Debug)]
pub struct ClientState<T: Config> {
    /// Genesis hash.
    pub genesis_hash: HashFor<T>,
    /// Runtime version.
    pub runtime_version: RuntimeVersion,
    /// Metadata.
    pub metadata: Arc<Metadata>,
}

/// Runtime version information needed to submit transactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeVersion {
    /// Version of the runtime specification. A full-node will not attempt to use its native
    /// runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    /// `spec_version` and `authoring_version` are the same between Wasm and native.
    pub spec_version: u32,
    /// All existing dispatches are fully compatible when this number doesn't change. If this
    /// number changes, then `spec_version` must change, also.
    ///
    /// This number must change when an existing dispatchable (module ID, dispatch ID) is changed,
    /// either through an alteration in its user-level semantics, a parameter
    /// added/removed/changed, a dispatchable being removed, a module being removed, or a
    /// dispatchable/module changing its index.
    ///
    /// It need *not* change when a new module is added or when a dispatchable is added.
    pub transaction_version: u32,
}

/// This trait allows you to configure the "signed extra" and
/// "additional" parameters that are a part of the transaction payload
/// or the signer payload respectively.
pub trait ExtrinsicParams<T: Config>: ExtrinsicParamsEncoder + Sized + Send + 'static {
    /// These parameters can be provided to the constructor along with
    /// some default parameters that `subxt` understands, in order to
    /// help construct your [`ExtrinsicParams`] object.
    type Params: Params<T>;

    /// Construct a new instance of our [`ExtrinsicParams`].
    fn new(client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError>;
}

/// This trait is expected to be implemented for any [`ExtrinsicParams`], and
/// defines how to encode the "additional" and "extra" params. Both functions
/// are optional and will encode nothing by default.
pub trait ExtrinsicParamsEncoder: 'static {
    /// This is expected to SCALE encode the transaction extension data to some
    /// buffer that has been provided. This data is attached to the transaction
    /// and also (by default) attached to the signer payload which is signed to
    /// provide a signature for the transaction.
    ///
    /// If [`ExtrinsicParamsEncoder::encode_signer_payload_value_to`] is implemented,
    /// then that will be used instead when generating a signer payload. Useful for
    /// eg the `VerifySignature` extension, which is send with the transaction but
    /// is not a part of the signer payload.
    fn encode_value_to(&self, _v: &mut Vec<u8>) {}

    /// See [`ExtrinsicParamsEncoder::encode_value_to`]. This defaults to calling that
    /// method, but if implemented will dictate what is encoded to the signer payload.
    fn encode_signer_payload_value_to(&self, v: &mut Vec<u8>) {
        self.encode_value_to(v);
    }

    /// This is expected to SCALE encode the "implicit" (formally "additional")
    /// parameters to some buffer that has been provided. These parameters are
    /// _not_ sent along with the transaction, but are taken into account when
    /// signing it, meaning the client and node must agree on their values.
    fn encode_implicit_to(&self, _v: &mut Vec<u8>) {}

    /// Set the signature. This happens after we have constructed the extrinsic params,
    /// and so is defined here rather than on the params, below. We need to use `&dyn Any`
    /// to keep this trait object safe, but can downcast in the impls.
    ///
    /// # Panics
    ///
    /// Implementations of this will likely try to downcast the provided `account_id`
    /// and `signature` into `T::AccountId` and `T::Signature` (where `T: Config`), and are
    /// free to panic if this downcasting does not succeed.
    ///
    /// In typical usage, this is not a problem, since this method is only called internally
    /// and provided values which line up with the relevant `Config`. In theory though, this
    /// method can be called manually with any types, hence this warning.
    fn inject_signature(&mut self, _account_id: &dyn Any, _signature: &dyn Any) {}
}

/// The parameters (ie [`ExtrinsicParams::Params`]) can also have data injected into them,
/// allowing Subxt to retrieve data from the chain and amend the parameters with it when
/// online.
pub trait Params<T: Config> {
    /// Set the account nonce.
    fn inject_account_nonce(&mut self, _nonce: u64) {}
    /// Set the current block.
    fn inject_block(&mut self, _number: u64, _hash: HashFor<T>) {}
}

impl<T: Config> Params<T> for () {}

macro_rules! impl_tuples {
    ($($ident:ident $index:tt),+) => {
        impl <Conf: Config, $($ident : Params<Conf>),+> Params<Conf> for ($($ident,)+){
            fn inject_account_nonce(&mut self, nonce: u64) {
                $(self.$index.inject_account_nonce(nonce);)+
            }

            fn inject_block(&mut self, number: u64, hash: HashFor<Conf>) {
                $(self.$index.inject_block(number, hash);)+
            }
        }
    }
}

#[rustfmt::skip]
const _: () = {
    impl_tuples!(A 0);
    impl_tuples!(A 0, B 1);
    impl_tuples!(A 0, B 1, C 2);
    impl_tuples!(A 0, B 1, C 2, D 3);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25);
};
