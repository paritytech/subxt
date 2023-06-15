use std::{
    fmt::Write,
    pin::Pin,
    sync::{Arc, Mutex},
};
use subxt::{
    rpc::{RawValue, RpcClientT, RpcFuture, RpcSubscription},
    OnlineClient, PolkadotConfig,
};

// A dummy RPC client that doesn't actually handle requests properly
// at all, but instead just logs what requests to it were made.
struct MyLoggingClient {
    log: Arc<Mutex<String>>,
}

// We have to implement this fairly low level trait to turn [`MyLoggingClient`]
// into an RPC client that we can make use of in Subxt. Here we just log the requests
// made but don't forward them to any real node, and instead just return nonsense.
impl RpcClientT for MyLoggingClient {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RpcFuture<'a, Box<RawValue>> {
        writeln!(
            self.log.lock().unwrap(),
            "{method}({})",
            params.as_ref().map(|p| p.get()).unwrap_or("[]")
        )
        .unwrap();

        // We've logged the request; just return garbage. Because a boxed future is returned,
        // you're able to run whatever async code you'd need to actually talk to a node.
        let res = RawValue::from_string("[]".to_string()).unwrap();
        Box::pin(std::future::ready(Ok(res)))
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RpcFuture<'a, RpcSubscription> {
        writeln!(
            self.log.lock().unwrap(),
            "{sub}({}) (unsub: {unsub})",
            params.as_ref().map(|p| p.get()).unwrap_or("[]")
        )
        .unwrap();

        // We've logged the request; just return garbage. Because a boxed future is returned,
        // and that will return a boxed Stream impl, you have a bunch of flexibility to build
        // and return whatever type of Stream you see fit.
        let res = RawValue::from_string("[]".to_string()).unwrap();
        let stream = futures::stream::once(async move { Ok(res) });
        let stream: Pin<Box<dyn futures::Stream<Item = _> + Send>> = Box::pin(stream);
        // This subscription does not provide an ID.
        Box::pin(std::future::ready(Ok(RpcSubscription { stream, id: None })))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instantiate our replacement RPC client.
    let log = Arc::default();
    let rpc_client = MyLoggingClient {
        log: Arc::clone(&log),
    };

    // Pass this into our OnlineClient to instantiate it. This will lead to some
    // RPC calls being made to fetch chain details/metadata, which will immediately
    // fail..
    let _ = OnlineClient::<PolkadotConfig>::from_rpc_client(Arc::new(rpc_client)).await;

    // But, we can see that the calls were made via our custom RPC client:
    println!("Log of calls made:\n\n{}", log.lock().unwrap().as_str());
    Ok(())
}
