// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

mod bit_sequence;
mod decode;
mod encode;
mod type_id;

/// The portable version of [`scale_info::Type`]
type ScaleType = scale_info::Type<scale_info::form::PortableForm>;

/// The portable version of a [`scale_info`] type ID.
type ScaleTypeId = scale_info::interner::UntrackedSymbol<std::any::TypeId>; // equivalent to: <scale_info::form::PortableForm as scale_info::form::Form>::Type;

/// The portable version of [`scale_info::TypeDef`]
type ScaleTypeDef = scale_info::TypeDef<scale_info::form::PortableForm>;

pub use bit_sequence::BitSequenceError;
pub use decode::{
    decode_value_as_type,
    DecodeError,
};
pub use encode::{
    encode_value_as_type,
    EncodeError,
};

pub use type_id::TypeId;
