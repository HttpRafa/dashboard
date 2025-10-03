// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Text,
        oidc -> Text,
        name -> Text,
        mail -> Text,
    }
}
