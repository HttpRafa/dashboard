use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
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

#[derive(Insertable)]
#[diesel(table_name = schema::accounts)]
pub struct NewAccount<'a> {
    pub id: &'a str,
    pub oidc: &'a str,
    pub name: &'a str,
    pub mail: &'a str,
}
