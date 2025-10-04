use eyre::Result;

use crate::{auth::client::AuthClient, database::connection::Database, route::launch_rocket};

mod auth;
mod component;
mod database;
mod route;

pub const DATABASE_URL_ENVIROMENT: &str = "DATABASE_URL";

#[rocket::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    let database = Database::establish()?;
    let oidc = AuthClient::create().await?;

    launch_rocket(database, oidc).await?;
    Ok(())
}
