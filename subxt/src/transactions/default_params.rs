/// This trait is used to create default values for extrinsic params. We use this instead of
/// [`Default`] because we want to be able to support params which are tuples of more than 12
/// entries (which is the maximum tuple size Rust currently implements [`Default`] for on tuples),
/// given that we aren't far off having more than 12 transaction extensions already.
///
/// If you have params which are _not_ a tuple and which you'd like to be instantiated automatically
/// when calling [`crate::transactions::TransactionsClient::sign_and_submit_default()`] or
/// [`crate::transactions::TransactionsClient::sign_and_submit_then_watch_default()`], then you'll
/// need to implement this trait for them.
pub trait DefaultParams: Sized {
    /// Instantiate a default instance of the parameters.
    fn default_params() -> Self;
}

impl<const N: usize, P: Default> DefaultParams for [P; N] {
    fn default_params() -> Self {
        core::array::from_fn(|_| P::default())
    }
}

macro_rules! impl_default_params_for_tuple {
    ($($ident:ident),+) => {
        impl <$($ident : Default),+> DefaultParams for ($($ident,)+){
            fn default_params() -> Self {
                (
                    $($ident::default(),)+
                )
            }
        }
    }
}

#[rustfmt::skip]
const _: () = {
    impl_default_params_for_tuple!(A);
    impl_default_params_for_tuple!(A, B);
    impl_default_params_for_tuple!(A, B, C);
    impl_default_params_for_tuple!(A, B, C, D);
    impl_default_params_for_tuple!(A, B, C, D, E);
    impl_default_params_for_tuple!(A, B, C, D, E, F);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
    impl_default_params_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
};
