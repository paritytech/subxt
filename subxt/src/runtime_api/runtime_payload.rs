// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use std::{
    borrow::Cow,
    marker::PhantomData,
};

use codec::{
    Decode,
    Encode,
};
use scale_value::Composite;

use crate::{
    dynamic::DecodedValueThunk,
    metadata::DecodeWithMetadata,
    Error,
    Metadata,
};

/// Payload for calling into a runtime API function.
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

/// RuntimeApiPayload
pub trait RuntimeApiPayload {
    /// The return type into which the result of the call is interpreted.
    type Target: DecodeWithMetadata;

    // TODO: Could do with some lifetimes.
    /// The function name of the runtime API.
    fn func_name(&self) -> String;

    /// Encode arguments to the provided output.
    fn encode_args_to(&self, metadata: &Metadata, out: &mut Vec<u8>)
        -> Result<(), Error>;

    /// Encode arguments and return the output. This is a convenience
    /// wrapper around [`RuntimeApiPayload::encode_params_to`].
    fn encode_args(&self, metadata: &Metadata) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        self.encode_args_to(metadata, &mut v)?;
        Ok(v)
    }
}

/// StaticRuntimeApiPayload
pub struct StaticRuntimeApiPayload<ArgData, ReturnTy> {
    func_name: &'static str,
    data: ArgData,
    _marker: PhantomData<ReturnTy>,
}

// impl<ArgData, ReturnTy> RuntimeApiPayload for StaticRuntimeApiPayload<ArgData, ReturnTy>
// where
//     ArgData: Encode,
//     ReturnTy: Decode,
// {
//     type Target = ReturnTy;

//     fn func_name(&self) -> String {
//         self.func_name.into()
//     }

//     fn encode_args_to(
//         &self,
//         _metadata: &Metadata,
//         out: &mut Vec<u8>,
//     ) -> Result<(), Error> {
//         self.data.encode_to(out);
//         Ok(())
//     }
// }

/// DynamicRuntimeApiPayload
pub struct DynamicRuntimeApiPayload {
    func_name: &'static str,
    fields: Composite<()>,
}

// impl<'a, ReturnTy> DynamicRuntimeApiPayload<'a, ReturnTy> {
// pub fn into_value(self) -> Value<()> {

// }
// }

/// Construct a dynamic runtime API call.
pub fn dynamic(
    func_name: &'static str,
    fields: impl Into<Composite<()>>,
) -> DynamicRuntimeApiPayload {
    DynamicRuntimeApiPayload {
        func_name: func_name.into(),
        fields: fields.into(),
    }
}

impl RuntimeApiPayload for DynamicRuntimeApiPayload {
    type Target = DecodedValueThunk;

    fn encode_args_to(
        &self,
        metadata: &Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), Error> {
        let args = match &self.fields {
            // TODO: Composite::Named WIP.
            Composite::Named(_) => panic!("Composite::Named unsupported yet."),
            Composite::Unnamed(args) => args,
        };

        let fn_metadata = metadata.runtime_fn(&self.func_name)?;
        let param_ty = fn_metadata.params_ty_ids();

        if param_ty.len() != args.len() {
            return Err(Error::Other(
                "Provided different number of params than expected".into(),
            ))
        }

        for (value, ty) in args.iter().zip(param_ty) {
            scale_value::scale::encode_as_type(value, *ty, metadata.types(), out)?;
        }
        Ok(())
    }

    fn func_name(&self) -> String {
        self.func_name.to_owned().into()
    }
}
