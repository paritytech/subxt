// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/// Use like:
///
/// ```rust,ignore
/// once_static!{
///     /// Some documentation.
///     fn foo() -> Vec<u8> {
///         vec![1,2,3,4]
///     }
/// }
/// ```
macro_rules! once_static {
    ($($(#[$doc:meta])* $vis:vis fn $name:ident() -> $ty:ty { $expr:expr } )+) => {
        $(
            $(#[$doc])*
            $vis fn $name() -> &'static $ty {
                static VAR: std::sync::OnceLock<$ty> = std::sync::OnceLock::new();
                VAR.get_or_init(|| { $expr })
            }
        )+
    };
}