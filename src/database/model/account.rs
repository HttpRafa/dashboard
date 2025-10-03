use diesel::{Selectable, prelude::Queryable};

use crate::database::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: String,
    pub oidc: String,
    pub name: String,
    pub mail: String,
}
