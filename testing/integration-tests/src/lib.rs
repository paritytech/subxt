// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
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
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

#![deny(unused_crate_dependencies)]

#[cfg(test)]
mod codegen;
#[cfg(test)]
mod utils;

#[cfg(test)]
mod client;
#[cfg(test)]
mod events;
#[cfg(test)]
mod frame;
#[cfg(test)]
mod metadata;
#[cfg(test)]
mod storage;

#[cfg(test)]
use test_runtime::node_runtime;
#[cfg(test)]
use utils::*;
