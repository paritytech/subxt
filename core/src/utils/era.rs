// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use alloc::{format, vec::Vec};
use codec::{Decode, Encode};
use scale_decode::{
    IntoVisitor, TypeResolver, Visitor,
    ext::scale_type_resolver,
    visitor::{TypeIdFor, types::Variant},
};
use scale_encode::EncodeAsType;

// Dev note: This and related bits taken from `sp_runtime::generic::Era`
/// An era to describe the longevity of a transaction.
#[derive(
    PartialEq,
    Default,
    Eq,
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    scale_info::TypeInfo,
)]
pub enum Era {
    /// The transaction is valid forever. The genesis hash must be present in the signed content.
    #[default]
    Immortal,

    /// The transaction will expire. Use [`Era::mortal`] to construct this with correct values.
    ///
    /// When used on `FRAME`-based runtimes, `period` cannot exceed `BlockHashCount` parameter
    /// of `system` module.
    Mortal {
        /// The number of blocks that the tx will be valid for after the checkpoint block
        /// hash found in the signer payload.
        period: u64,
        /// The phase in the period that this transaction's lifetime begins (and, importantly,
        /// implies which block hash is included in the signature material). If the `period` is
        /// greater than 1 << 12, then it will be a factor of the times greater than 1<<12 that
        /// `period` is.
        phase: u64,
    },
}

// E.g. with period == 4:
// 0         10        20        30        40
// 0123456789012345678901234567890123456789012
//              |...|
//    authored -/   \- expiry
// phase = 1
// n = Q(current - phase, period) + phase
impl Era {
    /// Create a new era based on a period (which should be a power of two between 4 and 65536
    /// inclusive) and a block number on which it should start (or, for long periods, be shortly
    /// after the start).
    ///
    /// If using `Era` in the context of `FRAME` runtime, make sure that `period`
    /// does not exceed `BlockHashCount` parameter passed to `system` module, since that
    /// prunes old blocks and renders transactions immediately invalid.
    pub fn mortal(period: u64, current: u64) -> Self {
        let period = period
            .checked_next_power_of_two()
            .unwrap_or(1 << 16)
            .clamp(4, 1 << 16);
        let phase = current % period;
        let quantize_factor = (period >> 12).max(1);
        let quantized_phase = phase / quantize_factor * quantize_factor;

        Self::Mortal {
            period,
            phase: quantized_phase,
        }
    }
}

// Both copied from `sp_runtime::generic::Era`; this is the wire interface and so
// it's really the most important bit here.
impl codec::Encode for Era {
    fn encode_to<T: codec::Output + ?Sized>(&self, output: &mut T) {
        match self {
            Self::Immortal => output.push_byte(0),
            Self::Mortal { period, phase } => {
                let quantize_factor = (*period >> 12).max(1);
                let encoded = (period.trailing_zeros() - 1).clamp(1, 15) as u16
                    | ((phase / quantize_factor) << 4) as u16;
                encoded.encode_to(output);
            }
        }
    }
}
impl codec::Decode for Era {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let first = input.read_byte()?;
        if first == 0 {
            Ok(Self::Immortal)
        } else {
            let encoded = first as u64 + ((input.read_byte()? as u64) << 8);
            let period = 2 << (encoded % (1 << 4));
            let quantize_factor = (period >> 12).max(1);
            let phase = (encoded >> 4) * quantize_factor;
            if period >= 4 && phase < period {
                Ok(Self::Mortal { period, phase })
            } else {
                Err("Invalid period and phase".into())
            }
        }
    }
}

/// Define manually how to encode an Era given some type information. Here we
/// basically check that the type we're targeting is called "Era" and then codec::Encode.
impl EncodeAsType for Era {
    fn encode_as_type_to<R: TypeResolver>(
        &self,
        type_id: R::TypeId,
        types: &R,
        out: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error> {
        // Visit the type to check that it is an Era. This is only a rough check.
        let visitor = scale_type_resolver::visitor::new((), |_, _| false)
            .visit_variant(|_, path, _variants| path.last() == Some("Era"));

        let is_era = types
            .resolve_type(type_id.clone(), visitor)
            .unwrap_or_default();
        if !is_era {
            return Err(scale_encode::Error::custom_string(format!(
                "Type {type_id:?} is not a valid Era type; expecting either Immortal or MortalX variant"
            )));
        }

        // if the type looks valid then just scale encode our Era.
        self.encode_to(out);
        Ok(())
    }
}

/// Define manually how to decode an Era given some type information. Here we check that the
/// variant we're decoding is one of the expected Era variants, and that the field is correct if so,
/// ensuring that this will fail if trying to decode something that isn't an Era.
pub struct EraVisitor<R>(core::marker::PhantomData<R>);

impl IntoVisitor for Era {
    type AnyVisitor<R: TypeResolver> = EraVisitor<R>;
    fn into_visitor<R: TypeResolver>() -> Self::AnyVisitor<R> {
        EraVisitor(core::marker::PhantomData)
    }
}

impl<R: TypeResolver> Visitor for EraVisitor<R> {
    type Value<'scale, 'resolver> = Era;
    type Error = scale_decode::Error;
    type TypeResolver = R;

    fn visit_variant<'scale, 'resolver>(
        self,
        value: &mut Variant<'scale, 'resolver, Self::TypeResolver>,
        _type_id: TypeIdFor<Self>,
    ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
        let variant = value.name();

        // If the variant is immortal, we know the outcome.
        if variant == "Immortal" {
            return Ok(Era::Immortal);
        }

        // Otherwise, we expect a variant Mortal1..Mortal255 where the number
        // here is the first byte, and the second byte is conceptually a field of this variant.
        // This weird encoding is because the Era is compressed to just 1 byte if immortal and
        // just 2 bytes if mortal.
        //
        // Note: We _could_ just assume we'll have 2 bytes to work with and decode the era directly,
        // but checking the variant names ensures that the thing we think is an Era actually _is_
        // one, based on the type info for it.
        let first_byte = variant
            .strip_prefix("Mortal")
            .and_then(|s| s.parse::<u8>().ok())
            .ok_or_else(|| {
                scale_decode::Error::custom_string(format!(
                    "Expected MortalX variant, but got {variant}"
                ))
            })?;

        // We need 1 field in the MortalN variant containing the second byte.
        let mortal_fields = value.fields();
        if mortal_fields.remaining() != 1 {
            return Err(scale_decode::Error::custom_string(format!(
                "Expected Mortal{} to have one u8 field, but got {} fields",
                first_byte,
                mortal_fields.remaining()
            )));
        }

        let second_byte = mortal_fields
            .decode_item(u8::into_visitor())
            .expect("At least one field should exist; checked above.")
            .map_err(|e| {
                scale_decode::Error::custom_string(format!(
                    "Expected mortal variant field to be u8, but: {e}"
                ))
            })?;

        // Now that we have both bytes we can decode them into the era using
        // the same logic as the codec::Decode impl does.
        Era::decode(&mut &[first_byte, second_byte][..]).map_err(|e| {
            scale_decode::Error::custom_string(format!(
                "Failed to codec::Decode Era from Mortal bytes: {e}"
            ))
        })
    }
}
