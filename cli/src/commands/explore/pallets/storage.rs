use clap::Args;
use color_eyre::eyre::{bail, eyre};
use colored::Colorize;
use indoc::{formatdoc, writedoc};
use scale_typegen_description::type_description;
use scale_value::Value;
use std::fmt::Write;
use std::write;

use subxt::OnlineClient;
use subxt::{
    config::SubstrateConfig,
    metadata::{
        types::{PalletMetadata, StorageEntryType, StorageMetadata},
        Metadata,
    },
};

use crate::utils::{
    create_client, encode_scale_value_as_bytes, first_paragraph_of_docs,
    parse_string_into_scale_value, type_example, FileOrUrl, Indent, SyntaxHighlight,
};

#[derive(Debug, Clone, Args)]
pub struct StorageSubcommand {
    storage_entry: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

pub async fn explore_storage(
    command: StorageSubcommand,
    pallet_metadata: PalletMetadata<'_>,
    metadata: &Metadata,
    file_or_url: FileOrUrl,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name();
    let trailing_args = command.trailing_args.join(" ");
    let trailing_args = trailing_args.trim();

    let Some(storage_metadata) = pallet_metadata.storage() else {
        writeln!(
            output,
            "The \"{pallet_name}\" pallet has no storage entries."
        )?;
        return Ok(());
    };

    let usage = || {
        let storage_entries = storage_entries_string(storage_metadata, pallet_name);
        formatdoc! {"
        Usage:
            subxt explore pallet {pallet_name} storage <STORAGE_ENTRY>
                view details for a specific storage entry
        
        {storage_entries}
        "}
    };

    // if no storage entry specified, show user the calls to choose from:
    let Some(entry_name) = command.storage_entry else {
        writeln!(output, "{}", usage())?;
        return Ok(());
    };

    // if specified call storage entry wrong, show user the storage entries to choose from (but this time as an error):
    let Some(storage) = storage_metadata
        .entries()
        .iter()
        .find(|entry| entry.name().to_lowercase() == entry_name.to_lowercase())
    else {
        bail!(
            "Storage entry \"{entry_name}\" not found in \"{pallet_name}\" pallet!\n\n{}",
            usage()
        );
    };

    let (return_ty_id, key_ty_id) = match storage.entry_type() {
        StorageEntryType::Plain(value) => (*value, None),
        StorageEntryType::Map {
            value_ty, key_ty, ..
        } => (*value_ty, Some(*key_ty)),
    };

    #[allow(non_snake_case)]
    let KEY_VALUE: String = "<KEY_VALUE>".blue().to_string();

    // only inform user about usage if a key can be provided:
    if key_ty_id.is_some() && trailing_args.is_empty() {
        writedoc! {output, "
        Usage:
            subxt explore pallet {pallet_name} storage {entry_name} {KEY_VALUE}
                retrieve a value from storage
        "}?;
    }

    let docs_string = first_paragraph_of_docs(storage.docs()).indent(4);
    if !docs_string.is_empty() {
        writedoc! {output, "

        Description:
        {docs_string}
        "}?;
    }

    let return_ty_description = type_description(return_ty_id, metadata.types(), true)
        .expect("No type Description")
        .indent(4)
        .highlight();

    writedoc! {output, "

    The storage entry has the following shape:
    {return_ty_description}
    "}?;

    // inform user about shape of the key if it can be provided:
    if let Some(key_ty_id) = key_ty_id {
        let key_ty_description = type_description(key_ty_id, metadata.types(), true)
            .expect("No type Description")
            .indent(4)
            .highlight();

        let key_ty_example = type_example(key_ty_id, metadata.types())
            .indent(4)
            .highlight();

        writedoc! {output, "

        The {KEY_VALUE} has the following shape:
        {key_ty_description}

        For example you could provide this {KEY_VALUE}:
        {key_ty_example}
        "}?;
    } else {
        writedoc! {output,"
        
        The storage entry can be accessed without providing a key.
        "}?;
    }

    let storage_entry_keys: Vec<Value> = match (trailing_args.is_empty(), key_ty_id) {
        (false, None) => {
            let warning = "Warning: You submitted a key, but no key is needed: \"{trailing_args}\". To access the storage value, please do not provide any key.".yellow();
            writeln!(output, "{warning}")?;
            return Ok(());
        }
        (true, Some(_)) => {
            // just return. The user was instructed above how to provide a value if they want to.
            return Ok(());
        }
        (true, None) => vec![],
        (false, Some(type_id)) => {
            let value = parse_string_into_scale_value(&trailing_args)?;
            let value_str = value.indent(4);
            writedoc! {output, "
    
            You submitted the following {KEY_VALUE}:
            {value_str}
            "}?;

            let key_bytes = encode_scale_value_as_bytes(&value, type_id, metadata.types())?;
            let bytes_composite = Value::from_bytes(&key_bytes);
            vec![bytes_composite]
        }
    };

    // construct the client:
    let client = create_client(&file_or_url).await?;

    let storage_query = subxt::dynamic::storage(pallet_name, storage.name(), storage_entry_keys);
    let decoded_value_thunk_or_none = client
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;

    let decoded_value_thunk =
        decoded_value_thunk_or_none.ok_or(eyre!("Value not found in storage."))?;

    let value = decoded_value_thunk.to_value()?.indent(4);
    writedoc! {output, "
    
    The value of the storage entry is:
    {value}
    "}?;

    Ok(())
}

fn storage_entries_string(storage_metadata: &StorageMetadata, pallet_name: &str) -> String {
    if storage_metadata.entries().is_empty() {
        format!("No <STORAGE_ENTRY>'s available in the \"{pallet_name}\" pallet.")
    } else {
        let mut output = format!(
            "Available <STORAGE_ENTRY>'s in the \"{}\" pallet:",
            pallet_name
        );
        let mut strings: Vec<_> = storage_metadata
            .entries()
            .iter()
            .map(|s| s.name())
            .collect();
        strings.sort();
        for entry in strings {
            write!(output, "\n    {}", entry).unwrap();
        }
        output
    }
}
