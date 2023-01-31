use std::path::{
    Path,
    PathBuf,
};
use subxt_codegen::utils::Uri;

/// The Source can be either a valid existing `File` or a `Url`
#[derive(Debug, PartialEq, Clone)]
pub enum Source {
    File(PathBuf),
    Url(Uri),
}

impl TryFrom<&str> for Source {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            s if (s.starts_with("ws://")
                | s.starts_with("wss://")
                | s.starts_with("http://")
                | s.starts_with("https://"))
                && s.parse::<Uri>().is_ok() =>
            {
                Ok(Source::Url(s.parse().unwrap()))
            }
            p if Path::new(p).exists() => Ok(Source::File(PathBuf::from(p))),
            _ => Err(format!("File does not exist: {s}")),
        }
    }
}
