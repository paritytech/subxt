// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Implements support for the frame_sudo module.

use crate::{
    frame::system::{
        System,
        SystemEventsDecoder,
    },
    Encoded,
};
use codec::Encode;
use core::marker::PhantomData;

/// The subset of the `frame_sudo::Trait` that a client must implement.
#[module]
pub trait Sudo: System {}

/// Execute a transaction with sudo permissions.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct SudoCall<'a, T: Sudo> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Encoded transaction.
    pub call: &'a Encoded,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::balances::TransferCall;

    subxt_test!({
        name: test_sudo,
        step: {
            call: SudoCall {
                _runtime: PhantomData,
                call: &client.encode(
                    TransferCall {
                        to: &bob.clone().into(),
                        amount: 10_000,
                    }
                ).unwrap(),
            },
        }
    });
}
