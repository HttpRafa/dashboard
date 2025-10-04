use openidconnect::{CsrfToken, Nonce, PkceCodeChallenge, Scope, core::CoreAuthenticationFlow};
use rocket::{
    State, get,
    http::{Cookie, CookieJar},
    response::Redirect,
};

use crate::auth::client::AuthClient;

#[get("/auth/login")]
pub async fn login(oidc: &State<AuthClient>, jar: &CookieJar<'_>) -> Redirect {
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

    jar.add_private(Cookie::build((
        "pkce_verifier",
        pkce_verifier.into_secret(),
    )));
    jar.add_private(Cookie::build(("csrf_token", csrf_token.into_secret())));
    jar.add_private(Cookie::build(("nonce", nonce.secret().clone())));

    Redirect::to(auth_url.to_string())
}
