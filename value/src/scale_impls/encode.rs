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

use super::{
    bit_sequence::{
        get_bitsequence_details,
        BitOrderTy,
        BitSequenceError,
        BitStoreTy,
    },
    type_id::TypeId,
    ScaleTypeDef as TypeDef,
};
use crate::value_type::{
    Composite,
    Primitive,
    Value,
    ValueDef,
    Variant,
};
use bitvec::{
    order::{
        Lsb0,
        Msb0,
    },
    vec::BitVec,
};
use codec::{
    Compact,
    Encode,
};
use scale_info::{
    form::PortableForm,
    Field,
    PortableRegistry,
    TypeDefArray,
    TypeDefBitSequence,
    TypeDefCompact,
    TypeDefComposite,
    TypeDefPrimitive,
    TypeDefSequence,
    TypeDefTuple,
    TypeDefVariant,
};

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum EncodeError<T> {
    #[error("Composite type is the wrong length; expected length is {expected_len}, but got {}", actual.len())]
    CompositeIsWrongLength {
        actual: Composite<T>,
        expected: TypeId,
        expected_len: usize,
    },
    #[error("The composite {actual:?} is not the same shape as the type we're trying to encode to ({expected})")]
    CompositeIsWrongShape {
        actual: Composite<T>,
        expected: TypeId,
    },
    #[error("Variant type has the wrong number of fields; expected {expected_len} fields, but got {}", actual.values.len())]
    VariantFieldLengthMismatch {
        actual: Variant<T>,
        expected_len: usize,
    },
    #[error("Cannot find type with ID {0}")]
    TypeIdNotFound(TypeId),
    #[error("Value type is wrong; expected type ID {expected}, but got value {actual:?}, which could not be coerced into it")]
    WrongType { actual: Value<T>, expected: TypeId },
    #[error("Variant {} was not found", actual.name)]
    VariantNotFound {
        actual: Variant<T>,
        expected: TypeId,
    },
    #[error("The variant field {missing_field_name} is present on the type we're trying to encode to but hasn't been provided")]
    VariantFieldIsMissing {
        missing_field_name: String,
        expected: TypeId,
    },
    #[error("Cannot encode bit sequence: {0}")]
    BitSequenceError(BitSequenceError),
    #[error("The type {0} cannot be compact encoded")]
    CannotCompactEncode(TypeId),
}

/// Attempt to SCALE Encode a Value according to the [`TypeId`] and
/// [`PortableRegistry`] provided.
pub fn encode_value_as_type<T, Id: Into<TypeId>>(
    value: Value<T>,
    ty_id: Id,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    let ty_id = ty_id.into();
    let ty = types
        .resolve(ty_id.id())
        .ok_or(EncodeError::TypeIdNotFound(ty_id))?;

    match ty.type_def() {
        TypeDef::Composite(inner) => {
            encode_composite_value(value, ty_id, inner, types, bytes)
        }
        TypeDef::Sequence(inner) => {
            encode_sequence_value(value, ty_id, inner, types, bytes)
        }
        TypeDef::Array(inner) => encode_array_value(value, ty_id, inner, types, bytes),
        TypeDef::Tuple(inner) => encode_tuple_value(value, ty_id, inner, types, bytes),
        TypeDef::Variant(inner) => {
            encode_variant_value(value, ty_id, inner, types, bytes)
        }
        TypeDef::Primitive(inner) => encode_primitive_value(value, ty_id, inner, bytes),
        TypeDef::Compact(inner) => {
            encode_compact_value(value, ty_id, inner, types, bytes)
        }
        TypeDef::BitSequence(inner) => {
            encode_bitsequence_value(value, ty_id, inner, types, bytes)
        }
    }?;

    Ok(())
}

fn encode_composite_value<T>(
    value: Value<T>,
    type_id: TypeId,
    ty: &TypeDefComposite<PortableForm>,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    match value.value {
        ValueDef::Composite(composite) => {
            encode_composite_fields(composite, ty.fields(), type_id, types, bytes)
        }
        _ => {
            if ty.fields().len() == 1 {
                // A 1-field composite type? try encoding inner content then.
                encode_value_as_type(value, ty.fields()[0].ty(), types, bytes)
            } else {
                Err(EncodeError::WrongType {
                    actual: value,
                    expected: type_id,
                })
            }
        }
    }
}

fn encode_sequence_value<T>(
    value: Value<T>,
    type_id: TypeId,
    ty: &TypeDefSequence<PortableForm>,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    match value.value {
        // Let's see whether our composite type is the right length,
        // and try to encode each inner value into what the sequence wants.
        ValueDef::Composite(c) => {
            // Compact encoded length comes first
            Compact(c.len() as u64).encode_to(bytes);
            let ty = ty.type_param();
            for value in c.into_values() {
                encode_value_as_type(value, ty, types, bytes)?;
            }
        }
        // As a special case, primitive U256/I256s are arrays, and may be compatible
        // with the sequence type being asked for, too.
        ValueDef::Primitive(Primitive::I256(a) | Primitive::U256(a)) => {
            // Compact encoded length comes first
            Compact(a.len() as u64).encode_to(bytes);
            let ty = ty.type_param();
            for val in a {
                if encode_value_as_type(Value::u8(val), ty, types, bytes).is_err() {
                    return Err(EncodeError::WrongType {
                        actual: value,
                        expected: type_id,
                    })
                }
            }
        }
        _ => {
            return Err(EncodeError::WrongType {
                actual: value,
                expected: type_id,
            })
        }
    };
    Ok(())
}

fn encode_array_value<T>(
    value: Value<T>,
    type_id: TypeId,
    ty: &TypeDefArray<PortableForm>,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    match value.value {
        // Let's see whether our composite type is the right length,
        // and try to encode each inner value into what the array wants.
        ValueDef::Composite(c) => {
            let arr_len = ty.len() as usize;
            if c.len() != arr_len {
                return Err(EncodeError::CompositeIsWrongLength {
                    actual: c,
                    expected: type_id,
                    expected_len: arr_len,
                })
            }

            let ty = ty.type_param();
            for value in c.into_values() {
                encode_value_as_type(value, ty, types, bytes)?;
            }
        }
        // As a special case, primitive U256/I256s are arrays, and may be compatible
        // with the array type being asked for, too.
        ValueDef::Primitive(Primitive::I256(a) | Primitive::U256(a)) => {
            let arr_len = ty.len() as usize;
            if a.len() != arr_len {
                return Err(EncodeError::WrongType {
                    actual: value,
                    expected: type_id,
                })
            }

            let ty = ty.type_param();
            for val in a {
                if encode_value_as_type(Value::u8(val), ty, types, bytes).is_err() {
                    return Err(EncodeError::WrongType {
                        actual: value,
                        expected: type_id,
                    })
                }
            }
        }
        _ => {
            return Err(EncodeError::WrongType {
                actual: value,
                expected: type_id,
            })
        }
    };
    Ok(())
}

fn encode_tuple_value<T>(
    value: Value<T>,
    type_id: TypeId,
    ty: &TypeDefTuple<PortableForm>,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    match value.value {
        ValueDef::Composite(composite) => {
            if composite.len() != ty.fields().len() {
                return Err(EncodeError::CompositeIsWrongLength {
                    actual: composite,
                    expected: type_id,
                    expected_len: ty.fields().len(),
                })
            }
            // We don't care whether the fields are named or unnamed
            // as long as we have the number of them that we expect..
            let field_value_pairs = ty.fields().iter().zip(composite.into_values());
            for (ty, value) in field_value_pairs {
                encode_value_as_type(value, ty, types, bytes)?;
            }
            Ok(())
        }
        _ => {
            if ty.fields().len() == 1 {
                // A 1-field tuple? try encoding inner content then.
                encode_value_as_type(value, ty.fields()[0], types, bytes)
            } else {
                Err(EncodeError::WrongType {
                    actual: value,
                    expected: type_id,
                })
            }
        }
    }
}

fn encode_variant_value<T>(
    value: Value<T>,
    type_id: TypeId,
    ty: &TypeDefVariant<PortableForm>,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    let variant = match value.value {
        ValueDef::Variant(variant) => variant,
        _ => {
            return Err(EncodeError::WrongType {
                actual: value,
                expected: type_id,
            })
        }
    };

    let variant_type = ty.variants().iter().find(|v| v.name() == &variant.name);

    let variant_type = match variant_type {
        None => {
            return Err(EncodeError::VariantNotFound {
                actual: variant,
                expected: type_id,
            })
        }
        Some(v) => v,
    };

    variant_type.index().encode_to(bytes);
    encode_composite_fields(variant.values, variant_type.fields(), type_id, types, bytes)
}

fn encode_composite_fields<T>(
    composite: Composite<T>,
    fields: &[Field<PortableForm>],
    type_id: TypeId,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    if fields.len() != composite.len() {
        return Err(EncodeError::CompositeIsWrongShape {
            actual: composite,
            expected: type_id,
        })
    }

    // 0 length? Nothing more to do!
    if composite.is_empty() {
        return Ok(())
    }

    // Does the type we're encoding to have named fields or not?
    let is_named = fields[0].name().is_some();

    match (composite, is_named) {
        (Composite::Named(mut values), true) => {
            // Match up named values with those of the type we're encoding to.
            for field in fields.iter() {
                let field_name =
                    field.name().expect("field should be named; checked above");
                let value = values
                    .iter()
                    .position(|(n, _)| field_name == n)
                    .map(|idx| values.swap_remove(idx).1);

                match value {
                    Some(value) => {
                        encode_value_as_type(value, field.ty(), types, bytes)?;
                    }
                    None => {
                        return Err(EncodeError::VariantFieldIsMissing {
                            expected: type_id,
                            missing_field_name: field_name.clone(),
                        })
                    }
                }
            }

            Ok(())
        }
        (Composite::Unnamed(values), false) => {
            // Expect values in correct order only and encode.
            for (field, value) in fields.iter().zip(values) {
                encode_value_as_type(value, field.ty(), types, bytes)?;
            }
            Ok(())
        }
        (values, _) => {
            // We expect named/unnamed fields and need the opposite.
            Err(EncodeError::CompositeIsWrongShape {
                actual: values,
                expected: type_id,
            })
        }
    }
}

// Attempt to convert a given primitive value into the integer type
// required, failing with an appropriate EncodeValueError if not successful.
macro_rules! primitive_to_integer {
    ($id:ident, $prim:ident, $context:expr => $ty:ident) => {{
        macro_rules! err {
            () => {
                EncodeError::WrongType {
                    actual: Value {
                        context: $context,
                        value: ValueDef::Primitive($prim),
                    },
                    expected: $id,
                }
            };
        }
        let out: Result<$ty, _> = match $prim {
            Primitive::U8(v) => v.try_into().map_err(|_| err!()),
            Primitive::U16(v) => v.try_into().map_err(|_| err!()),
            Primitive::U32(v) => v.try_into().map_err(|_| err!()),
            Primitive::U64(v) => v.try_into().map_err(|_| err!()),
            Primitive::U128(v) => v.try_into().map_err(|_| err!()),
            Primitive::I8(v) => v.try_into().map_err(|_| err!()),
            Primitive::I16(v) => v.try_into().map_err(|_| err!()),
            Primitive::I32(v) => v.try_into().map_err(|_| err!()),
            Primitive::I64(v) => v.try_into().map_err(|_| err!()),
            Primitive::I128(v) => v.try_into().map_err(|_| err!()),
            // Treat chars as u32s to mirror what we do for decoding:
            Primitive::Char(v) => (v as u32).try_into().map_err(|_| err!()),
            _ => Err(err!()),
        };
        out
    }};
}

fn encode_primitive_value<T>(
    value: Value<T>,
    type_id: TypeId,
    ty: &TypeDefPrimitive,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    let primitive = match value.value {
        ValueDef::Primitive(primitive) => primitive,
        _ => {
            return Err(EncodeError::WrongType {
                actual: value,
                expected: type_id,
            })
        }
    };

    // Attempt to encode our value type into the expected shape.
    match (ty, primitive) {
        (TypeDefPrimitive::Bool, Primitive::Bool(bool)) => {
            bool.encode_to(bytes);
        }
        (TypeDefPrimitive::Char, Primitive::Char(c)) => {
            // Treat chars as u32's
            (c as u32).encode_to(bytes);
        }
        (TypeDefPrimitive::Str, Primitive::String(s)) => {
            s.encode_to(bytes);
        }
        (TypeDefPrimitive::I256, Primitive::I256(a)) => {
            a.encode_to(bytes);
        }
        (TypeDefPrimitive::U256, Primitive::U256(a)) => {
            a.encode_to(bytes);
        }
        (TypeDefPrimitive::U8, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => u8)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::U16, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => u16)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::U32, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => u32)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::U64, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => u64)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::U128, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => u128)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::I8, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => i8)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::I16, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => i16)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::I32, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => i32)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::I64, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => i64)?
                .encode_to(bytes);
        }
        (TypeDefPrimitive::I128, primitive) => {
            primitive_to_integer!(type_id, primitive, value.context => i128)?
                .encode_to(bytes);
        }
        (_, primitive) => {
            return Err(EncodeError::WrongType {
                // Reconstruct a Value to give back:
                actual: Value {
                    context: value.context,
                    value: ValueDef::Primitive(primitive),
                },
                expected: type_id,
            })
        }
    }
    Ok(())
}

fn encode_compact_value<T>(
    value: Value<T>,
    type_id: TypeId,
    ty: &TypeDefCompact<PortableForm>,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    // Types that are compact encodable:
    enum CompactTy {
        U8,
        U16,
        U32,
        U64,
        U128,
    }

    // Resolve to a primitive type inside the compact encoded type (or fail if
    // we hit some type we wouldn't know how to work with).
    let mut inner_ty_id = ty.type_param().id();
    let inner_ty = loop {
        let inner_ty = types
            .resolve(inner_ty_id)
            .ok_or_else(|| EncodeError::TypeIdNotFound(inner_ty_id.into()))?
            .type_def();

        match inner_ty {
            TypeDef::Composite(c) => {
                if c.fields().len() == 1 {
                    inner_ty_id = c.fields()[0].ty().id();
                } else {
                    return Err(EncodeError::CannotCompactEncode(inner_ty_id.into()))
                }
            }
            TypeDef::Tuple(t) => {
                if t.fields().len() == 1 {
                    inner_ty_id = t.fields()[0].id();
                } else {
                    return Err(EncodeError::CannotCompactEncode(inner_ty_id.into()))
                }
            }
            TypeDef::Primitive(primitive) => {
                break match primitive {
                    // These are the primitives that we can compact encode:
                    TypeDefPrimitive::U8 => CompactTy::U8,
                    TypeDefPrimitive::U16 => CompactTy::U16,
                    TypeDefPrimitive::U32 => CompactTy::U32,
                    TypeDefPrimitive::U64 => CompactTy::U64,
                    TypeDefPrimitive::U128 => CompactTy::U128,
                    _ => return Err(EncodeError::CannotCompactEncode(inner_ty_id.into())),
                }
            }
            TypeDef::Variant(_)
            | TypeDef::Sequence(_)
            | TypeDef::Array(_)
            | TypeDef::Compact(_)
            | TypeDef::BitSequence(_) => {
                return Err(EncodeError::CannotCompactEncode(inner_ty_id.into()))
            }
        }
    };

    // resolve to the innermost value that we have in the same way, expecting to get out
    // a single primitive value.
    let mut value = value;
    let inner_primitive = {
        loop {
            match value.value {
                ValueDef::Composite(c) => {
                    if c.len() == 1 {
                        value = c
                            .into_values()
                            .next()
                            .expect("length of 1; value should exist");
                    } else {
                        return Err(EncodeError::WrongType {
                            actual: Value {
                                context: value.context,
                                value: ValueDef::Composite(c),
                            },
                            expected: inner_ty_id.into(),
                        })
                    }
                }
                ValueDef::Primitive(primitive) => break primitive,
                ValueDef::Variant(_) | ValueDef::BitSequence(_) => {
                    return Err(EncodeError::WrongType {
                        actual: value,
                        expected: inner_ty_id.into(),
                    })
                }
            }
        }
    };

    // Try to compact encode the primitive type we have into the type asked for:
    match inner_ty {
        CompactTy::U8 => {
            let val =
                primitive_to_integer!(type_id, inner_primitive, value.context => u8)?;
            Compact(val).encode_to(bytes);
        }
        CompactTy::U16 => {
            let val =
                primitive_to_integer!(type_id, inner_primitive, value.context => u16)?;
            Compact(val).encode_to(bytes);
        }
        CompactTy::U32 => {
            let val =
                primitive_to_integer!(type_id, inner_primitive, value.context => u32)?;
            Compact(val).encode_to(bytes);
        }
        CompactTy::U64 => {
            let val =
                primitive_to_integer!(type_id, inner_primitive, value.context => u64)?;
            Compact(val).encode_to(bytes);
        }
        CompactTy::U128 => {
            let val =
                primitive_to_integer!(type_id, inner_primitive, value.context => u128)?;
            Compact(val).encode_to(bytes);
        }
    };

    Ok(())
}

fn encode_bitsequence_value<T>(
    value: Value<T>,
    type_id: TypeId,
    ty: &TypeDefBitSequence<PortableForm>,
    types: &PortableRegistry,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeError<T>> {
    // First, try to convert whatever we have into a vec of bools:
    let bools: Vec<bool> = match value.value {
        ValueDef::BitSequence(bits) => bits.iter().by_vals().collect(),
        ValueDef::Composite(Composite::Unnamed(vals)) => {
            let mut bools = Vec::with_capacity(vals.len());
            for val in vals {
                match val.value {
                    ValueDef::Primitive(Primitive::Bool(b)) => bools.push(b),
                    _ => {
                        return Err(EncodeError::WrongType {
                            actual: val,
                            expected: type_id,
                        })
                    }
                }
            }
            bools
        }
        _ => {
            return Err(EncodeError::WrongType {
                actual: value,
                expected: type_id,
            })
        }
    };

    // next, turn those bools into a bit sequence of the expected shape.
    match get_bitsequence_details(ty, types).map_err(EncodeError::BitSequenceError)? {
        (BitOrderTy::U8, BitStoreTy::Lsb0) => {
            bools
                .into_iter()
                .collect::<BitVec<u8, Lsb0>>()
                .encode_to(bytes);
        }
        (BitOrderTy::U16, BitStoreTy::Lsb0) => {
            bools
                .into_iter()
                .collect::<BitVec<u16, Lsb0>>()
                .encode_to(bytes);
        }
        (BitOrderTy::U32, BitStoreTy::Lsb0) => {
            bools
                .into_iter()
                .collect::<BitVec<u32, Lsb0>>()
                .encode_to(bytes);
        }
        (BitOrderTy::U64, BitStoreTy::Lsb0) => {
            bools
                .into_iter()
                .collect::<BitVec<u64, Lsb0>>()
                .encode_to(bytes);
        }
        (BitOrderTy::U8, BitStoreTy::Msb0) => {
            bools
                .into_iter()
                .collect::<BitVec<u8, Msb0>>()
                .encode_to(bytes);
        }
        (BitOrderTy::U16, BitStoreTy::Msb0) => {
            bools
                .into_iter()
                .collect::<BitVec<u16, Msb0>>()
                .encode_to(bytes);
        }
        (BitOrderTy::U32, BitStoreTy::Msb0) => {
            bools
                .into_iter()
                .collect::<BitVec<u32, Msb0>>()
                .encode_to(bytes);
        }
        (BitOrderTy::U64, BitStoreTy::Msb0) => {
            bools
                .into_iter()
                .collect::<BitVec<u64, Msb0>>()
                .encode_to(bytes);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    /// Given a type definition, return the PortableType and PortableRegistry
    /// that our decode functions expect.
    fn make_type<T: scale_info::TypeInfo + 'static>() -> (TypeId, PortableRegistry) {
        let m = scale_info::MetaType::new::<T>();
        let mut types = scale_info::Registry::new();
        let id = types.register_type(&m);
        let portable_registry: PortableRegistry = types.into();

        (id.into(), portable_registry)
    }

    // Attempt to SCALE encode a Value and expect it to match the standard Encode impl for the second param given.
    fn assert_can_encode_to_type<T: Encode + scale_info::TypeInfo + 'static>(
        value: Value<()>,
        ty: T,
    ) {
        let expected = ty.encode();
        let mut buf = Vec::new();

        let (ty_id, types) = make_type::<T>();

        encode_value_as_type(value, ty_id, &types, &mut buf)
            .expect("error encoding value as type");
        assert_eq!(expected, buf);
    }

    #[test]
    fn can_encode_basic_primitive_values() {
        assert_can_encode_to_type(Value::i8(123), 123i8);
        assert_can_encode_to_type(Value::i16(123), 123i16);
        assert_can_encode_to_type(Value::i32(123), 123i32);
        assert_can_encode_to_type(Value::i64(123), 123i64);
        assert_can_encode_to_type(Value::i128(123), 123i128);

        assert_can_encode_to_type(Value::u8(123), 123u8);
        assert_can_encode_to_type(Value::u16(123), 123u16);
        assert_can_encode_to_type(Value::u32(123), 123u32);
        assert_can_encode_to_type(Value::u64(123), 123u64);
        assert_can_encode_to_type(Value::u128(123), 123u128);

        assert_can_encode_to_type(Value::bool(true), true);
        assert_can_encode_to_type(Value::bool(false), false);

        assert_can_encode_to_type(Value::string("Hello"), "Hello");
        assert_can_encode_to_type(Value::string("Hello"), "Hello".to_string());
    }

    #[test]
    fn chars_encoded_like_numbers() {
        assert_can_encode_to_type(Value::char('j'), 'j' as u32);
        assert_can_encode_to_type(Value::char('j'), 'j' as u8);
    }

    #[test]
    fn can_encode_primitive_arrs_to_array() {
        use crate::Primitive;

        assert_can_encode_to_type(
            Value::primitive(Primitive::U256([12u8; 32])),
            [12u8; 32],
        );
        assert_can_encode_to_type(
            Value::primitive(Primitive::I256([12u8; 32])),
            [12u8; 32],
        );
    }

    #[test]
    fn can_encode_primitive_arrs_to_vecs() {
        use crate::Primitive;

        assert_can_encode_to_type(
            Value::primitive(Primitive::U256([12u8; 32])),
            vec![12u8; 32],
        );
        assert_can_encode_to_type(
            Value::primitive(Primitive::I256([12u8; 32])),
            vec![12u8; 32],
        );
    }

    #[test]
    fn can_encode_arrays() {
        let value = Value::unnamed_composite(vec![
            Value::u16(1),
            Value::u16(2),
            Value::u16(3),
            Value::u16(4),
        ]);
        assert_can_encode_to_type(value, [1u16, 2, 3, 4]);
    }

    #[test]
    fn can_encode_variants() {
        #[derive(Encode, scale_info::TypeInfo)]
        enum Foo {
            Named { hello: String, foo: bool },
            Unnamed(u64, Vec<bool>),
        }

        let named_value = Value::variant(
            "Named".into(),
            Composite::Named(vec![
                // Deliverately a different order; order shouldn't matter:
                ("foo".into(), Value::bool(true)),
                ("hello".into(), Value::string("world")),
            ]),
        );
        assert_can_encode_to_type(
            named_value,
            Foo::Named {
                hello: "world".into(),
                foo: true,
            },
        );

        let unnamed_value = Value::variant(
            "Unnamed".into(),
            Composite::Unnamed(vec![
                Value::u64(123),
                Value::unnamed_composite(vec![
                    Value::bool(true),
                    Value::bool(false),
                    Value::bool(true),
                ]),
            ]),
        );
        assert_can_encode_to_type(
            unnamed_value,
            Foo::Unnamed(123, vec![true, false, true]),
        );
    }

    #[test]
    fn can_encode_structs() {
        #[derive(Encode, scale_info::TypeInfo)]
        struct Foo {
            hello: String,
            foo: bool,
        }

        let named_value = Value::named_composite(vec![
            // Deliverately a different order; order shouldn't matter:
            ("foo".into(), Value::bool(true)),
            ("hello".into(), Value::string("world")),
        ]);
        assert_can_encode_to_type(
            named_value,
            Foo {
                hello: "world".into(),
                foo: true,
            },
        );
    }

    #[test]
    fn can_encode_tuples_from_named_composite() {
        let named_value = Value::named_composite(vec![
            ("hello".into(), Value::string("world")),
            ("foo".into(), Value::bool(true)),
        ]);
        assert_can_encode_to_type(named_value, ("world", true));
    }

    #[test]
    fn can_encode_tuples_from_unnamed_composite() {
        let unnamed_value =
            Value::unnamed_composite(vec![Value::string("world"), Value::bool(true)]);
        assert_can_encode_to_type(unnamed_value, ("world", true));
    }

    #[test]
    fn can_encode_bitvecs() {
        use bitvec::{
            bitvec,
            order::{
                Lsb0,
                Msb0,
            },
        };

        let bits = bitvec![u8, Lsb0; 0, 1, 1, 0, 0, 1];
        let value = Value::bit_sequence(bits);

        // Support encoding our Value to the different underlying formats that bitvec can have:

        assert_can_encode_to_type(value.clone(), bitvec![u8, Lsb0; 0, 1, 1, 0, 0, 1]);
        assert_can_encode_to_type(value.clone(), bitvec![u8, Msb0; 0, 1, 1, 0, 0, 1]);

        assert_can_encode_to_type(value.clone(), bitvec![u16, Lsb0; 0, 1, 1, 0, 0, 1]);
        assert_can_encode_to_type(value.clone(), bitvec![u16, Msb0; 0, 1, 1, 0, 0, 1]);

        assert_can_encode_to_type(value.clone(), bitvec![u32, Lsb0; 0, 1, 1, 0, 0, 1]);
        assert_can_encode_to_type(value.clone(), bitvec![u32, Msb0; 0, 1, 1, 0, 0, 1]);

        assert_can_encode_to_type(value.clone(), bitvec![u64, Lsb0; 0, 1, 1, 0, 0, 1]);
        assert_can_encode_to_type(value.clone(), bitvec![u64, Msb0; 0, 1, 1, 0, 0, 1]);
    }

    #[test]
    fn can_encode_to_compact_types() {
        assert_can_encode_to_type(Value::u8(123), Compact(123u64));
        assert_can_encode_to_type(Value::u16(123), Compact(123u64));
        assert_can_encode_to_type(Value::u32(123), Compact(123u64));
        assert_can_encode_to_type(Value::u64(123), Compact(123u64));

        // As a special case, as long as ultimately we have a primitive value, we can compact encode it:
        assert_can_encode_to_type(
            Value::unnamed_composite(vec![Value::u64(123)]),
            Compact(123u64),
        );
        assert_can_encode_to_type(
            Value::unnamed_composite(vec![Value::named_composite(vec![(
                "foo".to_string(),
                Value::u64(123),
            )])]),
            Compact(123u64),
        );
    }
}
