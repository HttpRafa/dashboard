use eyre::Result;

use crate::{database::connection::Database, route::launch_rocket};

mod component;
mod database;
mod route;

pub const DATABASE_URL_ENVIROMENT: &str = "DATABASE_URL";

#[rocket::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    let database = Database::establish()?;

    launch_rocket().await?;
    Ok(())
}
