use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::requests::RequestsComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_v1_login},
};

#[get("/requests")]
pub async fn requests(account: Account) -> Markup {
    BaseComponent::build(
        "Dashboard | Requests",
        Page::Requests,
        &account,
        RequestsComponent::build(),
    )
}

#[get("/requests", rank = 2)]
pub async fn requests_redirect() -> Redirect {
    Redirect::to(uri!(v1_login))
}
