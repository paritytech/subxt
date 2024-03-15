// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use serde_json::Value;
use std::borrow::Cow;

/// Something went wrong building chain config.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum ChainConfigError {
    /// The provided chain spec is the wrong shape.
    #[error("Invalid chain spec format")]
    InvalidSpecFormat,
}

/// Configuration to connect to a chain.
pub struct ChainConfig<'a> {
    // The chain spec to use.
    chain_spec: Cow<'a, str>,
}

impl<'a> From<&'a str> for ChainConfig<'a> {
    fn from(chain_spec: &'a str) -> Self {
        ChainConfig::chain_spec(chain_spec)
    }
}

impl<'a> From<String> for ChainConfig<'a> {
    fn from(chain_spec: String) -> Self {
        ChainConfig::chain_spec(chain_spec)
    }
}

impl<'a> ChainConfig<'a> {
    /// Construct a chain config from a chain spec.
    pub fn chain_spec(chain_spec: impl Into<Cow<'a, str>>) -> Self {
        ChainConfig {
            chain_spec: chain_spec.into(),
        }
    }

    /// Set the bootnodes to the given ones.
    pub fn set_bootnodes<S: AsRef<str>>(
        self,
        bootnodes: impl IntoIterator<Item = S>,
    ) -> Result<Self, ChainConfigError> {
        let mut chain_spec_json: Value = serde_json::from_str(&self.chain_spec)
            .map_err(|_e| ChainConfigError::InvalidSpecFormat)?;

        if let Value::Object(map) = &mut chain_spec_json {
            let bootnodes = bootnodes
                .into_iter()
                .map(|s| Value::String(s.as_ref().to_owned()))
                .collect();

            map.insert("bootNodes".to_string(), Value::Array(bootnodes));
        } else {
            return Err(ChainConfigError::InvalidSpecFormat);
        }

        Ok(ChainConfig {
            chain_spec: Cow::Owned(chain_spec_json.to_string()),
        })
    }

    // Used internally to fetch the chain spec back out.
    pub(crate) fn as_chain_spec(&self) -> &str {
        &self.chain_spec
    }
}
