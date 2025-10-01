use rocket::{Build, Rocket, fs::FileServer, launch, routes};
use rocket_dyn_templates::Template;

use crate::route::{
    dashboard::dashboard, feedback::feedback, issues::issues, requests::requests,
    settings::settings,
};

mod route;

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", FileServer::from("public/"))
        .mount(
            "/",
            routes![dashboard, requests, issues, feedback, settings],
        )
        .attach(Template::fairing())
}
