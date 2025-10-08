use constant_time_eq::constant_time_eq;
use eyre::eyre;
use openidconnect::{AccessTokenHash, AuthorizationCode, OAuth2TokenResponse, TokenResponse};
use rocket::{
    State, get,
    http::{CookieJar, Status},
    response::Redirect,
    uri,
};

use crate::{
    auth::{SESSION_TOKEN_COOKIE_NAME, client::AuthClient},
    database::connection::Database,
    route::{
        api::v1::auth::login::{OIDC_TOKEN_COOKIE_NAME, build_cookie},
        web::dashboard::rocket_uri_macro_dashboard,
    },
};

#[get("/api/v1/auth/callback?<code>&<state>")]
pub async fn v1_callback(
    code: &str,
    state: &str,
    oidc: &State<AuthClient>,
    database: &State<Database>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, (Status, &'static str)> {
    let Some(token) = jar.get(OIDC_TOKEN_COOKIE_NAME) else {
        return Err((Status::PreconditionRequired, "Missing session token cookie"));
    };
    jar.remove(OIDC_TOKEN_COOKIE_NAME);

    let Some((pkce_verifier, csrf_token, nonce)) = oidc.find_oidc_request(token.value()).await
    else {
        return Err((
            Status::PreconditionRequired,
            "The session token is invalid/expired",
        ));
    };

    if !constant_time_eq(state.as_bytes(), csrf_token.secret().as_bytes()) {
        return Err((Status::BadRequest, "CSRF token mismatch"));
    }

    let request = match oidc
        .get_client()
        .exchange_code(AuthorizationCode::new(code.to_string()))
    {
        Ok(request) => request,
        Err(error) => {
            println!("{:?}", eyre!(error));
            return Err((Status::InternalServerError, "Configuration error"));
        }
    };

    let response = match request
        .set_pkce_verifier(pkce_verifier)
        .request_async(oidc.get_http())
        .await
    {
        Ok(response) => response,
        Err(error) => {
            println!("{:?}", eyre!(error));
            return Err((Status::InternalServerError, "Failed to exchange IdP code"));
        }
    };

    let Some(id_token) = response.id_token() else {
        return Err((
            Status::InternalServerError,
            "IdP did not return an valid ID token",
        ));
    };

    let claims: &openidconnect::IdTokenClaims<
        openidconnect::EmptyAdditionalClaims,
        openidconnect::core::CoreGenderClaim,
    > = match id_token.claims(&oidc.get_client().id_token_verifier(), &nonce) {
        Ok(claims) => claims,
        Err(error) => {
            println!("{:?}", eyre!(error));
            return Err((
                Status::InternalServerError,
                "Failed to get claims from provided ID token",
            ));
        }
    };

    if let Some(expected_token_hash) = claims.access_token_hash() {
        let signing_alg = match id_token.signing_alg() {
            Ok(signing_alg) => signing_alg,
            Err(error) => {
                println!("{:?}", error);
                return Err((
                    Status::InternalServerError,
                    "Failed to verify signature of the ID token",
                ));
            }
        };

        let verifier = oidc.get_client().id_token_verifier();
        let signing_key = match id_token.signing_key(&verifier) {
            Ok(signing_key) => signing_key,
            Err(error) => {
                println!("{:?}", error);
                return Err((
                    Status::InternalServerError,
                    "Failed to verify signature of the ID token",
                ));
            }
        };

        let actual_token_hash =
            match AccessTokenHash::from_token(response.access_token(), signing_alg, signing_key) {
                Ok(actual_token_hash) => actual_token_hash,
                Err(error) => {
                    println!("{:?}", error);
                    return Err((Status::InternalServerError, "Failed to sign auth token"));
                }
            };

        if actual_token_hash != *expected_token_hash {
            return Err((Status::BadRequest, "Access token mismatch"));
        }
    }

    let account = match database.find_or_create_account(claims).await {
        Ok(account) => account,
        Err(error) => {
            println!("{:?}", eyre!(error));
            return Err((
                Status::InternalServerError,
                "Failed to find user/create a new user",
            ));
        }
    };

    let token = match database.create_session(&account).await {
        Ok(token) => token,
        Err(error) => {
            println!("{:?}", eyre!(error));
            return Err((
                Status::InternalServerError,
                "Failed to create new session for user",
            ));
        }
    };
    jar.add(build_cookie(SESSION_TOKEN_COOKIE_NAME, token));

    println!(
        "User {} with e-mail address {} has authenticated successfully",
        account.name, account.mail,
    );

    Ok(Redirect::to(uri!(dashboard)))
}
