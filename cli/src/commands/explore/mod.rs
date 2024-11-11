use crate::utils::validate_url_security;
use crate::utils::FileOrUrl;
use clap::{command, Parser, Subcommand};
use codec::Decode;
use color_eyre::eyre::eyre;
use color_eyre::owo_colors::OwoColorize;
use indoc::writedoc;
use std::fmt::Write;
use std::write;

use subxt::Metadata;

use self::pallets::PalletSubcommand;

mod pallets;
mod runtime_apis;

/// Explore pallets, calls, call parameters, storage entries and constants. Also allows for creating (unsigned) extrinsics.
///
/// # Example
///
/// Show the pallets and runtime apis that are available:
/// ```text
/// subxt explore --file=polkadot_metadata.scale
/// ```
///
/// ## Pallets
///
/// each pallet has `calls`, `constants`, `storage` and `events` that can be explored.
///
/// ### Calls
///
/// Show the calls in a pallet:
///
/// ```text
/// subxt explore pallet Balances calls
/// ```
///
/// Show the call parameters a call expects:
///
/// ```text
/// subxt explore pallet Balances calls transfer
/// ```
///
/// Create an unsigned extrinsic from a scale value, validate it and output its hex representation
///
/// ```text
/// subxt explore pallet Grandpa calls note_stalled { "delay": 5, "best_finalized_block_number": 5 }
/// # Encoded call data:
/// # 0x2c0411020500000005000000
/// subxt explore pallet Balances calls transfer  "{ \"dest\": v\"Raw\"((255, 255, 255)), \"value\": 0 }"
/// # Encoded call data:
/// # 0x24040607020cffffff00
/// ```
///
/// ### Constants
///
/// Show the constants in a pallet:
///
/// ```text
/// subxt explore pallet Balances constants
/// ```
///
/// ### Storage
///
/// Show the storage entries in a pallet
///
/// ```text
/// subxt explore pallet Alliance storage
/// ```
///
/// Show the types and value of a specific storage entry
///
/// ```text
/// subxt explore pallet Alliance storage Announcements [KEY_SCALE_VALUE]
/// ```
///
/// ### Events
///
/// ```text
/// subxt explore pallet Balances events
/// ```
///
/// Show the type of a specific event
///
/// ```text
/// subxt explore pallet Balances events frozen
/// ```
///
/// ## Runtime APIs
/// Show the input and output types of a runtime api method.
/// In this example "core" is the name of the runtime api and "version" is a method on it:
///
/// ```text
/// subxt explore api core version
/// ```
///
/// Execute a runtime API call with the `--execute` (`-e`) flag, to see the return value.
/// For example here we get the "version", via the "core" runtime API from the connected node:
///
/// ```text
/// subxt explore api core version --execute
/// ```
///
#[derive(Debug, Parser)]
pub struct Opts {
    #[command(flatten)]
    file_or_url: FileOrUrl,
    #[command(subcommand)]
    subcommand: Option<PalletOrRuntimeApi>,
    /// Allow insecure URLs e.g. URLs starting with ws:// or http:// without SSL encryption
    #[clap(long, short)]
    allow_insecure: bool,
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
    #[clap(long, short, action)]
    pub execute: bool,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

pub async fn run(opts: Opts, output: &mut impl std::io::Write) -> color_eyre::Result<()> {
    validate_url_security(opts.file_or_url.url.as_ref(), opts.allow_insecure)?;

    // get the metadata
    let file_or_url = opts.file_or_url;
    let bytes = file_or_url.fetch().await?;
    let metadata = Metadata::decode(&mut &bytes[..])?;

    let pallet_placeholder = "<PALLET>".blue();
    let runtime_api_placeholder = "<RUNTIME_API>".blue();

    // if no pallet/runtime_api specified, show user the pallets/runtime_apis to choose from:
    let Some(pallet_or_runtime_api) = opts.subcommand else {
        let pallets = pallets_as_string(&metadata);
        let runtime_apis = runtime_apis_as_string(&metadata);
        writedoc! {output, "
        Usage:
            subxt explore pallet {pallet_placeholder}
                explore a specific pallet
            subxt explore api {runtime_api_placeholder}
                explore a specific runtime api

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
                    subxt explore pallet {pallet_placeholder}
                        explore a specific pallet

                {pallets}
                "}?;
                return Ok(());
            };

            if let Some(pallet) = metadata
                .pallets()
                .find(|e| e.name().eq_ignore_ascii_case(&name))
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
                    subxt explore api {runtime_api_placeholder}
                        explore a specific runtime api

                {runtime_apis}
                "}?;
                return Ok(());
            };

            if let Some(runtime_api) = metadata
                .runtime_api_traits()
                .find(|e| e.name().eq_ignore_ascii_case(&name))
            {
                runtime_apis::run(
                    opts.method,
                    opts.execute,
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
}

fn pallets_as_string(metadata: &Metadata) -> String {
    let pallet_placeholder = "<PALLET>".blue();
    if metadata.pallets().len() == 0 {
        format!("There are no {pallet_placeholder}'s available.")
    } else {
        let mut output = format!("Available {pallet_placeholder}'s are:");
        let mut strings: Vec<_> = metadata.pallets().map(|p| p.name()).collect();
        strings.sort();
        for pallet in strings {
            write!(output, "\n    {}", pallet).unwrap();
        }
        output
    }
}

pub fn runtime_apis_as_string(metadata: &Metadata) -> String {
    let runtime_api_placeholder = "<RUNTIME_API>".blue();
    if metadata.runtime_api_traits().len() == 0 {
        format!("There are no {runtime_api_placeholder}'s available.")
    } else {
        let mut output = format!("Available {runtime_api_placeholder}'s are:");
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

    use indoc::formatdoc;
    use pretty_assertions::assert_eq;

    use super::Opts;

    async fn run(cli_command: &str) -> color_eyre::Result<String> {
        let mut args = vec!["explore"];
        let mut split: Vec<&str> = cli_command.split(' ').filter(|e| !e.is_empty()).collect();
        args.append(&mut split);
        let opts: Opts = clap::Parser::try_parse_from(args)?;
        let mut output: Vec<u8> = Vec::new();
        let r = super::run(opts, &mut output)
            .await
            .map(|_| String::from_utf8(output).unwrap())?;
        Ok(r)
    }

    trait StripAnsi: ToString {
        fn strip_ansi(&self) -> String {
            let bytes = strip_ansi_escapes::strip(self.to_string().as_bytes());
            String::from_utf8(bytes).unwrap()
        }
    }

    impl<T: ToString> StripAnsi for T {}

    macro_rules! assert_eq_start {
        ($a:expr, $b:expr) => {
            assert_eq!(&$a[0..$b.len()], &$b[..]);
        };
    }

    async fn run_against_file(cli_command: &str) -> color_eyre::Result<String> {
        run(&format!(
            "--file=../artifacts/polkadot_metadata_small.scale {cli_command}"
        ))
        .await
    }

    #[tokio::test]
    async fn test_commands() {
        // shows pallets and runtime apis:
        let output = run_against_file("").await.unwrap().strip_ansi();
        let expected_output = formatdoc! {
            "Usage:
                subxt explore pallet <PALLET>
                    explore a specific pallet
                subxt explore api <RUNTIME_API>
                    explore a specific runtime api

            Available <PALLET>'s are:
                Balances
                Multisig
                ParaInherent
                System
                Timestamp

            Available <RUNTIME_API>'s are:
                AccountNonceApi
                AuthorityDiscoveryApi
                BabeApi
                BeefyApi
                BeefyMmrApi
                BlockBuilder
                Core
                DryRunApi
                GenesisBuilder
                GrandpaApi
                LocationToAccountApi
                Metadata
                MmrApi
                OffchainWorkerApi
                ParachainHost
                SessionKeys
                TaggedTransactionQueue
                TransactionPaymentApi
                TrustedQueryApi
                XcmPaymentApi
        "};
        assert_eq!(output, expected_output);
        // if incorrect pallet, error:
        let output = run_against_file("abc123").await;
        assert!(output.is_err());
        // if correct pallet, show options (calls, constants, storage)
        let output = run_against_file("pallet Balances")
            .await
            .unwrap()
            .strip_ansi();
        let expected_output = formatdoc! {"
        Usage:
            subxt explore pallet Balances calls
                explore the calls that can be made into a pallet
            subxt explore pallet Balances constants
                explore the constants of a pallet
            subxt explore pallet Balances storage
                explore the storage values of a pallet
            subxt explore pallet Balances events
                explore the events of a pallet
        "};
        assert_eq!(output, expected_output);
        // check that exploring calls, storage entries and constants is possible:
        let output = run_against_file("pallet Balances calls")
            .await
            .unwrap()
            .strip_ansi();
        let start = formatdoc! {"
            Usage:
                subxt explore pallet Balances calls <CALL>
                    explore a specific call of this pallet

            Available <CALL>'s in the \"Balances\" pallet:"};
        assert_eq_start!(output, start);
        let output = run_against_file("pallet Balances storage")
            .await
            .unwrap()
            .strip_ansi();
        let start = formatdoc! {"
            Usage:
                subxt explore pallet Balances storage <STORAGE_ENTRY>
                    explore a specific storage entry of this pallet

            Available <STORAGE_ENTRY>'s in the \"Balances\" pallet:
        "};
        assert_eq_start!(output, start);
        let output = run_against_file("pallet Balances constants")
            .await
            .unwrap()
            .strip_ansi();
        let start = formatdoc! {"
        Usage:
            subxt explore pallet Balances constants <CONSTANT>
                explore a specific constant of this pallet

        Available <CONSTANT>'s in the \"Balances\" pallet:
        "};
        assert_eq_start!(output, start);
        let output = run_against_file("pallet Balances events")
            .await
            .unwrap()
            .strip_ansi();
        let start = formatdoc! {"
        Usage:
            subxt explore pallet Balances events <EVENT>
                explore a specific event of this pallet

        Available <EVENT>'s in the \"Balances\" pallet:
        "};
        assert_eq_start!(output, start);
        // check that invalid subcommands don't work:
        let output = run_against_file("pallet Balances abc123").await;
        assert!(output.is_err());
        // check that we can explore a certain call:
        let output = run_against_file("pallet Balances calls transfer_keep_alive")
            .await
            .unwrap()
            .strip_ansi();
        // Note: at some point we want to switch to new metadata in the artifacts folder which has e.g. transfer_keep_alive instead of transfer.
        let start = formatdoc! {"
        Usage:
            subxt explore pallet Balances calls transfer_keep_alive <SCALE_VALUE>
                construct the call by providing a valid argument
        "};
        assert_eq_start!(output, start);
        // check that we can see methods of a runtime api:
        let output = run_against_file("api metadata").await.unwrap().strip_ansi();

        let start = formatdoc! {"
            Description:
                The `Metadata` api trait that returns metadata for the runtime.

            Usage:
                subxt explore api Metadata <METHOD>
                    explore a specific runtime api method

            Available <METHOD>'s available for the \"Metadata\" runtime api:
        "};
        assert_eq_start!(output, start);
    }

    #[tokio::test]
    async fn insecure_urls_get_denied() {
        // Connection should work fine:
        run("--url wss://rpc.polkadot.io:443").await.unwrap();

        // Errors, because the --allow-insecure is not set:
        assert!(run("--url ws://rpc.polkadot.io:443")
            .await
            .unwrap_err()
            .to_string()
            .contains("is not secure"));

        // This checks, that we never prevent (insecure) requests to localhost, even if the `--allow-insecure` flag is not set.
        // It errors, because there is no node running locally, which results in the "Request error".
        assert!(run("--url ws://localhost")
            .await
            .unwrap_err()
            .to_string()
            .contains("Request error"));
    }
}
