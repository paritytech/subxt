use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        lightclient: { any(feature = "light-client", feature = "light-client-long-running") },
        fullclient: { all(not(feature = "light-client"), not(feature = "light-client-long-running")) },
        legacy_backend: { not(feature = "chainhead-backend") },
        chainhead_backend: { feature = "chainhead-backend" },
    }
}
