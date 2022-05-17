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

use super::ScaleTypeDef as TypeDef;
use scale_info::{
    form::PortableForm,
    PortableRegistry,
    TypeDefBitSequence,
    TypeDefPrimitive,
};

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum BitSequenceError {
    #[error("Bit order type {0} not found in registry")]
    BitOrderTypeNotFound(u32),
    #[error("Bit store type {0} not found in registry")]
    BitStoreTypeNotFound(u32),
    #[error("Bit order cannot be identified")]
    NoBitOrderIdent,
    #[error("Bit store type {0} is not supported")]
    StoreTypeNotSupported(String),
    #[error("Bit order type {0} is not supported")]
    OrderTypeNotSupported(String),
}

/// Obtain details about a bit sequence.
pub fn get_bitsequence_details(
    ty: &TypeDefBitSequence<PortableForm>,
    types: &PortableRegistry,
) -> Result<(BitOrderTy, BitStoreTy), BitSequenceError> {
    let bit_store_ty = ty.bit_store_type().id();
    let bit_order_ty = ty.bit_order_type().id();

    // What is the backing store type expected?
    let bit_store_def = types
        .resolve(bit_store_ty)
        .ok_or(BitSequenceError::BitStoreTypeNotFound(bit_store_ty))?
        .type_def();

    // What is the bit order type expected?
    let bit_order_def = types
        .resolve(bit_order_ty)
        .ok_or(BitSequenceError::BitOrderTypeNotFound(bit_order_ty))?
        .path()
        .ident()
        .ok_or(BitSequenceError::NoBitOrderIdent)?;

    let bit_order_out = match bit_store_def {
        TypeDef::Primitive(TypeDefPrimitive::U8) => Some(BitOrderTy::U8),
        TypeDef::Primitive(TypeDefPrimitive::U16) => Some(BitOrderTy::U16),
        TypeDef::Primitive(TypeDefPrimitive::U32) => Some(BitOrderTy::U32),
        TypeDef::Primitive(TypeDefPrimitive::U64) => Some(BitOrderTy::U64),
        _ => None,
    }
    .ok_or_else(|| {
        BitSequenceError::OrderTypeNotSupported(format!("{bit_store_def:?}"))
    })?;

    let bit_store_out = match &*bit_order_def {
        "Lsb0" => Some(BitStoreTy::Lsb0),
        "Msb0" => Some(BitStoreTy::Msb0),
        _ => None,
    }
    .ok_or(BitSequenceError::StoreTypeNotSupported(bit_order_def))?;

    Ok((bit_order_out, bit_store_out))
}

#[derive(Copy, Clone, PartialEq)]
pub enum BitStoreTy {
    Lsb0,
    Msb0,
}

#[derive(Copy, Clone, PartialEq)]
pub enum BitOrderTy {
    U8,
    U16,
    U32,
    U64,
}
