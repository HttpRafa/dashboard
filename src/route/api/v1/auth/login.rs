use rocket::{get, response::Redirect, uri};

#[get("/auth/login")]
pub async fn login() -> Redirect {
    Redirect::to(uri!("https://google.net"))
}
