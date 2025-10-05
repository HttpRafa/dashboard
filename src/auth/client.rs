use std::{env, time::Duration};

use eyre::Result;
use moka::future::Cache;
use openidconnect::{
    Client, ClientId, ClientSecret, CsrfToken, EmptyAdditionalClaims, EmptyExtraTokenFields,
    EndpointMaybeSet, EndpointNotSet, EndpointSet, IdTokenFields, IssuerUrl, Nonce,
    PkceCodeVerifier, RedirectUrl, RevocationErrorResponseType, StandardErrorResponse,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreClient, CoreErrorResponseType, CoreGenderClaim,
        CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm,
        CoreProviderMetadata, CoreRevocableToken, CoreTokenType,
    },
    reqwest::{self, ClientBuilder, redirect::Policy},
};
use uuid::Uuid;

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

const OIDC_CACHE_TTL: Duration = Duration::from_secs(60 * 5);

pub struct AuthRequest {
    pkce_verifier: PkceCodeVerifier,
    csrf_token: CsrfToken,
    nonce: Nonce,
}

pub struct AuthClient {
    inner: InnerClient,
    http: reqwest::Client,

    cache: Cache<String, AuthRequest>,
}

impl AuthClient {
    pub async fn create() -> Result<Self> {
        let client_id = ClientId::new(env::var("OIDC_CLIENT_ID")?);
        let client_secret = ClientSecret::new(env::var("OIDC_CLIENT_SECRET")?);
        let issuer_url = IssuerUrl::new(env::var("OIDC_ISSUER_URL")?)?;
        let redirect_url = RedirectUrl::new(env::var("OIDC_REDIRECT_URL")?)?;

        let http = {
            let mut builder = ClientBuilder::new().redirect(Policy::none());

            // Conditionally apply the dangerous setting only in debug builds
            #[cfg(debug_assertions)]
            {
                builder = builder.danger_accept_invalid_certs(true);
            }

            builder.build()?
        };

        let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, &http).await?;

        let inner =
            CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
                .set_redirect_uri(redirect_url);

        let cache = Cache::builder()
            .max_capacity(5_000)
            .time_to_live(OIDC_CACHE_TTL)
            .build();

        Ok(Self { inner, http, cache })
    }

    pub async fn create_oidc_request(
        &self,
        pkce_verifier: PkceCodeVerifier,
        csrf_token: CsrfToken,
        nonce: Nonce,
    ) -> String {
        let token = Uuid::new_v4().to_string();

        let request = AuthRequest {
            pkce_verifier,
            csrf_token,
            nonce,
        };
        self.cache.insert(token.clone(), request).await;

        token
    }

    pub async fn find_oidc_request(
        &self,
        token: &str,
    ) -> Option<(PkceCodeVerifier, CsrfToken, Nonce)> {
        let request = self.cache.remove(token).await?;
        Some((request.pkce_verifier, request.csrf_token, request.nonce))
    }

    pub fn get_client(&self) -> &InnerClient {
        &self.inner
    }

    pub fn get_http(&self) -> &reqwest::Client {
        &self.http
    }
}

impl Clone for AuthRequest {
    fn clone(&self) -> Self {
        Self {
            pkce_verifier: PkceCodeVerifier::new(self.pkce_verifier.secret().clone()),
            csrf_token: self.csrf_token.clone(),
            nonce: self.nonce.clone(),
        }
    }
}
