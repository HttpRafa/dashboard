use openidconnect::{CsrfToken, Nonce, PkceCodeChallenge, Scope, core::CoreAuthenticationFlow};
use rocket::{
    State, get,
    http::{Cookie, CookieJar, SameSite, Status, private::cookie::CookieBuilder},
    response::Redirect,
};

use crate::auth::client::AuthClient;

pub const OIDC_TOKEN_COOKIE_NAME: &str = "oidc_token";

#[get("/api/v1/auth/login")]
pub async fn login(
    oidc: &State<AuthClient>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, (Status, &'static str)> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = oidc
        .get_client()
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let token = oidc
        .create_oidc_request(pkce_verifier, csrf_token, nonce)
        .await;
    jar.add(build_cookie(OIDC_TOKEN_COOKIE_NAME, token));

    Ok(Redirect::to(auth_url.to_string()))
}

fn build_cookie(key: &str, value: String) -> CookieBuilder<'_> {
    let mut builder = Cookie::build((key, value));
    builder = builder.same_site(SameSite::Lax).http_only(true);

    // Conditionally apply the secure setting only in release builds
    #[cfg(not(debug_assertions))]
    {
        builder = builder.secure(true);
    }

    builder
}
