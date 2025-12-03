# subxt-historic changelog

This is separate from the Subxt changelog as subxt-historic is currently releasaed separately.

Eventually this project will merge with Subxt and no longer exist, but until then it's being maintained and updated where needed.

## 0.0.7 (2025-12-03)

Expose `OfflineClientAtBlock`, `OfflineClientAtBlockT`, `OnlinelientAtBlock`, `OnlineClientAtBlockT`.

This is so that you can pass the `ClientAtBlock` into functions like so:

```rust
use subxt_historic::config::Config;
use subxt_historic::client::{ ClientAtBlock, OnlineClientAtBlock, OnlineClientAtBlockT };

fn accepts_client_at_block_concrete<T: Config>(client: &ClientAtBlock<OnlineClientAtBlock<'_, T>, T>) {
    // ...
}
fn accepts_client_at_block_generic<'conf, T: Config + 'conf, C: OnlineClientAtBlockT<'conf, T>>(client: &ClientAtBlock<C, T>) {
    // ...
}
```

## 0.0.6 (2025-12-01)

- Add `.metadata()` on `ClientAtBlock` to expose the current metadata at some block.

## 0.0.5 (2025-11-21)

- Rename some fields for consistency.
- Update versions of underlying libraries being used.
- Add `.visit()` methods to extrinsic fields and storage values, and examples of using this to our examples.
