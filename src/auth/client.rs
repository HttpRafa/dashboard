use std::env;

use diesel::RunQueryDsl;
use eyre::Result;
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
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::database::{
    connection::{Database, run_db},
    model::oidc::NewOidcRequest,
    schema,
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
    http: reqwest::Client,
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

        Ok(Self { inner, http })
    }

    pub async fn create_oidc_token(
        &self,
        database: &Database,
        pkce_verifier: PkceCodeVerifier,
        csrf_token: CsrfToken,
        nonce: Nonce,
    ) -> Result<String> {
        let pkce_verifier = pkce_verifier.into_secret();
        let csrf_token = csrf_token.into_secret();
        let nonce = nonce.secret().to_string();

        let token = Uuid::new_v4().to_string();
        let expires = OffsetDateTime::now_utc().date() + Duration::hours(1);
        let oidc = NewOidcRequest {
            token: token.clone(),
            pkce_verifier,
            csrf_token,
            nonce,
            expires,
        };

        run_db(database, move |connection| {
            diesel::insert_into(schema::oidc::table)
                .values(&oidc)
                .execute(connection)
        })
        .await??;

        Ok(token)
    }

    pub fn get_client(&self) -> &InnerClient {
        &self.inner
    }

    pub fn get_http(&self) -> &reqwest::Client {
        &self.http
    }
}
