// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Parser as ClapParser;
use serde_json::Value;
use std::{io::Write, path::PathBuf};
use subxt_codegen::fetch_metadata::Url;

mod fetch;

/// Download chainSpec from a substrate node.
#[derive(Debug, ClapParser)]
pub struct Opts {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(long)]
    url: Url,
    /// Write the output of the command to the provided file path.
    #[clap(long, short, value_parser)]
    output_file: Option<PathBuf>,
    /// Replaced the genesis raw entry with a stateRootHash to optimize
    /// the spec size and avoid the need to calculate the genesis storage.
    ///
    /// Defaults to `false`.
    #[clap(long)]
    state_root_hash: bool,
}

/// Error attempting to fetch chainSpec.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ChainSpecError {
    /// Failed to fetch the chain spec.
    #[error("Failed to fetch the chain spec: {0}")]
    FetchError(#[from] fetch::FetchSpecError),

    /// The provided chain spec is invalid.
    #[error("Error while parsing the chain spec: {0})")]
    ParseError(String),

    /// Cannot compute the state root hash.
    #[error("Error computing state root hash: {0})")]
    ComputeError(String),

    /// Other error.
    #[error("Other: {0})")]
    Other(String),
}

fn compute_state_root_hash(spec: &Value) -> Result<[u8; 32], ChainSpecError> {
    let chain_spec = smoldot::chain_spec::ChainSpec::from_json_bytes(spec.to_string().as_bytes())
        .map_err(|err| ChainSpecError::ParseError(err.to_string()))?;

    let genesis_chain_information = chain_spec.to_chain_information().map(|(ci, _)| ci);

    let state_root = match genesis_chain_information {
        Ok(genesis_chain_information) => {
            let header = genesis_chain_information.as_ref().finalized_block_header;
            *header.state_root
        }
        // From the smoldot code this error is encountered when the genesis already contains the
        // state root hash entry instead of the raw entry.
        Err(smoldot::chain_spec::FromGenesisStorageError::UnknownStorageItems) => *chain_spec
            .genesis_storage()
            .into_trie_root_hash()
            .ok_or_else(|| {
                ChainSpecError::ParseError(
                    "The chain spec does not contain the proper shape for the genesis.raw entry"
                        .to_string(),
                )
            })?,
        Err(err) => return Err(ChainSpecError::ComputeError(err.to_string())),
    };

    Ok(state_root)
}

pub async fn run(opts: Opts, output: &mut impl Write) -> color_eyre::Result<()> {
    let url = opts.url;

    let mut spec = fetch::fetch_chain_spec(url).await?;

    let mut output: Box<dyn Write> = match opts.output_file {
        Some(path) => Box::new(std::fs::File::create(path)?),
        None => Box::new(output),
    };

    if opts.state_root_hash {
        let state_root_hash = compute_state_root_hash(&mut spec)?;
        let state_root_hash = format!("0x{}", hex::encode(state_root_hash));

        if let Some(genesis) = spec.get_mut("genesis") {
            let object = genesis.as_object_mut().ok_or_else(|| {
                ChainSpecError::Other("The genesis entry must be an object".to_string())
            })?;

            object.remove("raw").ok_or_else(|| {
                ChainSpecError::Other("The genesis entry must contain a raw entry".to_string())
            })?;

            object.insert("stateRootHash".to_string(), Value::String(state_root_hash));
        }
    }

    let json = serde_json::to_string_pretty(&spec)?;
    write!(output, "{json}")?;
    Ok(())
}
