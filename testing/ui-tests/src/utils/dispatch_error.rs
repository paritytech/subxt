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

/// This type has TypeInfo compatible with the (old version of) the DispatchError
/// type (or at least, is defined enough to allow the subxt macro to generate
/// proper code if this exists in the metadata type registry).
pub enum DispatchError {}
impl scale_info::TypeInfo for DispatchError {
    type Identity = Self;
    fn type_info() -> scale_info::Type {
        scale_info::Type::builder()
            .path(scale_info::Path::new("DispatchError", "sp_runtime"))
            .variant(scale_info::build::Variants::new()
                .variant("Module", |builder| {
                    builder
                        .fields(
                            scale_info::build::FieldsBuilder::<scale_info::build::NamedFields>::default()
                                .field(|b| {
                                    b.name("error").ty::<u8>()
                                })
                                .field(|b| {
                                    b.name("index").ty::<u8>()
                                })
                        )
                        .index(0)
                }))
    }
}