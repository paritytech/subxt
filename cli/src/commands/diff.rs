use clap::Args;
use codec::Decode;

use frame_metadata::RuntimeMetadataPrefixed;
use std::collections::HashMap;
use std::hash::Hash;

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
#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    /// metadata file or node URL
    metadata_or_url_1: FileOrUrl,
    /// metadata file or node URL
    metadata_or_url_2: FileOrUrl,
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
    let bytes = opts.metadata_or_url_1.fetch().await?;
    let entry_1_metadata: Metadata =
        RuntimeMetadataPrefixed::decode(&mut &bytes[..])?.try_into()?;

    let bytes = opts.metadata_or_url_2.fetch().await?;
    let entry_2_metadata: Metadata =
        RuntimeMetadataPrefixed::decode(&mut &bytes[..])?.try_into()?;

    Ok((entry_1_metadata, entry_2_metadata))
}

fn storage_differences<'a>(
    pallet_metadata_1: &'a PalletMetadata<'a>,
    pallet_metadata_2: &'a PalletMetadata<'a>,
) -> Vec<Diff<&'a StorageEntryMetadata>> {
    diff(
        pallet_metadata_1
            .storage()
            .map(|s| s.entries())
            .unwrap_or_default(),
        pallet_metadata_2
            .storage()
            .map(|s| s.entries())
            .unwrap_or_default(),
        |e| {
            pallet_metadata_1
                .storage_hash(e.name())
                .expect("storage entry should be present")
        },
        |e| {
            pallet_metadata_2
                .storage_hash(e.name())
                .expect("storage entry should be present")
        },
        |e| e.name(),
    )
}

fn calls_differences<'a>(
    pallet_metadata_1: &'a PalletMetadata<'a>,
    pallet_metadata_2: &'a PalletMetadata<'a>,
) -> Vec<Diff<&'a Variant<PortableForm>>> {
    return diff(
        pallet_metadata_1.call_variants().unwrap_or_default(),
        pallet_metadata_2.call_variants().unwrap_or_default(),
        |e| {
            pallet_metadata_1
                .call_hash(&e.name)
                .expect("call should be present")
        },
        |e| {
            pallet_metadata_2
                .call_hash(&e.name)
                .expect("call should be present")
        },
        |e| &e.name,
    );
}

fn constants_differences<'a>(
    pallet_metadata_1: &'a PalletMetadata<'a>,
    pallet_metadata_2: &'a PalletMetadata<'a>,
) -> Vec<Diff<&'a ConstantMetadata>> {
    diff(
        pallet_metadata_1.constants(),
        pallet_metadata_2.constants(),
        |e| {
            pallet_metadata_1
                .constant_hash(e.name())
                .expect("constant should be present")
        },
        |e| {
            pallet_metadata_2
                .constant_hash(e.name())
                .expect("constant should be present")
        },
        |e| e.name(),
    )
}

fn runtime_api_differences<'a>(
    metadata_1: &'a Metadata,
    metadata_2: &'a Metadata,
) -> Vec<Diff<RuntimeApiMetadata<'a>>> {
    diff(
        metadata_1.runtime_api_traits(),
        metadata_2.runtime_api_traits(),
        RuntimeApiMetadata::hash,
        RuntimeApiMetadata::hash,
        RuntimeApiMetadata::name,
    )
}

fn pallet_differences<'a>(
    metadata_1: &'a Metadata,
    metadata_2: &'a Metadata,
) -> Vec<Diff<PalletMetadata<'a>>> {
    diff(
        metadata_1.pallets(),
        metadata_2.pallets(),
        PalletMetadata::hash,
        PalletMetadata::hash,
        PalletMetadata::name,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Diff<T> {
    Added(T),
    Changed { from: T, to: T },
    Removed(T),
}

fn diff<T, C: PartialEq, I: Hash + PartialEq + Eq + Ord>(
    items_a: impl IntoIterator<Item = T>,
    items_b: impl IntoIterator<Item = T>,
    hash_fn_a: impl Fn(&T) -> C,
    hash_fn_b: impl Fn(&T) -> C,
    key_fn: impl Fn(&T) -> I,
) -> Vec<Diff<T>> {
    let mut entries: HashMap<I, (Option<T>, Option<T>)> = HashMap::new();

    for t1 in items_a {
        let key = key_fn(&t1);
        let (e1, _) = entries.entry(key).or_default();
        *e1 = Some(t1);
    }

    for t2 in items_b {
        let key = key_fn(&t2);
        let (e1, e2) = entries.entry(key).or_default();
        // skip all entries with the same hash:
        if let Some(e1_inner) = e1 {
            let e1_hash = hash_fn_a(e1_inner);
            let e2_hash = hash_fn_b(&t2);
            if e1_hash == e2_hash {
                entries.remove(&key_fn(&t2));
                continue;
            }
        }
        *e2 = Some(t2);
    }

    // sort the values by key before returning
    let mut diff_vec_with_keys: Vec<_> = entries.into_iter().collect();
    diff_vec_with_keys.sort_by(|a, b| a.0.cmp(&b.0));
    diff_vec_with_keys
        .into_iter()
        .map(|(_, tuple)| match tuple {
            (None, None) => panic!("At least one value is inserted when the key exists; qed"),
            (Some(old), None) => Diff::Removed(old),
            (None, Some(new)) => Diff::Added(new),
            (Some(old), Some(new)) => Diff::Changed { from: old, to: new },
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::commands::diff::{diff, Diff};

    #[test]
    fn test_diff_fn() {
        let old_pallets = [("Babe", 7), ("Claims", 9), ("Balances", 23)];
        let new_pallets = [("Claims", 9), ("Balances", 22), ("System", 3), ("NFTs", 5)];
        let hash_fn = |e: &(&str, i32)| e.0.len() as i32 * e.1;
        let differences = diff(old_pallets, new_pallets, hash_fn, hash_fn, |e| e.0);
        let expected_differences = vec![
            Diff::Removed(("Babe", 7)),
            Diff::Changed {
                from: ("Balances", 23),
                to: ("Balances", 22),
            },
            Diff::Added(("NFTs", 5)),
            Diff::Added(("System", 3)),
        ];
        assert_eq!(differences, expected_differences);
    }
}
