use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        lightclient: { any(feature = "unstable-light-client", feature = "unstable-light-client-long-running") },
        fullclient: { all(not(feature = "unstable-light-client"), not(feature = "unstable-light-client-long-running")) },
    }
}
