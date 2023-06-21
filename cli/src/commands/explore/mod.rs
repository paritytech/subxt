use crate::utils::{print_first_paragraph_with_indent, FileOrUrl};
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
/// ```text
/// subxt explore --file=polkadot_metadata.scale
/// ```
///
/// ## Calls
///
/// Show the calls in a pallet:
/// ```text
/// subxt explore Balances calls
/// ```
/// Show the call parameters a call expects:
/// ```text
/// subxt explore Balances calls transfer
/// ```
/// Create an unsigned extrinsic from a scale value, validate it and output its hex representation
/// ```text
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
/// ```text
/// subxt explore Balances constants
/// ```
/// ## Storage
///
/// Show the storage entries in a pallet
/// ```text
/// subxt explore Alliance storage
/// ```
/// Show the types and value of a specific storage entry
/// ```text
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

pub async fn run(opts: Opts, output: &mut impl std::io::Write) -> color_eyre::Result<()> {
    // get the metadata
    let bytes = opts.file_or_url.fetch().await?;
    let metadata = Metadata::decode(&mut &bytes[..])?;

    // if no pallet specified, show user the pallets to choose from:
    let Some(pallet_name) = opts.pallet else {
        let available_pallets = print_available_pallets(&metadata);
        writeln!(output, "Usage:", )?;
        writeln!(output, "    subxt explore <PALLET>", )?;
        writeln!(output, "        explore a specific pallet", )?;
        writeln!(output, "\n{available_pallets}", )?;
        return Ok(());
    };

    // if specified pallet is wrong, show user the pallets to choose from (but this time as an error):
    let Some(pallet_metadata) = metadata.pallets().find(|pallet| pallet.name().to_lowercase() == pallet_name.to_lowercase()) else {
        return Err(eyre!("pallet \"{}\" not found in metadata!\n{}", pallet_name, print_available_pallets(&metadata)));
    };

    // if correct pallet was specified but no subcommand, instruct the user how to proceed:
    let Some(pallet_subcomand) = opts.pallet_subcommand else {
        let docs_string = print_first_paragraph_with_indent(pallet_metadata.docs(), 4);
        if !docs_string.is_empty() {
            writeln!(output, "Description:\n{docs_string}")?;
        }
        writeln!(output, "Usage:")?;
        writeln!(output, "    subxt explore {pallet_name} calls")?;
        writeln!(output, "        explore the calls that can be made into this pallet")?;
        writeln!(output, "    subxt explore {pallet_name} constants")?;
        writeln!(output, "        explore the constants held in this pallet")?;
        writeln!(output, "    subxt explore {pallet_name} storage")?;
        writeln!(output, "        explore the storage values held in this pallet")?;
        return Ok(());
    };

    match pallet_subcomand {
        PalletSubcommand::Calls(command) => {
            explore_calls(command, &metadata, pallet_metadata, output)
        }
        PalletSubcommand::Constants(command) => {
            explore_constants(command, &metadata, pallet_metadata, output)
        }
        PalletSubcommand::Storage(command) => {
            // if the metadata came from some url, we use that same url to make storage calls against.
            let node_url = opts.file_or_url.url.map(|url| url.to_string());
            explore_storage(command, &metadata, pallet_metadata, node_url, output).await
        }
    }
}

fn print_available_pallets(metadata: &Metadata) -> String {
    if metadata.pallets().len() == 0 {
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

#[cfg(test)]
pub mod tests {
    use super::{run, Opts};

    async fn simulate_run(cli_command: &str) -> color_eyre::Result<String> {
        let mut args = vec![
            "explore",
            "--file=../artifacts/polkadot_metadata_small.scale",
        ];
        let mut split: Vec<&str> = cli_command.split(' ').filter(|e| !e.is_empty()).collect();
        args.append(&mut split);
        let opts: Opts = clap::Parser::try_parse_from(args)?;
        let mut output: Vec<u8> = Vec::new();
        run(opts, &mut output)
            .await
            .map(|_| String::from_utf8(output).unwrap())
    }

    #[tokio::test]
    async fn test_commands() {
        // show pallets:
        let output = simulate_run("").await;
        assert_eq!(output.unwrap(), "Usage:\n    subxt explore <PALLET>\n        explore a specific pallet\n\nAvailable <PALLET> values are:\n    Balances\n    Multisig\n    Staking\n    System\n");
        // if incorrect pallet, error:
        let output = simulate_run("abc123").await;
        assert!(output.is_err());
        // if correct pallet, show options (calls, constants, storage)
        let output = simulate_run("Balances").await;
        assert_eq!(output.unwrap(), "Usage:\n    subxt explore Balances calls\n        explore the calls that can be made into this pallet\n    subxt explore Balances constants\n        explore the constants held in this pallet\n    subxt explore Balances storage\n        explore the storage values held in this pallet\n");
        // check that exploring calls, storage entries and constants is possible:
        let output = simulate_run("Balances calls").await;
        assert!(output.unwrap().starts_with("Usage:\n    subxt explore Balances calls <CALL>\n        explore a specific call within this pallet\n\nAvailable <CALL>'s in the \"Balances\" pallet:\n"));
        let output = simulate_run("Balances storage").await;
        assert!(output.unwrap().starts_with("Usage:\n    subxt explore Balances storage <STORAGE_ENTRY>\n        view details for a specific storage entry\n\nAvailable <STORAGE_ENTRY>'s in the \"Balances\" pallet:\n"));
        let output = simulate_run("Balances constants").await;
        assert!(output.unwrap().starts_with("Usage:\n    subxt explore Balances constants <CONSTANT>\n        explore a specific call within this pallet\n\nAvailable <CONSTANT>'s in the \"Balances\" pallet:\n"));
        // check that invalid subcommands don't work:
        let output = simulate_run("Balances abc123").await;
        assert!(output.is_err());
        // check that we can explore a certain call:
        let output = simulate_run("Balances calls transfer").await;
        assert!(output.unwrap().starts_with("Usage:\n    subxt explore Balances calls transfer <SCALE_VALUE>\n        construct the call by providing a valid argument\n\nThe call expect expects a <SCALE_VALUE> with this shape:\n    {\n        dest: enum MultiAddress"));
        // check that unsigned extrinsic can be constructed:
        let output =
            simulate_run("Balances calls transfer {\"dest\":v\"Raw\"((255,255, 255)),\"value\":0}")
                .await;
        assert_eq!(
            output.unwrap(),
            "Encoded call data:\n    0x24040507020cffffff00\n"
        );
        // check that we can explore a certain constant:
        let output = simulate_run("Balances constants ExistentialDeposit").await;
        assert_eq!(output.unwrap(), "Description:\n    The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!\n\nThe constant has the following shape:\n    u128\n\nThe value of the constant is:\n    10000000000\n");
        // check that we can explore a certain storage entry:
        let output = simulate_run("System storage Account").await;
        assert!(output.unwrap().starts_with("Usage:\n    subxt explore System storage Account <KEY_VALUE>\n\nDescription:\n    The full account information for a particular account ID."));
        // in the future we could also integrate with substrate-testrunner to spawn up a node and send an actual storage query to it: e.g. `subxt explore System storage Digest`
    }
}
