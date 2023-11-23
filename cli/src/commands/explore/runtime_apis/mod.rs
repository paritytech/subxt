use crate::utils::{
    fields_composite_example, fields_description, first_paragraph_of_docs, FileOrUrl, Indent,
};
use clap::{Args, Parser};
use color_eyre::eyre::eyre;
use indoc::{formatdoc, writedoc};
use scale_typegen_description::type_description;
use subxt::Metadata;
use subxt_metadata::RuntimeApiMetadata;

pub async fn run<'a>(
    method: Option<String>,
    execute: bool,
    trailing_args: Vec<String>,
    runtime_api_metadata: RuntimeApiMetadata<'a>,
    metadata: &'a Metadata,
    file_or_url: FileOrUrl,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let api_name = runtime_api_metadata.name();

    let usage = || {
        let available_methods = available_methods_string(&runtime_api_metadata);
        formatdoc! {"
        Usage:
            subxt explore api {api_name} <METHOD>
                explore a specific runtime api method
        
        {available_methods}
        "}
    };

    let Some(method_name) = method else {
        // show docs for runtime api
        let doc_string = first_paragraph_of_docs(runtime_api_metadata.docs()).indent(4);
        if !doc_string.is_empty() {
            writedoc! {output, "
            Description:
            {doc_string}
    
            "}?;
        }
        writeln!(output, "{}", usage())?;
        return Ok(());
    };
    let Some(method) = runtime_api_metadata
        .methods()
        .find(|e| e.name().to_lowercase() == method_name.to_lowercase())
    else {
        return Err(eyre!(
            "\"{method_name}\" method not found for \"{method_name}\" runtime api!\n\n{}",
            usage()
        ));
    };

    // show docs for method:
    let doc_string = first_paragraph_of_docs(method.docs()).indent(4);
    if !doc_string.is_empty() {
        writedoc! {output, "
    Method Docs:
    {doc_string}

    "}?;
    }

    let output_type_description = type_description(method.output_ty(), metadata.types(), true)
        .expect("No Type Description")
        .indent(4);

    writedoc! {output, "
    Output Type:
    {output_type_description}

    "}?;

    let accepts_input_values = method.inputs().len() > 0;

    let trailing_args = trailing_args.join(" ");

    if trailing_args.is_empty() && accepts_input_values {
        let fields: Vec<(Option<&str>, u32)> = method
            .inputs()
            .map(|f| (Some(f.name.as_str()), f.ty))
            .collect();
        let fields_description =
            fields_description(&fields, method.name(), metadata.types()).indent(4);

        let fields_example =
            fields_composite_example(method.inputs().map(|e| e.ty), metadata.types()).indent(4);

        writedoc! {output, "
        Usage:
            subxt explore api {api_name} {method_name} <INPUT_VALUE>
                make a runtime api request

        The method expects an <INPUT_VALUE> with this shape:
        {fields_description}

        For example you could provide this <INPUT_VALUE>:
        {fields_example}
        "}?;
        return Ok(());
    }

    Ok(())
}

fn available_methods_string(runtime_api_metadata: &RuntimeApiMetadata<'_>) -> String {
    let api_name = runtime_api_metadata.name();
    if runtime_api_metadata.methods().len() == 0 {
        return format!("No <METHOD>'s available for the \"{api_name}\" runtime api.");
    }

    let mut output = format!("Available <METHOD>'s available for the \"{api_name}\" runtime api:");
    let mut strings: Vec<_> = runtime_api_metadata.methods().map(|e| e.name()).collect();
    strings.sort();
    for variant in strings {
        output.push_str("\n    ");
        output.push_str(variant);
    }
    output
}
