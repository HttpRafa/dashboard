// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Text,
        oidc -> Text,
        name -> Text,
        mail -> Text,
    }
}

diesel::table! {
    sessions (token) {
        token -> Text,
        account_id -> Text,
        expires -> Date,
    }
}

diesel::table! {
    oidc (token) {
        token -> Text,
        pkce_verifier -> Text,
        csrf_token -> Text,
        nonce -> Text,
        expires -> Date,
    }
}

diesel::joinable!(sessions -> accounts (account_id));
