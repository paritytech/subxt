use clap::Subcommand;

use indoc::writedoc;
use subxt::Metadata;
use subxt_metadata::PalletMetadata;

use crate::utils::{first_paragraph_of_docs, FileOrUrl, Indent};

use self::{
    calls::CallsSubcommand,
    constants::ConstantsSubcommand,
    events::{explore_events, EventsSubcommand},
    storage::StorageSubcommand,
};

use calls::explore_calls;
use constants::explore_constants;
use storage::explore_storage;

mod calls;
mod constants;
mod events;
mod storage;

#[derive(Debug, Clone, Subcommand)]
pub enum PalletSubcommand {
    Calls(CallsSubcommand),
    Constants(ConstantsSubcommand),
    Storage(StorageSubcommand),
    Events(EventsSubcommand),
}

pub async fn run<'a>(
    subcommand: Option<PalletSubcommand>,
    pallet_metadata: PalletMetadata<'a>,
    metadata: &'a Metadata,
    file_or_url: FileOrUrl,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    let pallet_name = pallet_metadata.name();
    let Some(subcommand) = subcommand else {
        let docs_string = first_paragraph_of_docs(pallet_metadata.docs()).indent(4);
        if !docs_string.is_empty() {
            writedoc! {output, "
            Description:
            {docs_string}

            "}?;
        }

        writedoc! {output, "
        Usage:
            subxt explore pallet {pallet_name} calls
                explore the calls that can be made into a pallet
            subxt explore pallet {pallet_name} constants
                explore the constants of a pallet
            subxt explore pallet {pallet_name} storage
                explore the storage values of a pallet
            subxt explore pallet {pallet_name} events
                explore the events of a pallet
        "}?;
        return Ok(());
    };

    match subcommand {
        PalletSubcommand::Calls(command) => {
            explore_calls(command, pallet_metadata, metadata, output)
        }
        PalletSubcommand::Constants(command) => {
            explore_constants(command, pallet_metadata, metadata, output)
        }
        PalletSubcommand::Storage(command) => {
            // if the metadata came from some url, we use that same url to make storage calls against.
            explore_storage(command, pallet_metadata, metadata, file_or_url, output).await
        }
        PalletSubcommand::Events(command) => {
            explore_events(command, pallet_metadata, metadata, output)
        }
    }
}
