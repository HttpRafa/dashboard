use diesel::{
    Selectable,
    prelude::{Identifiable, Queryable},
};

use crate::database::schema;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::accounts)]
pub struct Account {
    pub id: String,
    pub oidc: String,
    pub name: String,
    pub mail: String,
}
