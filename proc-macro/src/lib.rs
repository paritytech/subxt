// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

extern crate proc_macro;

mod generate_types;
mod generate_runtime;

use generate_types::TypeGenerator;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::proc_macro_error;

#[proc_macro]
#[proc_macro_error]
pub fn runtime_types(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim_matches('"');

    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let root_path = std::path::Path::new(&root);
    let path = root_path.join(input);
    let mod_name = path.file_stem().unwrap_or_else(||
        proc_macro_error::abort_call_site!("Expected a file path"));

    generate_runtime::generate_runtime_types(&mod_name.to_string_lossy(), &path).into()
}
