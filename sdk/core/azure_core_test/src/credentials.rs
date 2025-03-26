// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Credentials for live and recorded tests.
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    date::OffsetDateTime,
    error::ErrorKind,
};
use azure_identity::{AzurePipelinesCredential, DefaultAzureCredential, TokenCredentialOptions};
use std::{env, sync::Arc, time::Duration};

/// A mock [`TokenCredential`] useful for testing.
#[derive(Clone, Debug, Default)]
pub struct MockCredential;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for MockCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let token: Secret = format!("TEST TOKEN {}", scopes.join(" ")).into();
        let expires_on = OffsetDateTime::now_utc().saturating_add(
            Duration::from_secs(60 * 5).try_into().map_err(|err| {
                azure_core::Error::full(ErrorKind::Other, err, "failed to compute expiration")
            })?,
        );
        Ok(AccessToken { token, expires_on })
    }
}

/// Gets a `TokenCredential` appropriate for the current environment.
///
/// When running in Azure Pipelines, this will return an [`AzurePipelinesCredential`];
/// otherwise, it will return a [`DefaultAzureCredential`].
pub fn from_env(
    options: Option<TokenCredentialOptions>,
) -> azure_core::Result<Arc<dyn TokenCredential>> {
    // cspell:ignore accesstoken azuresubscription
    let tenant_id = env::var("AZURESUBSCRIPTION_TENANT_ID").ok();
    let client_id = env::var("AZURESUBSCRIPTION_CLIENT_ID").ok();
    let connection_id = env::var("AZURESUBSCRIPTION_SERVICE_CONNECTION_ID").ok();
    let access_token = env::var("SYSTEM_ACCESSTOKEN").ok();

    if let (Some(tenant_id), Some(client_id), Some(connection_id), Some(access_token)) =
        (tenant_id, client_id, connection_id, access_token)
    {
        if !tenant_id.is_empty()
            && !client_id.is_empty()
            && !connection_id.is_empty()
            && !access_token.is_empty()
        {
            return Ok(AzurePipelinesCredential::new(
                tenant_id,
                client_id,
                &connection_id,
                access_token,
                options.map(Into::into),
            )? as Arc<dyn TokenCredential>);
        }
    }

    Ok(
        DefaultAzureCredential::with_options(options.unwrap_or_default())?
            as Arc<dyn TokenCredential>,
    )
}
