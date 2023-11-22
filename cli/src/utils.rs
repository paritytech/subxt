// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Args;
use color_eyre::eyre;
use heck::ToUpperCamelCase;
use scale_info::form::PortableForm;
use scale_info::PortableRegistry;
use scale_typegen_description::{format_type_description, type_description};

use std::fmt::Display;
use std::str::FromStr;
use std::{fs, io::Read, path::PathBuf};

use scale_value::Value;
use subxt_codegen::fetch_metadata::{fetch_metadata_from_url, MetadataVersion, Url};

// pub mod type_description;
// pub mod type_example;

/// The source of the metadata.
#[derive(Debug, Args, Clone)]
pub struct FileOrUrl {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(long, value_parser)]
    pub url: Option<Url>,
    /// The path to the encoded metadata file.
    #[clap(long, value_parser)]
    pub file: Option<PathBuf>,
    /// Specify the metadata version.
    ///
    ///  - "latest": Use the latest stable version available.
    ///  - "unstable": Use the unstable metadata, if present.
    ///  - a number: Use a specific metadata version.
    ///
    /// Defaults to asking for the latest stable metadata version.
    #[clap(long)]
    pub version: Option<MetadataVersion>,
}

impl FileOrUrl {
    /// Fetch the metadata bytes.
    pub async fn fetch(&self) -> color_eyre::Result<Vec<u8>> {
        match (&self.file, &self.url, self.version) {
            // Can't provide both --file and --url
            (Some(_), Some(_), _) => {
                eyre::bail!("specify one of `--url` or `--file` but not both")
            }
            // Load from --file path
            (Some(path), None, None) => {
                let mut file = fs::File::open(path)?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                Ok(bytes)
            }
            // Cannot load the metadata from the file and specify a version to fetch.
            (Some(_), None, Some(_)) => {
                // Note: we could provide the ability to convert between metadata versions
                // but that would be involved because we'd need to convert
                // from each metadata to the latest one and from the
                // latest one to each metadata version. For now, disable the conversion.
                eyre::bail!("`--file` is incompatible with `--version`")
            }
            // Fetch from --url
            (None, Some(uri), version) => {
                Ok(fetch_metadata_from_url(uri.clone(), version.unwrap_or_default()).await?)
            }
            // Default if neither is provided; fetch from local url
            (None, None, version) => {
                let url = Url::parse("ws://localhost:9944").expect("Valid URL; qed");
                Ok(fetch_metadata_from_url(url, version.unwrap_or_default()).await?)
            }
        }
    }
}

/// creates an example value for each of the fields and
/// packages all of them into one unnamed composite value.
pub fn fields_composite_example(
    fields: &[scale_info::Field<PortableForm>],
    types: &PortableRegistry,
) -> Value {
    let examples: Vec<Value> = fields
        .iter()
        .map(|e| type_example(e.ty.id, types))
        .collect();
    Value::unnamed_composite(examples)
}

/// Returns a field description that is already formatted.
pub fn fields_description(
    fields: &[scale_info::Field<PortableForm>],
    name: &str,
    types: &PortableRegistry,
) -> String {
    if fields.is_empty() {
        return format!("Zero Sized Type, no fields.");
    }
    let all_named = fields.iter().all(|f| f.name.is_some());

    let fields = fields
        .iter()
        .map(|field| {
            let field_description =
                type_description(field.ty.id, types, false).expect("No Description.");
            if all_named {
                let field_name = field.name.as_ref().unwrap();
                format!("{field_name}: {field_description}")
            } else {
                format!("{field_description}")
            }
        })
        .collect::<Vec<String>>()
        .join(",");

    let name = name.to_upper_camel_case();
    let end_result = if all_named {
        format!("struct {name} {{{fields}}}")
    } else {
        format!("struct {name} ({fields})")
    };
    // end_result
    format_type_description(&end_result)
}

pub fn format_scale_value<T>(value: &Value<T>) -> String {
    scale_typegen_description::format_type_description(&value.to_string())
}

pub fn type_example(type_id: u32, types: &PortableRegistry) -> Value {
    scale_typegen_description::scale_value_from_seed(type_id, types, time_based_seed()).expect("")
}

fn time_based_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("We should always live in the future.")
        .subsec_millis() as u64
}

pub fn first_paragraph_of_docs(docs: &[String]) -> String {
    // take at most the first paragraph of documentation, such that it does not get too long.
    let docs_str = docs
        .iter()
        .map(|e| e.trim())
        .take_while(|e| !e.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    docs_str
}

pub trait Indent: ToString {
    fn indent(&self, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        self.to_string()
            .lines()
            .map(|line| format!("{indent_str}{line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<T: Display> Indent for T {}

impl FromStr for FileOrUrl {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = std::path::Path::new(s);
        if path.exists() {
            Ok(FileOrUrl {
                url: None,
                file: Some(PathBuf::from(s)),
                version: None,
            })
        } else {
            Url::parse(s)
                .map_err(|_| "no path or uri could be crated")
                .map(|uri| FileOrUrl {
                    url: Some(uri),
                    file: None,
                    version: None,
                })
        }
    }
}
