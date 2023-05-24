use crate::utils::{print_docs_with_indent, FileOrUrl};
use clap::{Parser as ClapParser, Subcommand};

use std::fmt::Write;

use std::write;

use codec::Decode;
use color_eyre::eyre::eyre;

use crate::commands::explore::calls::{explore_calls, CallsSubcommand};
use crate::commands::explore::constants::{explore_constants, ConstantsSubcommand};
use crate::commands::explore::storage::{explore_storage, StorageSubcommand};

use subxt::Metadata;

mod calls;
mod constants;
mod storage;

/// Explore pallets, calls, call parameters, storage entries and constants. Also allows for creating (unsigned) extrinsics.
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
/// Show the storage entries in a pallet
/// ```
/// subxt explore Alliance storage
/// ```
/// Show the types and value of a specific storage entry
/// ```
/// subxt explore Alliance storage Announcements [KEY_SCALE_VALUE]
/// ```
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
    Constants(ConstantsSubcommand),
    Storage(StorageSubcommand),
}

/// cargo run -- explore --file=../artifacts/polkadot_metadata.scale
pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    // get the metadata
    let bytes = opts.file_or_url.fetch().await?;
    let metadata = Metadata::decode(&mut &bytes[..])?;

    // if no pallet specified, show user the pallets to choose from:
    let Some(pallet_name) = opts.pallet else {
        let available_pallets = print_available_pallets(&metadata);
        println!("Usage:\n    subxt explore <PALLET>\n        explore a specific pallet\n\n{available_pallets}", );
        return Ok(());
    };

    // if specified pallet is wrong, show user the pallets to choose from (but this time as an error):
    let Some(pallet_metadata) = metadata.pallets().find(|pallet| pallet.name().to_lowercase() == pallet_name.to_lowercase()) else {
        return Err(eyre!("pallet \"{}\" not found in metadata!\n{}", pallet_name, print_available_pallets(&metadata)));
    };

    // if correct pallet was specified but no subcommand, instruct the user how to proceed:
    let Some(pallet_subcomand) = opts.pallet_subcommand else {
        let docs_string = print_docs_with_indent(pallet_metadata.docs(), 4);
        let mut output = String::new();
        if !docs_string.is_empty() {
            write!(output, "Description:\n{docs_string}")?;
        }
        write!(output, "Usage:")?;
        write!(output, "\n    subxt explore {pallet_name} calls\n        explore the calls that can be made into this pallet")?;
        write!(output, "\n    subxt explore {pallet_name} constants\n        explore the constants held in this pallet")?;
        write!(output, "\n    subxt explore {pallet_name} storage\n        explore the storage values held in this pallet")?;
        println!("{output}");
        return Ok(());
    };

    match pallet_subcomand {
        PalletSubcommand::Calls(command) => explore_calls(command, &metadata, pallet_metadata),
        PalletSubcommand::Constants(command) => {
            explore_constants(command, &metadata, pallet_metadata)
        }
        PalletSubcommand::Storage(command) => {
            // if the metadata came from some url, we use that same url to make storage calls against.
            let node_url = opts.file_or_url.url.map(|url| url.to_string());
            explore_storage(command, &metadata, pallet_metadata, node_url).await
        }
    }
}

fn print_available_pallets(metadata: &Metadata) -> String {
    if metadata.pallets().next().is_none() {
        "There are no <PALLET> values available.".to_string()
    } else {
        let mut output = "Available <PALLET> values are:".to_string();
        let mut strings: Vec<_> = metadata.pallets().map(|p| p.name()).collect();
        strings.sort();
        for pallet in strings {
            write!(output, "\n    {}", pallet).unwrap();
        }
        output
    }
}
