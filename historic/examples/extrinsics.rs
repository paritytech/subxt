#![allow(missing_docs)]
use subxt_historic::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error + Send + Sync + 'static>> {
    // Configuration for the Polkadot relay chain.
    let config = PolkadotConfig::new();

    // Create an online client for the Polkadot relay chain, pointed at a Polkadot archive node.
    let client = OnlineClient::from_url(config, "wss://rpc.polkadot.io").await?;

    // Iterate through some randomly selected old blocks to show how to fetch and decode extrinsics.
    for block_number in 1234567.. {
        println!("=== Block {block_number} ===");

        // Point the client at a specific block number. By default this will download and cache
        // metadata for the required spec version (so it's cheaper to instantiate again), if it
        // hasn't already, and borrow the relevant legacy types from the client.
        let client_at_block = client.at(block_number).await?;

        // Fetch the extrinsics at that block.
        let extrinsics = client_at_block.extrinsics().fetch().await?;

        // Now, we have various operations to work with them. Here we print out various details
        // about each extrinsic.
        for extrinsic in extrinsics.iter() {
            println!(
                "{}.{}",
                extrinsic.call().pallet_name(),
                extrinsic.call().name()
            );

            if let Some(signature) = extrinsic.signature_bytes() {
                println!("  Signature: 0x{}", hex::encode(signature));
            }

            println!("  Call Data:");

            // We can decode each of the fields (in this example we decode everything into a
            // scale_value::Value type, which can represent any SCALE encoded data, but if you
            // have an idea of the type then you can try to decode into that type instead):
            for field in extrinsic.call().fields().iter() {
                // We can visit fields, which gives us the ability to inspect and decode information
                // from them selectively, returning whatever we like from it. Here we demo our
                // type name visitor which is defined below:
                let tn = field
                    .visit(type_name::GetTypeName::new())?
                    .unwrap_or_default();

                // When visiting fields we can also decode into a custom shape like so:
                let _custom_value =
                    field.visit(value::GetValue::new(&client_at_block.resolver()))?;

                // We can also obtain and decode things without the complexity of the above:
                println!(
                    "    {}: {} {}",
                    field.name(),
                    field.decode_as::<scale_value::Value>().unwrap(),
                    if tn.is_empty() {
                        String::new()
                    } else {
                        format!("(type name: {tn})")
                    },
                );
            }

            // Or, all of them at once:
            println!(
                "    All: {}",
                extrinsic
                    .call()
                    .fields()
                    .decode_as::<scale_value::Composite<_>>()
                    .unwrap()
            );

            // We can also look at things like the transaction extensions:
            if let Some(extensions) = extrinsic.transaction_extensions() {
                println!("  Transaction Extensions:");

                // We can decode each of them:
                for extension in extensions.iter() {
                    println!(
                        "    {}: {}",
                        extension.name(),
                        extension.decode_as::<scale_value::Value>().unwrap()
                    );
                }

                // Or all of them at once:
                println!(
                    "    All: {}",
                    extensions.decode_as::<scale_value::Composite<_>>().unwrap()
                );
            }
        }
    }

    Ok(())
}

/// This module defines an example visitor which retrieves the name of a type.
/// This is a more advanced use case and can typically be avoided.
mod type_name {
    use scale_decode::{
        Visitor,
        visitor::types::{Composite, Sequence, Variant},
        visitor::{TypeIdFor, Unexpected},
    };
    use scale_type_resolver::TypeResolver;

    /// This is a visitor which obtains type names.
    pub struct GetTypeName<R> {
        marker: core::marker::PhantomData<R>,
    }

    impl<R> GetTypeName<R> {
        /// Construct our TypeName visitor.
        pub fn new() -> Self {
            GetTypeName {
                marker: core::marker::PhantomData,
            }
        }
    }

    impl<R: TypeResolver> Visitor for GetTypeName<R> {
        type Value<'scale, 'resolver> = Option<&'resolver str>;
        type Error = scale_decode::Error;
        type TypeResolver = R;

        // Look at the path of types that have paths and return the ident from that.
        fn visit_composite<'scale, 'resolver>(
            self,
            value: &mut Composite<'scale, 'resolver, Self::TypeResolver>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(value.path().last())
        }
        fn visit_variant<'scale, 'resolver>(
            self,
            value: &mut Variant<'scale, 'resolver, Self::TypeResolver>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(value.path().last())
        }
        fn visit_sequence<'scale, 'resolver>(
            self,
            value: &mut Sequence<'scale, 'resolver, Self::TypeResolver>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(value.path().last())
        }

        // Else, we return nothing as we can't find a name for the type.
        fn visit_unexpected<'scale, 'resolver>(
            self,
            _unexpected: Unexpected,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(None)
        }
    }
}

/// This visitor demonstrates how to decode and return a custom Value shape
mod value {
    use scale_decode::{
        Visitor,
        visitor::TypeIdFor,
        visitor::types::{Array, BitSequence, Composite, Sequence, Str, Tuple, Variant},
    };
    use scale_type_resolver::TypeResolver;
    use std::collections::HashMap;

    /// A value type we're decoding into.
    #[allow(dead_code)]
    pub enum Value {
        Number(f64),
        BigNumber(String),
        Bool(bool),
        Char(char),
        Array(Vec<Value>),
        String(String),
        Address(Vec<u8>),
        I256([u8; 32]),
        U256([u8; 32]),
        Struct(HashMap<String, Value>),
        VariantWithoutData(String),
        VariantWithData(String, VariantFields),
    }

    pub enum VariantFields {
        Unnamed(Vec<Value>),
        Named(HashMap<String, Value>),
    }

    /// An error we can encounter trying to decode things into a [`Value`]
    #[derive(Debug, thiserror::Error)]
    pub enum ValueError {
        #[error("Decode error: {0}")]
        Decode(#[from] scale_decode::visitor::DecodeError),
        #[error("Cannot decode bit sequence: {0}")]
        CannotDecodeBitSequence(codec::Error),
        #[error("Cannot resolve variant type information: {0}")]
        CannotResolveVariantType(String),
    }

    /// This is a visitor which obtains type names.
    pub struct GetValue<'r, R> {
        resolver: &'r R,
    }

    impl<'r, R> GetValue<'r, R> {
        /// Construct our TypeName visitor.
        pub fn new(resolver: &'r R) -> Self {
            GetValue { resolver }
        }
    }

    impl<'r, R: TypeResolver> Visitor for GetValue<'r, R> {
        type Value<'scale, 'resolver> = Value;
        type Error = ValueError;
        type TypeResolver = R;

        fn visit_i256<'resolver>(
            self,
            value: &[u8; 32],
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'_, 'resolver>, Self::Error> {
            Ok(Value::I256(*value))
        }

        fn visit_u256<'resolver>(
            self,
            value: &[u8; 32],
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'_, 'resolver>, Self::Error> {
            Ok(Value::U256(*value))
        }

        fn visit_i128<'scale, 'resolver>(
            self,
            value: i128,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            let attempt = value as f64;
            if attempt as i128 == value {
                Ok(Value::Number(attempt))
            } else {
                Ok(Value::BigNumber(value.to_string()))
            }
        }

        fn visit_i64<'scale, 'resolver>(
            self,
            value: i64,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            self.visit_i128(value.into(), type_id)
        }

        fn visit_i32<'scale, 'resolver>(
            self,
            value: i32,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            self.visit_i128(value.into(), type_id)
        }

        fn visit_i16<'scale, 'resolver>(
            self,
            value: i16,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            self.visit_i128(value.into(), type_id)
        }

        fn visit_i8<'scale, 'resolver>(
            self,
            value: i8,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            self.visit_i128(value.into(), type_id)
        }

        fn visit_u128<'scale, 'resolver>(
            self,
            value: u128,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            let attempt = value as f64;
            if attempt as u128 == value {
                Ok(Value::Number(attempt))
            } else {
                Ok(Value::BigNumber(value.to_string()))
            }
        }

        fn visit_u64<'scale, 'resolver>(
            self,
            value: u64,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            self.visit_u128(value.into(), type_id)
        }

        fn visit_u32<'scale, 'resolver>(
            self,
            value: u32,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            self.visit_u128(value.into(), type_id)
        }

        fn visit_u16<'scale, 'resolver>(
            self,
            value: u16,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            self.visit_u128(value.into(), type_id)
        }

        fn visit_u8<'scale, 'resolver>(
            self,
            value: u8,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            self.visit_u128(value.into(), type_id)
        }

        fn visit_bool<'scale, 'resolver>(
            self,
            value: bool,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(Value::Bool(value))
        }

        fn visit_char<'scale, 'resolver>(
            self,
            value: char,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(Value::Char(value))
        }

        fn visit_array<'scale, 'resolver>(
            self,
            values: &mut Array<'scale, 'resolver, Self::TypeResolver>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(Value::Array(to_array(
                self.resolver,
                values.remaining(),
                values,
            )?))
        }

        fn visit_sequence<'scale, 'resolver>(
            self,
            values: &mut Sequence<'scale, 'resolver, Self::TypeResolver>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(Value::Array(to_array(
                self.resolver,
                values.remaining(),
                values,
            )?))
        }

        fn visit_str<'scale, 'resolver>(
            self,
            value: &mut Str<'scale>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(Value::String(value.as_str()?.to_owned()))
        }

        fn visit_tuple<'scale, 'resolver>(
            self,
            values: &mut Tuple<'scale, 'resolver, Self::TypeResolver>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            Ok(Value::Array(to_array(
                self.resolver,
                values.remaining(),
                values,
            )?))
        }

        fn visit_bitsequence<'scale, 'resolver>(
            self,
            value: &mut BitSequence<'scale>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            let bits = value.decode()?;
            let mut out = Vec::with_capacity(bits.len());
            for b in bits {
                let b = b.map_err(ValueError::CannotDecodeBitSequence)?;
                out.push(Value::Bool(b));
            }
            Ok(Value::Array(out))
        }

        fn visit_composite<'scale, 'resolver>(
            self,
            value: &mut Composite<'scale, 'resolver, Self::TypeResolver>,
            _type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            // Special case for ss58 addresses:
            if let Some(n) = value.name()
                && n == "AccountId32"
                && value.bytes_from_start().len() == 32
            {
                return Ok(Value::Address(value.bytes_from_start().to_vec()));
            }

            // Reuse logic for decoding variant fields:
            match to_variant_fieldish(self.resolver, value)? {
                VariantFields::Named(s) => Ok(Value::Struct(s)),
                VariantFields::Unnamed(a) => Ok(Value::Array(a)),
            }
        }

        fn visit_variant<'scale, 'resolver>(
            self,
            value: &mut Variant<'scale, 'resolver, Self::TypeResolver>,
            type_id: TypeIdFor<Self>,
        ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
            // Because we have access to a type resolver on self, we can
            // look up the type IDs we're given back and base decode decisions
            // on them. here we see whether the enum type has any data attached:
            let has_data_visitor = scale_type_resolver::visitor::new((), |_, _| false)
                .visit_variant(|_, _, variants| {
                    for mut variant in variants {
                        if variant.fields.next().is_some() {
                            return true;
                        }
                    }
                    false
                });

            // Do any variants have data in this enum type?
            let has_data = self
                .resolver
                .resolve_type(type_id, has_data_visitor)
                .map_err(|e| ValueError::CannotResolveVariantType(e.to_string()))?;

            let name = value.name().to_owned();

            // base our decoding on whether any data in enum type.
            if has_data {
                let fields = to_variant_fieldish(self.resolver, value.fields())?;
                Ok(Value::VariantWithData(name, fields))
            } else {
                Ok(Value::VariantWithoutData(name))
            }
        }
    }

    fn to_variant_fieldish<'r, 'scale, 'resolver, R: TypeResolver>(
        resolver: &'r R,
        value: &mut Composite<'scale, 'resolver, R>,
    ) -> Result<VariantFields, ValueError> {
        // If fields are unnamed, treat as array:
        if value.fields().iter().all(|f| f.name.is_none()) {
            return Ok(VariantFields::Unnamed(to_array(
                resolver,
                value.remaining(),
                value,
            )?));
        }

        // Otherwise object:
        let mut out = HashMap::new();
        for field in value {
            let field = field?;
            let name = field.name().unwrap().to_string();
            let value = field.decode_with_visitor(GetValue::new(resolver))?;
            out.insert(name, value);
        }
        Ok(VariantFields::Named(out))
    }

    fn to_array<'r, 'scale, 'resolver, R: TypeResolver>(
        resolver: &'r R,
        len: usize,
        mut values: impl scale_decode::visitor::DecodeItemIterator<'scale, 'resolver, R>,
    ) -> Result<Vec<Value>, ValueError> {
        let mut out = Vec::with_capacity(len);
        while let Some(value) = values.decode_item(GetValue::new(resolver)) {
            out.push(value?);
        }
        Ok(out)
    }
}
