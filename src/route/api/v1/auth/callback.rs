use rocket::{
    get,
    response::{Redirect, status::Unauthorized},
    uri,
};

#[get("/auth/callback")]
pub async fn callback() -> Result<Redirect, Unauthorized<String>> {
    Ok(Redirect::to(uri!("https://google.net")))
}
