use crate::utils::{first_paragraph_of_docs, FileOrUrl, Indent};
use clap::{Args, Parser};
use color_eyre::eyre::eyre;
use indoc::formatdoc;
use subxt::Metadata;
use subxt_metadata::RuntimeApiMetadata;

mod methods;

pub async fn run<'a>(
    method: Option<String>,
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
        let docs_string = first_paragraph_of_docs(runtime_api_metadata.docs()).indent(4);
        if !docs_string.is_empty() {
            writeln!(output, "Description:\n{docs_string}\n")?;
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

    let trailing_args = trailing_args.join(" ");

    if trailing_args.is_empty() {}

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
