use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::admin::services::ServicesComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_login},
};

pub mod new;

#[get("/admin/services")]
pub async fn services(account: Account) -> Markup {
    BaseComponent::build(
        "Dashboard | Services",
        Page::Services,
        &account,
        ServicesComponent::build(),
    )
}

#[get("/admin/services", rank = 2)]
pub async fn services_redirect() -> Redirect {
    Redirect::to(uri!(login))
}
