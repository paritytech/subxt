use crate::utils::type_description::{format_type_description, TypeDescription};
use crate::utils::type_example::TypeExample;
use crate::utils::FileOrUrl;
use clap::ValueEnum;
use clap::{Args, Parser as ClapParser, Subcommand};

use codec::Decode;
use color_eyre::eyre::eyre;
use frame_metadata::v15::{PalletMetadata, RuntimeMetadataV15};
use frame_metadata::RuntimeMetadataPrefixed;
use scale_info::form::PortableForm;
use scale_info::{PortableRegistry, Type, TypeDef, TypeDefVariant, Variant};
use scale_value::{Composite, ValueDef};
use subxt::tx;
use subxt::utils::H256;
use subxt::{config::SubstrateConfig, Metadata, OfflineClient};
use subxt::dynamic::constant;

/// Shows the pallets and calls available for a node and lets you build unsigned extrinsics.
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
/// ```
/// ## Constants
///
/// Show the constants in a pallet:
/// ```
/// subxt explore Balances constants
/// ```
/// ## Storage
///
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
    Constants,
    Storage(StorageSubcommand),
}

#[derive(Debug, Clone, Args)]
pub struct CallsSubcommand {
    call: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

#[derive(Debug, Clone, Args)]
pub struct StorageSubcommand {
    storage_item: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

/// cargo run -- explore --file=../artifacts/polkadot_metadata.scale
pub async fn run(opts: Opts) -> color_eyre::Result<()> {
    // get the metadata
    let bytes = opts.file_or_url.fetch().await?;
    let metadata_prefixed = <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;
    let metadata = Metadata::try_from(metadata_prefixed)?;

    // if no pallet specified, show user the pallets to choose from:
    let Some(pallet_name) = opts.pallet else {
        println!("If you want to explore a pallet: subxt explore <PALLET>\n{}", print_available_pallets(metadata.runtime_metadata()));
        return Ok(());
    };

    // if specified pallet is wrong, show user the pallets to choose from (but this time as an error):
    let Some(pallet_metadata) = metadata.runtime_metadata().pallets.iter().find(|pallet| pallet.name == pallet_name)else {
        return Err(eyre!("pallet \"{}\" not found in metadata!\n{}", pallet_name, print_available_pallets(metadata.runtime_metadata())));
    };

    // if correct pallet was specified but no subcommand, instruct the user how to proceed:
    let Some(pallet_subcomand) = opts.pallet_subcommand else {
        println!("To explore the \"{pallet_name}\" pallet further, use one of the following:\n\
        subxt explore {pallet_name} calls\n\
        subxt explore {pallet_name} constants\n\
        ");
        return Ok(());
    };

    match pallet_subcomand {
        PalletSubcommand::Calls(calls_subcommand) => explore_calls(calls_subcommand, &metadata, pallet_name, pallet_metadata),
        PalletSubcommand::Constants => explore_constants(&metadata, pallet_name, pallet_metadata),
        PalletSubcommand::Storage(storage_subcommand) => explore_storage(storage_subcommand, &metadata, pallet_name, pallet_metadata)
    }
}

fn explore_calls(
    calls_subcommand: CallsSubcommand,
    metadata: &Metadata,
    pallet_name: String,
    pallet_metadata: &PalletMetadata<PortableForm>,
) -> color_eyre::Result<()> {
    // get the enum that stores the possible calls:
    let (calls_enum_type_def, calls_enum_type) =
        get_calls_enum_type(pallet_metadata, &metadata.runtime_metadata().types)?;

    // if no call specified, show user the calls to choose from:
    let Some(call_name) = calls_subcommand.call else {
        println!("If you want to explore a pallet: subxt show {pallet_name} <CALL>\n{}", print_available_calls(calls_enum_type_def));
        return Ok(());
    };

    // if specified call is wrong, show user the calls to choose from (but this time as an error):
    let Some(call) = calls_enum_type_def.variants.iter().find(|variant| variant.name == call_name)   else {
        return Err(eyre!("call \"{}\" not found in pallet \"{}\"!\n{}", call_name,  pallet_name, print_available_calls(calls_enum_type_def)));
    };

    // collect all the trailing arguments into a single string that is later into a scale_value::Value
    let trailing_args = calls_subcommand.trailing_args.join(" ");

    // if no trailing arguments specified show user the expected type of arguments with examples:
    if trailing_args.is_empty() {
        let call_description = print_call_description(call, &metadata.runtime_metadata().types)?;
        println!(
            "If you want to create an unsigned extrinsic for {pallet_name}/{call_name}\nrepresenting a scale value of the type {}::{call_name}:\nsubxt show {pallet_name} {call_name} <SCALE_VALUE>\n{call_description}",
            calls_enum_type.path
        );
        return Ok(());
    }

    // parse scale_value from trailing arguments and try to create an unsigned extrinsic with it:
    let value = scale_value::stringify::from_str(&trailing_args).0.map_err(|err| eyre!("scale_value::stringify::from_str led to a ParseError.\n\ntried parsing: \"{}\"\n\n{}", trailing_args, err))?;
    let value_as_composite = value_into_composite(value);
    let offline_client = new_offline_client(metadata.clone());
    let payload = tx::dynamic(pallet_name, call_name, value_as_composite);
    let unsigned_extrinsic = offline_client.tx().create_unsigned(&payload)?;
    let hex_bytes = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));

    println!("Encoded call data:");
    println!("{hex_bytes}");

    Ok(())
}

fn print_available_pallets(metadata_v15: &RuntimeMetadataV15) -> String {
    if metadata_v15.pallets.is_empty() {
        "There are no pallets in this node.".to_string()
    } else {
        let mut output = "Available pallets are:".to_string();
        for pallet in metadata_v15.pallets.iter() {
            output.push_str(format!("\n- {}", pallet.name).as_str())
        }
        output
    }
}

fn get_calls_enum_type<'a>(
    pallet: &'a frame_metadata::v15::PalletMetadata<PortableForm>,
    registry: &'a PortableRegistry,
) -> color_eyre::Result<(
    &'a TypeDefVariant<PortableForm>,
    &'a Type<PortableForm>,
)> {
    let calls = pallet
        .calls
        .as_ref()
        .ok_or(eyre!("pallet {} has no calls.", pallet.name))?;
    let calls_enum_type = registry
        .resolve(calls.ty.id)
        .ok_or(eyre!("calls type with id {} not found.", calls.ty.id))?;
    // should always be a variant type, where each variant corresponds to one call.
    let calls_enum_type_def = match &calls_enum_type.type_def {
        TypeDef::Variant(variant) => variant,
        _ => {
            return Err(eyre!("calls type is not a variant"));
        }
    };
    Ok((calls_enum_type_def, calls_enum_type))
}

fn print_available_calls(pallet_calls: &TypeDefVariant<PortableForm>) -> String {
    if pallet_calls.variants.is_empty() {
        "There are no calls in this pallet.".to_string()
    } else {
        let mut output = "Available calls are:".to_string();
        for variant in pallet_calls.variants.iter() {
            output.push_str(format!("    \n{}", variant.name).as_str())
        }
        output
    }
}

fn print_call_description(
    call_variant: &Variant<PortableForm>,
    registry: &PortableRegistry,
) -> color_eyre::Result<String> {
    let type_description = call_variant.fields.type_description(registry)?;
    let type_description = format_type_description(&type_description);
    let type_examples = call_variant
        .fields
        .type_example(registry)
        .unwrap_or(Vec::new());

    let mut output = String::new();
    output.push_str("The type looks like this:\n");
    output.push_str(type_description.as_str());

    output.push_str("\n\n");
    match type_examples.len() {
        0 => {
            output.push_str(
                "There are no examples available for this type."
                    .to_string()
                    .as_str(),
            );
        }
        1 => {
            output.push_str("Here is an example of this type:".to_string().as_str());
        }
        i => {
            output.push_str(format!("Here are {i} examples of this type:").as_str());
        }
    };

    for composite in type_examples {
        let value = scale_value::Value {
            value: ValueDef::Composite(composite),
            context: (),
        };
        let example_str = scale_value::stringify::to_string(&value);
        output.push('\n');
        output.push_str(example_str.as_str());
    }

    Ok(output)
}

/// composites stay composites, all other types are converted into a 1-fielded unnamed composite
fn value_into_composite(value: scale_value::Value) -> scale_value::Composite<()> {
    match value.value {
        ValueDef::Composite(composite) => composite,
        _ => Composite::Unnamed(vec![value]),
    }
}

fn new_offline_client(metadata: Metadata) -> OfflineClient<SubstrateConfig> {
    let genesis_hash = {
        let h = "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";
        let bytes = hex::decode(h).unwrap();
        H256::from_slice(&bytes)
    };

    let runtime_version = subxt::rpc::types::RuntimeVersion {
        spec_version: 9370,
        transaction_version: 20,
        other: Default::default(),
    };

    OfflineClient::<SubstrateConfig>::new(genesis_hash, runtime_version, metadata)
}


fn explore_constants(
    metadata: &Metadata,
    pallet_name: String,
    pallet_metadata: &PalletMetadata<PortableForm>,
) -> color_eyre::Result<()> {
    // print all constants in this pallet together with their type, value and the docs as an explanation:
    let output = if pallet_metadata.constants.is_empty() {
        format!("There are no constants for the \"{pallet_name}\" pallet.")
    } else {
        let mut output = format!("The \"{pallet_name}\" pallet has the following constants:");
        for constant in pallet_metadata.constants.iter() {
            let type_description = constant.ty.id.type_description(metadata.types())?;
            let scale_val = scale_value::scale::decode_as_type(&mut &constant.value[..], constant.ty.id, metadata.types())?;
            let name_and_type = format!("\n\n    {}: {} = {}", constant.name, type_description, scale_value::stringify::to_string(&scale_val));
            output.push_str(name_and_type.as_str());
            for doc in constant.docs.iter().filter_map(|e|
                {
                    let trimmed = e.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(format!("\n        {trimmed}"))
                    }
                }
            ) {
                dbg!(&doc);
                output.push_str(doc.as_str());
            }
        }
        output
    };
    println!("{output}");
    Ok(())
}


fn explore_storage(
    storage_subcommand: StorageSubcommand,
    metadata: &Metadata,
    pallet_name: String,
    pallet_metadata: &PalletMetadata<PortableForm>,
) -> color_eyre::Result<()> {
    Ok(())
}

