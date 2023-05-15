use crate::utils::type_description::{print_description, TypeDescription};
use crate::utils::type_example::{print_examples};
use crate::utils::FileOrUrl;
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
/// # Encoded call data:
/// # 0x2c0411020500000005000000
/// subxt explore Balances calls transfer  "{ \"dest\": v\"Raw\"((255, 255, 255)), \"value\": 0 }"
/// # Encoded call data:
/// # 0x24040607020cffffff00
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
        let available_pallets = print_available_pallets(metadata.runtime_metadata());
        println!("{available_pallets}\n\nIf you want to explore a pallet:\n  - subxt explore <PALLET>", );
        return Ok(());
    };

    // if specified pallet is wrong, show user the pallets to choose from (but this time as an error):
    let Some(pallet_metadata) = metadata.runtime_metadata().pallets.iter().find(|pallet| pallet.name == pallet_name)else {
        return Err(eyre!("pallet \"{}\" not found in metadata!\n{}", pallet_name, print_available_pallets(metadata.runtime_metadata())));
    };

    // if correct pallet was specified but no subcommand, instruct the user how to proceed:
    let Some(pallet_subcomand) = opts.pallet_subcommand else {
        let docs_string = print_docs_with_indent(&pallet_metadata.docs, 4);
        if !docs_string.is_empty() {
            // currently it seems like all doc strings are empty
            println!("Pallet \"{pallet_name}\":\n{docs_string}");
        }
        println!("To explore the \"{pallet_name}\" pallet further, use one of the following:\n\
          - subxt explore {pallet_name} calls\n\
          - subxt explore {pallet_name} constants\n\
          - subxt explore {pallet_name} storage");
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
        let available_calls = print_available_calls(calls_enum_type_def, pallet_name);
        println!("{available_calls}\n\nIf you want to explore a call: \n  - subxt explore {pallet_name} calls <CALL>", );
        return Ok(());
    };

    // if specified call is wrong, show user the calls to choose from (but this time as an error):
    let Some(call) = calls_enum_type_def.variants.iter().find(|variant| variant.name == call_name)   else {
        return Err(eyre!("\"{call_name}\" call not found in \"{pallet_name}\" pallet!\n{}", print_available_calls(calls_enum_type_def, pallet_name)));
    };

    // collect all the trailing arguments into a single string that is later into a scale_value::Value
    let trailing_args = calls_subcommand.trailing_args.join(" ");

    // if no trailing arguments specified show user the expected type of arguments with examples:
    if trailing_args.is_empty() {
        let mut type_description =
            print_description(&call.fields, &metadata.runtime_metadata().types)?;
        type_description = with_indent(type_description, 4);
        let mut type_examples = print_examples(&call.fields, &metadata.runtime_metadata().types)?;
        type_examples = with_indent(type_examples, 4);
        let mut output = String::new();
        write!(output, "Call \"{call_name}\" in \"{pallet_name}\" pallet:\n  - expects a value of type {}::{call_name}\n", calls_enum_type.path)?;
        write!(
            output,
            "  - The type looks like this:\n{type_description}\n{type_examples}"
        )?;
        write!(output, "\n\nYou can create an unsigned extrinsic, by providing a scale value of this type to:\n  - subxt explore {pallet_name} calls {call_name} <SCALE_VALUE>\n")?;
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

    println!("Encoded call data:");
    println!("{hex_bytes}");

    Ok(())
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
                "\n  - {}: {} = {}",
                constant.name,
                type_description,
                scale_value::stringify::to_string(&scale_val)
            );
            write!(
                output,
                "{}\n{}",
                name_and_type,
                print_docs_with_indent(&constant.docs, 8)
            )?;
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

    let Some(storage_metadata) = &pallet_metadata.storage else {
        println!("The \"{pallet_name}\" pallet has no storage entries.");
        return Ok(());
    };

    // if no storage entry specified, show user the calls to choose from:
    let Some(entry_name) = storage_subcommand.storage_entry else {
        let storage_entries = print_available_storage_entries(storage_metadata, pallet_name);
        println!("{storage_entries}\n\nIf you want to explore a storage entry:\n  - subxt explore {pallet_name} storage <STORAGE_ENTRY>", );
        return Ok(());
    };

    // if specified call storage entry wrong, show user the storage entries to choose from (but this time as an error):
    let Some(storage) = storage_metadata.entries.iter().find(|entry| entry.name == entry_name)   else {
        return Err(eyre!("Storage entry \"{entry_name}\" not found in \"{pallet_name}\" pallet!\n{}", print_available_storage_entries(storage_metadata, pallet_name)));
    };

    let (return_ty_id, key_ty_id) = match storage.ty {
        StorageEntryType::Plain(value) => (value.id, None),
        StorageEntryType::Map { value, key, .. } => (value.id, Some(key.id)),
    };

    // get the type and type description for the return and key type:
    let mut output = format!("Storage entry \"{entry_name}\" in \"{pallet_name}\" pallet:");

    let docs_string = print_docs_with_indent(&storage.docs, 4);
    if !docs_string.is_empty() {
        write!(output, "\n{docs_string}")?;
    }

    if let Some(key_ty_id) = key_ty_id {
        let key_ty_description = print_description(&key_ty_id, metadata.types())?;
        write!(
            output,
            "\n  - Can be accessed by providing a key of type: {}",
            key_ty_description
        )?;
        let mut key_ty_examples = print_examples(&key_ty_id, metadata.types())?;
        key_ty_examples = with_indent_and_first_dash(key_ty_examples, 4);
        write!(output, "\n{}", key_ty_examples)?;
    } else {
        write!(output, "\n  - Can be accessed without providing a key.")?;
    }

    let mut return_ty_description = print_description(&return_ty_id, metadata.types())?;
    return_ty_description = if return_ty_description.contains('\n') {
        format!("\n{}", with_indent(return_ty_description, 4))
    } else {
        return_ty_description
    };
    write!(
        output,
        "\n  - The storage entry \"{entry_name}\" has a value of type: {}",
        return_ty_description
    )?;

    // construct the vector of scale_values that should be used as a key to the storage (often empty)
    let key_scale_values: Vec<scale_value::Value> = if trailing_args.is_empty()
        || key_ty_id.is_none()
    {
        Vec::new()
    } else {
        let key_scale_value = scale_value::stringify::from_str(trailing_args).0.map_err(|err| eyre!("scale_value::stringify::from_str led to a ParseError.\n\ntried parsing: \"{}\"\n\n{}", trailing_args, err))?;
        let stringified_key = scale_value::stringify::to_string(&key_scale_value);
        write!(
            output,
            "\nYou submitted the following value as a key: {stringified_key}"
        )?;
        let scale_val_as_composite = value_into_composite(key_scale_value);
        match scale_val_as_composite {
            Composite::Named(e) => e.into_iter().map(|(_s, v)| v).collect(),
            Composite::Unnamed(e) => e,
        }
    };

    if key_ty_id.is_none() && !trailing_args.is_empty() {
        write!(output, "\nWarning: You submitted the following value as a key, but it will be ignored, because the storage entry does not require a key: \"{}\"\n", trailing_args)?;
    }
    println!("{output}");
    // construct and submit the storage query if either no key is needed or som key was provided as a scale value

    if key_ty_id.is_none() || !key_scale_values.is_empty() {
        let online_client = match custom_online_client_url {
            None => OnlineClient::<SubstrateConfig>::new().await?,
            Some(url) => OnlineClient::<SubstrateConfig>::from_url(url).await?,
        };
        let storage_query = subxt::dynamic::storage(pallet_name, entry_name, key_scale_values);
        let decoded_value_thunk_or_none = online_client
            .storage()
            .at_latest()
            .await?
            .fetch(&storage_query)
            .await?;
        let decoded_value_thunk = decoded_value_thunk_or_none.ok_or(eyre!("DecodedValueThunk was None"))?;

        let value = decoded_value_thunk.to_value()?;
        let value_string = scale_value::stringify::to_string(&value);
        println!("\nValue in storage: {value_string}");
    } else {
        println!("\nIf you want to get the value of storage entry \"{entry_name}\" in pallet \"{pallet_name}\":\n  - subxt explore {pallet_name} storage {entry_name} <KEY_SCALE_VALUE>", );
    }

    Ok(())
}

fn print_available_pallets(metadata_v15: &RuntimeMetadataV15) -> String {
    if metadata_v15.pallets.is_empty() {
        "There are no pallets in this node.".to_string()
    } else {
        let mut output = "Available pallets are:".to_string();
        for pallet in metadata_v15.pallets.iter() {
            write!(output, "\n  - {}", pallet.name).unwrap();
        }
        output
    }
}

fn print_available_calls(pallet_calls: &TypeDefVariant<PortableForm>, pallet_name: &str) -> String {
    if pallet_calls.variants.is_empty() {
        return format!("The \"{}\" pallet has no calls.", pallet_name);
    }
    let mut output = format!("Calls in \"{pallet_name}\" pallet:");
    for variant in pallet_calls.variants.iter() {
        write!(output, "\n  - {}", variant.name).unwrap();
    }
    output
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
            write!(output, "\n  - {}", entry.name).unwrap();
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

fn print_docs_with_indent(docs: &[String], indent: usize) -> String {
    // take at most the first paragraph of documentation, such that it does not get too long.
    let docs_str = docs
        .iter()
        .map(|e| e.trim())
        .take_while(|e| !e.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    with_indent_and_first_dash(docs_str, indent)
}

fn with_indent(s: String, indent: usize) -> String {
    let indent = make_indent(indent);
    s.lines()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn with_indent_and_first_dash(s: String, indent: usize) -> String {
    let blank_indent = make_indent(indent);
    s.lines()
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                format!("{}- {line}", make_indent(indent - 2))
            } else {
                format!("{blank_indent}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn make_indent(indent: usize) -> String {
    let mut s = String::new();
    for _ in 0..indent {
        s.push(' ');
    }
    s
}
