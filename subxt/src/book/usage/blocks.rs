// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/*!
# Blocks

The [blocks API](crate::blocks::BlocksClient) in Subxt provides a way to:

- Access information about specific blocks (see [`crate::blocks::BlocksClient::at()`] and [`crate::blocks::BlocksClient::at_latest()`]).
- Subscribe to [all](crate::blocks::BlocksClient::subscribe_all()), [best](crate::blocks::BlocksClient::subscribe_best()) or [finalized](crate::blocks::BlocksClient::subscribe_finalized()) blocks as they are produced. Prefer to subscribe to finalized blocks unless you know what you're doing.

In either case, you'll end up with [`crate::blocks::Block`]'s, from which you can access the [storage](crate::blocks::Block::storage()), [events](crate::blocks::Block::events()) and [runtime APIs](crate::blocks::Block::runtime_api()) at that block, as well as acquire and iterate through the extrinsics in the block via [`crate::blocks::Block::extrinsics()`].

*/
