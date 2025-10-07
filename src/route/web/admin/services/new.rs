use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::admin::services::new::NewServiceComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_login},
};

#[get("/admin/services/new")]
pub async fn new(account: Account) -> Markup {
    BaseComponent::build(
        "Services | New",
        Page::Services,
        &account,
        NewServiceComponent::build(),
    )
}

#[get("/admin/services/new", rank = 2)]
pub async fn new_redirect() -> Redirect {
    Redirect::to(uri!(login))
}
