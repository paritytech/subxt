// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::PhantomDataSendSync;
use codec::{Compact, Decode, DecodeAll, Encode};
use derivative::Derivative;
use scale_decode::{IntoVisitor, Visitor};
use scale_encode::EncodeAsType;

/// A wrapper for any type `T` which implement encode/decode in a way compatible with `Vec<u8>`.
/// [`WrapperKeepOpaque`] stores the type only in its opaque format, aka as a `Vec<u8>`. To
/// access the real type `T` [`Self::try_decode`] needs to be used.
// Dev notes:
//
// - This is adapted from [here](https://github.com/paritytech/substrate/blob/master/frame/support/src/traits/misc.rs).
// - The encoded bytes will be a compact encoded length followed by that number of bytes.
// - However, the TypeInfo describes the type as a composite with first a compact encoded length and next the type itself.
//  [`Encode`] and [`Decode`] impls will "just work" to take this into a `Vec<u8>`, but we need a custom [`EncodeAsType`]
//  and [`Visitor`] implementation to encode and decode based on TypeInfo.
#[derive(Derivative, Encode, Decode)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Default(bound = ""),
    Hash(bound = "")
)]
pub struct WrapperKeepOpaque<T> {
    data: Vec<u8>,
    _phantom: PhantomDataSendSync<T>,
}

impl<T> WrapperKeepOpaque<T> {
    /// Try to decode the wrapped type from the inner `data`.
    ///
    /// Returns `None` if the decoding failed.
    pub fn try_decode(&self) -> Option<T>
    where
        T: Decode,
    {
        T::decode_all(&mut &self.data[..]).ok()
    }

    /// Returns the length of the encoded `T`.
    pub fn encoded_len(&self) -> usize {
        self.data.len()
    }

    /// Returns the encoded data.
    pub fn encoded(&self) -> &[u8] {
        &self.data
    }

    /// Create from the given encoded `data`.
    pub fn from_encoded(data: Vec<u8>) -> Self {
        Self {
            data,
            _phantom: PhantomDataSendSync::new(),
        }
    }

    /// Create from some raw value by encoding it.
    pub fn from_value(value: T) -> Self
    where
        T: Encode,
    {
        Self {
            data: value.encode(),
            _phantom: PhantomDataSendSync::new(),
        }
    }
}

impl<T> EncodeAsType for WrapperKeepOpaque<T> {
    fn encode_as_type_to(
        &self,
        type_id: u32,
        types: &scale_info::PortableRegistry,
        out: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error> {
        use scale_encode::error::{Error, ErrorKind, Kind};

        let Some(ty) = types.resolve(type_id) else {
            return Err(Error::new(ErrorKind::TypeNotFound(type_id)))
        };

        // Do a basic check that the target shape lines up.
        let scale_info::TypeDef::Composite(_) = &ty.type_def else {
            return Err(Error::new(ErrorKind::WrongShape {
                actual: Kind::Struct,
                expected: type_id,
            }))
        };

        // Check that the name also lines up.
        if ty.path.ident().as_deref() != Some("WrapperKeepOpaque") {
            return Err(Error::new(ErrorKind::WrongShape {
                actual: Kind::Struct,
                expected: type_id,
            }));
        }

        // Just blat the bytes out.
        self.data.encode_to(out);
        Ok(())
    }
}

pub struct WrapperKeepOpaqueVisitor<T>(std::marker::PhantomData<T>);
impl<T> Visitor for WrapperKeepOpaqueVisitor<T> {
    type Value<'scale, 'info> = WrapperKeepOpaque<T>;
    type Error = scale_decode::Error;

    fn visit_composite<'scale, 'info>(
        self,
        value: &mut scale_decode::visitor::types::Composite<'scale, 'info>,
        _type_id: scale_decode::visitor::TypeId,
    ) -> Result<Self::Value<'scale, 'info>, Self::Error> {
        use scale_decode::error::{Error, ErrorKind};

        if value.path().ident().as_deref() != Some("WrapperKeepOpaque") {
            return Err(Error::new(ErrorKind::Custom(
                "Type to decode is not 'WrapperTypeKeepOpaque'".into(),
            )));
        }
        if value.remaining() != 2 {
            return Err(Error::new(ErrorKind::WrongLength {
                actual_len: value.remaining(),
                expected_len: 2,
            }));
        }

        // The field to decode is a compact len followed by bytes. Decode the length, then grab the bytes.
        let Compact(len) = value
            .decode_item(Compact::<u32>::into_visitor())
            .expect("length checked")?;
        let field = value.next().expect("length checked")?;

        // Sanity check that the compact length we decoded lines up with the number of bytes encoded in the next field.
        if field.bytes().len() != len as usize {
            return Err(Error::new(ErrorKind::Custom("WrapperTypeKeepOpaque compact encoded length doesn't line up with encoded byte len".into())));
        }

        Ok(WrapperKeepOpaque {
            data: field.bytes().to_vec(),
            _phantom: PhantomDataSendSync::new(),
        })
    }
}

impl<T> IntoVisitor for WrapperKeepOpaque<T> {
    type Visitor = WrapperKeepOpaqueVisitor<T>;
    fn into_visitor() -> Self::Visitor {
        WrapperKeepOpaqueVisitor(std::marker::PhantomData)
    }
}

#[cfg(test)]
mod test {
    use scale_decode::DecodeAsType;

    use super::*;

    // Copied from https://github.com/paritytech/substrate/blob/master/frame/support/src/traits/misc.rs
    // and used for tests to check that we can work with the expected TypeInfo without needing to import
    // the frame_support crate, which has quite a lot of dependencies.
    impl<T: scale_info::TypeInfo + 'static> scale_info::TypeInfo for WrapperKeepOpaque<T> {
        type Identity = Self;
        fn type_info() -> scale_info::Type {
            use scale_info::{build::Fields, meta_type, Path, Type, TypeParameter};

            Type::builder()
                .path(Path::new("WrapperKeepOpaque", module_path!()))
                .type_params(vec![TypeParameter::new("T", Some(meta_type::<T>()))])
                .composite(
                    Fields::unnamed()
                        .field(|f| f.compact::<u32>())
                        .field(|f| f.ty::<T>().type_name("T")),
                )
        }
    }

    /// Given a type definition, return type ID and registry representing it.
    fn make_type<T: scale_info::TypeInfo + 'static>() -> (u32, scale_info::PortableRegistry) {
        let m = scale_info::MetaType::new::<T>();
        let mut types = scale_info::Registry::new();
        let id = types.register_type(&m);
        let portable_registry: scale_info::PortableRegistry = types.into();
        (id.id, portable_registry)
    }

    fn roundtrips_like_scale_codec<T>(t: T)
    where
        T: EncodeAsType
            + DecodeAsType
            + Encode
            + Decode
            + PartialEq
            + std::fmt::Debug
            + scale_info::TypeInfo
            + 'static,
    {
        let (type_id, types) = make_type::<T>();

        let scale_codec_encoded = t.encode();
        let encode_as_type_encoded = t.encode_as_type(type_id, &types).unwrap();

        assert_eq!(
            scale_codec_encoded, encode_as_type_encoded,
            "encoded bytes should match"
        );

        let decode_as_type_bytes = &mut &*scale_codec_encoded;
        let decoded_as_type = T::decode_as_type(decode_as_type_bytes, type_id, &types)
            .expect("decode-as-type decodes");

        let decode_scale_codec_bytes = &mut &*scale_codec_encoded;
        let decoded_scale_codec = T::decode(decode_scale_codec_bytes).expect("scale-codec decodes");

        assert!(
            decode_as_type_bytes.is_empty(),
            "no bytes should remain in decode-as-type impl"
        );
        assert!(
            decode_scale_codec_bytes.is_empty(),
            "no bytes should remain in codec-decode impl"
        );

        assert_eq!(
            decoded_as_type, decoded_scale_codec,
            "decoded values should match"
        );
    }

    #[test]
    fn wrapper_keep_opaque_roundtrips_ok() {
        roundtrips_like_scale_codec(WrapperKeepOpaque::from_value(123u64));
        roundtrips_like_scale_codec(WrapperKeepOpaque::from_value(true));
        roundtrips_like_scale_codec(WrapperKeepOpaque::from_value(vec![1u8, 2, 3, 4]));
    }
}
