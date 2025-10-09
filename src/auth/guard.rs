use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use eyre::eyre;
use rocket::{
    Request, State,
    http::Status,
    request::{FromRequest, Outcome},
};

use crate::{
    auth::SESSION_TOKEN_COOKIE_NAME,
    database::{
        connection::{Database, run_db},
        model::account::{Account, AdminAccount},
        schema,
    },
};

#[derive(Debug)]
pub enum AuthError {
    Internal(&'static str),
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Account {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(database) = request.guard::<&State<Database>>().await.succeeded() else {
            println!("{:?}", eyre!("Failed to get database from request guard"));
            return Outcome::Error((
                Status::InternalServerError,
                AuthError::Internal(
                    "Failed to get required internal resources to handle this request",
                ),
            ));
        };

        let Some(token) = request
            .cookies()
            .get(SESSION_TOKEN_COOKIE_NAME)
            .map(|cookie| cookie.value().to_string())
        else {
            return Outcome::Forward(Status::Unauthorized);
        };

        let account = match run_db(database, move |connection| {
            Ok(schema::sessions::table
                .inner_join(schema::accounts::table)
                .filter(schema::sessions::token.eq(&token))
                .filter(schema::sessions::expires.gt(Utc::now().naive_utc()))
                .select(Account::as_select())
                .first(connection)?)
        })
        .await
        {
            Ok(account) => account,
            Err(error) => {
                request.cookies().remove(SESSION_TOKEN_COOKIE_NAME);
                println!("{:?}", eyre!(error));
                return Outcome::Forward(Status::Unauthorized);
            }
        };

        Outcome::Success(account)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAccount {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(account) = request.guard::<Account>().await.succeeded() else {
            return Outcome::Error((Status::InternalServerError, ()));
        };

        if account.is_admin {
            Outcome::Success(AdminAccount { account })
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}
