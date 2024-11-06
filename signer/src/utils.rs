// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![allow(unused_macros)]

/// Use like:
///
/// ```rust,ignore
/// once_static_cloned!{
///     /// Some documentation.
///     fn foo() -> Vec<u8> {
///         vec![1,2,3,4]
///     }
/// }
/// ```
///
/// Clones the item out of static storage. Useful if it
/// takes a while to create the item but cloning it is fairly cheap.
macro_rules! once_static_cloned {
    ($($(#[$attr:meta])* $vis:vis fn $name:ident() -> $ty:ty { $expr:expr } )+) => {
        $(
            $(#[$attr])*
            #[allow(missing_docs)]
            $vis fn $name() -> $ty {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "std")] {
                        static VAR: std::sync::OnceLock<$ty> = std::sync::OnceLock::new();
                        VAR.get_or_init(|| { $expr }).clone()
                    } else {
                        { $expr }
                    }
                }
            }
        )+
    };
}
