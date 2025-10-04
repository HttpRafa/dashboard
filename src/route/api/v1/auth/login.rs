use openidconnect::{CsrfToken, Nonce, PkceCodeChallenge, Scope, core::CoreAuthenticationFlow};
use rocket::{
    State, get,
    http::{Cookie, CookieJar, SameSite, private::cookie::CookieBuilder},
    response::Redirect,
};

use crate::{auth::client::AuthClient, database::connection::Database};

#[get("/auth/login")]
pub async fn login(
    oidc: &State<AuthClient>,
    database: &State<Database>,
    jar: &CookieJar<'_>,
) -> Redirect {
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
        .create_oidc_token(database, pkce_verifier, csrf_token, nonce)
        .await
        .expect("Failed to create session token");

    println!("{}", token);

    Redirect::to(auth_url.to_string())
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
