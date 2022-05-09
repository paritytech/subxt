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

use crate::value_type::{BitSequence, Composite, Primitive, Value, ValueDef, Variant};
use codec::{Compact, Decode};
use super::{ScaleType as Type, ScaleTypeDef as TypeDef};
use scale_info::{
	form::PortableForm, Field, PortableRegistry, TypeDefArray, TypeDefBitSequence, TypeDefCompact, TypeDefComposite,
	TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant,
};
use super::type_id::TypeId;

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum EncodeValueError {
    #[error("Composite type is the wrong length; expected length is {expected}, but got {actual}")]
    CompositeIsWrongLength { actual: usize, expected: usize },
    #[error("Variant type has the wrong number of fields; expected {expected} fields, but got {actual}")]
    VariantFieldLengthMismatch { actual: usize, expected: usize },
	#[error("Cannot find type with ID {0}")]
	TypeIdNotFound(u32),
	#[error("Value type is wrong; expected a {expected} type, but got a {actual} type")]
    WrongType { actual: &'static str, expected: &'static str },
	#[error("Variant {0} was not found")]
    VariantNotFound(String),
	// #[error("{0}")]
	// CodecError(#[from] codec::Error),
	// #[error("Could not find variant with index {0} in {1:?}")]
	// VariantNotFound(u8, scale_info::TypeDefVariant<PortableForm>),
	// #[error("Could not decode compact encoded type into {0:?}")]
	// CannotDecodeCompactIntoType(Type),
}

/// Attempt to SCALE Encode a Value according to the [`TypeId`] and
/// [`PortableRegistry`] provided.
pub fn encode_value_as_type<T, Id: Into<TypeId>>(
	value: Value<T>,
	ty_id: Id,
	types: &PortableRegistry,
) -> Result<Vec<u8>, EncodeValueError> {
    let mut bytes = Vec::new();
	encode_value(value, ty_id, types, &mut bytes)?;
    Ok(bytes)
}

pub fn encode_value<T, Id: Into<TypeId>>(
	value: Value<T>,
	ty_id: Id,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>
) -> Result<(), EncodeValueError> {
	let ty_id = ty_id.into();
	let ty = types.resolve(ty_id.id()).ok_or_else(|| EncodeValueError::TypeIdNotFound(ty_id.id()))?;

	match ty.type_def() {
		TypeDef::Composite(inner) => encode_composite_value(value, inner, types, bytes),
		TypeDef::Sequence(inner) => encode_sequence_value(value, inner, types, bytes),
		TypeDef::Array(inner) => encode_array_value(value, inner, types, bytes),
		TypeDef::Tuple(inner) => encode_tuple_value(value, inner, types, bytes),
		TypeDef::Variant(inner) => encode_variant_value(value, inner, types, bytes),
		TypeDef::Primitive(inner) => encode_primitive_value(value, inner, types, bytes),
		TypeDef::Compact(inner) => encode_compact_value(value, inner, types, bytes),
		TypeDef::BitSequence(inner) => encode_bitsequence_value(value, inner, types, bytes),
	}?;

	Ok(())
}

fn encode_composite_value<T>(
    value: Value<T>,
	ty: &TypeDefComposite<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    match value.value {
        ValueDef::Composite(composite) => {
            if composite.len() != ty.fields().len() {
                return Err(EncodeValueError::CompositeIsWrongLength {
                    actual: composite.len(),
                    expected: ty.fields().len()
                })
            }
            // We don't care whether the fields are named or unnamed
            // as long as we have the number of them that we expect..
            let field_value_pairs = ty.fields().iter().zip(composite.into_values());
            for (field, value) in field_value_pairs {
                encode_value(value, field.ty(), types, bytes)?;
            }
            Ok(())
        },
        ValueDef::Variant(_) => {
            if ty.fields().len() == 1 {
                // A 1-field composite type? try encoding inner content then.
                encode_value(value, ty.fields()[0].ty(), types, bytes)
            } else {
                Err(EncodeValueError::WrongType {
                    actual: "variant",
                    expected: "composite",
                })
            }
        }
        ValueDef::BitSequence(_) => {
            if ty.fields().len() == 1 {
                // A 1-field composite type? try encoding inner content then.
                encode_value(value, ty.fields()[0].ty(), types, bytes)
            } else {
                Err(EncodeValueError::WrongType {
                    actual: "bit sequence",
                    expected: "composite",
                })
            }
        }
        ValueDef::Primitive(_) => {
            if ty.fields().len() == 1 {
                // A 1-field composite type? try encoding inner content then.
                encode_value(value, ty.fields()[0].ty(), types, bytes)
            } else {
                Err(EncodeValueError::WrongType {
                    actual: "primitive",
                    expected: "composite",
                })
            }
        }
    }
}

fn encode_sequence_value<T>(
    value: Value<T>,
	ty: &TypeDefSequence<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    match value.value {
        ValueDef::Composite(composite) => {
            // We ignore names or not, and just expect each value to be
            // able to encode into the sequence type provided.
            let ty = ty.type_param();
            for value in composite.into_values() {
                encode_value(value, ty, types, bytes)?;
            }
            Ok(())
        },
        ValueDef::Variant(_) => {
            Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "sequence",
            })
        }
        ValueDef::BitSequence(_) => {
            Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "sequence",
            })
        }
        ValueDef::Primitive(_) => {
            Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "sequence",
            })
        }
    }
}

fn encode_array_value<T>(
    value: Value<T>,
	ty: &TypeDefArray<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    match value.value {
        ValueDef::Composite(composite) => {
            let arr_len = ty.len() as usize;
            if composite.len() != arr_len {
                return Err(EncodeValueError::CompositeIsWrongLength {
                    actual: composite.len(),
                    expected: arr_len,
                })
            }
            // We ignore names or not, and just expect each value to be
            // able to encode into the array type provided.
            let ty = ty.type_param();
            for value in composite.into_values() {
                encode_value(value, ty, types, bytes)?;
            }
            Ok(())
        },
        ValueDef::Variant(_) => {
            Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "array",
            })
        }
        ValueDef::BitSequence(_) => {
            Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "array",
            })
        }
        ValueDef::Primitive(_) => {
            Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "array",
            })
        }
    }
}

fn encode_tuple_value<T>(
    value: Value<T>,
	ty: &TypeDefTuple<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    match value.value {
        ValueDef::Composite(composite) => {
            if composite.len() != ty.fields().len() {
                return Err(EncodeValueError::CompositeIsWrongLength {
                    actual: composite.len(),
                    expected: ty.fields().len()
                })
            }
            // We don't care whether the fields are named or unnamed
            // as long as we have the number of them that we expect..
            let field_value_pairs = ty.fields().iter().zip(composite.into_values());
            for (ty, value) in field_value_pairs {
                encode_value(value, ty, types, bytes)?;
            }
            Ok(())
        },
        ValueDef::Variant(_) => {
            if ty.fields().len() == 1 {
                // A 1-field tuple? try encoding inner content then.
                encode_value(value, ty.fields()[0], types, bytes)
            } else {
                Err(EncodeValueError::WrongType {
                    actual: "variant",
                    expected: "tuple",
                })
            }
        }
        ValueDef::BitSequence(_) => {
            if ty.fields().len() == 1 {
                // A 1-field tuple? try encoding inner content then.
                encode_value(value, ty.fields()[0], types, bytes)
            } else {
                Err(EncodeValueError::WrongType {
                    actual: "bit sequence",
                    expected: "tuple",
                })
            }
        }
        ValueDef::Primitive(_) => {
            if ty.fields().len() == 1 {
                // A 1-field tuple? try encoding inner content then.
                encode_value(value, ty.fields()[0], types, bytes)
            } else {
                Err(EncodeValueError::WrongType {
                    actual: "primitive",
                    expected: "tuple",
                })
            }
        }
    }
}

fn encode_variant_value<T>(
    value: Value<T>,
	ty: &TypeDefVariant<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    match value.value {
        ValueDef::Composite(_) => {
            Err(EncodeValueError::WrongType {
                actual: "composite",
                expected: "variant",
            })
        },
        ValueDef::Variant(variant) => {
            let variant_type = ty
                .variants()
                .iter()
                .find(|v| v.name() == &variant.name);

            let variant_type = match variant_type {
                None => return Err(EncodeValueError::VariantNotFound(variant.name)),
                Some(v) => v
            };

            if variant_type.fields().len() != variant.values.len() {
                return Err(EncodeValueError::VariantFieldLengthMismatch {
                    actual: variant.values.len(),
                    expected: variant_type.fields().len(),
                });
            }

            // Encode the variant index into our bytes, followed by the fields.
            let idx = variant_type.index();
            bytes.push(idx);

            let field_value_pairs = variant_type.fields().iter().zip(variant.values.into_values());
            for (field, value) in field_value_pairs {
                encode_value(value, field.ty(), types, bytes)?;
            }

            Ok(())
        }
        ValueDef::BitSequence(_) => {
            Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "variant",
            })
        }
        ValueDef::Primitive(_) => {
            Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "variant",
            })
        }
    }
}

fn encode_primitive_value<T>(
    value: Value<T>,
	ty: &TypeDefPrimitive,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    match value.value {
        ValueDef::Composite(_) => {
            Err(EncodeValueError::WrongType {
                actual: "composite",
                expected: "primitive",
            })
        },
        ValueDef::Variant(_) => {
            Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "primitive",
            })
        }
        ValueDef::BitSequence(_) => {
            Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "primitive",
            })
        }
        ValueDef::Primitive(_) => {
            Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "primitive",
            })
        }
    }
}

fn encode_compact_value<T>(
    value: Value<T>,
	ty: &TypeDefCompact<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    match value.value {
        ValueDef::Composite(_) => {
            Err(EncodeValueError::WrongType {
                actual: "composite",
                expected: "compact",
            })
        },
        ValueDef::Variant(_) => {
            Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "compact",
            })
        }
        ValueDef::BitSequence(_) => {
            Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "compact",
            })
        }
        ValueDef::Primitive(_) => {
            Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "compact",
            })
        }
    }
}

fn encode_bitsequence_value<T>(
    value: Value<T>,
	ty: &TypeDefBitSequence<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    match value.value {
        ValueDef::Composite(_) => {
            Err(EncodeValueError::WrongType {
                actual: "composite",
                expected: "bit sequence",
            })
        },
        ValueDef::Variant(_) => {
            Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "bit sequence",
            })
        }
        ValueDef::BitSequence(_) => {
            Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "bit sequence",
            })
        }
        ValueDef::Primitive(_) => {
            Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "bit sequence",
            })
        }
    }
}

