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
        expires -> Timestamp,
    }
}

diesel::joinable!(sessions -> accounts (account_id));
