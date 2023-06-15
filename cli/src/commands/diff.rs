use clap::Parser as ClapParser;
use codec::Decode;

use frame_metadata::RuntimeMetadataPrefixed;
use std::collections::HashMap;



use crate::utils::FileOrUrl;
use color_eyre::owo_colors::OwoColorize;

use scale_info::form::PortableForm;
use scale_info::Variant;

use subxt_metadata::{
    ConstantMetadata, Metadata, PalletMetadata, RuntimeApiMetadata, StorageEntryMetadata,
    StorageEntryType,
};

/// Explore the differences between two nodes
///
/// # Example
/// ```
/// subxt diff ./artifacts/polkadot_metadata_small.scale ./artifacts/polkadot_metadata_tiny.scale
/// subxt diff ./artifacts/polkadot_metadata_small.scale wss://rpc.polkadot.io:443
/// ```
#[derive(Debug, ClapParser)]
pub struct Opts {
    entry1: FileOrUrl,
    entry2: FileOrUrl,
}

pub async fn run(opts: Opts, output: &mut impl std::io::Write) -> color_eyre::Result<()> {
    let (entry_1_metadata, entry_2_metadata) = get_metadata(&opts).await?;

    let metadata_diff = MetadataDiff::construct(&entry_1_metadata, &entry_2_metadata);

    if metadata_diff.is_empty() {
        writeln!(output, "No difference in metadata found.")?;
        return Ok(());
    }
    if !metadata_diff.pallets.is_empty() {
        writeln!(output, "Pallets:")?;
        for diff in metadata_diff.pallets {
            match diff {
                Diff::Added(new) => {
                    writeln!(output, "{}", format!("    + {}", new.name()).green())?
                }
                Diff::Removed(old) => {
                    writeln!(output, "{}", format!("    - {}", old.name()).red())?
                }
                Diff::Changed { from, to } => {
                    writeln!(output, "{}", format!("    ~ {}", from.name()).yellow())?;

                    let pallet_diff = PalletDiff::construct(&from, &to);
                    if !pallet_diff.calls.is_empty() {
                        writeln!(output, "        Calls:")?;
                        for diff in pallet_diff.calls {
                            match diff {
                                Diff::Added(new) => writeln!(
                                    output,
                                    "{}",
                                    format!("            + {}", &new.name).green()
                                )?,
                                Diff::Removed(old) => writeln!(
                                    output,
                                    "{}",
                                    format!("            - {}", &old.name).red()
                                )?,
                                Diff::Changed { from, to: _ } => {
                                    writeln!(
                                        output,
                                        "{}",
                                        format!("            ~ {}", &from.name).yellow()
                                    )?;
                                }
                            }
                        }
                    }

                    if !pallet_diff.constants.is_empty() {
                        writeln!(output, "        Constants:")?;
                        for diff in pallet_diff.constants {
                            match diff {
                                Diff::Added(new) => writeln!(
                                    output,
                                    "{}",
                                    format!("            + {}", new.name()).green()
                                )?,
                                Diff::Removed(old) => writeln!(
                                    output,
                                    "{}",
                                    format!("            - {}", old.name()).red()
                                )?,
                                Diff::Changed { from, to: _ } => writeln!(
                                    output,
                                    "{}",
                                    format!("            ~ {}", from.name()).yellow()
                                )?,
                            }
                        }
                    }

                    if !pallet_diff.storage_entries.is_empty() {
                        writeln!(output, "        Storage Entries:")?;
                        for diff in pallet_diff.storage_entries {
                            match diff {
                                Diff::Added(new) => writeln!(
                                    output,
                                    "{}",
                                    format!("            + {}", new.name()).green()
                                )?,
                                Diff::Removed(old) => writeln!(
                                    output,
                                    "{}",
                                    format!("            - {}", old.name()).red()
                                )?,
                                Diff::Changed { from, to } => {
                                    let storage_diff = StorageEntryDiff::construct(
                                        from,
                                        to,
                                        &entry_1_metadata,
                                        &entry_2_metadata,
                                    );

                                    writeln!(
                                        output,
                                        "{}",
                                        format!(
                                            "            ~ {} (Changed: {})",
                                            from.name(),
                                            storage_diff.to_strings().join(", ")
                                        )
                                        .yellow()
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !metadata_diff.runtime_apis.is_empty() {
        writeln!(output, "Runtime APIs:")?;
        for diff in metadata_diff.runtime_apis {
            match diff {
                Diff::Added(new) => {
                    writeln!(output, "{}", format!("    + {}", new.name()).green())?
                }
                Diff::Removed(old) => {
                    writeln!(output, "{}", format!("    - {}", old.name()).red())?
                }
                Diff::Changed { from, to: _ } => {
                    writeln!(output, "{}", format!("    ~ {}", from.name()).yellow())?
                }
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
        MetadataDiff {
            pallets,
            runtime_apis,
        }
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
    fn construct(
        pallet_metadata_1: &'a PalletMetadata<'a>,
        pallet_metadata_2: &'a PalletMetadata<'a>,
    ) -> PalletDiff<'a> {
        let calls = calls_differences(pallet_metadata_1, pallet_metadata_2);
        let constants = constants_differences(pallet_metadata_1, pallet_metadata_2);
        let storage_entries = storage_differences(pallet_metadata_1, pallet_metadata_2);
        PalletDiff {
            calls,
            constants,
            storage_entries,
        }
    }
}

struct StorageEntryDiff {
    key_different: bool,
    value_different: bool,
    default_different: bool,
    modifier_different: bool,
}

impl StorageEntryDiff {
    fn construct(
        storage_entry_1: &StorageEntryMetadata,
        storage_entry_2: &StorageEntryMetadata,
        metadata_1: &Metadata,
        metadata_2: &Metadata,
    ) -> Self {
        let value_1_ty_id = match storage_entry_1.entry_type() {
            StorageEntryType::Plain(value_ty) | StorageEntryType::Map { value_ty, .. } => value_ty,
        };
        let value_1_hash = metadata_1
            .type_hash(*value_1_ty_id)
            .expect("type should be present");
        let value_2_ty_id = match storage_entry_2.entry_type() {
            StorageEntryType::Plain(value_ty) | StorageEntryType::Map { value_ty, .. } => value_ty,
        };
        let value_2_hash = metadata_1
            .type_hash(*value_2_ty_id)
            .expect("type should be present");
        let value_different = value_1_hash != value_2_hash;

        let key_1_hash = match storage_entry_1.entry_type() {
            StorageEntryType::Plain(_) => None,
            StorageEntryType::Map { key_ty, .. } => Some(*key_ty),
        }
        .map(|key_ty| {
            metadata_1
                .type_hash(key_ty)
                .expect("type should be present")
        })
        .unwrap_or_default();
        let key_2_hash = match storage_entry_2.entry_type() {
            StorageEntryType::Plain(_) => None,
            StorageEntryType::Map { key_ty, .. } => Some(*key_ty),
        }
        .map(|key_ty| {
            metadata_2
                .type_hash(key_ty)
                .expect("type should be present")
        })
        .unwrap_or_default();
        let key_different = key_1_hash != key_2_hash;

        StorageEntryDiff {
            key_different,
            value_different,
            default_different: storage_entry_1.default_bytes() != storage_entry_2.default_bytes(),
            modifier_different: storage_entry_1.modifier() != storage_entry_2.modifier(),
        }
    }

    fn to_strings(&self) -> Vec<&str> {
        let mut strings = Vec::<&str>::new();
        if self.key_different {
            strings.push("key type");
        }
        if self.value_different {
            strings.push("value type");
        }
        if self.modifier_different {
            strings.push("modifier");
        }
        if self.default_different {
            strings.push("default value");
        }
        strings
    }
}

async fn get_metadata(opts: &Opts) -> color_eyre::Result<(Metadata, Metadata)> {
    let bytes = opts.entry1.fetch().await?;
    let entry_1_metadata: Metadata =
        RuntimeMetadataPrefixed::decode(&mut &bytes[..])?.try_into()?;

    let bytes = opts.entry2.fetch().await?;
    let entry_2_metadata: Metadata =
        RuntimeMetadataPrefixed::decode(&mut &bytes[..])?.try_into()?;

    Ok((entry_1_metadata, entry_2_metadata))
}

fn storage_differences<'a>(
    pallet_metadata_1: &'a PalletMetadata<'a>,
    pallet_metadata_2: &'a PalletMetadata<'a>,
) -> Vec<Diff<&'a StorageEntryMetadata>> {
    let mut storage_entries: HashMap<
        &str,
        (
            Option<&'a StorageEntryMetadata>,
            Option<&'a StorageEntryMetadata>,
        ),
    > = HashMap::new();
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
                let e1_hash = pallet_metadata_1
                    .storage_hash(e1_inner.name())
                    .expect("storage entry should be present");
                let e2_hash = pallet_metadata_2
                    .storage_hash(storage_entry.name())
                    .expect("storage entry should be present");
                if e1_hash == e2_hash {
                    storage_entries.remove(storage_entry.name());
                    continue;
                }
            }
            *e2 = Some(storage_entry);
        }
    }
    storage_entries
        .into_values()
        .map(|tuple| Diff::try_from(tuple).unwrap())
        .collect()
}

type CallsHashmap<'a> = HashMap<
    &'a str,
    (
        Option<&'a Variant<PortableForm>>,
        Option<&'a Variant<PortableForm>>,
    ),
>;

fn calls_differences<'a>(
    pallet_metadata_1: &'a PalletMetadata<'a>,
    pallet_metadata_2: &'a PalletMetadata<'a>,
) -> Vec<Diff<&'a Variant<PortableForm>>> {
    let mut calls: CallsHashmap = HashMap::new();
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
                let e1_hash = pallet_metadata_1
                    .call_hash(&e1_inner.name)
                    .expect("call should be present");
                let e2_hash = pallet_metadata_2
                    .call_hash(&call_variant.name)
                    .expect("call should be present");
                if e1_hash == e2_hash {
                    calls.remove(call_variant.name.as_str());
                    continue;
                }
            }
            *e2 = Some(call_variant);
        }
    }
    calls
        .into_values()
        .map(|tuple| Diff::try_from(tuple).unwrap())
        .collect()
}

fn constants_differences<'a>(
    pallet_metadata_1: &'a PalletMetadata<'a>,
    pallet_metadata_2: &'a PalletMetadata<'a>,
) -> Vec<Diff<&'a ConstantMetadata>> {
    let mut constants: HashMap<&str, (Option<&'a ConstantMetadata>, Option<&'a ConstantMetadata>)> =
        HashMap::new();
    for constant in pallet_metadata_1.constants() {
        let (e1, _) = constants.entry(constant.name()).or_default();
        *e1 = Some(constant);
    }
    for constant in pallet_metadata_2.constants() {
        let (e1, e2) = constants.entry(constant.name()).or_default();
        // skip all entries with the same hash:
        if let Some(e1_inner) = e1 {
            let e1_hash = pallet_metadata_1
                .constant_hash(e1_inner.name())
                .expect("constant should be present");
            let e2_hash = pallet_metadata_2
                .constant_hash(constant.name())
                .expect("constant should be present");
            if e1_hash == e2_hash {
                constants.remove(constant.name());
                continue;
            }
        }
        *e2 = Some(constant);
    }
    constants
        .into_values()
        .map(|tuple| Diff::try_from(tuple).unwrap())
        .collect()
}

fn runtime_api_differences<'a>(
    metadata_1: &'a Metadata,
    metadata_2: &'a Metadata,
) -> Vec<Diff<RuntimeApiMetadata<'a>>> {
    let mut runtime_apis: HashMap<
        &str,
        (
            Option<RuntimeApiMetadata<'a>>,
            Option<RuntimeApiMetadata<'a>>,
        ),
    > = HashMap::new();

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
    runtime_apis
        .into_values()
        .map(|tuple| Diff::try_from(tuple).unwrap())
        .collect()
}

fn pallet_differences<'a>(
    metadata_1: &'a Metadata,
    metadata_2: &'a Metadata,
) -> Vec<Diff<PalletMetadata<'a>>> {
    let mut pallets: HashMap<&str, (Option<PalletMetadata<'a>>, Option<PalletMetadata<'a>>)> =
        HashMap::new();

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

    pallets
        .into_values()
        .map(|tuple| {
            Diff::try_from(tuple).expect("unreachable, fails only if two None values are present")
        })
        .collect()
}

#[derive(Debug, Clone)]
enum Diff<T> {
    Added(T),
    Changed { from: T, to: T },
    Removed(T),
}

impl<T> TryFrom<(Option<T>, Option<T>)> for Diff<T> {
    type Error = ();

    fn try_from(value: (Option<T>, Option<T>)) -> Result<Self, Self::Error> {
        match value {
            (None, None) => Err(()),
            (Some(old), None) => Ok(Diff::Removed(old)),
            (None, Some(new)) => Ok(Diff::Added(new)),
            (Some(old), Some(new)) => Ok(Diff::Changed { from: old, to: new }),
        }
    }
}
