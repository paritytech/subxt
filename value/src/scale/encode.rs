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

use crate::value_type::{Composite, Primitive, Value, ValueDef};
use bitvec::{ vec::BitVec, order::{ Lsb0, Msb0 } };
use codec::{Compact, Encode};
use super::{ScaleTypeDef as TypeDef};
use scale_info::{
	form::PortableForm, PortableRegistry, TypeDefArray, TypeDefBitSequence, TypeDefCompact, TypeDefComposite,
	TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant,
};
use super::type_id::TypeId;
use super::bit_sequence::{
    BitOrderTy, BitSequenceError, BitStoreTy, get_bitsequence_details
};

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
	#[error("Cannot encode bit sequence: {0}")]
    BitSequenceError(BitSequenceError),
	#[error("The type {0} cannot be compact encoded")]
    CannotCompactEncodeValueToType(u32),
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

fn encode_value<T, Id: Into<TypeId>>(
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
		TypeDef::Primitive(inner) => encode_primitive_value(value, inner, bytes),
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
    let composite = match value.value {
        ValueDef::Composite(composite) => {
            composite
        },
        ValueDef::Variant(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "sequence",
            })
        }
        ValueDef::BitSequence(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "sequence",
            })
        }
        ValueDef::Primitive(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "sequence",
            })
        }
    };

    // Encode the sequence length first:
    Compact(composite.len() as u64).encode_to(bytes);

    // We ignore names or not, and just expect each value to be
    // able to encode into the sequence type provided.
    let ty = ty.type_param();
    for value in composite.into_values() {
        encode_value(value, ty, types, bytes)?;
    }
    Ok(())
}

fn encode_array_value<T>(
    value: Value<T>,
	ty: &TypeDefArray<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    let composite = match value.value {
        ValueDef::Composite(composite) => {
            composite
        },
        ValueDef::Variant(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "array",
            })
        }
        ValueDef::BitSequence(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "array",
            })
        }
        ValueDef::Primitive(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "array",
            })
        }
    };

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
    let variant = match value.value {
        ValueDef::Composite(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "composite",
                expected: "variant",
            })
        },
        ValueDef::BitSequence(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "variant",
            })
        }
        ValueDef::Primitive(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "variant",
            })
        },
        ValueDef::Variant(variant) => {
            variant
        },
    };

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
    variant_type.index().encode_to(bytes);
    let field_value_pairs = variant_type.fields().iter().zip(variant.values.into_values());
    for (field, value) in field_value_pairs {
        encode_value(value, field.ty(), types, bytes)?;
    }

    Ok(())
}

// Attempt to convert a given primitive value into the integer type
// required, failing with an appropriate EncodeValueError if not successful.
macro_rules! primitive_to_integer {
    ($val:expr => $ty:ident) => {{
        let err = || EncodeValueError::WrongType {
            actual: "primitive",
            expected: std::any::type_name::<$ty>(),
        };
        let out: Result<$ty, _> = match $val {
            Primitive::U8(v) => v.try_into().map_err(|_| err()),
            Primitive::U16(v) => v.try_into().map_err(|_| err()),
            Primitive::U32(v) => v.try_into().map_err(|_| err()),
            Primitive::U64(v) => v.try_into().map_err(|_| err()),
            Primitive::U128(v) => v.try_into().map_err(|_| err()),
            Primitive::I8(v) => v.try_into().map_err(|_| err()),
            Primitive::I16(v) => v.try_into().map_err(|_| err()),
            Primitive::I32(v) => v.try_into().map_err(|_| err()),
            Primitive::I64(v) => v.try_into().map_err(|_| err()),
            Primitive::I128(v) => v.try_into().map_err(|_| err()),
            _ => Err(err()),
        };
        out
    }}
}

fn encode_primitive_value<T>(
    value: Value<T>,
	ty: &TypeDefPrimitive,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    let primitive = match value.value {
        ValueDef::Composite(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "composite",
                expected: "primitive",
            })
        },
        ValueDef::Variant(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "primitive",
            })
        }
        ValueDef::BitSequence(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "bit sequence",
                expected: "primitive",
            })
        }
        ValueDef::Primitive(primitive) => {
            primitive
        }
    };

    // Attempt to encode our value type into the expected shape.
    match ty {
        TypeDefPrimitive::Bool => {
            match primitive {
                Primitive::Bool(bool) => {
                    bool.encode_to(bytes);
                    Ok(())
                },
                _ => {
                    Err(EncodeValueError::WrongType {
                        actual: "primitive",
                        expected: "boolean",
                    })
                }
            }
        },
        TypeDefPrimitive::Char => {
            match primitive {
                Primitive::Char(c) => {
			        // Treat chars as u32's
                    (c as u32).encode_to(bytes);
                    Ok(())
                },
                _ => {
                    Err(EncodeValueError::WrongType {
                        actual: "primitive",
                        expected: "char",
                    })
                }
            }
        },
        TypeDefPrimitive::Str => {
            match primitive {
                Primitive::Str(s) => {
                    s.encode_to(bytes);
                    Ok(())
                },
                _ => {
                    Err(EncodeValueError::WrongType {
                        actual: "primitive",
                        expected: "string",
                    })
                }
            }
        },
        TypeDefPrimitive::I256 | TypeDefPrimitive::U256 => {
            match primitive {
                Primitive::I256(a) | Primitive::U256(a) => {
                    a.encode_to(bytes);
                    Ok(())
                },
                _ => {
                    Err(EncodeValueError::WrongType {
                        actual: "primitive",
                        expected: "[u8; 32] (a primitive u256/i256)",
                    })
                }
            }
        },
        TypeDefPrimitive::U8 => {
            primitive_to_integer!(primitive => u8)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::U16 => {
            primitive_to_integer!(primitive => u16)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::U32 => {
            primitive_to_integer!(primitive => u32)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::U64 => {
            primitive_to_integer!(primitive => u64)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::U128 => {
            primitive_to_integer!(primitive => u128)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::I8 => {
            primitive_to_integer!(primitive => i8)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::I16 => {
            primitive_to_integer!(primitive => i16)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::I32 => {
            primitive_to_integer!(primitive => i32)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::I64 => {
            primitive_to_integer!(primitive => i64)?.encode_to(bytes);
            Ok(())
        },
        TypeDefPrimitive::I128 => {
            primitive_to_integer!(primitive => i128)?.encode_to(bytes);
            Ok(())
        },

    }
}

fn encode_compact_value<T>(
    value: Value<T>,
	ty: &TypeDefCompact<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    // Types that are compact encodable:
    enum CompactTy {
        U8,
        U16,
        U32,
        U64,
        U128
    }

    // Resolve to a primitive type inside the compact encoded type (or fail if
    // we hit some type we wouldn't know how to work with).
    let inner_ty = {
        let mut inner_ty_id = ty.type_param().id();
        loop {
            let inner_ty = types
                .resolve(ty.type_param().id())
                .ok_or(EncodeValueError::TypeIdNotFound(ty.type_param().id()))?
                .type_def();

            match inner_ty {
                TypeDef::Composite(c) => {
                    if c.fields().len() == 1 {
                        inner_ty_id = c.fields()[0].ty().id();
                    } else {
                        return Err(EncodeValueError::CannotCompactEncodeValueToType(inner_ty_id));
                    }
                },
                TypeDef::Tuple(t) => {
                    if t.fields().len() == 1 {
                        inner_ty_id = t.fields()[0].id();
                    } else {
                        return Err(EncodeValueError::CannotCompactEncodeValueToType(inner_ty_id));
                    }
                },
                TypeDef::Primitive(primitive) => {
                    break match primitive {
                        // These are the primitives that we can compact encode:
                        TypeDefPrimitive::U8 => CompactTy::U8,
                        TypeDefPrimitive::U16 => CompactTy::U16,
                        TypeDefPrimitive::U32 => CompactTy::U32,
                        TypeDefPrimitive::U64 => CompactTy::U64,
                        TypeDefPrimitive::U128 => CompactTy::U128,
                        _ => {
                            return Err(EncodeValueError::CannotCompactEncodeValueToType(inner_ty_id));
                        }
                    }
                },
                TypeDef::Variant(_) |
                TypeDef::Sequence(_) |
                TypeDef::Array(_) |
                TypeDef::Compact(_) |
                TypeDef::BitSequence(_) => {
                    return Err(EncodeValueError::CannotCompactEncodeValueToType(inner_ty_id));
                },
            }
        }
    };

    // resolve to the innermost value that we have in the same way, expecting to get out
    // a single primitive value.
    let inner_primitive = {
        let mut value = value;
        loop {
            match value.value {
                ValueDef::Composite(c) => {
                    if c.len() == 1 {
                        value = c
                            .into_values()
                            .next()
                            .expect("length of 1; value should exist");
                    } else {
                        return Err(EncodeValueError::WrongType {
                            actual: "composite (length > 1)",
                            expected: "compact compatible value",
                        })
                    }
                },
                ValueDef::Primitive(primitive) => {
                    break primitive
                },
                ValueDef::Variant(_) => {
                    return Err(EncodeValueError::WrongType {
                        actual: "variant",
                        expected: "compact compatible value",
                    })
                },
                ValueDef::BitSequence(_) => {
                    return Err(EncodeValueError::WrongType {
                        actual: "bit sequence",
                        expected: "compact compatible value",
                    })
                }
            }
        }
    };

    // Try to compact encode the primitive type we have into the type asked for:
    match inner_ty {
        CompactTy::U8 => {
            let val = primitive_to_integer!(inner_primitive => u8)?;
            Compact(val).encode_to(bytes);
        },
        CompactTy::U16 => {
            let val = primitive_to_integer!(inner_primitive => u16)?;
            Compact(val).encode_to(bytes);
        },
        CompactTy::U32 => {
            let val = primitive_to_integer!(inner_primitive => u32)?;
            Compact(val).encode_to(bytes);
        },
        CompactTy::U64 => {
            let val = primitive_to_integer!(inner_primitive => u64)?;
            Compact(val).encode_to(bytes);
        },
        CompactTy::U128 => {
            let val = primitive_to_integer!(inner_primitive => u128)?;
            Compact(val).encode_to(bytes);
        }
    };

    Ok(())
}

fn encode_bitsequence_value<T>(
    value: Value<T>,
	ty: &TypeDefBitSequence<PortableForm>,
	types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeValueError> {
    // First, try to convert whatever we have into a vec of bools:
    let bools: Vec<bool> = match value.value {
        ValueDef::BitSequence(bits) => {
            bits.iter().by_vals().collect()
        },
        ValueDef::Composite(Composite::Unnamed(vals)) => {
            let mut bools = Vec::with_capacity(vals.len());
            for val in vals {
                match val.value {
                    ValueDef::Primitive(Primitive::Bool(b)) => {
                        bools.push(b)
                    },
                    _ => {
                        return Err(EncodeValueError::WrongType {
                            actual: "sequence",
                            expected: "sequence of booleans",
                        })
                    }
                }
            }
            bools
        },
        ValueDef::Composite(Composite::Named(_)) => {
            return Err(EncodeValueError::WrongType {
                actual: "named composite values",
                expected: "bit sequence",
            })
        }
        ValueDef::Variant(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "variant",
                expected: "bit sequence",
            })
        },
        ValueDef::Primitive(_) => {
            return Err(EncodeValueError::WrongType {
                actual: "primitive",
                expected: "bit sequence",
            })
        }
    };

    // next, turn those bools into a bit sequence of the expected shape.
    match get_bitsequence_details(ty, types).map_err(EncodeValueError::BitSequenceError)? {
        (BitOrderTy::U8, BitStoreTy::Lsb0) => {
            bools.into_iter().collect::<BitVec::<u8, Lsb0>>().encode_to(bytes);
        },
        (BitOrderTy::U16, BitStoreTy::Lsb0) => {
            bools.into_iter().collect::<BitVec::<u16, Lsb0>>().encode_to(bytes);
        },
        (BitOrderTy::U32, BitStoreTy::Lsb0) => {
            bools.into_iter().collect::<BitVec::<u32, Lsb0>>().encode_to(bytes);
        },
        (BitOrderTy::U64, BitStoreTy::Lsb0) => {
            bools.into_iter().collect::<BitVec::<u64, Lsb0>>().encode_to(bytes);
        },
        (BitOrderTy::U8, BitStoreTy::Msb0) => {
            bools.into_iter().collect::<BitVec::<u8, Msb0>>().encode_to(bytes);
        },
        (BitOrderTy::U16, BitStoreTy::Msb0) => {
            bools.into_iter().collect::<BitVec::<u16, Msb0>>().encode_to(bytes);
        },
        (BitOrderTy::U32, BitStoreTy::Msb0) => {
            bools.into_iter().collect::<BitVec::<u32, Msb0>>().encode_to(bytes);
        },
        (BitOrderTy::U64, BitStoreTy::Msb0) => {
            bools.into_iter().collect::<BitVec::<u64, Msb0>>().encode_to(bytes);
        },
    }

    Ok(())
}

