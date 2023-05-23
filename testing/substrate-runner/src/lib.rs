// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

mod error;

use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::{BufRead, BufReader, Read};
use std::process::{self, Command};

pub use error::Error;

type CowStr = Cow<'static, str>;

pub struct SubstrateNodeBuilder {
    binary_path: OsString,
    custom_flags: HashMap<CowStr, Option<CowStr>>,
}

impl Default for SubstrateNodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SubstrateNodeBuilder {
    /// Configure a new Substrate node.
    pub fn new() -> Self {
        SubstrateNodeBuilder {
            binary_path: "substrate".into(),
            custom_flags: Default::default(),
        }
    }

    /// Set the path to the `substrate` binary; defaults to "substrate".
    pub fn binary_path(&mut self, path: impl Into<OsString>) -> &mut Self {
        self.binary_path = path.into();
        self
    }

    /// Provide a boolean argument like `--alice`
    pub fn arg(&mut self, s: impl Into<CowStr>) -> &mut Self {
        self.custom_flags.insert(s.into(), None);
        self
    }

    /// Provide an argument with a value.
    pub fn arg_val(&mut self, key: impl Into<CowStr>, val: impl Into<CowStr>) -> &mut Self {
        self.custom_flags.insert(key.into(), Some(val.into()));
        self
    }

    /// Spawn the node, handing back an object which, when dropped, will stop it.
    pub fn spawn(self) -> Result<SubstrateNode, Error> {
        let mut cmd = Command::new(self.binary_path);

        cmd.env("RUST_LOG", "info")
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .arg("--dev")
            .arg("--port=0");

        for (key, val) in self.custom_flags {
            let arg = match val {
                Some(val) => format!("--{key}={val}"),
                None => format!("--{key}"),
            };
            cmd.arg(arg);
        }

        let mut proc = cmd.spawn().map_err(Error::Io)?;

        // Wait for RPC port to be logged (it's logged to stderr).
        let stderr = proc.stderr.take().unwrap();
        let ws_port =
            try_find_substrate_port_from_output(stderr).ok_or(Error::CouldNotExtractPort)?;

        Ok(SubstrateNode { proc, ws_port })
    }
}

pub struct SubstrateNode {
    proc: process::Child,
    ws_port: u16,
}

impl SubstrateNodeBuilder {
    /// Configure and spawn a new [`SubstrateNode`].
    pub fn builder() -> SubstrateNodeBuilder {
        SubstrateNodeBuilder::new()
    }

    /// Return the ID of the running process.
    pub fn id(&self) -> u32 {
        self.proc.id()
    }

    /// Return the port that WS connections are accepted on.
    pub fn ws_port(&self) -> u16 {
        self.ws_port
    }

    /// Kill the process.
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.proc.kill()
    }
}

impl Drop for SubstrateNode {
    fn drop(&mut self) {
        let _ = self.kill();
    }
}

// Consume a stderr reader from a spawned substrate command and
// locate the port number that is logged out to it.
fn try_find_substrate_port_from_output(r: impl Read + Send + 'static) -> Option<u16> {
    BufReader::new(r).lines().take(50).find_map(|line| {
        let line = line.expect("failed to obtain next line from stdout for port discovery");

        // does the line contain our port (we expect this specific output from substrate).
        let line_end = line
            // oldest message:
            .rsplit_once("Listening for new connections on 127.0.0.1:")
            // slightly newer message:
            .or_else(|| line.rsplit_once("Running JSON-RPC WS server: addr=127.0.0.1:"))
            // newest message (jsonrpsee merging http and ws servers):
            .or_else(|| line.rsplit_once("Running JSON-RPC server: addr=127.0.0.1:"))
            .map(|(_, port_str)| port_str)?;

        // trim non-numeric chars from the end of the port part of the line.
        let port_str = line_end.trim_end_matches(|b: char| !b.is_ascii_digit());

        // expect to have a number here (the chars after '127.0.0.1:') and parse them into a u16.
        let port_num = port_str
            .parse()
            .unwrap_or_else(|_| panic!("valid port expected for log line, got '{port_str}'"));

        Some(port_num)
    })
}
