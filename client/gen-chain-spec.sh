#!/bin/sh
NODE_TEMPLATE=../target/release/test-node
$NODE_TEMPLATE purge-chain --dev
$NODE_TEMPLATE build-spec --dev > dev-chain.json
rm -rf /tmp/subxt-light-client
