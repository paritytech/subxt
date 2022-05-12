// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
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
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! For performing runtime updates.

use crate::{
    rpc::{
        Rpc,
        RuntimeVersion,
    },
    BasicError,
    Config,
    Metadata,
};
use parking_lot::RwLock;
use std::sync::Arc;

/// Client wrapper for performing runtime updates.
pub struct UpdateClient<T: Config> {
    rpc: Rpc<T>,
    metadata: Arc<RwLock<Metadata>>,
    runtime_version: Arc<RwLock<RuntimeVersion>>,
}

impl<T: Config> UpdateClient<T> {
    /// Create a new [`UpdateClient`].
    pub fn new(
        rpc: Rpc<T>,
        metadata: Arc<RwLock<Metadata>>,
        runtime_version: Arc<RwLock<RuntimeVersion>>,
    ) -> Self {
        Self {
            rpc,
            metadata,
            runtime_version,
        }
    }

    /// Performs runtime updates indefinitely unless encountering an error.
    ///
    /// *Note:* This should be called from a dedicated background task.
    pub async fn perform_runtime_updates(&self) -> Result<(), BasicError> {
        // Obtain an update subscription to further detect changes in the runtime version of the node.
        let mut update_subscription = self.rpc.subscribe_runtime_version().await?;

        while let Some(update_runtime_version) = update_subscription.next().await {
            // The Runtime Version obtained via subscription.
            let update_runtime_version = update_runtime_version?;

            // To ensure there are no races between:
            // - starting the subxt::Client (fetching runtime version / metadata)
            // - subscribing to the runtime updates
            // the node provides its runtime version immediately after subscribing.
            //
            // In those cases, set the Runtime Version on the client if and only if
            // the provided runtime version is different than what the client currently
            // has stored.
            {
                // The Runtime Version of the client, as set during building the client
                // or during updates.
                let runtime_version = self.runtime_version.read();
                if runtime_version.spec_version == update_runtime_version.spec_version {
                    tracing::debug!(
                        "Runtime update not performed for spec_version={}, client has spec_version={}",
                        update_runtime_version.spec_version, runtime_version.spec_version
                    );
                    continue
                }
            }

            // Update the RuntimeVersion first.
            {
                let mut runtime_version = self.runtime_version.write();
                // Update both the `RuntimeVersion` and `Metadata` of the client.
                tracing::info!(
                    "Performing runtime update from {} to {}",
                    runtime_version.spec_version,
                    update_runtime_version.spec_version,
                );
                *runtime_version = update_runtime_version;
            }

            // Fetch the new metadata of the runtime node.
            let update_metadata = self.rpc.metadata().await?;
            tracing::debug!("Performing metadata update");
            let mut metadata = self.metadata.write();
            *metadata = update_metadata;
            tracing::debug!("Runtime update completed");
        }

        Ok(())
    }
}
