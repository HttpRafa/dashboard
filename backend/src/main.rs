use rocket::{fs::FileServer, launch, Build, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", FileServer::from("static/"))
}