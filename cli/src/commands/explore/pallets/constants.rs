use clap::Args;
use color_eyre::eyre::eyre;
use scale_typegen_description::type_description;
use std::fmt::Write;

use subxt::metadata::{types::PalletMetadata, Metadata};

use crate::utils::{format_scale_value, print_first_paragraph_with_indent, Indent};

#[derive(Debug, Clone, Args)]
pub struct ConstantsSubcommand {
    constant: Option<String>,
}

pub fn explore_constants(
    command: ConstantsSubcommand,
    pallet_metadata: PalletMetadata,
    metadata: &Metadata,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name();
    let Some(constant_name) = command.constant else {
        let available_constants = print_available_constants(pallet_metadata, pallet_name);
        writeln!(output, "Usage:")?;
        writeln!(
            output,
            "    subxt explore {pallet_name} constants <CONSTANT>"
        )?;
        writeln!(
            output,
            "        explore a specific call within this pallet\n\n{available_constants}"
        )?;
        return Ok(());
    };

    // if specified constant is wrong, show user the constants to choose from (but this time as an error):
    let Some(constant) = pallet_metadata
        .constants()
        .find(|constant| constant.name().to_lowercase() == constant_name.to_lowercase())
    else {
        let available_constants = print_available_constants(pallet_metadata, pallet_name);
        let mut description = "Usage:".to_string();
        writeln!(
            description,
            "    subxt explore {pallet_name} constants <CONSTANT>"
        )?;
        writeln!(
            description,
            "        explore a specific call within this pallet\n\n{available_constants}"
        )?;
        let err = eyre!(
            "constant \"{constant_name}\" not found in \"{pallet_name}\" pallet!\n\n{description}"
        );
        return Err(err);
    };

    // docs
    let doc_string = print_first_paragraph_with_indent(constant.docs(), 4);
    if !doc_string.is_empty() {
        writeln!(output, "Description:\n{doc_string}")?;
    }

    // shape
    let type_description = type_description(constant.ty(), metadata.types(), true)
        .expect("No Type Description")
        .indent(4);
    writeln!(
        output,
        "\nThe constant has the following shape:\n{type_description}"
    )?;

    // value
    let value =
        scale_value::scale::decode_as_type(&mut constant.value(), constant.ty(), metadata.types())?;
    let value = format_scale_value(&value);
    writeln!(output, "\nThe value of the constant is:\n    {value}",)?;
    Ok(())
}

fn print_available_constants(pallet_metadata: PalletMetadata, pallet_name: &str) -> String {
    if pallet_metadata.constants().len() == 0 {
        return format!("No <CONSTANT>'s available in the \"{pallet_name}\" pallet.");
    }
    let mut output = format!("Available <CONSTANT>'s in the \"{pallet_name}\" pallet:");
    let mut strings: Vec<_> = pallet_metadata.constants().map(|c| c.name()).collect();
    strings.sort();
    for constant in strings {
        output.push_str("\n    ");
        output.push_str(constant);
    }
    output
}
