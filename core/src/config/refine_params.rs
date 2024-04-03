// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Refining params with values fetched from the chain

use crate::Config;

/// Data that can be used to refine the params of signed extensions.
pub struct RefineParamsData<T: Config> {
    account_nonce: u64,
    block_number: u64,
    block_hash: T::Hash,
}

impl<T: Config> RefineParamsData<T> {
    #[doc(hidden)]
    /// Creates a new [`RefineParamsData`] instance. Called from `subxt` when refining signed extensions.
    pub fn new(account_nonce: u64, block_number: u64, block_hash: T::Hash) -> Self {
        RefineParamsData {
            account_nonce,
            block_number,
            block_hash,
        }
    }

    /// account nonce for extrinsic author
    pub fn account_nonce(&self) -> u64 {
        self.account_nonce
    }

    /// latest finalized block number
    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    /// latest finalized block hash
    pub fn block_hash(&self) -> T::Hash {
        self.block_hash
    }
}

/// Types implementing [`RefineParams`] can be modified to reflect live information from the chain.
pub trait RefineParams<T: Config> {
    /// Refine params to an extrinsic. There is usually some notion of 'the param is already set/unset' in types implementing this trait.
    /// The refinement should most likely not affect cases where a param is in a 'is already set by the user' state.
    fn refine(&mut self, _data: &RefineParamsData<T>) {}
}

impl<T: Config> RefineParams<T> for () {}

macro_rules! impl_tuples {
    ($($ident:ident $index:tt),+) => {

        impl <T: Config, $($ident : RefineParams<T>),+> RefineParams<T> for ($($ident,)+){
            fn refine(&mut self, data: &RefineParamsData<T>) {
                $(self.$index.refine(data);)+
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
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, U 19);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, U 19, V 20);
};
