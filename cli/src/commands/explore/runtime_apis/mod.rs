use clap::{Args, Parser};
use subxt::Metadata;
use subxt_metadata::RuntimeApiMetadata;

use crate::utils::FileOrUrl;

mod methods;

#[derive(Debug, Parser)]
pub struct RuntimeApiOpts {
    pub name: String,
    #[command(flatten)]
    pub subcommand: RuntimeApiSubcommand,
}

#[derive(Debug, Clone, Args)]
pub struct RuntimeApiSubcommand {
    method: Option<String>,
    #[clap(required = false)]
    trailing_args: Vec<String>,
}

pub async fn run<'a>(
    opts: RuntimeApiOpts,
    runtime_api_metadata: RuntimeApiMetadata<'a>,
    metadata: &'a Metadata,
    file_or_url: FileOrUrl,
    output: &mut impl std::io::Write,
) -> color_eyre::Result<()> {
    Ok(())
}
