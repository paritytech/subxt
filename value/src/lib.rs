// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-desub.
//
// substrate-desub is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// substrate-desub is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-desub.  If not, see <http://www.gnu.org/licenses/>.

/*!
This crate exposes the [`Value`] type and related subtypes, which are used as the runtime
representations of SCALE encoded data (much like `serde_json::Value` is a runtime representation
of JSON data).
*/

mod value_type;
#[cfg(feature = "serde")]
mod serde_impls;
#[cfg(feature = "serde")]
pub use serde_impls::{ DeserializeError, deserialize_from_value };

pub use value_type::{
    BitSequence,
    Composite,
    Primitive,
    Value,
    ValueDef,
    Variant,
};