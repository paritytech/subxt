// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use crate::config::{Config, ClientState, HashFor};
use crate::error::ExtrinsicParamsError;
use scale_decode::DecodeAsType;
use scale_info::PortableRegistry;

/// A set of [`TransactionExtensions`]. This:
/// - Implements [`frame_decode::extrinsics::TransactionExtensions`], meaning that it
///   can be encoded to value or implicit bytes.
/// - Accepts custom params on construction, allowing user configuration to be provided.
/// - Can be decoded into some output type.
/// - Can have an account ID and signature injected (useful for V5 transaction extensions).
pub trait TransactionExtensions<T: Config>: frame_decode::extrinsics::TransactionExtensions<PortableRegistry> + Sized {
    /// These parameters can be provided to the constructor along with
    /// some default parameters that Subxt understands.
    type Params: Params<T>;

    /// Construct a new instance of our [`TransactionExtensions`].
    fn new(client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError>;

    /// Set the signature and account ID for any transaction extensions that care.
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
    fn inject_signature(&mut self, account_id: &T::AccountId, signature: &T::Signature);
}

/// A single transaction extension.
pub trait TransactionExtension<T: Config>: frame_decode::extrinsics::TransactionExtension<PortableRegistry> + Sized {
    /// The type representing the `extra` / value bytes of a transaction extension.
    /// Decoding from this type should be symmetrical to the respective
    /// [`frame_decode::extrinsics::TransactionExtension::encode_value_to()`] implementation 
    /// for this transaction extension.
    type Decoded: DecodeAsType;

    /// These parameters can be provided to the constructor along with
    /// some default parameters that Subxt understands.
    type Params: Params<T>;

    /// Construct a new instance of our [`TransactionExtensions`].
    fn new(client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError>;

    /// Set the signature and accountID for this transaction extension. Defaults to doing nothing.
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
    fn inject_signature(&mut self, _account_id: &T::AccountId, _signature: &T::Signature) {}
}

/// The parameters (ie [`TransactionExtensions::Params`]) can also have data injected into them,
/// allowing Subxt to retrieve data from the chain and amend the parameters with it when
/// online.
pub trait Params<T: Config> {
    /// Set the account nonce.
    fn inject_account_nonce(&mut self, _nonce: u64) {}
    /// Set the current block.
    fn inject_block(&mut self, _number: u64, _hash: HashFor<T>) {}
}

// empty tuples impl Params and do nothing.
impl<T: Config> Params<T> for () {}

// tuples of Params are also valid Params.
macro_rules! impl_params_tuple {
    ($($ident:ident $index:tt),+) => {
        impl <Conf: Config, $($ident : Params<Conf>),+> Params<Conf> for ($($ident,)+) {
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
    impl_params_tuple!(A 0);
    impl_params_tuple!(A 0, B 1);
    impl_params_tuple!(A 0, B 1, C 2);
    impl_params_tuple!(A 0, B 1, C 2, D 3);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24);
    impl_params_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25);
};

// tuples of TransactionExtensions are valid so long as each item is a TransactionExtension.
macro_rules! impl_extensions_tuple {
    ($($ident:ident $index:tt),+) => {
        impl<Conf: Config, $($ident: TransactionExtension<Conf>),+>
            TransactionExtensions<Conf> for ($($ident,)+)
        where
            ($($ident::Params,)+): Params<Conf>,
        {
            type Params = ($($ident::Params,)+);

            fn new(client: &ClientState<Conf>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
                Ok((
                    $($ident::new(client, params.$index)?,)+
                ))
            }

            fn inject_signature(&mut self, account_id: &Conf::AccountId, signature: &Conf::Signature) {
                $(self.$index.inject_signature(account_id, signature);)+
            }
        }
    }
}

#[rustfmt::skip]
const _: () = {
    impl_extensions_tuple!(A 0);
    impl_extensions_tuple!(A 0, B 1);
    impl_extensions_tuple!(A 0, B 1, C 2);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24);
    impl_extensions_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25);
};