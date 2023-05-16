use crate::utils::type_description::print_type_description;
use crate::utils::type_example::print_type_examples;
use crate::utils::{with_indent, FileOrUrl};
use clap::{Args, Parser as ClapParser, Subcommand};

use std::fmt::Write;
use std::write;

use codec::Decode;
use color_eyre::eyre::eyre;
use frame_metadata::v15::{
    PalletMetadata, PalletStorageMetadata, RuntimeMetadataV15, StorageEntryType,
};
use frame_metadata::RuntimeMetadataPrefixed;
use scale_info::form::PortableForm;
use scale_info::{PortableRegistry, Type, TypeDef, TypeDefVariant};
use scale_value::{Composite, ValueDef};

use subxt::utils::H256;
use subxt::{config::SubstrateConfig, Metadata, OfflineClient};
use subxt::{tx, OnlineClient};

#[derive(Debug, Clone, Args)]
pub struct CallsSubcommand {
    call: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

pub(crate) fn explore_calls(
    command: CallsSubcommand,
    metadata: &Metadata,
    pallet_metadata: &PalletMetadata<PortableForm>,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name.as_str();

    // get the enum that stores the possible calls:
    let (calls_enum_type_def, calls_enum_type) =
        get_calls_enum_type(pallet_metadata, &metadata.runtime_metadata().types)?;

    // if no call specified, show user the calls to choose from:
    let Some(call_name) = command.call else {
        let available_calls = print_available_calls(calls_enum_type_def, pallet_name);
        println!("Usage:\n    subxt explore {pallet_name} calls <CALL>\n        explore a specific call within this pallet\n\n{available_calls}", );
        return Ok(());
    };

    // if specified call is wrong, show user the calls to choose from (but this time as an error):
    let Some(call) = calls_enum_type_def.variants.iter().find(|variant| variant.name.to_lowercase() == call_name.to_lowercase())   else {
        let available_calls = print_available_calls(calls_enum_type_def, pallet_name);
        let description = format!("Usage:\n    subxt explore {pallet_name} calls <CALL>\n        explore a specific call within this pallet\n\n{available_calls}", );
        return Err(eyre!("\"{call_name}\" call not found in \"{pallet_name}\" pallet!\n\n{description}"));
    };

    // collect all the trailing arguments into a single string that is later into a scale_value::Value
    let trailing_args = command.trailing_args.join(" ");

    // if no trailing arguments specified show user the expected type of arguments with examples:
    if trailing_args.is_empty() {
        let mut type_description =
            print_type_description(&call.fields, &metadata.runtime_metadata().types)?;
        type_description = with_indent(type_description, 4);
        let mut type_examples = print_type_examples(
            &call.fields,
            &metadata.runtime_metadata().types,
            "SCALE_VALUE",
        )?;
        type_examples = with_indent(type_examples, 4);
        let mut output = String::new();
        write!(output, "Usage:\n    subxt explore {pallet_name} calls {call_name} <SCALE_VALUE>\n        construct the call by providing a valid argument\n\n")?;
        write!(
            output,
            "The call expect expects a <SCALE_VALUE> with this shape:\n{type_description}\n\n{}\n\nYou may need to surround the value in single quotes when providing it as an argument."
            , &type_examples[4..])?;
        println!("{output}");
        return Ok(());
    }

    // parse scale_value from trailing arguments and try to create an unsigned extrinsic with it:
    let value = scale_value::stringify::from_str(&trailing_args).0.map_err(|err| eyre!("scale_value::stringify::from_str led to a ParseError.\n\ntried parsing: \"{}\"\n\n{}", trailing_args, err))?;
    let value_as_composite = value_into_composite(value);
    let offline_client = new_offline_client(metadata.clone());
    let payload = tx::dynamic(pallet_name, call_name, value_as_composite);
    let unsigned_extrinsic = offline_client.tx().create_unsigned(&payload)?;
    let hex_bytes = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));
    println!("Encoded call data:\n    {hex_bytes}");

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
        write!(output, "\n    {}", variant).unwrap();
    }
    output
}

fn get_calls_enum_type<'a>(
    pallet: &'a frame_metadata::v15::PalletMetadata<PortableForm>,
    registry: &'a PortableRegistry,
) -> color_eyre::Result<(&'a TypeDefVariant<PortableForm>, &'a Type<PortableForm>)> {
    let calls = pallet
        .calls
        .as_ref()
        .ok_or(eyre!("The \"{}\" pallet has no calls.", pallet.name))?;
    let calls_enum_type = registry
        .resolve(calls.ty.id)
        .ok_or(eyre!("calls type with id {} not found.", calls.ty.id))?;
    // should always be a variant type, where each variant corresponds to one call.
    let calls_enum_type_def = match &calls_enum_type.type_def {
        TypeDef::Variant(variant) => variant,
        _ => {
            return Err(eyre!("calls type is not a variant"));
        }
    };
    Ok((calls_enum_type_def, calls_enum_type))
}

fn new_offline_client(metadata: Metadata) -> OfflineClient<SubstrateConfig> {
    let genesis_hash = {
        let h = "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";
        let bytes = hex::decode(h).unwrap();
        H256::from_slice(&bytes)
    };

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
