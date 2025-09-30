// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/// A unit marker enum.
pub enum Yes {}
/// A unit marker enum.
pub enum Maybe {}

/// This is implemented for [`Yes`] and [`Maybe`] and
/// allows us to check at runtime which of these types is present. 
pub trait YesMaybe {
    /// [`Yes`]
    fn is_yes() -> bool { false }
    /// [`Maybe`]
    fn is_maybe() -> bool { false }
}

impl YesMaybe for Yes {
    fn is_yes() -> bool { true }
}
impl YesMaybe for Maybe {
    fn is_maybe() -> bool { true }
}