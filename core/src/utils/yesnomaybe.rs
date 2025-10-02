// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/// A unit marker enum.
pub enum Yes {}
/// A unit marker enum.
pub enum Maybe {}
/// A unit marker enum.
pub enum No {}

/// This is implemented for [`Yes`] and [`No`] and
/// allows us to check at runtime which of these types is present. 
pub trait YesNo {
    /// [`Yes`]
    fn is_yes() -> bool { false }
    /// [`No`]
    fn is_no() -> bool { false }
}

impl YesNo for Yes {
    fn is_yes() -> bool { true }
}
impl YesNo for No {
    fn is_no() -> bool { true }
}

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

/// This is implemented for [`No`] and [`Maybe`] and
/// allows us to check at runtime which of these types is present. 
pub trait NoMaybe {
    /// [`No`]
    fn is_no() -> bool { false }
    /// [`Maybe`]
    fn is_maybe() -> bool { false }
}

impl NoMaybe for No {
    fn is_no() -> bool { true }
}
impl NoMaybe for Maybe {
    fn is_maybe() -> bool { true }
}