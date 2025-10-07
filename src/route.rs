use eyre::Result;
use rocket::fs::FileServer;

use crate::{auth::client::AuthClient, database::connection::Database, route::api::v1};

pub mod api;
pub mod web;

pub enum Page {
    Dashboard,
    Services,
    Feedback,
    Issues,
    Requests,
    Accounts,
    Settings,
}

pub async fn launch_rocket(database: Database, oidc: AuthClient) -> Result<()> {
    rocket::build()
        .mount("/", FileServer::from("public/"))
        .mount("/", web::web_routes())
        .mount("/", v1::v1_routes())
        .manage(database)
        .manage(oidc)
        .launch()
        .await?;
    Ok(())
}
