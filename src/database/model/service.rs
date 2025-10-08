use diesel::{
    Selectable,
    prelude::{Identifiable, Queryable},
};
use rocket::FromForm;

use crate::database::schema;

#[derive(Queryable, Selectable, Identifiable, FromForm)]
#[diesel(table_name = schema::services)]
pub struct Service {
    #[field(default = -1)]
    pub id: i32,
    pub name: String,
    pub technology: String,
    pub website: String,
    pub instance: String,
    pub icon: String,
    #[field(validate = range(0..))]
    pub icon_height: i32,
    #[field(validate = range(0..))]
    pub icon_width: i32,
}
