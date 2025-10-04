use std::env;

use eyre::Result;
use openidconnect::{
    Client, ClientId, ClientSecret, EmptyAdditionalClaims, EmptyExtraTokenFields, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, IdTokenFields, IssuerUrl, RedirectUrl,
    RevocationErrorResponseType, StandardErrorResponse, StandardTokenIntrospectionResponse,
    StandardTokenResponse,
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreClient, CoreErrorResponseType, CoreGenderClaim,
        CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm,
        CoreProviderMetadata, CoreRevocableToken, CoreTokenType,
    },
    reqwest::{ClientBuilder, redirect::Policy},
};

pub type InnerClient = Client<
    EmptyAdditionalClaims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    StandardTokenResponse<
        IdTokenFields<
            EmptyAdditionalClaims,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
        >,
        CoreTokenType,
    >,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, CoreTokenType>,
    CoreRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

pub struct AuthClient {
    inner: InnerClient,
}

impl AuthClient {
    pub async fn create() -> Result<Self> {
        let client_id = ClientId::new(env::var("OIDC_CLIENT_ID")?);
        let client_secret = ClientSecret::new(env::var("OIDC_CLIENT_SECRET")?);
        let issuer_url = IssuerUrl::new(env::var("OIDC_ISSUER_URL")?)?;
        let redirect_url = RedirectUrl::new(env::var("OIDC_REDIRECT_URL")?)?;

        let http_client = {
            let mut builder = ClientBuilder::new().redirect(Policy::none());

            // Conditionally apply the dangerous setting only in debug builds
            #[cfg(debug_assertions)]
            {
                builder = builder.danger_accept_invalid_certs(true);
            }

            builder.build()?
        };

        let provider_metadata =
            CoreProviderMetadata::discover_async(issuer_url, &http_client).await?;

        let inner =
            CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
                .set_redirect_uri(redirect_url);

        Ok(Self { inner })
    }

    pub fn get_client(&self) -> &InnerClient {
        &self.inner
    }
}
