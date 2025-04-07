use crate::{node_runtime_unstable, subxt_test, test_context};

#[subxt_test]
async fn call_view_function() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();

    use node_runtime_unstable::proxy::view_functions::check_permissions::{Call, ProxyType};

    // This is one of only two view functions that currently exists at the time of writing.
    let call = Call::System(node_runtime_unstable::system::Call::remark {
        remark: b"hi".to_vec(),
    });
    let proxy_type = ProxyType::Any;
    let view_function_call = node_runtime_unstable::view_functions()
        .proxy()
        .check_permissions(call, proxy_type);

    // Submit the call and get back a result.
    let _is_call_allowed = api
        .view_functions()
        .at_latest()
        .await?
        .call(view_function_call)
        .await?;

    Ok(())
}
