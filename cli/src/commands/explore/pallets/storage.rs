use clap::Args;
use color_eyre::{
    eyre::{bail, eyre},
    owo_colors::OwoColorize,
};
use indoc::{formatdoc, writedoc};
use scale_typegen_description::type_description;
use scale_value::Value;
use std::fmt::Write;
use std::write;
use subxt::metadata::{
    Metadata,
    types::{PalletMetadata, StorageEntryType, StorageMetadata},
};

use crate::utils::{
    FileOrUrl, Indent, SyntaxHighlight, create_client, first_paragraph_of_docs,
    parse_string_into_scale_value, type_example,
};

#[derive(Debug, Clone, Args)]
pub struct StorageSubcommand {
    storage_entry: Option<String>,
    #[clap(long, short, action)]
    execute: bool,
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
    let trailing_args = command.trailing_args;

    let Some(storage_metadata) = pallet_metadata.storage() else {
        writeln!(
            output,
            "The \"{pallet_name}\" pallet has no storage entries."
        )?;
        return Ok(());
    };

    let storage_entry_placeholder = "<STORAGE_ENTRY>".blue();
    let usage = || {
        let storage_entries = storage_entries_string(storage_metadata, pallet_name);
        formatdoc! {"
        Usage:
            subxt explore pallet {pallet_name} storage {storage_entry_placeholder}
                explore a specific storage entry of this pallet

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
        .find(|entry| entry.name().eq_ignore_ascii_case(&entry_name))
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

    let key_value_placeholder = "<KEY_VALUE>".blue();

    let docs_string = first_paragraph_of_docs(storage.docs()).indent(4);
    if !docs_string.is_empty() {
        writedoc! {output, "
        Description:
        {docs_string}

        "}?;
    }

    // only inform user about usage if `execute` flag not provided
    if !command.execute {
        writedoc! {output, "
        Usage:
            subxt explore pallet {pallet_name} storage {entry_name} --execute {key_value_placeholder}
                retrieve a value from storage

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

        The {key_value_placeholder} has the following shape:
        {key_ty_description}

        For example you could provide this {key_value_placeholder}:
        {key_ty_example}
        "}?;
    } else {
        writedoc! {output,"

        Can be accessed without providing a {key_value_placeholder}.
        "}?;
    }

    // if `--execute`/`-e` flag is set, try to execute the storage entry request
    if !command.execute {
        return Ok(());
    }

    let storage_entry_keys: Vec<Value> = match (!trailing_args.is_empty(), key_ty_id.is_some()) {
        // keys provided, keys not needed.
        (true, false) => {
            let trailing_args_str = trailing_args.join(" ");
            let warning = format!(
                "Warning: You submitted one or more keys \"{trailing_args_str}\", but no key is needed. To access the storage value, please do not provide any keys."
            );
            writeln!(output, "{}", warning.yellow())?;
            return Ok(());
        }
        // Keys not provided, keys needed.
        (false, true) => {
            // just return. The user was instructed above how to provide a value if they want to.
            return Ok(());
        }
        // Keys not provided, keys not needed.
        (false, false) => vec![],
        // Keys provided, keys needed.
        (true, true) => {
            // Each trailing arg is parsed into its own value, to be provided as a separate storage key.
            let values = trailing_args
                .iter()
                .map(|arg| parse_string_into_scale_value(arg))
                .collect::<color_eyre::Result<Vec<_>>>()?;

            // We do this just to print them out.
            let values_str = values
                .iter()
                .map(|v| v.to_string().highlight())
                .collect::<Vec<_>>()
                .join("\n");
            let value_str = values_str.indent(4);

            writedoc! {output, "

            You submitted the following {key_value_placeholder}:
            {value_str}
            "}?;

            values
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

    let value = decoded_value_thunk.to_value()?.to_string().highlight();
    writedoc! {output, "

    The value of the storage entry is:
        {value}
    "}?;

    Ok(())
}

fn storage_entries_string(storage_metadata: &StorageMetadata, pallet_name: &str) -> String {
    let storage_entry_placeholder = "<STORAGE_ENTRY>".blue();
    if storage_metadata.entries().is_empty() {
        format!("No {storage_entry_placeholder}'s available in the \"{pallet_name}\" pallet.")
    } else {
        let mut output =
            format!("Available {storage_entry_placeholder}'s in the \"{pallet_name}\" pallet:");
        let mut strings: Vec<_> = storage_metadata
            .entries()
            .iter()
            .map(|s| s.name())
            .collect();
        strings.sort();
        for entry in strings {
            write!(output, "\n    {entry}").unwrap();
        }
        output
    }
}
