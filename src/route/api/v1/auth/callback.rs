use rocket::{
    State, get,
    http::CookieJar,
    response::{Redirect, status::Custom},
    uri,
};

use crate::auth::client::AuthClient;

#[get("/auth/callback?<code>&<state>")]
pub async fn callback(
    code: &str,
    state: &str,
    oidc: &State<AuthClient>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, Custom<String>> {
    /*let pkce_verifier = PkceCodeVerifier::new(
        jar.get_private("pkce_verifier")
            .ok_or(Custom(
                Status::PreconditionFailed,
                "Missing PKCE-Verifier cookie".into(),
            ))?
            .value()
            .into(),
    );
    let csrf_token = jar.get_private("csrf_token").ok_or(Custom(
        Status::PreconditionFailed,
        "Missing CSRF-Token cookie".into(),
    ))?;
    let nonce = Nonce::new(
        jar.get_private("nonce")
            .ok_or(Custom(
                Status::PreconditionFailed,
                "Missing NOnce cookie".into(),
            ))?
            .value()
            .into(),
    );
    jar.remove_private("pkce_verifier");
    jar.remove_private("csrf_token");
    jar.remove_private("nonce");

    if !constant_time_eq(state.as_bytes(), csrf_token.value().as_bytes()) {
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
    }*/

    Ok(Redirect::to(uri!("/")))
}
