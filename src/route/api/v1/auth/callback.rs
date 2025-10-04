use constant_time_eq::constant_time_eq;
use openidconnect::{AccessTokenHash, AuthorizationCode, OAuth2TokenResponse, TokenResponse};
use rocket::{
    State, get,
    http::{CookieJar, Status},
    response::{Redirect, status::Custom},
    uri,
};

use crate::{auth::client::AuthClient, database::connection::Database};

#[get("/auth/callback?<code>&<state>")]
pub async fn callback(
    code: &str,
    state: &str,
    oidc: &State<AuthClient>,
    database: &State<Database>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, Custom<String>> {
    let token = jar
        .get("session_token")
        .ok_or(Custom(
            Status::PreconditionFailed,
            "Missing SESSION-Token cookie".into(),
        ))?
        .value()
        .to_string();

    let (pkce_verifier, csrf_token, nonce) = oidc
        .find_oidc_request(database, token)
        .await
        .map_err(|_| Custom(Status::InternalServerError, "Invalid session token".into()))?;

    if !constant_time_eq(state.as_bytes(), csrf_token.secret().as_bytes()) {
        return Err(Custom(Status::BadRequest, "CSRF token mismatch".into()));
    }

    let token_response = oidc
        .get_client()
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .map_err(|_| Custom(Status::InternalServerError, "Token exchange failed".into()))?
        .set_pkce_verifier(pkce_verifier)
        .request_async(oidc.get_http())
        .await
        .map_err(|_| Custom(Status::InternalServerError, "Token exchange failed".into()))?;

    let id_token = token_response.id_token().ok_or(Custom(
        Status::InternalServerError,
        "Server did not return an ID token".into(),
    ))?;
    let claims = id_token
        .claims(&oidc.get_client().id_token_verifier(), &nonce)
        .map_err(|_| Custom(Status::InternalServerError, "Failed to get claims".into()))?;

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            id_token
                .signing_alg()
                .map_err(|_| Custom(Status::InternalServerError, "Token is invalid".into()))?,
            id_token
                .signing_key(&oidc.get_client().id_token_verifier())
                .map_err(|_| Custom(Status::InternalServerError, "Token is invalid".into()))?,
        )
        .map_err(|_| Custom(Status::InternalServerError, "Token is invalid".into()))?;
        if actual_access_token_hash != *expected_access_token_hash {
            return Err(Custom(
                Status::InternalServerError,
                "Invalid access token".into(),
            ));
        }
    }

    println!(
        "User {} with e-mail address {} has authenticated successfully",
        claims.preferred_username().map(|username| username.as_str())
            .unwrap_or("<not provided>"),
        claims
            .email()
            .map(|email| email.as_str())
            .unwrap_or("<not provided>"),
    );

    Ok(Redirect::to(uri!("/")))
}
