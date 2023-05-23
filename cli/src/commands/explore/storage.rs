use crate::utils::type_description::print_type_description;
use crate::utils::type_example::print_type_examples;
use crate::utils::{print_docs_with_indent, with_indent};
use clap::Args;

use std::fmt::Write;
use std::write;

use color_eyre::eyre::eyre;
use frame_metadata::v15::{PalletMetadata, PalletStorageMetadata, StorageEntryType};

use scale_info::form::PortableForm;

use subxt::OnlineClient;
use subxt::{config::SubstrateConfig, Metadata};

#[derive(Debug, Clone, Args)]
pub struct StorageSubcommand {
    storage_entry: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

pub(crate) async fn explore_storage(
    command: StorageSubcommand,
    metadata: &Metadata,
    pallet_metadata: &PalletMetadata<PortableForm>,
    custom_online_client_url: Option<String>,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name.as_str();
    let trailing_args = command.trailing_args.join(" ");
    let trailing_args = trailing_args.trim();

    let Some(storage_metadata) = &pallet_metadata.storage else {
        println!("The \"{pallet_name}\" pallet has no storage entries.");
        return Ok(());
    };

    // if no storage entry specified, show user the calls to choose from:
    let Some(entry_name) = command.storage_entry else {
        let storage_entries = print_available_storage_entries(storage_metadata, pallet_name);
        println!("Usage:\n    subxt explore {pallet_name} storage <STORAGE_ENTRY>\n        view details for a specific storage entry\n\n{storage_entries}");
        return Ok(());
    };

    // if specified call storage entry wrong, show user the storage entries to choose from (but this time as an error):
    let Some(storage) = storage_metadata.entries.iter().find(|entry| entry.name.to_lowercase() == entry_name.to_lowercase())   else {
        let storage_entries = print_available_storage_entries(storage_metadata, pallet_name);
        let description = format!("Usage:\n    subxt explore {pallet_name} storage <STORAGE_ENTRY>\n        view details for a specific storage entry\n\n{storage_entries}");
        return Err(eyre!("Storage entry \"{entry_name}\" not found in \"{pallet_name}\" pallet!\n\n{description}"));
    };

    let (return_ty_id, key_ty_id) = match storage.ty {
        StorageEntryType::Plain(value) => (value.id, None),
        StorageEntryType::Map { value, key, .. } => (value.id, Some(key.id)),
    };

    // get the type and type description for the return and key type:
    let mut output = String::new();

    // only inform user about usage if a key can be provided:
    if key_ty_id.is_some() && trailing_args.is_empty() {
        write!(
            output,
            "Usage:\n    subxt explore {pallet_name} storage {entry_name} <KEY_VALUE>\n\n"
        )?;
    }

    let docs_string = print_docs_with_indent(&storage.docs, 4);
    if !docs_string.is_empty() {
        write!(output, "Description:\n{docs_string}")?;
    }

    // inform user about shape of key if it can be provided:
    if let Some(key_ty_id) = key_ty_id {
        let mut key_ty_description = print_type_description(&key_ty_id, metadata.types())?;
        key_ty_description = with_indent(key_ty_description, 4);
        let mut key_ty_examples = print_type_examples(&key_ty_id, metadata.types(), "<KEY_VALUE>")?;
        key_ty_examples = with_indent(key_ty_examples, 4);
        write!(
            output,
            "\n\nThe <KEY_VALUE> has the following shape:\n    {key_ty_description}\n\n{}",
            &key_ty_examples[4..]
        )?;
    } else {
        write!(
            output,
            "\n\nThe constant can be accessed without providing a key."
        )?;
    }

    let mut return_ty_description = print_type_description(&return_ty_id, metadata.types())?;
    return_ty_description = if return_ty_description.contains('\n') {
        format!("\n{}", with_indent(return_ty_description, 4))
    } else {
        return_ty_description
    };
    write!(
        output,
        "\n\nThe storage entry has the following shape: {}",
        return_ty_description
    )?;

    // construct the vector of scale_values that should be used as a key to the storage (often empty)

    let key_scale_values = if let Some(key_ty_id) = key_ty_id.filter(|_| !trailing_args.is_empty())
    {
        let key_scale_value = scale_value::stringify::from_str(trailing_args).0.map_err(|err| eyre!("scale_value::stringify::from_str led to a ParseError.\n\ntried parsing: \"{}\"\n\n{}", trailing_args, err))?;
        write!(
            output,
            "\n\nYou submitted the following value as a key:\n{}",
            with_indent(scale_value::stringify::to_string(&key_scale_value), 4)
        )?;
        let mut key_bytes: Vec<u8> = Vec::new();
        scale_value::scale::encode_as_type(
            &key_scale_value,
            key_ty_id,
            metadata.types(),
            &mut key_bytes,
        )?;
        let bytes_composite = scale_value::Value::from_bytes(&key_bytes);
        vec![bytes_composite]
    } else {
        Vec::new()
    };

    if key_ty_id.is_none() && !trailing_args.is_empty() {
        write!(output, "\n\nWarning: You submitted the following value as a key, but it will be ignored, because the storage entry does not require a key: \"{}\"", trailing_args)?;
    }
    println!("{output}");

    // construct and submit the storage entry request if either no key is needed or som key was provided as a scale value
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

        let decoded_value_thunk =
            decoded_value_thunk_or_none.ok_or(eyre!("Value not found in storage."))?;

        let value = decoded_value_thunk.to_value()?;
        let mut value_string = scale_value::stringify::to_string(&value);
        value_string = with_indent(value_string, 4);
        println!("\nThe value of the storage entry is:\n{value_string}");
    }

    Ok(())
}

fn print_available_storage_entries(
    storage_metadata: &PalletStorageMetadata<PortableForm>,
    pallet_name: &str,
) -> String {
    if storage_metadata.entries.is_empty() {
        format!("No <STORAGE_ENTRY>'s available in the \"{pallet_name}\" pallet.")
    } else {
        let mut output = format!(
            "Available <STORAGE_ENTRY>'s in the \"{}\" pallet:",
            pallet_name
        );
        let mut strings: Vec<_> = storage_metadata.entries.iter().map(|s| &s.name).collect();
        strings.sort();
        for entry in strings {
            write!(output, "\n    {}", entry).unwrap();
        }
        output
    }
}
