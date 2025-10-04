use std::{
    env,
    sync::{Arc, Mutex},
};

use diesel::{Connection, RunQueryDsl, SqliteConnection, sql_query};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use eyre::Result;
use tokio::task::spawn_blocking;

use crate::DATABASE_URL_ENVIROMENT;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct Database {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    pub fn establish() -> Result<Self> {
        let url = env::var(DATABASE_URL_ENVIROMENT)?;

        // Run migrations
        Self::run_migrations(&url)?;

        // Establish a connection to the sqlite database
        let connection = Arc::new(Mutex::new(SqliteConnection::establish(&url)?));
        Ok(Self { connection })
    }

    fn run_migrations(url: &str) -> Result<()> {
        // Establish a connection to the sqlite database (this will create a new one, if it does
        // not exist, and exit if there is an error).
        let mut connection = SqliteConnection::establish(url)?;

        // Run the migrations after successfully establishing a connection
        // Disable Foreign Key Checks during migration
        // Scoped to a connection.
        sql_query("PRAGMA foreign_keys = OFF")
            .execute(&mut connection)
            .expect("Failed to disable Foreign Key Checks during migrations");

        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
        Ok(())
    }
}

pub async fn run_db<T: Send + 'static, D: Send + 'static>(
    connection: &Database,
    function: T,
) -> Result<D>
where
    T: Fn(&mut SqliteConnection) -> Result<D>,
{
    let connection = connection.connection.clone();
    spawn_blocking(move || {
        function(
            &mut connection
                .lock()
                .expect("Failed to lock database connection"),
        )
    })
    .await?
}
