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

impl From<&str> for Source {
    fn from(s: &str) -> Self {
        match s {
            s if (s.starts_with("ws://")
                | s.starts_with("wss://")
                | s.starts_with("http://")
                | s.starts_with("https://"))
                && s.parse::<Uri>().is_ok() =>
            {
                Source::Url(s.parse().unwrap())
            }
            p if Path::new(p).exists() => Source::File(PathBuf::from(p)),
            _ => panic!("Invalid source: {s}"),
        }
    }
}

#[cfg(test)]
mod test_source {
    use super::*;

    #[test]
    fn test_from_str() {
        const TEST_HTTP: &str = "http://foo/bar";
        assert_eq!(
            Source::Url(TEST_HTTP.parse().unwrap()),
            Source::from(TEST_HTTP)
        );
        const TEST_HTTPS: &str = "https://foo/bar";
        assert_eq!(
            Source::Url(TEST_HTTPS.parse().unwrap()),
            Source::from(TEST_HTTPS)
        );
        const TEST_WS: &str = "ws://foo/bar";
        assert_eq!(Source::Url(TEST_WS.parse().unwrap()), Source::from(TEST_WS));
        const TEST_WSS: &str = "wss://foo/bar";
        assert_eq!(
            Source::Url(TEST_WSS.parse().unwrap()),
            Source::from(TEST_WSS)
        );

        const TEST_FILE: &str = "Cargo.toml"; // subxt/cli/Cargo.toml
        assert_eq!(
            Source::File(PathBuf::from(TEST_FILE)),
            Source::from(TEST_FILE)
        );
    }
}
