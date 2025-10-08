use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};

use crate::database::schema;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::accounts)]
pub struct Account {
    pub id: i32,
    pub oidc: String,
    pub name: String,
    pub mail: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = schema::accounts)]
pub struct NewAccount<'a> {
    pub oidc: &'a str,
    pub name: &'a str,
    pub mail: &'a str,
    pub admin: bool,
}
