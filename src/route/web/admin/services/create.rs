use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::admin::services::create::CreateServiceComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_v1_login},
};

#[get("/admin/services/create")]
pub async fn create(account: Account) -> Markup {
    BaseComponent::build(
        "Services | Create",
        Page::Services,
        &account,
        CreateServiceComponent::build(),
    )
}

#[get("/admin/services/create", rank = 2)]
pub async fn create_redirect() -> Redirect {
    Redirect::to(uri!(v1_login))
}
