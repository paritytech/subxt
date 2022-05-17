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

//! This [`Serialize`] impl allows [`Value`]s to be serialized to some format
//! (eg JSON); we do our best to hand the [`Serializer`] values which most
//! accurately represent what we've stored, but there is always some amount of
//! converstion between our [`Value`] type and the types supported by the
//! serde data model that we're serializing things into.

use super::bitvec_helpers;
use crate::{
    Composite,
    Primitive,
    Value,
    ValueDef,
    Variant,
};
use serde::{
    ser::{
        SerializeMap,
        SerializeSeq,
    },
    Serialize,
    Serializer,
};

impl<T> Serialize for Value<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<T> Serialize for ValueDef<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValueDef::Composite(val) => val.serialize(serializer),
            ValueDef::Variant(val) => val.serialize(serializer),
            ValueDef::Primitive(val) => val.serialize(serializer),
            ValueDef::BitSequence(val) => {
                bitvec_helpers::serialize_bitvec(val, serializer)
            }
        }
    }
}

impl<T> Serialize for Composite<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Composite::Named(vals) => {
                let mut map = serializer.serialize_map(Some(vals.len()))?;
                for (key, val) in vals {
                    map.serialize_entry(key, val)?;
                }
                map.end()
            }
            Composite::Unnamed(vals) => {
                let mut seq = serializer.serialize_seq(Some(vals.len()))?;
                for val in vals {
                    seq.serialize_element(val)?;
                }
                seq.end()
            }
        }
    }
}

impl Serialize for Primitive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Delegate to the serialization strategy used by the primitive types.
        match self {
            Primitive::Bool(v) => v.serialize(serializer),
            Primitive::Char(v) => v.serialize(serializer),
            Primitive::String(v) => v.serialize(serializer),
            Primitive::U8(v) => v.serialize(serializer),
            Primitive::U16(v) => v.serialize(serializer),
            Primitive::U32(v) => v.serialize(serializer),
            Primitive::U64(v) => v.serialize(serializer),
            Primitive::U128(v) => v.serialize(serializer),
            Primitive::U256(v) => v.serialize(serializer),
            Primitive::I8(v) => v.serialize(serializer),
            Primitive::I16(v) => v.serialize(serializer),
            Primitive::I32(v) => v.serialize(serializer),
            Primitive::I64(v) => v.serialize(serializer),
            Primitive::I128(v) => v.serialize(serializer),
            Primitive::I256(v) => v.serialize(serializer),
        }
    }
}

impl<T> Serialize for Variant<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // We can't use the enum serializing in the serde data model because that requires static
        // strs and enum indexes, which we don't have (since this is a runtime value), so we serialize
        // as a map with a type and a value, and make sure that we allow this format when attempting to
        // deserialize into a `Variant` type for a bit of symmetry (although note that if you try to deserialize
        // this into a `Value` type it'll have no choice but to deserialize straight into a `Composite::Named` map).
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("values", &self.values)?;
        map.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    fn assert_value(value: Value<()>, expected: serde_json::Value) {
        let val =
            serde_json::to_value(&value).expect("can serialize to serde_json::Value");
        assert_eq!(val, expected);
    }

    #[test]
    fn serialize_primitives() {
        // a subset of the primitives to sanity check that they are unwrapped:
        assert_value(Value::u8(1), json!(1));
        assert_value(Value::u16(1), json!(1));
        assert_value(Value::u32(1), json!(1));
        assert_value(Value::u64(1), json!(1));
        assert_value(Value::bool(true), json!(true));
        assert_value(Value::bool(false), json!(false));
    }

    #[test]
    fn serialize_composites() {
        assert_value(
            Value::named_composite(vec![
                ("a".into(), Value::bool(true)),
                ("b".into(), Value::string("hello")),
                ("c".into(), Value::char('c')),
            ]),
            json!({
                "a": true,
                "b": "hello",
                "c": 'c'
            }),
        );
        assert_value(
            Value::unnamed_composite(vec![
                Value::bool(true),
                Value::string("hello"),
                Value::char('c'),
            ]),
            json!([true, "hello", 'c']),
        )
    }

    #[test]
    fn serialize_variants() {
        assert_value(
            Value::variant(
                "Foo".into(),
                Composite::Named(vec![
                    ("a".into(), Value::bool(true)),
                    ("b".into(), Value::string("hello")),
                    ("c".into(), Value::char('c')),
                ]),
            ),
            json!({
                "name": "Foo",
                "values": {
                    "a": true,
                    "b": "hello",
                    "c": 'c'
                }
            }),
        );
        assert_value(
            Value::variant(
                "Bar".into(),
                Composite::Unnamed(vec![
                    Value::bool(true),
                    Value::string("hello"),
                    Value::char('c'),
                ]),
            ),
            json!({
                "name": "Bar",
                "values": [
                    true,
                    "hello",
                    'c'
                ]
            }),
        )
    }
}
