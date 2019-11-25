// Copyright 2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implements support for built-in runtime modules.

use parity_scale_codec::Encode;

pub mod balances;
pub mod contracts;
pub mod system;

/// Creates module calls
pub struct Call<T: Encode> {
    pub module: &'static str,
    pub function: &'static str,
    pub args: T,
}

impl<T: Encode> Call<T> {
    pub fn new(module: &'static str, function: &'static str, args: T) -> Self {
        Call { module, function, args }
    }
}
