use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;
use clap::{ArgMatches, Args, Command, Error, FromArgMatches, Parser as ClapParser};
use codec::Decode;
use color_eyre::eyre::eyre;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use frame_metadata::v15::RuntimeMetadataV15;
use jsonrpsee::client_transport::ws::Uri;
use subxt_codegen::utils::{MetadataVersion};
use subxt_metadata::{ConstantMetadata, Metadata, PalletMetadata, RuntimeApiMetadata};
use crate::utils::FileOrUrl;

/// todo: add docs
#[derive(Debug, ClapParser)]
pub struct Opts {
    node1: String,
    node2: String,
    #[clap(long, short)]
    version: Option<MetadataVersion>,
    #[clap(long, short)]
    pallet: Option<String>,
}

// cargo run -- diff ../artifacts/polkadot_metadata_small.scale ../artifacts/polkadot_metadata_tiny.scale
pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    println!("{} {}", opts.node1, opts.node2);

    let node_1_file_or_url: FileOrUrl = (opts.node1.as_str(), opts.version).try_into().map_err(|_| eyre!("{} is not a valid url nor path for node 1.", opts.node1))?;
    let node_2_file_or_url: FileOrUrl = (opts.node2.as_str(), opts.version).try_into().map_err(|_| eyre!("{} is not a valid url nor path for node 2.", opts.node2))?;

    let bytes = node_1_file_or_url.fetch().await?;
    let node_1_metadata: Metadata = RuntimeMetadataPrefixed::decode(&mut &bytes[..])?.try_into()?;

    let bytes = node_2_file_or_url.fetch().await?;
    let node_2_metadata: Metadata = RuntimeMetadataPrefixed::decode(&mut &bytes[..])?.try_into()?;

    let pallet_differences = pallet_differences(&node_1_metadata, &node_2_metadata);
    let pallet_differences_names: Vec<Diff<String>> = pallet_differences.into_iter().map(|diff| diff.map(|p| p.name().to_string())).collect();
    dbg!(pallet_differences_names);

    let runtime_api_differences = runtime_api_differences(&node_1_metadata, &node_2_metadata);
    let runtime_api_differences_names: Vec<Diff<String>> = runtime_api_differences.into_iter().map(|diff| diff.map(|p| p.name().to_string())).collect();
    dbg!(runtime_api_differences_names);


    Ok(())
}

impl TryFrom<(&str, Option<MetadataVersion>)> for FileOrUrl {
    type Error = ();

    fn try_from(value: (&str, Option<MetadataVersion>)) -> Result<Self, Self::Error> {
        match PathBuf::from_str(value.0) {
            Ok(path_buf) => Ok(FileOrUrl {
                url: None,
                file: Some(path_buf),
                version: value.1,
            }),
            Err(_) => {
                Uri::from_str(value.0).map_err(|_| ()).map(|uri| FileOrUrl {
                    url: Some(uri),
                    file: None,
                    version: value.1,
                })
            }
        }
    }
}

fn constants_differences<'a>(pallet_metadata_1: &'a PalletMetadata<'a>, pallet_metadata_2: &'a PalletMetadata<'a>) -> Vec<Diff<RuntimeApiMetadata<'a>>> {
    let mut constants: HashMap<&str, (Option<&'a ConstantMetadata>, Option<&'a ConstantMetadata>)> = HashMap::new();
    for constant in pallet_metadata_1.constants() {
        let (e1, _) = constants.entry(constant.name()).or_default();
        *e1 = Some(constant);
    }

    for constant in pallet_metadata_2.constants() {
        let (e1, e2) = constants.entry(constant.name()).or_default();
        // skip all entries with the same hash:
        if let Some(e1_inner) = e1 {
            let e1_hash = get_type_hash(pallet_metadata_1, e1_inner.ty(), &mut HashSet::new());
            if constan == runtime_api.hash() {
                *e1 = None;
                continue;
            }
        }
        *e2 = Some(constant);
    }
    todo!()
}


fn runtime_api_differences<'a>(metadata_1: &'a Metadata, metadata_2: &'a Metadata) -> Vec<Diff<RuntimeApiMetadata<'a>>> {
    let mut runtime_apis: HashMap<&str, (Option<RuntimeApiMetadata<'a>>, Option<RuntimeApiMetadata<'a>>)> = HashMap::new();

    for runtime_api in metadata_1.runtime_api_traits() {
        let (e1, _) = runtime_apis.entry(runtime_api.name()).or_default();
        *e1 = Some(runtime_api);
    }

    for runtime_api in metadata_2.runtime_api_traits() {
        let (e1, e2) = runtime_apis.entry(runtime_api.name()).or_default();
        // skip all entries with the same hash:
        if let Some(e1_inner) = e1 {
            if e1_inner.hash() == runtime_api.hash() {
                *e1 = None;
                continue;
            }
        }
        *e2 = Some(runtime_api);
    }
    runtime_apis.into_iter().map(|(_, tuple)|
        Diff::try_from(tuple).unwrap()
    ).collect()
}

fn pallet_differences<'a>(metadata_1: &'a Metadata, metadata_2: &'a Metadata) -> Vec<Diff<PalletMetadata<'a>>> {
    let mut pallets: HashMap<&str, (Option<PalletMetadata<'a>>, Option<PalletMetadata<'a>>)> = HashMap::new();

    for pallet_metadata in metadata_1.pallets() {
        let (e1, _) = pallets.entry(pallet_metadata.name()).or_default();
        *e1 = Some(pallet_metadata);
    }

    for pallet_metadata in metadata_2.pallets() {
        let (e1, e2) = pallets.entry(pallet_metadata.name()).or_default();
        // skip all entries with the same hash:
        if let Some(e1_inner) = e1 {
            if e1_inner.hash() == pallet_metadata.hash() {
                *e1 = None;
                continue;
            }
        }
        *e2 = Some(pallet_metadata);
    }

    pallets.into_iter().map(|(_, tuple)|
        Diff::try_from(tuple).unwrap()
    ).collect()
}

#[derive(Debug, Clone)]
enum Diff<T> {
    Added(T),
    Changed { from: T, to: T },
    Removed(T),
}

impl<T> Diff<T> {
    pub fn map<F, R>(self, mut f: F) -> Diff<R> where F: FnMut(T) -> R {
        match self {
            Diff::Added(new) => Diff::Added(f(new)),
            Diff::Changed { from, to } => Diff::Changed { from: f(from), to: f(to) },
            Diff::Removed(old) => Diff::Removed(f(old)),
        }
    }
}

impl<T> TryFrom<(Option<T>, Option<T>)> for Diff<T> {
    type Error = ();

    fn try_from(value: (Option<T>, Option<T>)) -> Result<Self, Self::Error> {
        match value {
            (None, None) => Err(()),
            (Some(old), None) => { Ok(Diff::Removed(old)) }
            (None, Some(new)) => { Ok(Diff::Added(new)) }
            (Some(old), Some(new)) => {
                Ok(Diff::Changed { from: old, to: new })
            }
        }
    }
}


