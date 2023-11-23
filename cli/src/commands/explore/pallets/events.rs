use clap::Args;
use color_eyre::eyre::eyre;
use indoc::{formatdoc, writedoc};
use scale_info::{form::PortableForm, Variant};
use subxt::metadata::{types::PalletMetadata, Metadata};

use crate::utils::{fields_description, first_paragraph_of_docs, Indent};

#[derive(Debug, Clone, Args)]
pub struct EventsSubcommand {
    event: Option<String>,
}

pub fn explore_events(
    command: EventsSubcommand,
    pallet_metadata: PalletMetadata,
    metadata: &Metadata,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name();
    let event_variants = pallet_metadata.event_variants().unwrap_or(&[]);

    let usage = || {
        let events = available_events_string(event_variants, pallet_name);
        formatdoc! {"
        Usage:
            subxt explore pallet {pallet_name} events <EVENT>
                explore a specific event of this pallet
        
        {events}
        "}
    };

    let Some(event_name) = command.event else {
        writeln!(output, "{}", usage())?;
        return Ok(());
    };

    // if specified event is wrong, show user the events to choose from (but this time as an error):
    let Some(event) = event_variants
        .iter()
        .find(|event| event.name.to_lowercase() == event_name.to_lowercase())
    else {
        let err = eyre!(
            "event \"{event_name}\" not found in \"{pallet_name}\" pallet!\n\n{}",
            usage()
        );
        return Err(err);
    };

    let doc_string = first_paragraph_of_docs(&event.docs).indent(4);
    if !doc_string.is_empty() {
        writedoc! {output, "
        Description:
        {doc_string}

        "}?;
    }

    let fields: Vec<(Option<&str>, u32)> = event
        .fields
        .iter()
        .map(|f| (f.name.as_deref(), f.ty.id))
        .collect();
    let type_description = fields_description(&fields, &event.name, metadata.types()).indent(4);
    writedoc!(
        output,
        "
        The event has the following shape:
        {type_description}
        "
    )?;
    Ok(())
}

fn available_events_string(event_variants: &[Variant<PortableForm>], pallet_name: &str) -> String {
    if event_variants.is_empty() {
        return format!("No <EVENTS>'s available in the \"{pallet_name}\" pallet.");
    }
    let mut output = format!("Available <EVENTS>'s in the \"{pallet_name}\" pallet:");
    let mut strings: Vec<_> = event_variants.iter().map(|c| &c.name).collect();
    strings.sort();
    for event in strings {
        output.push_str("\n    ");
        output.push_str(event);
    }
    output
}
