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

use std::fmt::Display;

use super::ScaleTypeId;

/// This represents the ID of a type found in the metadata. A scale info type representation can
/// be converted into this, and we get this back directly when decoding types into Values.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct TypeId(u32);

impl TypeId {
    /// Return the u32 ID expected by a PortableRegistry.
    pub(crate) fn id(self) -> u32 {
        self.0
    }
}

impl Display for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ScaleTypeId> for TypeId {
    fn from(id: ScaleTypeId) -> Self {
        TypeId(id.id())
    }
}

impl From<&ScaleTypeId> for TypeId {
    fn from(id: &ScaleTypeId) -> Self {
        TypeId(id.id())
    }
}

impl From<&TypeId> for TypeId {
    fn from(id: &TypeId) -> Self {
        *id
    }
}

impl From<u32> for TypeId {
    fn from(id: u32) -> Self {
        TypeId(id)
    }
}
