// Copyright (c) 2019-2022 Parity Technologies Limited
// This file is part of subxt.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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
