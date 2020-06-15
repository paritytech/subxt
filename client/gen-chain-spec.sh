#!/bin/sh
NODE_TEMPLATE=../../substrate/target/release/node-template
$NODE_TEMPLATE purge-chain --dev
$NODE_TEMPLATE build-spec --dev > dev-chain.json
rm -rf /tmp/subxt-light-client
