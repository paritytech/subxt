use crate::client::{OfflineClientAtBlockT, OnlineClientAtBlockT};
use crate::config::Config;
// use crate::error::StorageError;

/// Work with storage.
pub struct StorageClient<'atblock, Client, T> {
    client: &'atblock Client,
    marker: std::marker::PhantomData<T>,
}

impl<'atblock, Client, T> StorageClient<'atblock, Client, T> {
    /// Work with storage.
    pub(crate) fn new(client: &'atblock Client) -> Self {
        Self {
            client,
            marker: std::marker::PhantomData,
        }
    }
}

// Things that we can do online with storage.
impl<'atblock, 'client: 'atblock, Client, T> StorageClient<'atblock, Client, T>
where
    T: Config + 'client,
    Client: OnlineClientAtBlockT<'client, T>,
{
}

// Things that we can do offline with storage.
impl<'atblock, 'client: 'atblock, Client, T> StorageClient<'atblock, Client, T>
where
    T: Config + 'client,
    Client: OfflineClientAtBlockT<'client, T>,
{
}

/*
What APis do we want with storage items:

- Get specific keys (and able to get keys in Nmaps)
- Iterate keys under some key (same sort of key as above allowed)
  - Decode keys and values from this.
- Probably later: https://paritytech.github.io/json-rpc-interface-spec/api/archive_v1_storageDiff.html
*/
