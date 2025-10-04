use eyre::Result;
use rocket::fs::FileServer;

use crate::route::api::v1;

pub mod api;
pub mod web;

pub enum Page {
    Dashboard,
    Feedback,
    Issues,
    Requests,
    Settings,
}

pub async fn launch_rocket() -> Result<()> {
    rocket::build()
        .mount("/", FileServer::from("public/"))
        .mount("/", web::web_routes())
        .mount("/api/v1", v1::v1_routes())
        .launch()
        .await?;
    Ok(())
}
