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

use crate::{
    test_context,
    TestContext,
};
use frame_metadata::{
    RuntimeMetadataPrefixed,
    RuntimeMetadataV14,
};
use subxt::{
    ClientBuilder,
    DefaultConfig,
    Metadata,
    SubstrateExtrinsicParams,
};

use crate::utils::node_runtime;

type RuntimeApi =
    node_runtime::RuntimeApi<DefaultConfig, SubstrateExtrinsicParams<DefaultConfig>>;

async fn metadata_to_api(metadata: RuntimeMetadataV14, cxt: &TestContext) -> RuntimeApi {
    let prefixed = RuntimeMetadataPrefixed::from(metadata);
    let metadata = Metadata::try_from(prefixed).unwrap();

    ClientBuilder::new()
            .set_url(cxt.node_proc.ws_url().to_string())
            .set_metadata(metadata)
            .build()
            .await
            .unwrap()
            .to_runtime_api::<node_runtime::RuntimeApi<
                DefaultConfig,
                SubstrateExtrinsicParams<DefaultConfig>,
            >>()
}

#[tokio::test]
async fn full_metadata_check() {
    let cxt = test_context().await;
    let api = &cxt.api;

    // Runtime metadata is identical to the metadata used during API generation.
    assert!(api.validate_metadata().is_ok());

    // Modify the metadata
    let mut metadata: RuntimeMetadataV14 =
        api.client.metadata().runtime_metadata().clone();
    metadata.pallets[0].name = "NewPallet".to_string();

    let new_api = metadata_to_api(metadata, &cxt).await;
    assert_eq!(
        new_api
            .validate_metadata()
            .err()
            .expect("Validation should fail for incompatible metadata"),
        ::subxt::MetadataError::IncompatibleMetadata
    );
}
