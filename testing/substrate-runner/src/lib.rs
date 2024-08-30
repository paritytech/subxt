// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

mod error;

use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::{self, BufRead, BufReader, Read};
use std::process::{self, Child, Command};

pub use error::Error;

type CowStr = Cow<'static, str>;

pub struct SubstrateNodeBuilder {
    binary_paths: Vec<OsString>,
    custom_flags: HashMap<CowStr, Option<CowStr>>,
}

impl Default for SubstrateNodeBuilder {
    fn default() -> Self {
        SubstrateNodeBuilder::new()
    }
}

impl SubstrateNodeBuilder {
    /// Configure a new Substrate node.
    pub fn new() -> Self {
        SubstrateNodeBuilder {
            binary_paths: vec![],
            custom_flags: Default::default(),
        }
    }

    /// Provide "substrate-node" and "substrate" as binary paths
    pub fn substrate(&mut self) -> &mut Self {
        self.binary_paths = vec!["substrate-node".into(), "substrate".into()];
        self
    }

    /// Provide "polkadot" as binary path.
    pub fn polkadot(&mut self) -> &mut Self {
        self.binary_paths = vec!["polkadot".into()];
        self
    }

    /// Set the path to the `substrate` binary; defaults to "substrate-node"
    /// or "substrate".
    pub fn binary_paths<Paths, S>(&mut self, paths: Paths) -> &mut Self
    where
        Paths: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.binary_paths = paths.into_iter().map(|p| p.into()).collect();
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
    pub fn spawn(mut self) -> Result<SubstrateNode, Error> {
        // Try to spawn the binary at each path, returning the
        // first "ok" or last error that we encountered.
        let mut res = Err(io::Error::new(
            io::ErrorKind::Other,
            "No binary path provided",
        ));

        let path = Command::new("mktemp")
            .arg("-d")
            .output()
            .expect("failed to create base dir");
        let path = String::from_utf8(path.stdout).expect("bad path");
        let mut bin_path = OsString::new();
        for binary_path in &self.binary_paths {
            self.custom_flags
                .insert("base-path".into(), Some(path.clone().into()));

            res = SubstrateNodeBuilder::try_spawn(binary_path, &self.custom_flags);
            if res.is_ok() {
                bin_path.clone_from(binary_path);
                break;
            }
        }

        let mut proc = match res {
            Ok(proc) => proc,
            Err(e) => return Err(Error::Io(e)),
        };

        // Wait for RPC port to be logged (it's logged to stderr).
        let stderr = proc.stderr.take().unwrap();
        let (ws_port, p2p_address, p2p_port) = try_find_substrate_port_from_output(stderr);

        let ws_port = ws_port.ok_or(Error::CouldNotExtractPort)?;
        let p2p_address = p2p_address.ok_or(Error::CouldNotExtractP2pAddress)?;
        let p2p_port = p2p_port.ok_or(Error::CouldNotExtractP2pPort)?;

        Ok(SubstrateNode {
            binary_path: bin_path,
            custom_flags: self.custom_flags,
            proc,
            ws_port,
            p2p_address,
            p2p_port,
            base_path: path,
        })
    }

    // Attempt to spawn a binary with the path/flags given.
    fn try_spawn(
        binary_path: &OsString,
        custom_flags: &HashMap<CowStr, Option<CowStr>>,
    ) -> Result<Child, std::io::Error> {
        let mut cmd = Command::new(binary_path);

        cmd.env("RUST_LOG", "info,libp2p_tcp=debug")
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .arg("--dev")
            .arg("--port=0");

        for (key, val) in custom_flags {
            let arg = match val {
                Some(val) => format!("--{key}={val}"),
                None => format!("--{key}"),
            };
            cmd.arg(arg);
        }

        cmd.spawn()
    }
}

pub struct SubstrateNode {
    binary_path: OsString,
    custom_flags: HashMap<CowStr, Option<CowStr>>,
    proc: process::Child,
    ws_port: u16,
    p2p_address: String,
    p2p_port: u32,
    base_path: String,
}

impl SubstrateNode {
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

    /// Return the libp2p address of the running node.
    pub fn p2p_address(&self) -> String {
        self.p2p_address.clone()
    }

    /// Return the libp2p port of the running node.
    pub fn p2p_port(&self) -> u32 {
        self.p2p_port
    }

    /// Kill the process.
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.proc.kill()
    }

    /// restart the node, handing back an object which, when dropped, will stop it.
    pub fn restart(&mut self) -> Result<(), std::io::Error> {
        let res: Result<(), io::Error> = self.kill();

        match res {
            Ok(_) => (),
            Err(e) => {
                self.cleanup();
                return Err(e);
            }
        }

        let proc = self.try_spawn()?;

        self.proc = proc;
        // Wait for RPC port to be logged (it's logged to stderr).

        Ok(())
    }

    // Attempt to spawn a binary with the path/flags given.
    fn try_spawn(&mut self) -> Result<Child, std::io::Error> {
        let mut cmd = Command::new(&self.binary_path);

        cmd.env("RUST_LOG", "info,libp2p_tcp=debug")
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .arg("--dev");

        for (key, val) in &self.custom_flags {
            let arg = match val {
                Some(val) => format!("--{key}={val}"),
                None => format!("--{key}"),
            };
            cmd.arg(arg);
        }

        cmd.arg(format!("--rpc-port={}", self.ws_port));
        cmd.arg(format!("--port={}", self.p2p_port));
        cmd.spawn()
    }

    fn cleanup(&self) {
        let _ = Command::new("rm")
            .args(["-rf", &self.base_path])
            .output()
            .expect("success");
    }
}

impl Drop for SubstrateNode {
    fn drop(&mut self) {
        let _ = self.kill();
        self.cleanup()
    }
}

// Consume a stderr reader from a spawned substrate command and
// locate the port number that is logged out to it.
fn try_find_substrate_port_from_output(
    r: impl Read + Send + 'static,
) -> (Option<u16>, Option<String>, Option<u32>) {
    let mut port = None;
    let mut p2p_address = None;
    let mut p2p_port = None;

    for line in BufReader::new(r).lines().take(50) {
        let line = line.expect("failed to obtain next line from stdout for port discovery");

        // Parse the port lines
        let line_port = line
            // oldest message:
            .rsplit_once("Listening for new connections on 127.0.0.1:")
            // slightly newer message:
            .or_else(|| line.rsplit_once("Running JSON-RPC WS server: addr=127.0.0.1:"))
            // newest message (jsonrpsee merging http and ws servers):
            .or_else(|| line.rsplit_once("Running JSON-RPC server: addr=127.0.0.1:"))
            .map(|(_, port_str)| port_str);

        if let Some(ports) = line_port {
            // If more than one rpc server is started the log will capture multiple ports
            // such as `addr=127.0.0.1:9944,[::1]:9944`
            let port_str: String = ports.chars().take_while(|c| c.is_numeric()).collect();

            // expect to have a number here (the chars after '127.0.0.1:') and parse them into a u16.
            let port_num = port_str
                .parse()
                .unwrap_or_else(|_| panic!("valid port expected for log line, got '{port_str}'"));
            port = Some(port_num);
        }

        // Parse the p2p address line
        let line_address = line
            .rsplit_once("Local node identity is: ")
            .map(|(_, address_str)| address_str);

        if let Some(line_address) = line_address {
            let address = line_address.trim_end_matches(|b: char| b.is_ascii_whitespace());
            p2p_address = Some(address.into());
        }

        // Parse the p2p port line (present in debug logs)
        let p2p_port_line = line
            .rsplit_once("New listen address: /ip4/127.0.0.1/tcp/")
            .map(|(_, address_str)| address_str);

        if let Some(line_port) = p2p_port_line {
            // trim non-numeric chars from the end of the port part of the line.
            let port_str = line_port.trim_end_matches(|b: char| !b.is_ascii_digit());

            // expect to have a number here (the chars after '127.0.0.1:') and parse them into a u16.
            let port_num = port_str
                .parse()
                .unwrap_or_else(|_| panic!("valid port expected for log line, got '{port_str}'"));
            p2p_port = Some(port_num);
        }
    }

    (port, p2p_address, p2p_port)
}
