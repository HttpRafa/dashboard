use std::{
    env,
    sync::{Arc, Mutex},
};

use chrono::Utc;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SqliteConnection,
    dsl::count_star, sql_query,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use eyre::{Result, eyre};
use openidconnect::{EmptyAdditionalClaims, IdTokenClaims, core::CoreGenderClaim};
use tokio::task::spawn_blocking;
use uuid::Uuid;

use crate::{
    DATABASE_URL_ENVIROMENT,
    auth::SESSION_TTL,
    database::{
        model::{
            account::{Account, NewAccount},
            session::NewSession,
        },
        schema,
    },
};

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

    pub async fn find_or_create_account(
        &self,
        claims: &IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>,
    ) -> Result<Account> {
        let oidc = claims.subject().to_string();
        let Some(name) = claims
            .name()
            .and_then(|name| name.get(None))
            .map(|name| name.to_string())
        else {
            return Err(eyre!("IdP did not provide a valid Name"));
        };
        let Some(mail) = claims.email().map(|email| email.to_string()) else {
            return Err(eyre!("IdP did not provide a valid E-Mail"));
        };

        {
            let oidc = oidc.clone();
            if let Some(account) = run_db(self, move |connection| {
                Ok(schema::accounts::table
                    .filter(schema::accounts::oidc.eq(&oidc))
                    .first::<Account>(connection)
                    .optional()?)
            })
            .await?
            {
                if account.name != name && account.mail != mail {
                    // Data changed
                    return run_db(self, move |connection| {
                        Ok(diesel::update(schema::accounts::table.find(&account.id))
                            .set((
                                schema::accounts::name.eq(&name),
                                schema::accounts::mail.eq(&mail),
                            ))
                            .get_result::<Account>(connection)?)
                    })
                    .await;
                } else {
                    return Ok(account);
                }
            }
        }

        run_db(self, move |connection| {
            let is_first = schema::accounts::table
                .select(count_star())
                .first::<i64>(connection)
                .map(|count| count == 0)?;

            if is_first {
                println!("{:?}", eyre!("First user created! {} is now admin.", &name));
            }

            let account = NewAccount {
                id: &Uuid::new_v4().to_string(),
                oidc: &oidc,
                name: &name,
                mail: &mail,
                // The first every created account is the admin user
                admin: is_first,
            };
            Ok(diesel::insert_into(schema::accounts::table)
                .values(&account)
                .get_result::<Account>(connection)?)
        })
        .await
    }

    pub async fn create_session(&self, account: &Account) -> Result<String> {
        let token = Uuid::new_v4().to_string();
        let expires = Utc::now()
            .checked_add_signed(SESSION_TTL)
            .ok_or(eyre!("Failed to compute expire date"))?
            .naive_utc();

        {
            let id = account.id.clone();
            let token = token.clone();
            run_db(self, move |connection| {
                let session = NewSession {
                    token: &token,
                    account_id: &id,
                    expires,
                };

                diesel::insert_into(schema::sessions::table)
                    .values(&session)
                    .execute(connection)?;

                Ok(())
            })
            .await?;
        }

        Ok(token)
    }
}

pub async fn run_db<T, D>(connection: &Database, function: T) -> Result<D>
where
    T: Send + 'static + Fn(&mut SqliteConnection) -> Result<D>,
    D: Send + 'static,
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
