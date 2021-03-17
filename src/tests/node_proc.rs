// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
    Client,
    ClientBuilder,
    Runtime,
};
use sp_keyring::AccountKeyring;
use std::{
    ffi::{
        OsStr,
        OsString,
    },
    net::TcpListener,
    ops::Range,
    process,
    thread,
    time,
};

/// Spawn a local substrate node for testing subxt.
pub struct TestNodeProcess<R: Runtime> {
    proc: process::Child,
    client: Client<R>,
}

impl<R> Drop for TestNodeProcess<R>
where
    R: Runtime,
{
    fn drop(&mut self) {
        let _ = self.kill();
    }
}

impl<R> TestNodeProcess<R>
where
    R: Runtime,
{
    /// Construct a builder for spawning a test node process.
    pub fn build<S>(program: S) -> TestNodeProcessBuilder
    where
        S: AsRef<OsStr> + Clone,
    {
        TestNodeProcessBuilder::new(program)
    }

    /// Attempt to kill the running substrate process.
    pub fn kill(&mut self) -> Result<(), String> {
        log::info!("Killing contracts node process {}", self.proc.id());
        if let Err(err) = self.proc.kill() {
            let err = format!(
                "Error killing contracts node process {}: {}",
                self.proc.id(),
                err
            );
            log::error!("{}", err);
            return Err(err.into())
        }
        Ok(())
    }

    /// Returns the subxt client connected to the running node.
    pub fn client(&self) -> &Client<R> {
        &self.client
    }
}

/// Construct a test node process.
pub struct TestNodeProcessBuilder {
    node_path: OsString,
    authority: Option<AccountKeyring>,
    scan_port_range: Option<PortRange>
}

impl TestNodeProcessBuilder {
    pub fn new<P>(node_path: P) -> TestNodeProcessBuilder
    where
        P: AsRef<OsStr>,
    {
        Self {
            node_path: node_path.as_ref().into(),
            authority: None,
            scan_port_range: None,
        }
    }

    /// Set the authority dev account for a node in validator mode e.g. --alice.
    pub fn with_authority(&mut self, account: AccountKeyring) -> &mut Self {
        self.authority = Some(account);
        self
    }

    /// Enable port scanning to scan for open ports in the given range.
    /// Allows multiple node instances to run at once so tests can run in parallel.
    pub fn scan_for_open_ports(&mut self, port_range: PortRange) -> &mut Self {
        self.scan_port_range = Some(port_range);
        self
    }

    /// Spawn the substrate node at the given path, and wait for rpc to be initialized.
    pub async fn spawn<R>(&self) -> Result<TestNodeProcess<R>, String>
    where
        R: Runtime,
    {
        let mut cmd = process::Command::new(&self.node_path);
        cmd.env("RUST_LOG", "error").arg("--dev").arg("--tmp");

        if let Some(authority) = self.authority {
            let authority = format!("{:?}", authority);
            let arg = format!("--{}", authority.as_str().to_lowercase());
            cmd.arg(arg);
        }

        if let Some(ref port_range) = self.scan_port_range {
            let (p2p_port, http_port, ws_port) = port_range.next_open()
                .ok_or("No available ports in the given port range".to_owned())?;

            cmd.arg(format!("--port={}", p2p_port));
            cmd.arg(format!("--rpc-port={}", http_port));
            cmd.arg(format!("--ws-port={}", ws_port));
        }

        let mut proc = cmd.spawn().map_err(|e| {
            format!(
                "Error spawning substrate node '{}': {}",
                self.node_path.to_string_lossy(),
                e
            )
        })?;
        // wait for rpc to be initialized
        const MAX_ATTEMPTS: u32 = 10;
        let mut attempts = 1;
        let client = loop {
            thread::sleep(time::Duration::from_secs(1));
            log::info!(
                "Connecting to contracts enabled node, attempt {}/{}",
                attempts,
                MAX_ATTEMPTS
            );
            let result = ClientBuilder::<R>::new().build().await;
            if let Ok(client) = result {
                break Ok(client)
            }
            if attempts < MAX_ATTEMPTS {
                attempts += 1;
                continue
            }
            if let Err(err) = result {
                break Err(err)
            }
        };
        match client {
            Ok(client) => Ok(TestNodeProcess { proc, client }),
            Err(err) => {
                let err = format!(
                    "Failed to connect to node rpc after {} attempts: {}",
                    attempts, err
                );
                log::error!("{}", err);
                proc.kill().map_err(|e| {
                    format!("Error killing substrate process '{}': {}", proc.id(), e)
                })?;
                Err(err.into())
            }
        }
    }
}

#[derive(Debug)]
pub struct PortRange(Range<u16>);

impl PortRange {
    /// Construct a new port range.
    pub fn new(start: u16, end: u16) -> Self {
        Self (start..end)
    }

    /// Returns the next set of 3 open ports in the port range.
    ///
    /// Returns None if there are not 3 open ports available in the range.
    pub fn next_open(&self) -> Option<(u16, u16, u16)> {
        let mut ports = Vec::new();
        for port in self.0.clone() {
            match TcpListener::bind(("127.0.0.1", port)) {
                Ok(_) => {
                    ports.push(port);
                    if ports.len() == 3 {
                        return Some((ports[0], ports[1], ports[2]))
                    }
                },
                Err(_) => continue,
            }
        }
        return None
    }
}

impl Default for PortRange {
    fn default() -> Self {
        Self::new(9900, 10100)
    }
}
