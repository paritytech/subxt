#!/bin/sh
NODE_TEMPLATE=../../substrate/target/release/node-template
$NODE_TEMPLATE purge-chain --chain=dev-chain.json
rm -rf /tmp/subxt-light-client
