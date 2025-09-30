// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/// A unit marker enum.
pub enum Yes {}
/// A unit marker enum.
pub enum Maybe {}
/// A unit marker enum.
pub enum No {}

/// This is implemented for [`Yes`], [`No`] and [`Maybe`] and
/// allows us to check at runtime which of these types is present. 
pub trait YesNoMaybe {
    /// [`Yes`]
    fn is_yes() -> bool { false }
    /// [`No`]
    fn is_no() -> bool { false }
    /// [`Maybe`]
    fn is_maybe() -> bool { false }
}

impl YesNoMaybe for Yes {
    fn is_yes() -> bool { true }
}
impl YesNoMaybe for No {
    fn is_no() -> bool { true }
}
impl YesNoMaybe for Maybe {
    fn is_maybe() -> bool { true }
}