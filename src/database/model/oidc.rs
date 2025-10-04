use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use time::Date;

use crate::database::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::oidc)]
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
