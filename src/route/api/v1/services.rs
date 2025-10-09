use eyre::eyre;
use rocket::{State, form::Form, http::Status, post, response::Redirect, uri};

use crate::{
    database::{
        connection::Database,
        model::{account::AdminAccount, service::Service},
    },
    route::web::admin::services::rocket_uri_macro_services,
};

#[post("/api/v1/services", data = "<form>")]
pub async fn v1_services(
    _account: AdminAccount,
    form: Form<Service>,
    database: &State<Database>,
) -> Result<Redirect, (Status, &'static str)> {
    if let Err(error) = database.create_service(form.into_inner()).await {
        println!("{:?}", eyre!(error));
        return Err((Status::InternalServerError, "Failed to create new service"));
    }
    Ok(Redirect::to(uri!(services)))
}
