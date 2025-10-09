// @generated automatically by Diesel CLI.

/* Users */
diesel::table! {
    accounts (id) {
        id -> Text,
        oidc -> Text,
        name -> Text,
        mail -> Text,
        is_admin -> Bool,
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
diesel::allow_tables_to_appear_in_same_query!(accounts, sessions,);

/* Services */
diesel::table! {
    services (id) {
        id -> Text,
        name -> Text,
        technology -> Text,
        website -> Text,
        instance -> Text,
        icon -> Text,
        icon_height -> Integer,
        icon_width -> Integer,
    }
}
