use rocket::{Route, routes};

use crate::route::api::v1::{
    auth::{callback::v1_callback, login::v1_login},
    services::v1_services,
};

pub mod auth;
pub mod services;

pub fn v1_routes() -> Vec<Route> {
    routes![v1_login, v1_callback, v1_services,]
}
