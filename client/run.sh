#!/bin/sh
NODE_TEMPLATE=../../substrate/target/release/node-template
$NODE_TEMPLATE --chain=dev-chain.json --alice
