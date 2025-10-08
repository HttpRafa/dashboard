use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::issues::IssuesComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_v1_login},
};

#[get("/issues")]
pub async fn issues(account: Account) -> Markup {
    BaseComponent::build(
        "Dashboard | Issues",
        Page::Issues,
        &account,
        IssuesComponent::build(),
    )
}

#[get("/issues", rank = 2)]
pub async fn issues_redirect() -> Redirect {
    Redirect::to(uri!(v1_login))
}
