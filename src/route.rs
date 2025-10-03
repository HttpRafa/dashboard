use eyre::Result;
use rocket::fs::FileServer;

pub mod web;

pub enum Route {
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
        .launch()
        .await?;
    Ok(())
}
