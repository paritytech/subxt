use clap::Args;
use color_eyre::eyre::eyre;
use color_eyre::owo_colors::OwoColorize;
use indoc::{formatdoc, writedoc};
use scale_info::form::PortableForm;
use scale_info::{PortableRegistry, Type, TypeDef, TypeDefVariant};
use scale_value::{Composite, ValueDef};
use std::str::FromStr;

use subxt::tx;
use subxt::utils::H256;
use subxt::{
    config::SubstrateConfig,
    metadata::{types::PalletMetadata, Metadata},
    OfflineClient,
};

use crate::utils::{
    fields_composite_example, fields_description, parse_string_into_scale_value, Indent,
    SyntaxHighlight,
};

#[derive(Debug, Clone, Args)]
pub struct CallsSubcommand {
    call: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

pub fn explore_calls(
    command: CallsSubcommand,
    pallet_metadata: PalletMetadata,
    metadata: &Metadata,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name();

    // get the enum that stores the possible calls:
    let (calls_enum_type_def, _calls_enum_type) =
        get_calls_enum_type(pallet_metadata, metadata.types())?;

    let usage = || {
        let calls = calls_to_string(calls_enum_type_def, pallet_name);
        formatdoc! {"
        Usage:
            subxt explore pallet {pallet_name} calls <CALL>
                explore a specific call of this pallet
        
        {calls}
        "}
    };

    // if no call specified, show user the calls to choose from:
    let Some(call_name) = command.call else {
        writeln!(output, "{}", usage())?;
        return Ok(());
    };

    // if specified call is wrong, show user the calls to choose from (but this time as an error):
    let Some(call) = calls_enum_type_def
        .variants
        .iter()
        .find(|variant| variant.name.eq_ignore_ascii_case(&call_name))
    else {
        return Err(eyre!(
            "\"{call_name}\" call not found in \"{pallet_name}\" pallet!\n\n{}",
            usage()
        ));
    };

    // collect all the trailing arguments into a single string that is later into a scale_value::Value
    let trailing_args = command.trailing_args.join(" ");

    // if no trailing arguments specified show user the expected type of arguments with examples:
    if trailing_args.is_empty() {
        let fields: Vec<(Option<&str>, u32)> = call
            .fields
            .iter()
            .map(|f| (f.name.as_deref(), f.ty.id))
            .collect();
        let type_description = fields_description(&fields, &call.name, metadata.types()).indent(4);
        let fields_example =
            fields_composite_example(call.fields.iter().map(|e| e.ty.id), metadata.types())
                .indent(4)
                .highlight();

        let scale_value_placeholder = "<SCALE_VALUE>".blue();

        writedoc! {output, "
        Usage:
            subxt explore pallet {pallet_name} calls {call_name} {scale_value_placeholder}
                construct the call by providing a valid argument

        The call expects a {scale_value_placeholder} with this shape:
        {type_description}

        For example you could provide this {scale_value_placeholder}:
        {fields_example}
        "}?;
        return Ok(());
    }

    // parse scale_value from trailing arguments and try to create an unsigned extrinsic with it:
    let value = parse_string_into_scale_value(&trailing_args)?;
    let value_as_composite = value_into_composite(value);
    let offline_client = mocked_offline_client(metadata.clone());
    let payload = tx::dynamic(pallet_name, call_name, value_as_composite);
    let unsigned_extrinsic = offline_client.tx().create_unsigned(&payload)?;
    let hex_bytes = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));
    writedoc! {output, "
    Encoded call data:
        {hex_bytes}
    "}?;

    Ok(())
}

fn calls_to_string(pallet_calls: &TypeDefVariant<PortableForm>, pallet_name: &str) -> String {
    if pallet_calls.variants.is_empty() {
        return format!("No <CALL>'s available in the \"{pallet_name}\" pallet.");
    }
    let mut output = format!("Available <CALL>'s in the \"{pallet_name}\" pallet:");

    let mut strings: Vec<_> = pallet_calls.variants.iter().map(|c| &c.name).collect();
    strings.sort();
    for variant in strings {
        output.push_str("\n    ");
        output.push_str(variant);
    }
    output
}

fn get_calls_enum_type<'a>(
    pallet: PalletMetadata,
    registry: &'a PortableRegistry,
) -> color_eyre::Result<(&'a TypeDefVariant<PortableForm>, &'a Type<PortableForm>)> {
    let call_ty = pallet
        .call_ty_id()
        .ok_or(eyre!("The \"{}\" pallet has no calls.", pallet.name()))?;
    let calls_enum_type = registry
        .resolve(call_ty)
        .ok_or(eyre!("calls type with id {} not found.", call_ty))?;

    // should always be a variant type, where each variant corresponds to one call.
    let TypeDef::Variant(calls_enum_type_def) = &calls_enum_type.type_def else {
        return Err(eyre!("calls type is not a variant"));
    };
    Ok((calls_enum_type_def, calls_enum_type))
}

/// The specific values used for construction do not matter too much, we just need any OfflineClient to create unsigned extrinsics
fn mocked_offline_client(metadata: Metadata) -> OfflineClient<SubstrateConfig> {
    let genesis_hash =
        H256::from_str("91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3")
            .expect("Valid hash; qed");

    let runtime_version = subxt::backend::RuntimeVersion {
        spec_version: 9370,
        transaction_version: 20,
    };

    OfflineClient::<SubstrateConfig>::new(genesis_hash, runtime_version, metadata)
}

/// composites stay composites, all other types are converted into a 1-fielded unnamed composite
fn value_into_composite(value: scale_value::Value) -> scale_value::Composite<()> {
    match value.value {
        ValueDef::Composite(composite) => composite,
        _ => Composite::Unnamed(vec![value]),
    }
}
