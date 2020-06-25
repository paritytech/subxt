#!/bin/sh
NODE_TEMPLATE=../target/release/test-node
$NODE_TEMPLATE --chain=dev-chain.json --alice
