use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::dashboard::DashboardComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_v1_login},
};

#[get("/")]
pub async fn dashboard(account: Account) -> Markup {
    BaseComponent::build(
        "Dashboard | Home",
        Page::Dashboard,
        &account,
        DashboardComponent::build(),
    )
}

#[get("/", rank = 2)]
pub async fn dashboard_redirect() -> Redirect {
    Redirect::to(uri!(v1_login))
}
