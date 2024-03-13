use clap::Args;
use color_eyre::eyre::eyre;
use indoc::{formatdoc, writedoc};
use scale_typegen_description::type_description;
use subxt::metadata::{types::PalletMetadata, Metadata};

use crate::utils::{first_paragraph_of_docs, format_scale_value, Indent, SyntaxHighlight};

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

    let usage = || {
        let constants = constants_to_string(pallet_metadata, pallet_name);
        formatdoc! {"
        Usage:
            subxt explore pallet {pallet_name} constants <CONSTANT>
                explore a specific constant of this pallet
        
        {constants}
        "}
    };

    let Some(constant_name) = command.constant else {
        writeln!(output, "{}", usage())?;
        return Ok(());
    };

    // if specified constant is wrong, show user the constants to choose from (but this time as an error):
    let Some(constant) = pallet_metadata
        .constants()
        .find(|constant| constant.name().eq_ignore_ascii_case(&constant_name))
    else {
        let err = eyre!(
            "constant \"{constant_name}\" not found in \"{pallet_name}\" pallet!\n\n{}",
            usage()
        );
        return Err(err);
    };

    // docs
    let doc_string = first_paragraph_of_docs(constant.docs()).indent(4);
    if !doc_string.is_empty() {
        writedoc! {output, "
        Description:
        {doc_string}
        
        "}?;
    }

    // shape
    let type_description = type_description(constant.ty(), metadata.types(), true)
        .expect("No Type Description")
        .indent(4)
        .highlight();

    // value
    let value = scale_value::scale::decode_as_type(
        &mut constant.value(),
        &constant.ty(),
        metadata.types(),
    )?;
    let value = format_scale_value(&value).indent(4);

    writedoc!(
        output,
        "
        The constant has the following shape:
        {type_description}

        The value of the constant is:
        {value}
        "
    )?;
    Ok(())
}

fn constants_to_string(pallet_metadata: PalletMetadata, pallet_name: &str) -> String {
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
