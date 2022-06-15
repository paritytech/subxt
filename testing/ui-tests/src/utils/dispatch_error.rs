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

use scale_info::{
    build::{
        FieldsBuilder,
        NamedFields,
        UnnamedFields,
        Variants,
    },
    Path,
    Type,
    TypeInfo,
};

/// See the `ModuleErrorType` in `subxt_codegen` for more info on the different DispatchError
/// types that we've encountered. We need the path to match `sp_runtime::DispatchError`, otherwise
/// we could just implement roughly the correct types and derive TypeInfo on them.

/// This type has TypeInfo compatible with the `NamedField` version of the DispatchError.
/// This is the oldest version that subxt supports:
/// `DispatchError::Module { index: u8, error: u8 }`
pub enum NamedFieldDispatchError {}
impl TypeInfo for NamedFieldDispatchError {
    type Identity = Self;
    fn type_info() -> Type {
        Type::builder()
            .path(Path::new("DispatchError", "sp_runtime"))
            .variant(Variants::new().variant("Module", |builder| {
                builder
                    .fields(
                        FieldsBuilder::<NamedFields>::default()
                            .field(|b| b.name("error").ty::<u8>())
                            .field(|b| b.name("index").ty::<u8>()),
                    )
                    .index(0)
            }))
    }
}

/// This type has TypeInfo compatible with the `LegacyError` version of the DispatchError.
/// This is the version wasn't around for long:
/// `DispatchError::Module ( sp_runtime::ModuleError { index: u8, error: u8 } )`
pub enum LegacyDispatchError {}
impl TypeInfo for LegacyDispatchError {
    type Identity = Self;
    fn type_info() -> Type {
        struct ModuleError;
        impl TypeInfo for ModuleError {
            type Identity = Self;
            fn type_info() -> Type {
                Type::builder()
                    .path(Path::new("ModuleError", "sp_runtime"))
                    .composite(
                        FieldsBuilder::<NamedFields>::default()
                            .field(|b| b.name("index").ty::<u8>())
                            .field(|b| b.name("error").ty::<u8>()),
                    )
            }
        }

        Type::builder()
            .path(Path::new("DispatchError", "sp_runtime"))
            .variant(Variants::new().variant("Module", |builder| {
                builder
                    .fields(
                        FieldsBuilder::<UnnamedFields>::default()
                            .field(|b| b.ty::<ModuleError>()),
                    )
                    .index(0)
            }))
    }
}

/// This type has TypeInfo compatible with the `ArrayError` version of the DispatchError.
/// This is the current version:
/// `DispatchError::Module ( sp_runtime::ModuleError { index: u8, error: [u8; 4] } )`
pub enum ArrayDispatchError {}
impl TypeInfo for ArrayDispatchError {
    type Identity = Self;
    fn type_info() -> Type {
        struct ModuleError;
        impl TypeInfo for ModuleError {
            type Identity = Self;
            fn type_info() -> Type {
                Type::builder()
                    .path(Path::new("ModuleError", "sp_runtime"))
                    .composite(
                        FieldsBuilder::<NamedFields>::default()
                            .field(|b| b.name("index").ty::<u8>())
                            .field(|b| b.name("error").ty::<[u8; 4]>()),
                    )
            }
        }

        Type::builder()
            .path(Path::new("DispatchError", "sp_runtime"))
            .variant(Variants::new().variant("Module", |builder| {
                builder
                    .fields(
                        FieldsBuilder::<UnnamedFields>::default()
                            .field(|b| b.ty::<ModuleError>()),
                    )
                    .index(0)
            }))
    }
}
