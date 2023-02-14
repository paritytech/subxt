// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::marker::PhantomData;

/// Payload for a runtime API fn.
#[derive(Debug)]
pub struct RuntimeAPIPayload<ReturnTy> {
    func_name: &'static str,
    data: Vec<u8>,
    _validation_hash: Option<[u8; 32]>,
    _marker: PhantomData<ReturnTy>,
}

impl<ReturnTy> RuntimeAPIPayload<ReturnTy> {
    /// Create a new [`RuntimeAPIPayload`] from static data.
    pub fn new(
        func_name: &'static str,
        data: Vec<u8>,
        validation_hash: [u8; 32],
    ) -> Self {
        RuntimeAPIPayload {
            func_name,
            data,
            _validation_hash: Some(validation_hash),
            _marker: PhantomData,
        }
    }

    /// Do not validate this prior to submitting it.
    pub fn unvalidated(self) -> Self {
        Self {
            _validation_hash: None,
            ..self
        }
    }

    /// Returns the function name.
    pub fn func_name(&self) -> &'static str {
        &self.func_name
    }

    /// Returns the parameter data.
    pub fn param_data(&self) -> &[u8] {
        &self.data
    }
}
