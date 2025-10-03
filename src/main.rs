use eyre::Result;

use crate::route::launch_rocket;

mod component;
mod database;
mod route;

#[rocket::main]
async fn main() -> Result<()> {
    launch_rocket().await?;
    Ok(())
}
