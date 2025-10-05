use rocket::{Route, routes};

use crate::route::api::v1::auth::{callback::callback, login::login};

pub mod auth;

pub fn v1_routes() -> Vec<Route> {
    routes![login, callback,]
}
