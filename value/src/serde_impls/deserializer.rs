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

//! This module implements the [`Deserializer`] (note the 'r') trait on our Value enum.
//!
//! A deserializer is a thing which implements methods like `deserialize_i128`. Each of these
//! methods serves as a hint about what the thing calling it (probably a thing implementing
//! [`Deserialize`]) actually wants back. The methods are given a "visitor" which actually accepts
//! values back. We might not give the visitor back the value that it hinted that it wanted, but
//! it's up to the visitor to do its best to accept what it's handed, or reject it if it's simply
//! not going to work out.

use super::bitvec_helpers;
use crate::{
    Composite,
    Primitive,
    Value,
    ValueDef,
    Variant,
};
use serde::{
    de::{
        self,
        EnumAccess,
        IntoDeserializer,
        VariantAccess,
    },
    forward_to_deserialize_any,
    ser,
    Deserialize,
    Deserializer,
};
use std::{
    borrow::Cow,
    fmt::Display,
};

/// An opaque error to describe in human terms what went wrong.
/// Many internal serialization/deserialization errors are relayed
/// to this in string form, and so we use basic strings for custom
/// errors as well for simplicity.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
#[error("{0}")]
pub struct Error(Cow<'static, str>);

impl Error {
    fn from_string<S: Into<String>>(s: S) -> Error {
        Error(Cow::Owned(s.into()))
    }
    fn from_str(s: &'static str) -> Error {
        Error(Cow::Borrowed(s))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::from_string(msg.to_string())
    }
}
impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::from_string(msg.to_string())
    }
}

/// Spit out the simple deserialize methods to avoid loads of repetition.
macro_rules! deserialize_x {
    ($fn_name:ident) => {
        fn $fn_name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            self.value.$fn_name(visitor)
        }
    };
}

// Our Value type has some context, which we ignore, and some definition, whose deserializer
// impl we forward to.
impl<'de, T> Deserializer<'de> for Value<T> {
    type Error = Error;

    deserialize_x!(deserialize_any);
    deserialize_x!(deserialize_bool);
    deserialize_x!(deserialize_i8);
    deserialize_x!(deserialize_i16);
    deserialize_x!(deserialize_i32);
    deserialize_x!(deserialize_i64);
    deserialize_x!(deserialize_i128);
    deserialize_x!(deserialize_u8);
    deserialize_x!(deserialize_u16);
    deserialize_x!(deserialize_u32);
    deserialize_x!(deserialize_u64);
    deserialize_x!(deserialize_u128);
    deserialize_x!(deserialize_f32);
    deserialize_x!(deserialize_f64);
    deserialize_x!(deserialize_char);
    deserialize_x!(deserialize_str);
    deserialize_x!(deserialize_string);
    deserialize_x!(deserialize_bytes);
    deserialize_x!(deserialize_byte_buf);
    deserialize_x!(deserialize_option);
    deserialize_x!(deserialize_unit);
    deserialize_x!(deserialize_seq);
    deserialize_x!(deserialize_map);
    deserialize_x!(deserialize_identifier);
    deserialize_x!(deserialize_ignored_any);

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value.deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.value.deserialize_enum(name, variants, visitor)
    }
}

// Our ValueDef deserializer needs to handle BitSeq itself, but otherwise delegates to
// the inner implementations of things to handle. This macro makes that less repetitive
// to write by only requiring a bitseq impl.
macro_rules! delegate_except_bitseq {
    (
        $name:ident ( $self:ident, $($arg:ident),* ),
            $seq:pat => $expr:expr
    ) => {
        match $self {
            ValueDef::BitSequence($seq) => {
                $expr
            },
            ValueDef::Composite(composite) => {
                composite.$name( $($arg),* )
            },
            ValueDef::Variant(variant) => {
                variant.$name( $($arg),* )
            },
            ValueDef::Primitive(prim) => {
                prim.$name( $($arg),* )
            },
        }
    }
}

// The goal here is simply to forward deserialization methods of interest to
// the relevant subtype. The exception is our BitSequence type, which doesn't
// have a sub type to forward to and so is handled here.
impl<'de, T> Deserializer<'de> for ValueDef<T> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_any(self, visitor),
            seq => {
                let map = bitvec_helpers::map_access(seq);
                visitor.visit_map(map)
            }
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_newtype_struct(self, name, visitor),
            _ => {
                Err(Error::from_str("Cannot deserialize BitSequence into a newtype struct"))
            }
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_tuple(self, len, visitor),
            _ => {
                Err(Error::from_str("Cannot deserialize BitSequence into a tuple"))
            }
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_tuple_struct(self, name, len, visitor),
            _ => {
                Err(Error::from_str("Cannot deserialize BitSequence into a tuple struct"))
            }
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_unit(self, visitor),
            _ => {
                Err(Error::from_str("Cannot deserialize BitSequence into a ()"))
            }
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_unit_struct(self, name, visitor),
            _ => {
                Err(Error::from_string(format!("Cannot deserialize BitSequence into the unit struct {}", name)))
            }
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_enum(self, name, variants, visitor),
            _ => {
                Err(Error::from_string(format!("Cannot deserialize BitSequence into the enum {}", name)))
            }
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_bytes(self, visitor),
            _ => {
                Err(Error::from_str("Cannot deserialize BitSequence into raw bytes"))
            }
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_byte_buf(self, visitor),
            _ => {
                Err(Error::from_str("Cannot deserialize BitSequence into raw bytes"))
            }
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_seq(self, visitor),
            _ => {
                Err(Error::from_str("Cannot deserialize BitSequence into a sequence"))
            }
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        delegate_except_bitseq! { deserialize_map(self, visitor),
            _ => {
                Err(Error::from_str("Cannot deserialize BitSequence into a map"))
            }
        }
    }

    // None of the sub types particularly care about these, so we just allow them to forward to
    // deserialize_any and go from there.
    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        option struct identifier ignored_any
    }
}

impl<'de, T> IntoDeserializer<'de, Error> for Value<T> {
    type Deserializer = Value<T>;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de, T> Deserializer<'de> for Composite<T> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            Composite::Named(values) => {
                visitor.visit_map(de::value::MapDeserializer::new(values.into_iter()))
            }
            Composite::Unnamed(values) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(values.into_iter()))
            }
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Composite::Named(values) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(
                    values.into_iter().map(|(_, v)| v),
                ))
            }
            Composite::Unnamed(values) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(values.into_iter()))
            }
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            // A sequence of named values? just ignores the names:
            Composite::Named(values) => {
                if values.len() != len {
                    return Err(Error::from_string(format!(
						"Cannot deserialize composite of length {} into tuple of length {}",
						values.len(),
						len
					)))
                }
                visitor.visit_seq(de::value::SeqDeserializer::new(
                    values.into_iter().map(|(_, v)| v),
                ))
            }
            // A sequence of unnamed values is ideal:
            Composite::Unnamed(values) => {
                if values.len() != len {
                    return Err(Error::from_string(format!(
						"Cannot deserialize composite of length {} into tuple of length {}",
						values.len(),
						len
					)))
                }
                visitor.visit_seq(de::value::SeqDeserializer::new(values.into_iter()))
            }
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // 0 length composite types can be treated as the unit type:
        if self.is_empty() {
            visitor.visit_unit()
        } else {
            Err(Error::from_str(
                "Cannot deserialize non-empty Composite into a unit value",
            ))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(de::value::SeqDeserializer::new(Some(self).into_iter()))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Composite::Named(values) => {
                let bytes = values
					.into_iter()
					.map(|(_n, v)| {
						if let ValueDef::Primitive(Primitive::U8(byte)) = v.value {
							Ok(byte)
						} else {
							Err(Error::from_str("Cannot deserialize composite that is not entirely U8's into bytes"))
						}
					})
					.collect::<Result<_, Error>>()?;
                visitor.visit_byte_buf(bytes)
            }
            Composite::Unnamed(values) => {
                let bytes = values
					.into_iter()
					.map(|v| {
						if let ValueDef::Primitive(Primitive::U8(byte)) = v.value {
							Ok(byte)
						} else {
							Err(Error::from_str("Cannot deserialize composite that is not entirely U8's into bytes"))
						}
					})
					.collect::<Result<_, Error>>()?;
                visitor.visit_byte_buf(bytes)
            }
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        option struct map
        enum identifier ignored_any
    }
}

impl<'de, T> IntoDeserializer<'de, Error> for Composite<T> {
    type Deserializer = Composite<T>;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

// Because composite types are used to represent variant fields, we allow
// variant accesses to be called on it, which just delegate to methods defined above.
impl<'de, T> VariantAccess<'de> for Composite<T> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Deserialize::deserialize(self)
    }

    fn newtype_variant_seed<S>(self, seed: S) -> Result<S::Value, Self::Error>
    where
        S: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de, T> Deserializer<'de> for Variant<T> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(de::value::SeqDeserializer::new(Some(self).into_iter()))
    }

    // All of the below functions delegate to the Composite deserializing methods using the enum values.

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.values.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.values.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.values.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.values.deserialize_unit(visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.values.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.values.deserialize_map(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.values.deserialize_seq(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option identifier ignored_any
    }
}

impl<'de, T> IntoDeserializer<'de, Error> for Variant<T> {
    type Deserializer = Variant<T>;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

// Variant types can be treated as serde enums. Here we just hand back
// the pair of name and values, where values is a composite type that impls
// VariantAccess to actually allow deserializing of those values.
impl<'de, T> EnumAccess<'de> for Variant<T> {
    type Error = Error;

    type Variant = Composite<T>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let name = self.name.into_deserializer();
        let values = self.values;
        seed.deserialize(name).map(|name| (name, values))
    }
}

impl<'de> Deserializer<'de> for Primitive {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            Primitive::Bool(v) => visitor.visit_bool(v),
            Primitive::Char(v) => visitor.visit_char(v),
            Primitive::String(v) => visitor.visit_string(v),
            Primitive::U8(v) => visitor.visit_u8(v),
            Primitive::U16(v) => visitor.visit_u16(v),
            Primitive::U32(v) => visitor.visit_u32(v),
            Primitive::U64(v) => visitor.visit_u64(v),
            Primitive::U128(v) => visitor.visit_u128(v),
            Primitive::U256(v) => visitor.visit_bytes(&v),
            Primitive::I8(v) => visitor.visit_i8(v),
            Primitive::I16(v) => visitor.visit_i16(v),
            Primitive::I32(v) => visitor.visit_i32(v),
            Primitive::I64(v) => visitor.visit_i64(v),
            Primitive::I128(v) => visitor.visit_i128(v),
            Primitive::I256(v) => visitor.visit_bytes(&v),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(de::value::SeqDeserializer::new(Some(self).into_iter()))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> IntoDeserializer<'de, Error> for Primitive {
    type Deserializer = Primitive;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn de_into_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Foo {
            a: u8,
            b: bool,
        }

        let val = ValueDef::Composite(Composite::Named(vec![
            // Order shouldn't matter; match on names:
            ("b".into(), Value::bool(true)),
            ("a".into(), Value::u8(123)),
        ]));

        assert_eq!(Foo::deserialize(val), Ok(Foo { a: 123, b: true }))
    }

    #[test]
    fn de_unwrapped_into_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Foo {
            a: u8,
            b: bool,
        }

        let val = Composite::Named(vec![
            // Order shouldn't matter; match on names:
            ("b".into(), Value::bool(true)),
            ("a".into(), Value::u8(123)),
        ]);

        assert_eq!(Foo::deserialize(val), Ok(Foo { a: 123, b: true }))
    }

    #[test]
    fn de_into_tuple_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Foo(u8, bool, String);

        let val = ValueDef::Composite(Composite::Unnamed(vec![
            Value::u8(123),
            Value::bool(true),
            Value::string("hello"),
        ]));

        assert_eq!(Foo::deserialize(val), Ok(Foo(123, true, "hello".into())))
    }

    #[test]
    fn de_unwrapped_into_tuple_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Foo(u8, bool, String);

        let val = Composite::Unnamed(vec![
            Value::u8(123),
            Value::bool(true),
            Value::string("hello"),
        ]);

        assert_eq!(Foo::deserialize(val), Ok(Foo(123, true, "hello".into())))
    }

    #[test]
    fn de_into_newtype_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct FooStr(String);
        let val = ValueDef::<()>::Primitive(Primitive::String("hello".into()));
        assert_eq!(FooStr::deserialize(val), Ok(FooStr("hello".into())));
        let val = Value::string("hello");
        assert_eq!(FooStr::deserialize(val), Ok(FooStr("hello".into())));

        #[derive(Deserialize, Debug, PartialEq)]
        struct FooVecU8(Vec<u8>);
        let val = ValueDef::Composite(Composite::Unnamed(vec![
            Value::u8(1),
            Value::u8(2),
            Value::u8(3),
        ]));
        assert_eq!(FooVecU8::deserialize(val), Ok(FooVecU8(vec![1, 2, 3])));

        #[derive(Deserialize, Debug, PartialEq)]
        enum MyEnum {
            Foo(u8, u8, u8),
        }
        #[derive(Deserialize, Debug, PartialEq)]
        struct FooVar(MyEnum);
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Unnamed(vec![Value::u8(1), Value::u8(2), Value::u8(3)]),
        });
        assert_eq!(FooVar::deserialize(val), Ok(FooVar(MyEnum::Foo(1, 2, 3))));
    }

    #[test]
    fn de_unwrapped_into_newtype_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct FooStr(String);
        let val = Primitive::String("hello".into());
        assert_eq!(FooStr::deserialize(val), Ok(FooStr("hello".into())));

        #[derive(Deserialize, Debug, PartialEq)]
        struct FooVecU8(Vec<u8>);
        let val = Composite::Unnamed(vec![Value::u8(1), Value::u8(2), Value::u8(3)]);
        assert_eq!(FooVecU8::deserialize(val), Ok(FooVecU8(vec![1, 2, 3])));

        #[derive(Deserialize, Debug, PartialEq)]
        enum MyEnum {
            Foo(u8, u8, u8),
        }
        #[derive(Deserialize, Debug, PartialEq)]
        struct FooVar(MyEnum);
        let val = Variant {
            name: "Foo".into(),
            values: Composite::Unnamed(vec![Value::u8(1), Value::u8(2), Value::u8(3)]),
        };
        assert_eq!(FooVar::deserialize(val), Ok(FooVar(MyEnum::Foo(1, 2, 3))));
    }

    #[test]
    fn de_into_vec() {
        let val = ValueDef::Composite(Composite::Unnamed(vec![
            Value::u8(1),
            Value::u8(2),
            Value::u8(3),
        ]));
        assert_eq!(<Vec<u8>>::deserialize(val), Ok(vec![1, 2, 3]));

        let val = ValueDef::Composite(Composite::Unnamed(vec![
            Value::string("a"),
            Value::string("b"),
            Value::string("c"),
        ]));
        assert_eq!(
            <Vec<String>>::deserialize(val),
            Ok(vec!["a".into(), "b".into(), "c".into()])
        );
    }

    #[test]
    fn de_unwrapped_into_vec() {
        let val = Composite::Unnamed(vec![Value::u8(1), Value::u8(2), Value::u8(3)]);
        assert_eq!(<Vec<u8>>::deserialize(val), Ok(vec![1, 2, 3]));

        let val = Composite::Named(vec![
            ("a".into(), Value::u8(1)),
            ("b".into(), Value::u8(2)),
            ("c".into(), Value::u8(3)),
        ]);
        assert_eq!(<Vec<u8>>::deserialize(val), Ok(vec![1, 2, 3]));

        let val = Composite::Unnamed(vec![
            Value::string("a"),
            Value::string("b"),
            Value::string("c"),
        ]);
        assert_eq!(
            <Vec<String>>::deserialize(val),
            Ok(vec!["a".into(), "b".into(), "c".into()])
        );
    }

    #[test]
    fn de_into_map() {
        use std::collections::HashMap;

        let val = ValueDef::Composite(Composite::Named(vec![
            ("a".into(), Value::u8(1)),
            ("b".into(), Value::u8(2)),
            ("c".into(), Value::u8(3)),
        ]));
        assert_eq!(
            <HashMap<String, u8>>::deserialize(val),
            Ok(vec![("a".into(), 1), ("b".into(), 2), ("c".into(), 3)]
                .into_iter()
                .collect())
        );

        let val = ValueDef::Composite(Composite::Unnamed(vec![
            Value::u8(1),
            Value::u8(2),
            Value::u8(3),
        ]));
        <HashMap<String, u8>>::deserialize(val).expect_err("no names; can't be map");
    }

    #[test]
    fn de_into_tuple() {
        let val = ValueDef::Composite(Composite::Unnamed(vec![
            Value::string("hello"),
            Value::bool(true),
        ]));
        assert_eq!(
            <(String, bool)>::deserialize(val),
            Ok(("hello".into(), true))
        );

        // names will just be ignored:
        let val = ValueDef::Composite(Composite::Named(vec![
            ("a".into(), Value::string("hello")),
            ("b".into(), Value::bool(true)),
        ]));
        assert_eq!(
            <(String, bool)>::deserialize(val),
            Ok(("hello".into(), true))
        );

        // Enum variants are allowed! The variant name will be ignored:
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Unnamed(vec![Value::string("hello"), Value::bool(true)]),
        });
        assert_eq!(
            <(String, bool)>::deserialize(val),
            Ok(("hello".into(), true))
        );

        // Enum variants with names values are allowed! The variant name will be ignored:
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Named(vec![
                ("a".into(), Value::string("hello")),
                ("b".into(), Value::bool(true)),
            ]),
        });
        assert_eq!(
            <(String, bool)>::deserialize(val),
            Ok(("hello".into(), true))
        );

        // Wrong number of values should fail:
        let val = ValueDef::Composite(Composite::Unnamed(vec![
            Value::string("hello"),
            Value::bool(true),
            Value::u8(123),
        ]));
        <(String, bool)>::deserialize(val).expect_err("Wrong length, should err");
    }

    #[test]
    fn de_unwrapped_into_tuple() {
        let val = Composite::Unnamed(vec![Value::string("hello"), Value::bool(true)]);
        assert_eq!(
            <(String, bool)>::deserialize(val),
            Ok(("hello".into(), true))
        );

        // names will just be ignored:
        let val = Composite::Named(vec![
            ("a".into(), Value::string("hello")),
            ("b".into(), Value::bool(true)),
        ]);
        assert_eq!(
            <(String, bool)>::deserialize(val),
            Ok(("hello".into(), true))
        );

        // Wrong number of values should fail:
        let val = Composite::Unnamed(vec![
            Value::string("hello"),
            Value::bool(true),
            Value::u8(123),
        ]);
        <(String, bool)>::deserialize(val).expect_err("Wrong length, should err");
    }

    #[test]
    fn de_bitvec() {
        use bitvec::{
            bitvec,
            order::Lsb0,
        };

        // If we deserialize a bitvec value into a value, it should come back out the same.
        let val = Value::bit_sequence(bitvec![u8, Lsb0; 0, 1, 1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(Value::deserialize(val.clone()), Ok(val.clone()));

        // We can serialize a bitvec Value to something like JSON and deserialize it again, too.
        let json_val = serde_json::to_value(&val).expect("can encode to json");
        let new_val: Value<()> =
            serde_json::from_value(json_val).expect("can decode back from json");
        assert_eq!(new_val, val);
    }

    #[test]
    fn de_into_tuple_variant() {
        #[derive(Deserialize, Debug, PartialEq)]
        enum MyEnum {
            Foo(String, bool, u8),
        }

        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Unnamed(vec![
                Value::string("hello"),
                Value::bool(true),
                Value::u8(123),
            ]),
        });
        assert_eq!(
            MyEnum::deserialize(val),
            Ok(MyEnum::Foo("hello".into(), true, 123))
        );

        // it's fine to name the fields; we'll just ignore the names
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Named(vec![
                ("a".into(), Value::string("hello")),
                ("b".into(), Value::bool(true)),
                ("c".into(), Value::u8(123)),
            ]),
        });
        assert_eq!(
            MyEnum::deserialize(val),
            Ok(MyEnum::Foo("hello".into(), true, 123))
        );
    }

    #[test]
    fn de_unwrapped_into_tuple_variant() {
        #[derive(Deserialize, Debug, PartialEq)]
        enum MyEnum {
            Foo(String, bool, u8),
        }

        let val = Variant {
            name: "Foo".into(),
            values: Composite::Unnamed(vec![
                Value::string("hello"),
                Value::bool(true),
                Value::u8(123),
            ]),
        };
        assert_eq!(
            MyEnum::deserialize(val),
            Ok(MyEnum::Foo("hello".into(), true, 123))
        );

        // it's fine to name the fields; we'll just ignore the names
        let val = Variant {
            name: "Foo".into(),
            values: Composite::Named(vec![
                ("a".into(), Value::string("hello")),
                ("b".into(), Value::bool(true)),
                ("c".into(), Value::u8(123)),
            ]),
        };
        assert_eq!(
            MyEnum::deserialize(val),
            Ok(MyEnum::Foo("hello".into(), true, 123))
        );
    }

    #[test]
    fn de_into_struct_variant() {
        #[derive(Deserialize, Debug, PartialEq)]
        enum MyEnum {
            Foo { hi: String, a: bool, b: u8 },
        }

        // If names given, order doesn't matter:
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Named(vec![
                // Deliberately out of order: names should ensure alignment:
                ("b".into(), Value::u8(123)),
                ("a".into(), Value::bool(true)),
                ("hi".into(), Value::string("hello")),
            ]),
        });
        assert_eq!(
            MyEnum::deserialize(val),
            Ok(MyEnum::Foo {
                hi: "hello".into(),
                a: true,
                b: 123
            })
        );

        // No names needed if order is OK:
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Unnamed(vec![
                Value::string("hello"),
                Value::bool(true),
                Value::u8(123),
            ]),
        });
        assert_eq!(
            MyEnum::deserialize(val),
            Ok(MyEnum::Foo {
                hi: "hello".into(),
                a: true,
                b: 123
            })
        );

        // Wrong order won't work if no names:
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Unnamed(vec![
                Value::bool(true),
                Value::u8(123),
                Value::string("hello"),
            ]),
        });
        MyEnum::deserialize(val).expect_err("Wrong order shouldn't work");

        // Wrong names won't work:
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Named(vec![
                ("b".into(), Value::u8(123)),
                // Whoops; wrong name:
                ("c".into(), Value::bool(true)),
                ("hi".into(), Value::string("hello")),
            ]),
        });
        MyEnum::deserialize(val).expect_err("Wrong names shouldn't work");

        // Too many names is OK; we can ignore fields we don't care about:
        let val = ValueDef::Variant(Variant {
            name: "Foo".into(),
            values: Composite::Named(vec![
                ("foo".into(), Value::u8(40)),
                ("b".into(), Value::u8(123)),
                ("a".into(), Value::bool(true)),
                ("bar".into(), Value::bool(false)),
                ("hi".into(), Value::string("hello")),
            ]),
        });
        assert_eq!(
            MyEnum::deserialize(val),
            Ok(MyEnum::Foo {
                hi: "hello".into(),
                a: true,
                b: 123
            })
        );
    }

    #[test]
    fn de_into_unit_variants() {
        let val = Value::variant("Foo".into(), Composite::Named(vec![]));
        let unwrapped_val = Variant::<()> {
            name: "Foo".into(),
            values: Composite::Named(vec![]),
        };

        #[derive(Deserialize, Debug, PartialEq)]
        enum MyEnum {
            Foo,
        }
        assert_eq!(MyEnum::deserialize(val.clone()), Ok(MyEnum::Foo));
        assert_eq!(MyEnum::deserialize(unwrapped_val.clone()), Ok(MyEnum::Foo));

        #[derive(Deserialize, Debug, PartialEq)]
        enum MyEnum2 {
            Foo(),
        }
        assert_eq!(MyEnum2::deserialize(val.clone()), Ok(MyEnum2::Foo()));
        assert_eq!(
            MyEnum2::deserialize(unwrapped_val.clone()),
            Ok(MyEnum2::Foo())
        );

        #[derive(Deserialize, Debug, PartialEq)]
        enum MyEnum3 {
            Foo {},
        }
        assert_eq!(MyEnum3::deserialize(val), Ok(MyEnum3::Foo {}));
        assert_eq!(MyEnum3::deserialize(unwrapped_val), Ok(MyEnum3::Foo {}));
    }
}
