// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

// TODO: Re-enable these once V16 is stable in Substrate nightlies,
// and test-runtime is updated to pull in V16 metadata by default.
/*
use crate::{subxt_test, test_context};
use test_runtime::node_runtime_unstable;

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

#[subxt_test]
async fn call_view_function_dynamically() -> Result<(), subxt::Error> {
    let ctx = test_context().await;
    let api = ctx.client();
    let metadata = api.metadata();

    let query_id = metadata
        .pallet_by_name("Proxy")
        .unwrap()
        .view_function_by_name("check_permissions")
        .unwrap()
        .query_id();

    use scale_value::value;

    let view_function_call = subxt::dynamic::view_function_call(
        *query_id,
        vec![value!(System(remark(b"hi".to_vec()))), value!(Any())],
    );

    // Submit the call and get back a result.
    let _is_call_allowed = api
        .view_functions()
        .at_latest()
        .await?
        .call(view_function_call)
        .await?;

    Ok(())
}
*/
