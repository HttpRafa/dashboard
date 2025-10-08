use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Associations, Insertable, Queryable},
};

use crate::database::{model::account, schema};

#[derive(Queryable, Selectable, Associations)]
#[diesel(belongs_to(account::Account))]
#[diesel(table_name = schema::sessions)]
pub struct Session {
    pub token: String,
    pub account_id: String,
    pub expires: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::sessions)]
pub struct NewSession<'a> {
    pub token: &'a str,
    pub account_id: i32,
    pub expires: NaiveDateTime,
}
