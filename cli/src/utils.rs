// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use clap::Args;
use color_eyre::eyre::{bail, eyre};
use color_eyre::owo_colors::OwoColorize;
use heck::ToUpperCamelCase;
use scale_info::PortableRegistry;
use scale_typegen_description::{format_type_description, type_description};
use std::fmt::Display;
use std::str::FromStr;
use std::{fs, io::Read, path::PathBuf};
use subxt::{OnlineClient, PolkadotConfig};

use scale_value::Value;
use subxt_codegen::fetch_metadata::{fetch_metadata_from_url, MetadataVersion, Url};

/// The source of the metadata.
#[derive(Debug, Args, Clone)]
pub struct FileOrUrl {
    /// The url of the substrate node to query for metadata for codegen.
    #[clap(long, value_parser)]
    pub url: Option<Url>,
    /// The path to the encoded metadata file.
    #[clap(long, value_parser)]
    pub file: Option<PathOrStdIn>,
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

impl FromStr for FileOrUrl {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(path) = PathOrStdIn::from_str(s) {
            Ok(FileOrUrl {
                url: None,
                file: Some(path),
                version: None,
            })
        } else {
            Url::parse(s)
                .map_err(|_| "Parsing Path or Uri failed.")
                .map(|uri| FileOrUrl {
                    url: Some(uri),
                    file: None,
                    version: None,
                })
        }
    }
}

/// If `--path -` is provided, read bytes for metadata from stdin
const STDIN_PATH_NAME: &str = "-";
#[derive(Debug, Clone)]
pub enum PathOrStdIn {
    Path(PathBuf),
    StdIn,
}

impl FromStr for PathOrStdIn {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s == STDIN_PATH_NAME {
            Ok(PathOrStdIn::StdIn)
        } else {
            let path = std::path::Path::new(s);
            if path.exists() {
                Ok(PathOrStdIn::Path(PathBuf::from(path)))
            } else {
                Err("Path does not exist.")
            }
        }
    }
}

impl FileOrUrl {
    /// Fetch the metadata bytes.
    pub async fn fetch(&self) -> color_eyre::Result<Vec<u8>> {
        match (&self.file, &self.url, self.version) {
            // Can't provide both --file and --url
            (Some(_), Some(_), _) => {
                bail!("specify one of `--url` or `--file` but not both")
            }
            // Load from --file path
            (Some(PathOrStdIn::Path(path)), None, None) => {
                let mut file = fs::File::open(path)?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                Ok(bytes)
            }
            (Some(PathOrStdIn::StdIn), None, None) => {
                let res = std::io::stdin().bytes().collect::<Result<Vec<u8>, _>>();

                match res {
                    Ok(bytes) => Ok(bytes),
                    Err(err) => bail!("reading bytes from stdin (`--file -`) failed: {err}"),
                }
            }
            // Cannot load the metadata from the file and specify a version to fetch.
            (Some(_), None, Some(_)) => {
                // Note: we could provide the ability to convert between metadata versions
                // but that would be involved because we'd need to convert
                // from each metadata to the latest one and from the
                // latest one to each metadata version. For now, disable the conversion.
                bail!("`--file` is incompatible with `--version`")
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
    fields: impl Iterator<Item = u32>,
    types: &PortableRegistry,
) -> Value {
    let examples: Vec<Value> = fields.map(|e| type_example(e, types)).collect();
    Value::unnamed_composite(examples)
}

/// Returns a field description that is already formatted.
pub fn fields_description(
    fields: &[(Option<&str>, u32)],
    name: &str,
    types: &PortableRegistry,
) -> String {
    if fields.is_empty() {
        return "Zero Sized Type, no fields.".to_string();
    }
    let all_named = fields.iter().all(|f| f.0.is_some());

    let fields = fields
        .iter()
        .map(|field| {
            let field_description =
                type_description(field.1, types, false).expect("No Description.");
            if all_named {
                let field_name = field.0.unwrap();
                format!("{field_name}: {field_description}")
            } else {
                field_description.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(",");

    let name = name.to_upper_camel_case();
    let end_result = if all_named {
        format!("{name} {{{fields}}}")
    } else {
        format!("{name} ({fields})")
    };
    // end_result
    format_type_description(&end_result).highlight()
}

pub fn format_scale_value<T>(value: &Value<T>) -> String {
    scale_typegen_description::format_type_description(&value.to_string()).highlight()
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

pub async fn create_client(
    file_or_url: &FileOrUrl,
) -> color_eyre::Result<OnlineClient<PolkadotConfig>> {
    let client = match &file_or_url.url {
        Some(url) => OnlineClient::<PolkadotConfig>::from_url(url).await?,
        None => OnlineClient::<PolkadotConfig>::new().await?,
    };
    Ok(client)
}

pub fn parse_string_into_scale_value(str: &str) -> color_eyre::Result<Value> {
    let value = scale_value::stringify::from_str(str).0.map_err(|err| {
        eyre!(
            "scale_value::stringify::from_str led to a ParseError.\n\ntried parsing: \"{str}\"\n\n{err}",
        )
    })?;
    Ok(value)
}

pub trait SyntaxHighlight {
    fn highlight(&self) -> String;
}

impl<T: AsRef<str>> SyntaxHighlight for T {
    fn highlight(&self) -> String {
        let _e = 323.0;
        let mut output: String = String::new();
        let mut word: String = String::new();

        let mut in_word: Option<InWord> = None;

        for c in self.as_ref().chars() {
            match c {
                '{' | '}' | ',' | '(' | ')' | ':' | '<' | '>' | ' ' | '\n' | '[' | ']' | ';' => {
                    // flush the current word:
                    if let Some(is_word) = in_word {
                        let word = if word == "enum" {
                            word.blue().to_string()
                        } else {
                            is_word.colorize(&word)
                        };
                        output.push_str(&word);
                    }

                    in_word = None;
                    word.clear();
                    // push the symbol itself:
                    output.push(c);
                }
                l => {
                    if in_word.is_none() {
                        in_word = Some(InWord::from_first_char(l))
                    }
                    word.push(l);
                }
            }
        }
        // flush if ending on a word:
        if let Some(word_kind) = in_word {
            output.push_str(&word_kind.colorize(&word));
        }

        return output;

        enum InWord {
            Lower,
            Upper,
            Number,
        }

        impl InWord {
            fn colorize(&self, str: &str) -> String {
                let color = match self {
                    InWord::Lower => (156, 220, 254),
                    InWord::Upper => (78, 201, 176),
                    InWord::Number => (181, 206, 168),
                };
                str.truecolor(color.0, color.1, color.2).to_string()
            }

            fn from_first_char(c: char) -> Self {
                if c.is_numeric() {
                    Self::Number
                } else if c.is_uppercase() {
                    Self::Upper
                } else {
                    Self::Lower
                }
            }
        }
    }
}

pub fn validate_url_security(url: Option<&Url>, allow_insecure: bool) -> color_eyre::Result<()> {
    let Some(url) = url else {
        return Ok(());
    };
    match subxt::utils::url_is_secure(url.as_str()) {
        Ok(is_secure) => {
            if !allow_insecure && !is_secure {
                bail!("URL {url} is not secure!\nIf you are really want to use this URL, try using --allow-insecure (-a)");
            }
        }
        Err(err) => {
            bail!("URL {url} is not valid: {err}")
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::utils::{FileOrUrl, PathOrStdIn};
    use std::str::FromStr;

    #[test]
    fn parsing() {
        assert!(matches!(
            FileOrUrl::from_str("-"),
            Ok(FileOrUrl {
                url: None,
                file: Some(PathOrStdIn::StdIn),
                version: None
            })
        ),);

        assert!(matches!(
            FileOrUrl::from_str("  -  "),
            Ok(FileOrUrl {
                url: None,
                file: Some(PathOrStdIn::StdIn),
                version: None
            })
        ),);

        assert!(matches!(
            FileOrUrl::from_str("./src/main.rs"),
            Ok(FileOrUrl {
                url: None,
                file: Some(PathOrStdIn::Path(_)),
                version: None
            })
        ),);

        assert!(FileOrUrl::from_str("./src/i_dont_exist.rs").is_err());

        assert!(matches!(
            FileOrUrl::from_str("https://github.com/paritytech/subxt"),
            Ok(FileOrUrl {
                url: Some(_),
                file: None,
                version: None
            })
        ));
    }
}
