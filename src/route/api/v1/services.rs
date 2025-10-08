use rocket::{form::Form, http::Status, post, response::Redirect, uri};

use crate::{
    database::model::{account::Account, service::Service},
    route::web::admin::services::rocket_uri_macro_services,
};

#[post("/api/v1/services", data = "<form>")]
pub async fn v1_services(
    account: Account,
    form: Form<Service>,
) -> Result<Redirect, (Status, &'static str)> {
    Ok(Redirect::to(uri!(services)))
}
