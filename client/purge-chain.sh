#!/bin/sh
NODE_TEMPLATE=../target/release/test-node
$NODE_TEMPLATE purge-chain --chain=dev-chain.json
rm -rf /tmp/subxt-light-client
