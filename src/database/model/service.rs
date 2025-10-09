use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};
use rocket::FromForm;
use uuid::Uuid;

use crate::database::schema;

#[derive(Queryable, Selectable, Identifiable, Insertable, FromForm)]
#[diesel(table_name = schema::services)]
pub struct Service {
    #[field(default = Uuid::new_v4())]
    pub id: String,
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
