use clap::Args;
use color_eyre::eyre::eyre;
use scale_info::form::PortableForm;
use scale_info::{PortableRegistry, Type, TypeDef, TypeDefVariant};
use scale_value::{Composite, ValueDef};
use std::str::FromStr;

use subxt::tx;
use subxt::utils::H256;
use subxt::{
    config::SubstrateConfig,
    metadata::{types::PalletMetadata, Metadata},
    OfflineClient,
};

use crate::utils::type_description::print_type_description;
use crate::utils::type_example::print_type_examples;
use crate::utils::with_indent;

#[derive(Debug, Clone, Args)]
pub struct CallsSubcommand {
    call: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

pub fn explore_calls(
    command: CallsSubcommand,
    metadata: &Metadata,
    pallet_metadata: PalletMetadata,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name();

    // get the enum that stores the possible calls:
    let (calls_enum_type_def, _calls_enum_type) =
        get_calls_enum_type(pallet_metadata, metadata.types())?;

    // if no call specified, show user the calls to choose from:
    let Some(call_name) = command.call else {
        let available_calls = print_available_calls(calls_enum_type_def, pallet_name);
        writeln!(output, "Usage:\n    subxt explore {pallet_name} calls <CALL>\n        explore a specific call within this pallet\n\n{available_calls}")?;
        return Ok(());
    };

    // if specified call is wrong, show user the calls to choose from (but this time as an error):
    let Some(call) = calls_enum_type_def.variants.iter().find(|variant| variant.name.to_lowercase() == call_name.to_lowercase()) else {
        let available_calls = print_available_calls(calls_enum_type_def, pallet_name);
        let description = format!("Usage:\n    subxt explore {pallet_name} calls <CALL>\n        explore a specific call within this pallet\n\n{available_calls}", );
        return Err(eyre!("\"{call_name}\" call not found in \"{pallet_name}\" pallet!\n\n{description}"));
    };

    // collect all the trailing arguments into a single string that is later into a scale_value::Value
    let trailing_args = command.trailing_args.join(" ");

    // if no trailing arguments specified show user the expected type of arguments with examples:
    if trailing_args.is_empty() {
        let mut type_description = print_type_description(&call.fields, metadata.types())?;
        type_description = with_indent(type_description, 4);
        let mut type_examples = print_type_examples(&call.fields, metadata.types(), "SCALE_VALUE")?;
        type_examples = with_indent(type_examples, 4);
        writeln!(output, "Usage:")?;
        writeln!(
            output,
            "    subxt explore {pallet_name} calls {call_name} <SCALE_VALUE>"
        )?;
        writeln!(
            output,
            "        construct the call by providing a valid argument\n"
        )?;
        writeln!(
            output,
            "The call expect expects a <SCALE_VALUE> with this shape:\n{type_description}\n\n{}\n\nYou may need to surround the value in single quotes when providing it as an argument."
            , &type_examples[4..])?;
        return Ok(());
    }

    // parse scale_value from trailing arguments and try to create an unsigned extrinsic with it:
    let value = scale_value::stringify::from_str(&trailing_args).0.map_err(|err| eyre!("scale_value::stringify::from_str led to a ParseError.\n\ntried parsing: \"{}\"\n\n{}", trailing_args, err))?;
    let value_as_composite = value_into_composite(value);
    let offline_client = mocked_offline_client(metadata.clone());
    let payload = tx::dynamic(pallet_name, call_name, value_as_composite);
    let unsigned_extrinsic = offline_client.tx().create_unsigned(&payload)?;
    let hex_bytes = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));
    writeln!(output, "Encoded call data:\n    {hex_bytes}")?;
    Ok(())
}

fn print_available_calls(pallet_calls: &TypeDefVariant<PortableForm>, pallet_name: &str) -> String {
    if pallet_calls.variants.is_empty() {
        return format!("No <CALL>'s available in the \"{pallet_name}\" pallet.");
    }
    let mut output = format!("Available <CALL>'s in the \"{pallet_name}\" pallet:");

    let mut strings: Vec<_> = pallet_calls.variants.iter().map(|c| &c.name).collect();
    strings.sort();
    for variant in strings {
        output.push_str("\n    ");
        output.push_str(variant);
    }
    output
}

fn get_calls_enum_type<'a>(
    pallet: PalletMetadata,
    registry: &'a PortableRegistry,
) -> color_eyre::Result<(&'a TypeDefVariant<PortableForm>, &'a Type<PortableForm>)> {
    let call_ty = pallet
        .call_ty_id()
        .ok_or(eyre!("The \"{}\" pallet has no calls.", pallet.name()))?;
    let calls_enum_type = registry
        .resolve(call_ty)
        .ok_or(eyre!("calls type with id {} not found.", call_ty))?;

    // should always be a variant type, where each variant corresponds to one call.
    let TypeDef::Variant(calls_enum_type_def) = &calls_enum_type.type_def else {
        return Err(eyre!("calls type is not a variant"));
    };
    Ok((calls_enum_type_def, calls_enum_type))
}

/// The specific values used for construction do not matter too much, we just need any OfflineClient to create unsigned extrinsics
fn mocked_offline_client(metadata: Metadata) -> OfflineClient<SubstrateConfig> {
    let genesis_hash =
        H256::from_str("91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3")
            .expect("Valid hash; qed");

    let runtime_version = subxt::rpc::types::RuntimeVersion {
        spec_version: 9370,
        transaction_version: 20,
        other: Default::default(),
    };

    OfflineClient::<SubstrateConfig>::new(genesis_hash, runtime_version, metadata)
}

/// composites stay composites, all other types are converted into a 1-fielded unnamed composite
fn value_into_composite(value: scale_value::Value) -> scale_value::Composite<()> {
    match value.value {
        ValueDef::Composite(composite) => composite,
        _ => Composite::Unnamed(vec![value]),
    }
}
