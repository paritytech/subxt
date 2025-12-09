// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use frame_decode::helpers::IntoEncodableValues;
use scale_encode::EncodeAsType;

/// For a given set of values that can be used as keys for a storage entry,
/// this is implemented for any prefixes of that set. ie if the keys `(A,B,C)`
/// would access a storage value, then `PrefixOf<(A,B,C)>` is implemented for
/// `(A,B)`, `(A,)` and `()`.
pub trait PrefixOf<Keys>: IntoEncodableValues {}

// If T impls PrefixOf<K>, &T impls PrefixOf<K>.
impl<K, T: PrefixOf<K>> PrefixOf<K> for &T {}

// Impls for tuples up to length 6 (storage maps rarely require more than 2 entries
// so it's very unlikely we'll ever need to go this deep).
impl<A> PrefixOf<(A,)> for () {}

impl<A, B> PrefixOf<(A, B)> for () {}
impl<A, B> PrefixOf<(A, B)> for (A,) where (A,): IntoEncodableValues {}

impl<A, B, C> PrefixOf<(A, B, C)> for () {}
impl<A, B, C> PrefixOf<(A, B, C)> for (A,) where (A,): IntoEncodableValues {}
impl<A, B, C> PrefixOf<(A, B, C)> for (A, B) where (A, B): IntoEncodableValues {}

impl<A, B, C, D> PrefixOf<(A, B, C, D)> for () {}
impl<A, B, C, D> PrefixOf<(A, B, C, D)> for (A,) where (A,): IntoEncodableValues {}
impl<A, B, C, D> PrefixOf<(A, B, C, D)> for (A, B) where (A, B): IntoEncodableValues {}
impl<A, B, C, D> PrefixOf<(A, B, C, D)> for (A, B, C) where (A, B, C): IntoEncodableValues {}

impl<A, B, C, D, E> PrefixOf<(A, B, C, D, E)> for () {}
impl<A, B, C, D, E> PrefixOf<(A, B, C, D, E)> for (A,) where (A,): IntoEncodableValues {}
impl<A, B, C, D, E> PrefixOf<(A, B, C, D, E)> for (A, B) where (A, B): IntoEncodableValues {}
impl<A, B, C, D, E> PrefixOf<(A, B, C, D, E)> for (A, B, C) where (A, B, C): IntoEncodableValues {}
impl<A, B, C, D, E> PrefixOf<(A, B, C, D, E)> for (A, B, C, D) where
    (A, B, C, D): IntoEncodableValues
{
}

impl<A, B, C, D, E, F> PrefixOf<(A, B, C, D, E, F)> for () {}
impl<A, B, C, D, E, F> PrefixOf<(A, B, C, D, E, F)> for (A,) where (A,): IntoEncodableValues {}
impl<A, B, C, D, E, F> PrefixOf<(A, B, C, D, E, F)> for (A, B) where (A, B): IntoEncodableValues {}
impl<A, B, C, D, E, F> PrefixOf<(A, B, C, D, E, F)> for (A, B, C) where
    (A, B, C): IntoEncodableValues
{
}
impl<A, B, C, D, E, F> PrefixOf<(A, B, C, D, E, F)> for (A, B, C, D) where
    (A, B, C, D): IntoEncodableValues
{
}
impl<A, B, C, D, E, F> PrefixOf<(A, B, C, D, E, F)> for (A, B, C, D, E) where
    (A, B, C, D, E): IntoEncodableValues
{
}

// Vecs are prefixes of vecs. The length is not statically known and so
// these would be given dynamically only, leaving the correct length to the user.
impl<T: EncodeAsType> PrefixOf<Vec<T>> for Vec<T> {}

// We don't use arrays in Subxt for storage entry access, but `IntoEncodableValues`
// supports them so let's allow impls which do use them to benefit too.
macro_rules! array_impl {
    ($n:literal: $($p:literal)+) => {
        $(
            impl <T: EncodeAsType> PrefixOf<[T; $n]> for [T; $p] {}
        )+
    }
}

array_impl!(1: 0);
array_impl!(2: 1 0);
array_impl!(3: 2 1 0);
array_impl!(4: 3 2 1 0);
array_impl!(5: 4 3 2 1 0);
array_impl!(6: 5 4 3 2 1 0);

#[cfg(test)]
mod test {
    use super::*;

    struct Test<Keys: IntoEncodableValues>(core::marker::PhantomData<Keys>);

    impl<Keys: IntoEncodableValues> Test<Keys> {
        fn new() -> Self {
            Test(core::marker::PhantomData)
        }
        fn accepts_prefix_of<P: PrefixOf<Keys>>(&self, keys: P) {
            let _encoder = keys.into_encodable_values();
        }
    }

    #[test]
    fn test_prefix_of() {
        // In real life we'd have a struct a bit like this:
        let t = Test::<(bool, String, u64)>::new();

        // And we'd want to be able to call some method like this:
        //// This shouldn't work:
        // t.accepts_prefix_of((true, String::from("hi"), 0));
        t.accepts_prefix_of(&(true, String::from("hi")));
        t.accepts_prefix_of((true, String::from("hi")));
        t.accepts_prefix_of((true,));
        t.accepts_prefix_of(());

        let t = Test::<[u64; 5]>::new();

        //// This shouldn't work:
        // t.accepts_prefix_of([0,1,2,3,4]);
        t.accepts_prefix_of([0, 1, 2, 3]);
        t.accepts_prefix_of([0, 1, 2, 3]);
        t.accepts_prefix_of([0, 1, 2]);
        t.accepts_prefix_of([0, 1]);
        t.accepts_prefix_of([0]);
        t.accepts_prefix_of([]);
    }
}
