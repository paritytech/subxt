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
                let tn = if let Some(tn) = field.visit(type_name::GetTypeName::new())? {
                    tn
                } else {
                    "".into()
                };

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
    use scale_info_legacy::LookupName;
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

    impl<R> Visitor for GetTypeName<R>
    where
        R: TypeResolver,
        R::TypeId: TryInto<LookupName>,
    {
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
