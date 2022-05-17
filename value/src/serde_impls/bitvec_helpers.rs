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

use super::deserializer::Error as DeserializerError;
use crate::{
    BitSequence,
    Composite,
    ValueDef,
};
use serde::{
    de::{
        value::MapDeserializer,
        MapAccess,
        Visitor,
    },
    ser::SerializeMap,
    Serializer,
};

/// We use this identifier in a map to uniquely identify a bitvec payload, so that it can
/// be differentiated from a standard [`ValueDef::Composite`] payload (which could also be a map).
pub static BITVEC_SERDE_IDENT: &str = "__bitvec__values__";

/// Serialize a bitvec so that the special deserializing is compatible with it.
pub fn serialize_bitvec<S>(seq: &BitSequence, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bools: Vec<bool> = seq.iter().by_vals().collect();
    let mut map = serializer.serialize_map(Some(1))?;
    map.serialize_entry(BITVEC_SERDE_IDENT, &bools)?;
    map.end()
}

/// Turn a [`BitSequence`] into a [`MapAccess`] impl that can be handed to a visitor to be consumed.
pub fn map_access<'de>(
    seq: BitSequence,
) -> impl MapAccess<'de, Error = DeserializerError> {
    let bools: Vec<bool> = seq.iter().by_vals().collect();
    MapDeserializer::new([(BITVEC_SERDE_IDENT, bools)].into_iter())
}

/// This visits a map, and will extract from that either a [`ValueDef::Composite`] or a
/// [`ValueDef::BitSequence`] depending on the content of the map.
pub struct MapOrBitSeqVisitor;

impl<'de> Visitor<'de> for MapOrBitSeqVisitor {
    type Value = ValueDef<()>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map-like type that can be decoded into a Value::BitSequence or Value::Composite")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        // get the first key from the map:
        let first_key = match map.next_key::<String>()? {
            Some(key) => key,
            // No errors but the map appears to be empty; return an empty named composite.
            None => return Ok(ValueDef::Composite(Composite::Named(Vec::new()))),
        };

        // See whether the key identifies a bitvec payload:
        if first_key == BITVEC_SERDE_IDENT {
            // We should be able to decode a vec of bools as the value, then:
            let bits = map.next_value::<Vec<bool>>()?;
            // .. and we turn that into a bitvec to return:
            let mut bitvec = BitSequence::new();
            for bit in bits {
                bitvec.push(bit);
            }
            return Ok(ValueDef::BitSequence(bitvec))
        }

        // That didn't work, so decode the first value as a Value<()> instead:
        let mut values = Vec::with_capacity(map.size_hint().unwrap_or(0));
        values.push((first_key, map.next_value()?));

        // .. and then decode all of the other key-value pairs and add them too:
        while let Some(key_val) = map.next_entry()? {
            values.push(key_val);
        }
        Ok(ValueDef::Composite(Composite::Named(values)))
    }
}
