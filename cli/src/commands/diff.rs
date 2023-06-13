use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;
use clap::{ArgMatches, Args, Command, Error, FromArgMatches, Parser as ClapParser};
use codec::Decode;
use color_eyre::eyre::eyre;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use frame_metadata::v15::RuntimeMetadataV15;
use jsonrpsee::client_transport::ws::Uri;
use scale_info::form::PortableForm;
use scale_info::Variant;
use subxt_codegen::utils::{MetadataVersion};
use subxt_metadata::{ConstantMetadata, Metadata, PalletMetadata, RuntimeApiMetadata, StorageEntryMetadata};
use crate::utils::{FileOrUrl};
use std::io::Write;
use color_eyre::owo_colors::OwoColorize;

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
    let (node_1_metadata, node_2_metadata) = get_metadata(&opts).await?;

    let node_diff = MetadataDiff::construct(&node_1_metadata, &node_2_metadata);
    let mut output = std::io::stdout();


    if node_diff.is_empty() {
        writeln!(output, "No difference in Metadata found.")?;
        return Ok(());
    }
    if !node_diff.pallets.is_empty() {
        writeln!(output, "Pallets:")?;
        for diff in node_diff.pallets {
            match diff {
                Diff::Added(new) => writeln!(output, "{}", format!("    + {} (Added)", new.name()).green())?,
                Diff::Removed(old) => writeln!(output, "{}", format!("    - {} (Removed)", old.name()).red())?,
                Diff::Changed { from, to } => {
                    writeln!(output, "{}", format!("    ~ {} (Changed)", from.name()).yellow())?;

                    let pallet_diff = PalletDiff::construct(&from, &to);
                    if !pallet_diff.calls.is_empty() {
                        writeln!(output, "        Calls:")?;
                        for diff in pallet_diff.calls {
                            match diff {
                                Diff::Added(new) => writeln!(output, "{}", format!("            + {} (Added)", &new.name).green())?,
                                Diff::Removed(old) => writeln!(output, "{}", format!("            - {} (Removed)", &old.name).red())?,
                                Diff::Changed { from, to } => writeln!(output, "{}", format!("            ~ {} (Changed)", &from.name).yellow())?,
                            }
                        }
                    }

                    if !pallet_diff.constants.is_empty() {
                        writeln!(output, "        Constants:")?;
                        for diff in pallet_diff.constants {
                            match diff {
                                Diff::Added(new) => writeln!(output, "{}", format!("            + {} (Added)", new.name()).green())?,
                                Diff::Removed(old) => writeln!(output, "{}", format!("            - {} (Removed)", old.name()).red())?,
                                Diff::Changed { from, to } => writeln!(output, "{}", format!("            ~ {} (Changed)", from.name()).yellow())?,
                            }
                        }
                    }

                    if !pallet_diff.storage_entries.is_empty() {
                        writeln!(output, "        Storage Entries:")?;
                        for diff in pallet_diff.storage_entries {
                            match diff {
                                Diff::Added(new) => writeln!(output, "{}", format!("            + {} (Added)", new.name()).green())?,
                                Diff::Removed(old) => writeln!(output, "{}", format!("            - {} (Removed)", old.name()).red())?,
                                Diff::Changed { from, to } => writeln!(output, "{}", format!("            ~ {} (Changed)", from.name()).yellow())?,
                            }
                        }
                    }
                }
            }
        }
    }

    if !node_diff.runtime_apis.is_empty() {
        writeln!(output, "Runtime APIs:")?;
        for diff in node_diff.runtime_apis {
            match diff {
                Diff::Added(new) => writeln!(output, "{}", format!("    + {} (Added)", new.name()).green())?,
                Diff::Removed(old) => writeln!(output, "{}", format!("    - {} (Removed)", old.name()).red())?,
                Diff::Changed { from, to } => writeln!(output, "{}", format!("    ~ {} (Changed)", from.name()).yellow())?,
            }
        }
    }
    Ok(())
}


struct MetadataDiff<'a> {
    pallets: Vec<Diff<PalletMetadata<'a>>>,
    runtime_apis: Vec<Diff<RuntimeApiMetadata<'a>>>,
}

impl<'a> MetadataDiff<'a> {
    fn construct(metadata_1: &'a Metadata, metadata_2: &'a Metadata) -> MetadataDiff<'a> {
        let pallets = pallet_differences(metadata_1, metadata_2);
        let runtime_apis = runtime_api_differences(metadata_1, metadata_2);
        MetadataDiff { pallets, runtime_apis }
    }

    fn is_empty(&self) -> bool {
        self.pallets.is_empty() && self.runtime_apis.is_empty()
    }
}


#[derive(Default)]
struct PalletDiff<'a> {
    calls: Vec<Diff<&'a Variant<PortableForm>>>,
    constants: Vec<Diff<&'a ConstantMetadata>>,
    storage_entries: Vec<Diff<&'a StorageEntryMetadata>>,
}

impl<'a> PalletDiff<'a> {
    fn construct(pallet_metadata_1: &'a PalletMetadata<'a>, pallet_metadata_2: &'a PalletMetadata<'a>) -> PalletDiff<'a> {
        let calls = calls_differences(&pallet_metadata_1, &pallet_metadata_2);
        let constants = constants_differences(&pallet_metadata_1, &pallet_metadata_2);
        let storage_entries = storage_differences(&pallet_metadata_1, &pallet_metadata_2);
        PalletDiff { calls, constants, storage_entries }
    }

    fn is_empty(&self) -> bool {
        self.calls.is_empty() && self.constants.is_empty() && self.storage_entries.is_empty()
    }
}


async fn get_metadata(opts: &Opts) -> color_eyre::Result<(Metadata, Metadata)> {
    let node_1_file_or_url: FileOrUrl = (opts.node1.as_str(), opts.version).try_into().map_err(|_| eyre!("{} is not a valid url nor path for node 1.", opts.node1))?;
    let node_2_file_or_url: FileOrUrl = (opts.node2.as_str(), opts.version).try_into().map_err(|_| eyre!("{} is not a valid url nor path for node 2.", opts.node2))?;

    let bytes = node_1_file_or_url.fetch().await?;
    let node_1_metadata: Metadata = RuntimeMetadataPrefixed::decode(&mut &bytes[..])?.try_into()?;

    let bytes = node_2_file_or_url.fetch().await?;
    let node_2_metadata: Metadata = RuntimeMetadataPrefixed::decode(&mut &bytes[..])?.try_into()?;

    Ok((node_1_metadata, node_2_metadata))
}

impl TryFrom<(&str, Option<MetadataVersion>)> for FileOrUrl {
    type Error = ();

    fn try_from(value: (&str, Option<MetadataVersion>)) -> Result<Self, Self::Error> {
        println!("value {}", &value.0);
        let path = std::path::Path::new(&value.0);
        if path.exists() {
            Ok(FileOrUrl {
                url: None,
                file: Some(PathBuf::from(value.0)),
                version: value.1,
            })
        } else {
            Uri::from_str(value.0).map_err(|_| ()).map(|uri| FileOrUrl {
                url: Some(uri),
                file: None,
                version: value.1,
            })
        }
    }
}

fn storage_differences<'a>(pallet_metadata_1: &'a PalletMetadata<'a>, pallet_metadata_2: &'a PalletMetadata<'a>) -> Vec<Diff<&'a StorageEntryMetadata>> {
    let mut storage_entries: HashMap<&str, (Option<&'a StorageEntryMetadata>, Option<&'a StorageEntryMetadata>)> = HashMap::new();
    if let Some(storage_metadata) = pallet_metadata_1.storage() {
        for storage_entry in storage_metadata.entries() {
            let (e1, _) = storage_entries.entry(storage_entry.name()).or_default();
            *e1 = Some(storage_entry);
        }
    }
    if let Some(storage_metadata) = pallet_metadata_2.storage() {
        for storage_entry in storage_metadata.entries() {
            let (e1, e2) = storage_entries.entry(storage_entry.name()).or_default();
            // skip all entries with the same hash:
            if let Some(e1_inner) = e1 {
                let e1_hash = pallet_metadata_1.storage_hash(e1_inner.name()).expect("storage entry should be present");
                let e2_hash = pallet_metadata_2.storage_hash(storage_entry.name()).expect("storage entry should be present");
                if e1_hash == e2_hash {
                    storage_entries.remove(storage_entry.name());
                    continue;
                }
            }
            *e2 = Some(storage_entry);
        }
    }
    storage_entries.into_iter().map(|(_, tuple)|
        Diff::try_from(tuple).unwrap()
    ).collect()
}


fn calls_differences<'a>(pallet_metadata_1: &'a PalletMetadata<'a>, pallet_metadata_2: &'a PalletMetadata<'a>) -> Vec<Diff<&'a Variant<PortableForm>>> {
    let mut calls: HashMap<&str, (Option<&'a Variant<PortableForm>>, Option<&'a Variant<PortableForm>>)> = HashMap::new();
    if let Some(call_variants) = pallet_metadata_1.call_variants() {
        for call_variant in call_variants {
            let (e1, _) = calls.entry(&call_variant.name).or_default();
            *e1 = Some(call_variant);
        }
    }
    if let Some(call_variants) = pallet_metadata_2.call_variants() {
        for call_variant in call_variants {
            let (e1, e2) = calls.entry(&call_variant.name).or_default();
            // skip all entries with the same hash:
            if let Some(e1_inner) = e1 {
                let e1_hash = pallet_metadata_1.call_hash(&e1_inner.name).expect("call should be present");
                let e2_hash = pallet_metadata_2.call_hash(&call_variant.name).expect("call should be present");
                if e1_hash == e2_hash {
                    calls.remove(call_variant.name.as_str());
                    continue;
                }
            }
            *e2 = Some(call_variant);
        }
    }
    calls.into_iter().map(|(_, tuple)|
        Diff::try_from(tuple).unwrap()
    ).collect()
}


fn constants_differences<'a>(pallet_metadata_1: &'a PalletMetadata<'a>, pallet_metadata_2: &'a PalletMetadata<'a>) -> Vec<Diff<&'a ConstantMetadata>> {
    let mut constants: HashMap<&str, (Option<&'a ConstantMetadata>, Option<&'a ConstantMetadata>)> = HashMap::new();
    for constant in pallet_metadata_1.constants() {
        let (e1, _) = constants.entry(constant.name()).or_default();
        *e1 = Some(constant);
    }
    for constant in pallet_metadata_2.constants() {
        let (e1, e2) = constants.entry(constant.name()).or_default();
        // skip all entries with the same hash:
        if let Some(e1_inner) = e1 {
            let e1_hash = pallet_metadata_1.constant_hash(e1_inner.name()).expect("constant should be present");
            let e2_hash = pallet_metadata_2.constant_hash(constant.name()).expect("constant should be present");
            if e1_hash == e2_hash {
                constants.remove(constant.name());
                continue;
            }
        }
        *e2 = Some(constant);
    }
    constants.into_iter().map(|(_, tuple)|
        Diff::try_from(tuple).unwrap()
    ).collect()
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
                runtime_apis.remove(runtime_api.name());
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
                pallets.remove(pallet_metadata.name());
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
    fn map<F, R>(self, mut f: F) -> Diff<R> where F: FnMut(T) -> R {
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

