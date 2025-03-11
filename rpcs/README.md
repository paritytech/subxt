# subxt-rpcs

This crate provides an interface for interacting with Substrate nodes via the available RPC methods.

```rust
use subxt_rpcs::{RpcClient, ChainHeadRpcMethods};

// Connect to a local node:
let client = RpcClient::from_url("ws://127.0.0.1:9944").await?;
// Use a set of methods, here the V2 "chainHead" ones:
let methods = ChainHeadRpcMethods::new(client);

// Call some RPC methods (in this case a subscription):
let mut follow_subscription = methods.chainhead_v1_follow(false).await.unwrap();
while let Some(follow_event) = follow_subscription.next().await {
    // do something with events..
}
```