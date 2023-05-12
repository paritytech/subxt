use crate::utils::type_description::{format_type_description, TypeDescription};
use crate::utils::type_example::TypeExample;
use crate::utils::FileOrUrl;
use clap::{Args, Parser as ClapParser, Subcommand};
use std::fmt::format;
use std::io::Write;

use codec::Decode;
use color_eyre::eyre::eyre;
use frame_metadata::v15::{
    PalletMetadata, PalletStorageMetadata, RuntimeMetadataV15, StorageEntryType,
};
use frame_metadata::RuntimeMetadataPrefixed;
use scale_info::form::PortableForm;
use scale_info::{PortableRegistry, Type, TypeDef, TypeDefVariant, Variant};
use scale_value::{Composite, ValueDef};
use subxt::storage::DynamicAddress;
use subxt::utils::H256;
use subxt::{config::SubstrateConfig, Metadata, OfflineClient};
use subxt::{tx, OnlineClient};

/// Shows the pallets and calls available for a node and lets you build unsigned extrinsics.
///
/// # Example
///
/// ## Pallets
///
/// Show the pallets that are available:
/// ```
/// subxt explore --file=polkadot_metadata.scale
/// ```
///
/// ## Calls
///
/// Show the calls in a pallet:
/// ```
/// subxt explore Balances calls
/// ```
/// Show the call parameters a call expects:
/// ```
/// subxt explore Balances calls transfer
/// ```
/// Create an unsigned extrinsic from a scale value, validate it and output its hex representation
/// ```
/// subxt explore Grandpa calls note_stalled { "delay": 5, "best_finalized_block_number": 5 }
/// ```
/// ## Constants
///
/// Show the constants in a pallet:
/// ```
/// subxt explore Balances constants
/// ```
/// ## Storage
///
///
#[derive(Debug, ClapParser)]
pub struct Opts {
    #[command(flatten)]
    file_or_url: FileOrUrl,
    pallet: Option<String>,
    #[command(subcommand)]
    pallet_subcommand: Option<PalletSubcommand>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum PalletSubcommand {
    Calls(CallsSubcommand),
    Constants,
    Storage(StorageSubcommand),
}

#[derive(Debug, Clone, Args)]
pub struct CallsSubcommand {
    call: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

#[derive(Debug, Clone, Args)]
pub struct StorageSubcommand {
    storage_entry: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

/// cargo run -- explore --file=../artifacts/polkadot_metadata.scale
pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    // get the metadata
    let bytes = opts.file_or_url.fetch().await?;
    let metadata_prefixed = <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;
    let metadata = Metadata::try_from(metadata_prefixed)?;

    // if no pallet specified, show user the pallets to choose from:
    let Some(pallet_name) = opts.pallet else {
        println!("If you want to explore a pallet: subxt explore <PALLET>");
        println!("{}", print_available_pallets(metadata.runtime_metadata()));
        return Ok(());
    };

    // if specified pallet is wrong, show user the pallets to choose from (but this time as an error):
    let Some(pallet_metadata) = metadata.runtime_metadata().pallets.iter().find(|pallet| pallet.name == pallet_name)else {
        return Err(eyre!("pallet \"{}\" not found in metadata!\n{}", pallet_name, print_available_pallets(metadata.runtime_metadata())));
    };

    // if correct pallet was specified but no subcommand, instruct the user how to proceed:
    let Some(pallet_subcomand) = opts.pallet_subcommand else {
        let docs_string = print_docs_with_indent(&pallet_metadata.docs, 4);
        if !docs_string.is_empty(){
            println!("Pallet \"{pallet_name}\":{docs_string}");
        }
        println!("To explore the \"{pallet_name}\" pallet further, use one of the following:\n\
        subxt explore {pallet_name} calls\n\
        subxt explore {pallet_name} constants\n\
        subxt explore {pallet_name} storage");
        return Ok(());
    };

    match pallet_subcomand {
        PalletSubcommand::Calls(calls_subcommand) => {
            explore_calls(calls_subcommand, &metadata, pallet_metadata)
        }
        PalletSubcommand::Constants => explore_constants(&metadata, pallet_metadata),
        PalletSubcommand::Storage(storage_subcommand) => {
            // if the metadata came from some url, we use that same url to make storage calls against.
            let node_url = opts.file_or_url.url.map(|url| url.to_string());
            explore_storage(storage_subcommand, &metadata, pallet_metadata, node_url).await
        }
    }
}

fn explore_calls(
    calls_subcommand: CallsSubcommand,
    metadata: &Metadata,
    pallet_metadata: &PalletMetadata<PortableForm>,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name.as_str();

    // get the enum that stores the possible calls:
    let (calls_enum_type_def, calls_enum_type) =
        get_calls_enum_type(pallet_metadata, &metadata.runtime_metadata().types)?;

    // if no call specified, show user the calls to choose from:
    let Some(call_name) = calls_subcommand.call else {
        println!("If you want to explore a call: subxt explore {pallet_name} call <CALL>");
        println!("{}", print_available_calls(calls_enum_type_def, pallet_name));
        return Ok(());
    };

    // if specified call is wrong, show user the calls to choose from (but this time as an error):
    let Some(call) = calls_enum_type_def.variants.iter().find(|variant| variant.name == call_name)   else {
        return Err(eyre!("call \"{}\" not found in pallet \"{}\"!\n{}", call_name,  pallet_name, print_available_calls(calls_enum_type_def, pallet_name)));
    };

    // collect all the trailing arguments into a single string that is later into a scale_value::Value
    let trailing_args = calls_subcommand.trailing_args.join(" ");

    // if no trailing arguments specified show user the expected type of arguments with examples:
    if trailing_args.is_empty() {
        let type_and_examples =
            print_type_and_examples(&call.fields, &metadata.runtime_metadata().types)?;
        println!("The call \"{call_name}\" of pallet \"{pallet_name}\" expects a value of type {}::{call_name}\n", calls_enum_type.path);
        println!("You can create an unsigned extrinsic, by providing a scale value of this type to:\n    subxt explore {pallet_name} calls {call_name} <SCALE_VALUE>\n");
        println!("{type_and_examples}");

        return Ok(());
    }

    // parse scale_value from trailing arguments and try to create an unsigned extrinsic with it:
    let value = scale_value::stringify::from_str(&trailing_args).0.map_err(|err| eyre!("scale_value::stringify::from_str led to a ParseError.\n\ntried parsing: \"{}\"\n\n{}", trailing_args, err))?;
    let value_as_composite = value_into_composite(value);
    let offline_client = new_offline_client(metadata.clone());
    let payload = tx::dynamic(pallet_name, call_name, value_as_composite);
    let unsigned_extrinsic = offline_client.tx().create_unsigned(&payload)?;
    let hex_bytes = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));

    println!("Encoded call data:");
    println!("{hex_bytes}");

    Ok(())
}

fn print_available_pallets(metadata_v15: &RuntimeMetadataV15) -> String {
    if metadata_v15.pallets.is_empty() {
        "There are no pallets in this node.".to_string()
    } else {
        let mut output = "Available pallets are:".to_string();
        for pallet in metadata_v15.pallets.iter() {
            output.push_str(format!("\n    {}", pallet.name).as_str())
        }
        output
    }
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

fn print_available_calls(pallet_calls: &TypeDefVariant<PortableForm>, pallet_name: &str) -> String {
    if pallet_calls.variants.is_empty() {
        return format!("The \"{}\" pallet has no calls.", pallet_name);
    }
    let mut output = "Available calls are:".to_string();
    for variant in pallet_calls.variants.iter() {
        output.push_str(format!("\n    {}", variant.name).as_str())
    }
    output
}

fn print_type_and_examples<T>(ty: &T, registry: &PortableRegistry) -> color_eyre::Result<String>
where
    T: TypeExample + TypeDescription,
{
    let type_description = ty.type_description(registry)?;
    let type_description = format_type_description(&type_description);
    let type_examples = ty.type_example(registry).unwrap_or(Vec::new());

    let mut output = String::new();
    output.push_str("The type looks like this:\n");
    output.push_str(type_description.as_str());

    output.push_str("\n\n");
    match type_examples.len() {
        0 => {
            output.push_str(
                "There are no examples available for this type."
                    .to_string()
                    .as_str(),
            );
        }
        1 => {
            output.push_str(
                "Here is an example of this type as a scale value:"
                    .to_string()
                    .as_str(),
            );
        }
        i => {
            output
                .push_str(format!("Here are {i} examples of this type as a scale value:").as_str());
        }
    };

    for self_value in type_examples {
        let value = <T as TypeExample>::upcast(self_value);
        let example_str = scale_value::stringify::to_string(&value);
        output.push('\n');
        output.push_str(example_str.as_str());
    }

    Ok(output)
}

/// composites stay composites, all other types are converted into a 1-fielded unnamed composite
fn value_into_composite(value: scale_value::Value) -> scale_value::Composite<()> {
    match value.value {
        ValueDef::Composite(composite) => composite,
        _ => Composite::Unnamed(vec![value]),
    }
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

fn explore_constants(
    metadata: &Metadata,
    pallet_metadata: &PalletMetadata<PortableForm>,
) -> color_eyre::Result<()> {
    // print all constants in this pallet together with their type, value and the docs as an explanation:
    let pallet_name = pallet_metadata.name.as_str();
    let output = if pallet_metadata.constants.is_empty() {
        format!("The \"{pallet_name}\" pallet has no constants.")
    } else {
        let mut output = format!("The \"{pallet_name}\" pallet has the following constants:");
        for constant in pallet_metadata.constants.iter() {
            let type_description = constant.ty.id.type_description(metadata.types())?;
            let scale_val = scale_value::scale::decode_as_type(
                &mut &constant.value[..],
                constant.ty.id,
                metadata.types(),
            )?;
            let name_and_type = format!(
                "\n\n    {}: {} = {}",
                constant.name,
                type_description,
                scale_value::stringify::to_string(&scale_val)
            );
            output.push_str(name_and_type.as_str());
            output.push_str(print_docs_with_indent(&constant.docs, 8).as_str());
        }
        output
    };
    println!("{output}");
    Ok(())
}

async fn explore_storage(
    storage_subcommand: StorageSubcommand,
    metadata: &Metadata,
    pallet_metadata: &PalletMetadata<PortableForm>,
    custom_online_client_url: Option<String>,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name.as_str();
    let trailing_args = storage_subcommand.trailing_args.join(" ");
    let trailing_args = trailing_args.trim();

    let Some(storage_metadata) = &pallet_metadata.storage else{
        println!("The \"{pallet_name}\" pallet has no storage entries.");
        return Ok(());
    };

    // if no storage entry specified, show user the calls to choose from:
    let Some(entry_name) = storage_subcommand.storage_entry else{
        println!("If you want to explore a storage entry: subxt explore {pallet_name} storage <entry>\n{}", print_available_storage_entries(storage_metadata, pallet_name));
        return Ok(());
    };

    // if specified call storage entry wrong, show user the storage entries to choose from (but this time as an error):
    let Some(storage) = storage_metadata.entries.iter().find(|entry| entry.name == entry_name)   else {
        return Err(eyre!("Storage entry \"{entry_name}\" not found in the \"{pallet_name}\" pallet!\n{}", print_available_storage_entries(storage_metadata, pallet_name)));
    };

    let (return_ty_id, key_ty_id) = match storage.ty {
        StorageEntryType::Plain(value) => (value.id, None),
        StorageEntryType::Map { value, key, .. } => (value.id, Some(key.id)),
    };

    // get the type and type description for the return and key type:
    let mut output =
        format!("Storage entry \"{entry_name}\" in the \"{pallet_name}\" pallet can be accessed ");
    if let Some(key_ty_id) = key_ty_id {
        let key_ty_description_and_example = print_type_and_examples(&key_ty_id, metadata.types())?;
        let key_ty = metadata
            .types()
            .resolve(key_ty_id)
            .ok_or(eyre!("type with id {key_ty_id} not found."))?;
        output.push_str(
            format!(
                "by providing a key of type {}.\n{}",
                key_ty.path, key_ty_description_and_example
            )
            .as_str(),
        );
    } else {
        output.push_str("without providing a key.");
    }

    let return_ty = metadata
        .types()
        .resolve(return_ty_id)
        .ok_or(eyre!("type with id {return_ty_id} not found."))?;
    let return_ty_description_and_example =
        print_type_and_examples(&return_ty_id, metadata.types())?;
    output.push_str(
        format!(
            "\nIt returns a value of type {}\n{}",
            return_ty.path, return_ty_description_and_example
        )
        .as_str(),
    );

    // print docs at the end if there are some:
    let docs_string = print_docs_with_indent(&storage.docs, 4);
    if !docs_string.is_empty() {
        output.push_str(
            format!(
                "Here is some more information about this storage entry:{}",
                docs_string
            )
            .as_str(),
        );
    }

    // construct the scale_value that should be used as a key to the storage (often empty)
    let key_scale_val: scale_value::Value = if trailing_args.is_empty() || key_ty_id.is_none() {
        scale_value::Value {
            value: ValueDef::Composite(scale_value::Composite::Unnamed(Vec::new())),
            context: (),
        }
    } else {
        scale_value::stringify::from_str(trailing_args).0.map_err(|err| eyre!("scale_value::stringify::from_str led to a ParseError.\n\ntried parsing: \"{}\"\n\n{}", trailing_args, err))?
    };

    if key_ty_id.is_none() {
        if !trailing_args.is_empty() {
            output.push_str(format!("\nWarning: You submitted the following value as a key, but it will be ignored, because the storage entry does not require a key: \"{}\"\n", trailing_args).as_str())
        }
    } else {
        let stringified_key = scale_value::stringify::to_string(&key_scale_val);
        output.push_str(
            format!("You submitted the following value as a key: {stringified_key}").as_str(),
        );
    }

    println!("{output}");
    println!(
        "...communicating with node at {}",
        custom_online_client_url
            .as_ref()
            .map(|e| e.as_str())
            .unwrap_or("ws://127.0.0.1:9944")
    );

    // construct and submit the storage query
    let online_client = match custom_online_client_url {
        None => OnlineClient::<SubstrateConfig>::new().await?,
        Some(url) => OnlineClient::<SubstrateConfig>::from_url(url).await?,
    };
    let storage_query = subxt::dynamic::storage(pallet_name, entry_name, vec![key_scale_val]);
    let result = online_client
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let value = result.unwrap().to_value()?;
    dbg!(value);

    Ok(())
}

fn print_available_storage_entries(
    storage_metadata: &PalletStorageMetadata<PortableForm>,
    pallet_name: &str,
) -> String {
    if storage_metadata.entries.is_empty() {
        format!("The \"{}\" pallet has no storage entries.", pallet_name)
    } else {
        let mut output = format!(
            "The \"{}\" pallet has the following storage entries:",
            pallet_name
        );
        for entry in storage_metadata.entries.iter() {
            output.push_str(format!("\n    {}", entry.name).as_str())
        }
        output
    }
}

fn print_docs_with_indent(docs: &[String], indent: usize) -> String {
    let indent = {
        let mut s = String::new();
        for _ in 0..indent {
            s.push(' ');
        }
        s
    };

    let mut output = String::new();
    for doc in docs.iter() {
        let trimmed = doc.trim();
        if !trimmed.is_empty() {
            output.push_str(format!("\n{indent}{trimmed}").as_str());
        }
    }
    output
}
