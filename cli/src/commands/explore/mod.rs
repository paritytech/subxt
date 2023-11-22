use crate::utils::{first_paragraph_of_docs, FileOrUrl};
use clap::{command, Parser, Subcommand};
use indoc::writedoc;
use subxt_metadata::{PalletMetadata, RuntimeApiMetadata};

use std::fmt::Write;
use std::write;

use codec::Decode;
use color_eyre::eyre::eyre;

use subxt::Metadata;

use self::pallets::PalletSubcommand;

mod pallets;
mod runtime_apis;

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
#[derive(Debug, Parser)]
pub struct Opts {
    #[command(flatten)]
    file_or_url: FileOrUrl,
    #[command(subcommand)]
    subcommand: Option<PalletOrRuntimeApi>,
}

#[derive(Debug, Subcommand)]
pub enum PalletOrRuntimeApi {
    Pallet(PalletOpts),
    Api(RuntimeApiOpts),
}

#[derive(Debug, Parser)]
pub struct PalletOpts {
    pub name: Option<String>,
    #[command(subcommand)]
    pub subcommand: Option<PalletSubcommand>,
}

#[derive(Debug, Parser)]
pub struct RuntimeApiOpts {
    pub name: Option<String>,
    #[clap(required = false)]
    pub method: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

pub async fn run(opts: Opts, output: &mut impl std::io::Write) -> color_eyre::Result<()> {
    // get the metadata
    let file_or_url = opts.file_or_url;
    let bytes = file_or_url.fetch().await?;
    let metadata = Metadata::decode(&mut &bytes[..])?;

    // if no pallet/runtime_api specified, show user the pallets/runtime_apis to choose from:
    let Some(pallet_or_runtime_api) = opts.subcommand else {
        let pallets = pallets_as_string(&metadata);
        let runtime_apis = runtime_apis_as_string(&metadata);
        writedoc! {output, "
        Usage:
            subxt explore pallet <PALLET>
                explore a specific pallet
            subxt explore api <RUNTIME_API>
                explore a specific runtime api trait

        {pallets}

        {runtime_apis}
        "}?;
        return Ok(());
    };

    match pallet_or_runtime_api {
        PalletOrRuntimeApi::Pallet(opts) => {
            let Some(name) = opts.name else {
                let pallets = pallets_as_string(&metadata);
                writedoc! {output, "
                Usage:
                    subxt explore pallet <PALLET>
                        explore a specific pallet
    
                {pallets}
                "}?;
                return Ok(());
            };

            let name_lower_case = name.to_lowercase();
            if let Some(pallet) = metadata
                .pallets()
                .find(|e| e.name().to_lowercase() == name_lower_case)
            {
                pallets::run(opts.subcommand, pallet, &metadata, file_or_url, output).await
            } else {
                Err(eyre!(
                    "pallet \"{name}\" not found in metadata!\n{}",
                    pallets_as_string(&metadata),
                ))
            }
        }
        PalletOrRuntimeApi::Api(opts) => {
            let Some(name) = opts.name else {
                let runtime_apis = runtime_apis_as_string(&metadata);
                writedoc! {output, "
                Usage:
                    subxt explore api <RUNTIME_API>
                        explore a specific runtime api trait

                {runtime_apis}
                "}?;
                return Ok(());
            };

            let name_lower_case = name.to_lowercase();
            if let Some(runtime_api) = metadata
                .runtime_api_traits()
                .find(|e| e.name().to_lowercase() == name_lower_case)
            {
                runtime_apis::run(
                    opts.method,
                    opts.trailing_args,
                    runtime_api,
                    &metadata,
                    file_or_url,
                    output,
                )
                .await
            } else {
                Err(eyre!(
                    "runtime api \"{name}\" not found in metadata!\n{}",
                    runtime_apis_as_string(&metadata),
                ))
            }
        }
    }

    // let e = metadata.runtime_api_traits().find(|e| true).unwrap();

    // // if specified pallet is wrong, show user the pallets to choose from (but this time as an error):

    // // if correct pallet was specified but no subcommand, instruct the user how to proceed:
    // let Some(subcommand) = opts.subcommand else {
    //     let docs_string = print_first_paragraph_with_indent(pallet_metadata.docs(), 4);
    //     if !docs_string.is_empty() {
    //         writeln!(output, "Description:\n{docs_string}")?;
    //     }
    //     writeln!(output, "Usage:")?;
    //     writeln!(output, "    subxt explore {pallet_name} calls")?;
    //     writeln!(
    //         output,
    //         "        explore the calls that can be made into this pallet"
    //     )?;
    //     writeln!(output, "    subxt explore {pallet_name} constants")?;
    //     writeln!(output, "        explore the constants held in this pallet")?;
    //     writeln!(output, "    subxt explore {pallet_name} storage")?;
    //     writeln!(
    //         output,
    //         "        explore the storage values held in this pallet"
    //     )?;
    //     return Ok(());
    // };

    // match subcommand {
    //     PalletSubcommand::Calls(command) => {
    //         explore_calls(command, &metadata, pallet_metadata, output)
    //     }
    //     PalletSubcommand::Constants(command) => {
    //         explore_constants(command, &metadata, pallet_metadata, output)
    //     }
    //     PalletSubcommand::Storage(command) => {
    //         // if the metadata came from some url, we use that same url to make storage calls against.
    //         let node_url = opts.file_or_url.url.map(|url| url.to_string());
    //         explore_storage(command, &metadata, pallet_metadata, node_url, output).await
    //     }
    // }
}

fn pallets_as_string(metadata: &Metadata) -> String {
    if metadata.pallets().len() == 0 {
        "There are no <PALLET>'s available.".to_string()
    } else {
        let mut output = "Available <PALLET>'s are:".to_string();
        let mut strings: Vec<_> = metadata.pallets().map(|p| p.name()).collect();
        strings.sort();
        for pallet in strings {
            write!(output, "\n    {}", pallet).unwrap();
        }
        output
    }
}

pub fn runtime_apis_as_string(metadata: &Metadata) -> String {
    if metadata.runtime_api_traits().len() == 0 {
        "There are no <RUNTIME_API>'s available.".to_string()
    } else {
        let mut output = "Available <RUNTIME_API>'s are:".to_string();
        let mut strings: Vec<_> = metadata.runtime_api_traits().map(|p| p.name()).collect();
        strings.sort();
        for api in strings {
            write!(output, "\n    {}", api).unwrap();
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
        assert_eq!(output.unwrap(), "Usage:\n    subxt explore <PALLET>\n        explore a specific pallet\n\nAvailable <PALLET> values are:\n    Balances\n    Multisig\n    ParaInherent\n    Staking\n    System\n    Timestamp\n");
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
