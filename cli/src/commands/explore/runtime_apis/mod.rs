use crate::utils::{
    create_client, fields_composite_example, fields_description, first_paragraph_of_docs,
    parse_string_into_scale_value, FileOrUrl, Indent, SyntaxHighlight,
};

use color_eyre::{
    eyre::{bail, eyre},
    owo_colors::OwoColorize,
};

use indoc::{formatdoc, writedoc};
use scale_typegen_description::type_description;
use scale_value::Value;
use subxt::{
    ext::{scale_decode::DecodeAsType, scale_encode::EncodeAsType},
    Metadata,
};
use subxt_metadata::RuntimeApiMetadata;

/// Runs for a specified runtime API trait.
/// Cases to consider:
/// ```norun
/// method is:
///   None => Show pallet docs + available methods
///   Some (invalid) => Show Error + available methods
///   Some (valid)   => Show method docs + output type description
///                       exectute is:
///                         false => Show input type description + Example Value
///                         true  => validate (trailing args + build node connection)
///                           validation is:  
///                             Err => Show Error
///                             Ok  => Make a runtime api call witht the provided args.
///                               response is:
///                                 Err => Show Error
///                                 Ok  => Show the result
/// ```
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
        let methods = methods_to_string(&runtime_api_metadata);
        formatdoc! {"
        Usage:
            subxt explore api {api_name} <METHOD>
                explore a specific runtime api method
        
        {methods}
        "}
    };

    // If method is None: Show pallet docs + available methods
    let Some(method_name) = method else {
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

    // If method is invalid: Show Error + available methods
    let Some(method) = runtime_api_metadata
        .methods()
        .find(|e| e.name().eq_ignore_ascii_case(&method_name))
    else {
        return Err(eyre!(
            "\"{method_name}\" method not found for \"{method_name}\" runtime api!\n\n{}",
            usage()
        ));
    };
    // redeclare to not use the wrong capitalization of the input from here on:
    let method_name = method.name();

    // Method is valid. Show method docs + output type description
    let doc_string = first_paragraph_of_docs(method.docs()).indent(4);
    if !doc_string.is_empty() {
        writedoc! {output, "
        Description:
        {doc_string}
        
        "}?;
    }

    let input_value_placeholder = "<INPUT_VALUE>".blue();

    // Output type description
    let input_values = || {
        if method.inputs().len() == 0 {
            return format!("The method does not require an {input_value_placeholder}");
        }

        let fields: Vec<(Option<&str>, u32)> = method
            .inputs()
            .map(|f| (Some(f.name.as_str()), f.ty))
            .collect();
        let fields_description =
            fields_description(&fields, method.name(), metadata.types()).indent(4);

        let fields_example =
            fields_composite_example(method.inputs().map(|e| e.ty), metadata.types())
                .indent(4)
                .highlight();

        formatdoc! {"
        The method expects an {input_value_placeholder} with this shape:
        {fields_description}
    
        For example you could provide this {input_value_placeholder}:
        {fields_example}"}
    };

    let execute_usage = || {
        let output = type_description(method.output_ty(), metadata.types(), true)
            .expect("No Type Description")
            .indent(4)
            .highlight();
        let input = input_values();
        formatdoc! {"
        Usage:
            subxt explore api {api_name} {method_name} --execute {input_value_placeholder}
                make a runtime api request

        The Output of this method has the following shape:
        {output}

        {input}"}
    };

    writeln!(output, "{}", execute_usage())?;
    if !execute {
        return Ok(());
    }

    if trailing_args.len() != method.inputs().len() {
        bail!("The number of trailing arguments you provided after the `execute` flag does not match the expected number of inputs!\n{}", execute_usage());
    }

    // encode each provided input as bytes of the correct type:
    let args_data: Vec<Value> = method
        .inputs()
        .zip(trailing_args.iter())
        .map(|(ty, arg)| {
            let value = parse_string_into_scale_value(arg)?;
            let value_str = value.indent(4);
            // convert to bytes:
            writedoc! {output, "
            
            You submitted the following {input_value_placeholder}:
            {value_str}
            "}?;
            // encode, then decode. This ensures that the scale value is of the correct shape for the param:
            let bytes = value.encode_as_type(ty.ty, metadata.types())?;
            let value = Value::decode_as_type(&mut &bytes[..], ty.ty, metadata.types())?
                .map_context(|_| ());
            Ok(value)
        })
        .collect::<color_eyre::Result<Vec<Value>>>()?;

    let method_call = subxt::dynamic::runtime_api_call(api_name, method.name(), args_data);
    let client = create_client(&file_or_url).await?;
    let output_value = client
        .runtime_api()
        .at_latest()
        .await?
        .call(method_call)
        .await?;

    let output_value = output_value.to_value()?.to_string().highlight();
    writedoc! {output, "
    
    Returned value:
        {output_value}
    "}?;
    Ok(())
}

fn methods_to_string(runtime_api_metadata: &RuntimeApiMetadata<'_>) -> String {
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
