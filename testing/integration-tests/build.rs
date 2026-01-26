use cfg_aliases::cfg_aliases;

fn main() {
    // The idea here is that we can run tests by selecting an RPC client and a backend
    // to use and then all applicable tests will be executed against that combination.
    cfg_aliases! {
        // Select the RPC client to use, defaulting to our normal jsonrpsee RPC client if no selection made.
        lightclient_rpc: { feature = "light-client-rpc" },
        reconnecting_rpc: { feature = "reconnecting-rpc" },
        default_rpc: { all(not(feature = "light-client-rpc"), not(feature = "reconnecting-rpc") ) },

        // Select a backend to use, defaulting to the combined backend if no selection made
        legacy_backend: { feature = "legacy-backend" },
        chainhead_backend: { feature = "chainhead-backend" },
        default_backend: { all(not(feature = "legacy-backend"), not(feature = "chainhead-backend")) }
    }
}
