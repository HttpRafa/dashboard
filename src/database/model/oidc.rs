use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};
use time::Date;

use crate::database::schema;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::oidc)]
#[diesel(primary_key(token))]
pub struct OidcRequest {
    pub token: String,
    pub pkce_verifier: String,
    pub csrf_token: String,
    pub nonce: String,
    pub expires: Date,
}

#[derive(Insertable)]
#[diesel(table_name = schema::oidc)]
pub struct NewOidcRequest {
    pub token: String,
    pub pkce_verifier: String,
    pub csrf_token: String,
    pub nonce: String,
    pub expires: Date,
}
